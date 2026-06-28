//! Pure buffer helpers for the module workbench's Layout view.
//!
//! A template module is a flat run of `<b:widget>` blocks (Blogger widgets never
//! nest other widgets), so a linear scan is enough to list, reorder, toggle, or
//! drop them — no full XML parse, matching the string-patching style already used
//! in `widget_generator.rs`.

const OPEN: &str = "<b:widget";
const CLOSE: &str = "</b:widget>";

/// `<b:widget` is a prefix of `<b:widget-settings>` / `<b:widgets>`; only a real
/// widget tag is followed by whitespace, `>`, or `/`. Find the next true widget
/// open at or after `from`, or None.
fn find_widget_open(xml: &str, from: usize) -> Option<usize> {
    let mut start = from;
    while let Some(rel) = xml[start..].find(OPEN) {
        let pos = start + rel;
        let after = pos + OPEN.len();
        match xml.as_bytes().get(after) {
            Some(b' ' | b'\t' | b'\n' | b'\r' | b'>' | b'/') => return Some(pos),
            None => return Some(pos),
            _ => start = after, // e.g. "<b:widget-settings" — keep looking
        }
    }
    None
}

/// One `<b:widget>` block surfaced as a Layout card.
#[derive(Clone, PartialEq)]
pub struct WidgetSlot {
    pub id: String,
    pub w_type: String,
    pub title: String,
    pub visible: bool,
    /// Byte range of the whole block (`<b:widget ...>...</b:widget>`).
    start: usize,
    end: usize,
}

/// Read a single quoted attribute value out of an opening tag (handles ' or ").
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    for q in ['\'', '"'] {
        let pat = format!("{name}={q}");
        if let Some(p) = tag.find(&pat) {
            let rest = &tag[p + pat.len()..];
            if let Some(e) = rest.find(q) {
                return Some(&rest[..e]);
            }
        }
    }
    None
}

/// Byte offset just past the opening tag's `>`, clamped to the block end.
fn tag_end(xml: &str, start: usize, end: usize) -> usize {
    xml[start..end].find('>').map(|i| start + i + 1).unwrap_or(end)
}

/// List every top-level widget block in document order.
pub fn parse_slots(xml: &str) -> Vec<WidgetSlot> {
    let mut slots = Vec::new();
    let mut cursor = 0;
    while let Some(start) = find_widget_open(xml, cursor) {
        let after = start + OPEN.len();
        let end = match xml[after..].find(CLOSE) {
            Some(c) => after + c + CLOSE.len(),
            None => xml.len(),
        };
        let tag = &xml[start..tag_end(xml, start, end)];
        slots.push(WidgetSlot {
            id: attr(tag, "id").unwrap_or("").to_string(),
            w_type: attr(tag, "type").unwrap_or("Widget").to_string(),
            title: attr(tag, "title").unwrap_or("").to_string(),
            visible: attr(tag, "visible").map(|v| v != "false").unwrap_or(true),
            start,
            end,
        });
        cursor = end;
    }
    slots
}

/// Move the widget at `from` to position `to`, preserving the text before the
/// first widget and after the last. Inter-widget whitespace is normalised to `\n`.
pub fn reorder(xml: &str, from: usize, to: usize) -> String {
    let slots = parse_slots(xml);
    if from >= slots.len() || to >= slots.len() || from == to {
        return xml.to_string();
    }
    let head = &xml[..slots[0].start];
    let tail = &xml[slots[slots.len() - 1].end..];
    let mut blocks: Vec<&str> = slots.iter().map(|s| &xml[s.start..s.end]).collect();
    let moved = blocks.remove(from);
    blocks.insert(to, moved);
    format!("{head}{}{tail}", blocks.join("\n"))
}

/// Flip a widget's `visible` attribute, inserting one if absent.
pub fn set_visible(xml: &str, index: usize, visible: bool) -> String {
    let slots = parse_slots(xml);
    let Some(s) = slots.get(index) else {
        return xml.to_string();
    };
    let te = tag_end(xml, s.start, s.end);
    let tag = &xml[s.start..te];
    let want = if visible { "true" } else { "false" };
    let new_tag = match attr(tag, "visible") {
        Some(cur) => tag
            .replacen(&format!("visible='{cur}'"), &format!("visible='{want}'"), 1)
            .replacen(&format!("visible=\"{cur}\""), &format!("visible=\"{want}\""), 1),
        None => tag.replacen(OPEN, &format!("{OPEN} visible='{want}'"), 1),
    };
    format!("{}{}{}", &xml[..s.start], new_tag, &xml[te..])
}

/// Remove a widget block from the buffer.
pub fn remove(xml: &str, index: usize) -> String {
    let slots = parse_slots(xml);
    let Some(s) = slots.get(index) else {
        return xml.to_string();
    };
    let mut out = String::with_capacity(xml.len());
    out.push_str(xml[..s.start].trim_end());
    out.push('\n');
    out.push_str(xml[s.end..].trim_start());
    out
}

// ─── Full structural parts ────────────────────────────────────────────────────
//
// A template module is more than its literal `<b:widget>` blocks: sidebars/layouts
// express their content as `<b:section>` containers, `{{SOCKET_*}}` widget slots
// (filled from the config `widget_map` at render), and `{{*_MODULE}}` sub-module
// refs. The Layout view lists every one of these so nothing the module uses is
// hidden.

/// One structural part of a module, in document order.
#[derive(Clone, PartialEq)]
pub enum Part {
    /// A literal `<b:widget>`; `slot` is its index among widgets (for the buffer
    /// edit helpers above, which index by widget order).
    Widget {
        slot: usize,
        id: String,
        w_type: String,
        title: String,
        visible: bool,
    },
    /// A `<b:section>` container.
    Section { id: String },
    /// A `{{SOCKET_*}}` widget slot. `key` is the matching `widget_map` key
    /// (e.g. socket `SIDEBAR_LEFT` → key `sidebar-left`).
    Socket { name: String, key: String },
    /// A `{{*_MODULE}}` / `{{LAYOUT_BLOCKS}}` reference to another module.
    ModuleRef { name: String },
    /// Any other `{{TOKEN}}` (titles, icons, …) — a styling/text field. `path` is
    /// the `data-field-path` of the wrapping element when present; it encodes the
    /// module's structure (e.g. `footer.columns.0.links.1.label`) so fields can be
    /// grouped into a tree instead of a flat dump.
    Field { name: String, path: Option<String> },
}

/// The `data-field-path` of the element enclosing the token at `pos`, if any.
/// Walks back to the nearest `<`, reads that opening tag, and pulls the attribute —
/// works whether the token is the tag's text (`<h4 …>{{T}}`) or in an attribute
/// value (`<a href='{{T}}' …>`).
fn nearest_field_path(xml: &str, pos: usize) -> Option<String> {
    let lt = xml[..pos].rfind('<')?;
    let gt = xml[lt..].find('>').map(|i| lt + i)?;
    attr(&xml[lt..=gt], "data-field-path").map(str::to_string)
}

fn classify_token(name: &str, path: Option<String>) -> Part {
    if let Some(rest) = name.strip_prefix("SOCKET_") {
        Part::Socket {
            name: rest.to_string(),
            key: rest.to_lowercase().replace('_', "-"),
        }
    } else if name == "LAYOUT_BLOCKS" || name.ends_with("_MODULE") {
        Part::ModuleRef {
            name: name.to_string(),
        }
    } else {
        Part::Field {
            name: name.to_string(),
            path,
        }
    }
}

/// Walk the module XML and emit every widget, section, socket and token in order.
/// Tokens inside a widget block are skipped (they belong to that widget).
pub fn parse_parts(xml: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut cursor = 0;
    let mut slot = 0;
    while cursor < xml.len() {
        let nearest = [
            find_widget_open(xml, cursor).map(|i| (i, 'w')),
            xml[cursor..].find("<b:section").map(|i| (cursor + i, 's')),
            xml[cursor..].find("{{").map(|i| (cursor + i, 't')),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(p, _)| *p);
        let Some((pos, kind)) = nearest else { break };
        match kind {
            'w' => {
                let after = pos + OPEN.len();
                let end = match xml[after..].find(CLOSE) {
                    Some(c) => after + c + CLOSE.len(),
                    None => xml.len(),
                };
                let tag = &xml[pos..tag_end(xml, pos, end)];
                parts.push(Part::Widget {
                    slot,
                    id: attr(tag, "id").unwrap_or("").to_string(),
                    w_type: attr(tag, "type").unwrap_or("Widget").to_string(),
                    title: attr(tag, "title").unwrap_or("").to_string(),
                    visible: attr(tag, "visible").map(|v| v != "false").unwrap_or(true),
                });
                slot += 1;
                cursor = end;
            }
            's' => {
                let close = xml[pos..].find('>').map(|i| pos + i + 1).unwrap_or(xml.len());
                let tag = &xml[pos..close];
                parts.push(Part::Section {
                    id: attr(tag, "id").unwrap_or("").to_string(),
                });
                cursor = close;
            }
            _ => {
                // Unmatched `{{` (e.g. truncated edit): bail rather than slice-panic.
                let Some(rel) = xml[pos..].find("}}") else { break };
                let close = pos + rel + 2;
                let name = xml.get(pos + 2..close - 2).unwrap_or("").trim();
                parts.push(classify_token(name, nearest_field_path(xml, pos)));
                cursor = close;
            }
        }
    }
    parts
}

// ─── Field grouping ───────────────────────────────────────────────────────────
//
// Tokens carry a `data-field-path` (e.g. `footer.columns.0.links.1.label`). Grouping
// fields by that path turns a flat wall of chips into the module's actual structure —
// footer columns, link rows, etc. — without any module-specific code.

/// One row of the indented field tree, in pre-order. `tokens` are the chips that
/// terminate at this node (a link maps both its url and label token here).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct FieldGroup {
    pub depth: usize,
    pub label: String,
    pub tokens: Vec<String>,
}

#[derive(Default)]
struct PathNode {
    label: String,
    tokens: Vec<String>,
    children: Vec<PathNode>,
}

impl PathNode {
    fn child_mut(&mut self, label: &str) -> &mut PathNode {
        if let Some(i) = self.children.iter().position(|c| c.label == label) {
            &mut self.children[i]
        } else {
            self.children.push(PathNode {
                label: label.to_string(),
                ..Default::default()
            });
            self.children.last_mut().unwrap()
        }
    }
    fn flatten(&self, depth: usize, out: &mut Vec<FieldGroup>) {
        for c in &self.children {
            out.push(FieldGroup {
                depth,
                label: c.label.clone(),
                tokens: c.tokens.clone(),
            });
            c.flatten(depth + 1, out);
        }
    }
}

/// Prettify a path segment: a numeric index becomes 1-based (`0` → `1`), otherwise
/// it's split on `_`/`-` and title-cased (`site_title` → `Site Title`).
fn pretty_segment(seg: &str) -> String {
    if let Ok(n) = seg.parse::<usize>() {
        return (n + 1).to_string();
    }
    seg.split(['_', '-'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().chain(c).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Group `Field` parts into an indented tree by `data-field-path`. Fields with no
/// path collapse into one unlabelled bucket at the end (header chrome tokens, etc.).
pub fn group_fields(parts: &[Part]) -> Vec<FieldGroup> {
    let mut root = PathNode::default();
    let mut pathless: Vec<String> = Vec::new();
    for p in parts {
        let Part::Field { name, path } = p else { continue };
        match path.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            Some(path) => {
                let mut node = &mut root;
                for seg in path.split('.').filter(|s| !s.is_empty()) {
                    node = node.child_mut(&pretty_segment(seg));
                }
                node.tokens.push(name.clone());
            }
            None => pathless.push(name.clone()),
        }
    }
    let mut out = Vec::new();
    root.flatten(0, &mut out);
    if !pathless.is_empty() {
        out.push(FieldGroup {
            depth: 0,
            label: String::new(),
            tokens: pathless,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = "<b:widget id='A1' type='HTML' title='Alpha' visible='true'>aa</b:widget>";
    const B: &str = "<b:widget id='B1' type='Blog' title='Beta' visible='false'>bb</b:widget>";

    fn doc() -> String {
        format!("<section>\n{A}\n{B}\n</section>")
    }

    #[test]
    fn parses_attrs_and_visibility() {
        let s = parse_slots(&doc());
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].id, "A1");
        assert_eq!(s[0].w_type, "HTML");
        assert!(s[0].visible);
        assert!(!s[1].visible); // visible='false'
    }

    #[test]
    fn reorder_swaps_blocks_keeps_wrapper() {
        let out = reorder(&doc(), 0, 1);
        let s = parse_slots(&out);
        assert_eq!(s[0].id, "B1");
        assert_eq!(s[1].id, "A1");
        assert!(out.starts_with("<section>"));
        assert!(out.trim_end().ends_with("</section>"));
    }

    #[test]
    fn toggle_visibility_flips_attr() {
        let out = set_visible(&doc(), 0, false);
        assert!(parse_slots(&out)[0].visible == false);
    }

    #[test]
    fn remove_drops_one_block() {
        let out = remove(&doc(), 0);
        let s = parse_slots(&out);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].id, "B1");
    }

    #[test]
    fn parse_parts_surfaces_section_and_socket() {
        let sidebar = "<aside><b:section id='sidebar-left' showaddelement='yes'>{{SOCKET_SIDEBAR_LEFT}}</b:section></aside>";
        let parts = parse_parts(sidebar);
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], Part::Section { id } if id == "sidebar-left"));
        assert!(matches!(&parts[1], Part::Socket { key, .. } if key == "sidebar-left"));
    }

    #[test]
    fn parse_parts_classifies_modules_and_fields() {
        let layout = "{{LAYOUT_BLOCKS}}<b:section id='m'>{{MAIN_CONTENT_MODULE}}</b:section>{{LEFT_PANEL_TITLE}}";
        let parts = parse_parts(layout);
        assert!(matches!(&parts[0], Part::ModuleRef { name } if name == "LAYOUT_BLOCKS"));
        assert!(matches!(&parts[1], Part::Section { .. }));
        assert!(matches!(&parts[2], Part::ModuleRef { name } if name == "MAIN_CONTENT_MODULE"));
        assert!(matches!(&parts[3], Part::Field { name, .. } if name == "LEFT_PANEL_TITLE"));
    }

    #[test]
    fn fields_capture_data_field_path() {
        // Text token, attribute token, and a token with no field-path.
        let xml = "<h4 data-field-path='footer.columns.0.heading'>{{COL_H}}</h4>\
                   <a href='{{L_URL}}' data-field-path='footer.columns.0.links.0.label'>{{L_LABEL}}</a>\
                   {{NO_PATH}}";
        let parts = parse_parts(xml);
        let paths: Vec<Option<&str>> = parts
            .iter()
            .filter_map(|p| match p {
                Part::Field { path, .. } => Some(path.as_deref()),
                _ => None,
            })
            .collect();
        assert_eq!(paths[0], Some("footer.columns.0.heading"));
        // Both the href token and the text token resolve to the link's element path.
        assert_eq!(paths[1], Some("footer.columns.0.links.0.label"));
        assert_eq!(paths[2], Some("footer.columns.0.links.0.label"));
        assert_eq!(paths[3], None);
    }

    #[test]
    fn group_fields_builds_indented_tree() {
        let xml = "<h4 data-field-path='footer.columns.0.heading'>{{COL_H}}</h4>\
                   <a href='{{L_URL}}' data-field-path='footer.columns.0.links.0.label'>{{L_LABEL}}</a>\
                   {{NO_PATH}}";
        let g = group_fields(&parse_parts(xml));
        let labels: Vec<&str> = g.iter().map(|n| n.label.as_str()).collect();
        // Pre-order: Footer > Columns > 1 > {Heading, Links > 1 > Label}, then pathless bucket.
        assert_eq!(labels[0], "Footer");
        assert!(labels.contains(&"Columns"));
        assert!(labels.contains(&"1")); // numeric index shown 1-based
        assert!(labels.contains(&"Heading"));
        assert!(labels.contains(&"Links"));
        // url + label of the same link land on one node as two chips.
        let leaf = g.iter().find(|n| n.tokens.iter().any(|t| t == "L_LABEL")).unwrap();
        assert!(leaf.tokens.contains(&"L_URL".to_string()));
        // pathless token in the trailing unlabelled bucket.
        assert!(g.iter().any(|n| n.label.is_empty() && n.tokens == vec!["NO_PATH".to_string()]));
        assert!(g.iter().all(|n| n.depth <= 5));
    }

    #[test]
    fn widget_settings_prefix_is_not_mistaken_for_a_widget() {
        // `<b:widget-settings>` must not be parsed as a widget open tag.
        let xml = "<b:widget id='Blog1' type='Blog'><b:widget-settings><b:widget-setting name='x'>1</b:widget-setting></b:widget-settings></b:widget>";
        let s = parse_slots(xml);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].id, "Blog1");
        let p = parse_parts(xml);
        assert_eq!(p.len(), 1);
        assert!(matches!(&p[0], Part::Widget { id, .. } if id == "Blog1"));
    }

    #[test]
    fn unmatched_open_brace_does_not_panic() {
        assert_eq!(parse_parts("text {{").len(), 0);
        assert_eq!(parse_parts("{{").len(), 0);
    }

    #[test]
    fn parse_parts_skips_tokens_inside_widgets_and_tracks_slot() {
        // The widget's internal title token must NOT surface as a Field.
        let xml = "<b:widget id='Blog1' type='Blog' title='{{BLOG_WIDGET_TITLE}}'>x</b:widget>{{FOOTER_MODULE}}";
        let parts = parse_parts(xml);
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], Part::Widget { slot: 0, id, .. } if id == "Blog1"));
        assert!(matches!(&parts[1], Part::ModuleRef { name } if name == "FOOTER_MODULE"));
    }
}
