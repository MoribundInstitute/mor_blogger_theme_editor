//! String surgery on a single `<b:widget>` block's `<b:widget-setting>`
//! entries. Byte-range splices only — never a DOM parse-and-serialize, which
//! would normalize away Blogger's fragile CDATA / `expr:` / entity quirks.
//!
//! All functions take *one widget block* (`<b:widget ...>...</b:widget>`),
//! as sliced by the layout helpers — not a whole theme document.

/// `<b:widget-setting` is a prefix of `<b:widget-settings`; a real setting tag
/// is followed by whitespace or `>`/`/`, the block tag by `s`. Next true
/// setting open at/after `from`.
fn find_setting_open(xml: &str, from: usize) -> Option<usize> {
    const OPEN: &str = "<b:widget-setting";
    let mut start = from;
    while let Some(rel) = xml[start..].find(OPEN) {
        let pos = start + rel;
        match xml.as_bytes().get(pos + OPEN.len()) {
            Some(b's') => start = pos + OPEN.len(), // "<b:widget-settings" block tag
            _ => return Some(pos),
        }
    }
    None
}

/// Read a quoted attribute value out of an opening tag (handles ' or ").
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

/// Escape a value for use as element inner text, the way Blogger writes it.
pub fn escape_text(v: &str) -> String {
    v.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Inverse of [`escape_text`] plus the quote entities Blogger sometimes emits.
pub fn unescape_text(v: &str) -> String {
    v.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

/// Locate the setting with this `name`: (open-tag start, open-tag `>` index).
fn find_setting(widget_xml: &str, name: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(pos) = find_setting_open(widget_xml, from) {
        let tag_close = pos + widget_xml[pos..].find('>')?;
        if attr(&widget_xml[pos..tag_close], "name") == Some(name) {
            return Some((pos, tag_close));
        }
        from = tag_close + 1;
    }
    None
}

/// Current (unescaped) value of `<b:widget-setting name='..'>` in this block.
pub fn get_widget_setting(widget_xml: &str, name: &str) -> Option<String> {
    let (pos, tag_close) = find_setting(widget_xml, name)?;
    if widget_xml[pos..tag_close].ends_with('/') {
        return Some(String::new()); // self-closing, e.g. <b:widget-setting name='x'/>
    }
    let inner_start = tag_close + 1;
    let inner_end = inner_start + widget_xml[inner_start..].find("</b:widget-setting>")?;
    Some(unescape_text(&widget_xml[inner_start..inner_end]))
}

/// Set `name` to `value` inside one widget block, by string surgery:
/// replace the existing setting's inner text; else insert a new entry into the
/// `<b:widget-settings>` block; else create that block right after the
/// `<b:widget ...>` opening tag. Everything else in the block — CDATA,
/// includables, formatting — is byte-for-byte untouched.
pub fn set_widget_setting(widget_xml: &str, name: &str, value: &str) -> Result<String, String> {
    let escaped = escape_text(value);

    if let Some((pos, tag_close)) = find_setting(widget_xml, name) {
        if widget_xml[pos..tag_close].ends_with('/') {
            // Self-closing: expand to a full element carrying the new text.
            return Ok(format!(
                "{}<b:widget-setting name='{name}'>{escaped}</b:widget-setting>{}",
                &widget_xml[..pos],
                &widget_xml[tag_close + 1..]
            ));
        }
        let inner_start = tag_close + 1;
        let inner_end = inner_start
            + widget_xml[inner_start..]
                .find("</b:widget-setting>")
                .ok_or_else(|| format!("unclosed <b:widget-setting name='{name}'>"))?;
        return Ok(format!(
            "{}{escaped}{}",
            &widget_xml[..inner_start],
            &widget_xml[inner_end..]
        ));
    }

    let entry = format!("<b:widget-setting name='{name}'>{escaped}</b:widget-setting>");

    if let Some(pos) = widget_xml.find("<b:widget-settings") {
        let tag_close = pos + widget_xml[pos..]
            .find('>')
            .ok_or("unclosed <b:widget-settings> tag")?;
        if widget_xml[pos..tag_close].ends_with('/') {
            // <b:widget-settings/> — replace with a real block.
            return Ok(format!(
                "{}<b:widget-settings>\n      {entry}\n    </b:widget-settings>{}",
                &widget_xml[..pos],
                &widget_xml[tag_close + 1..]
            ));
        }
        return Ok(format!(
            "{}\n      {entry}{}",
            &widget_xml[..tag_close + 1],
            &widget_xml[tag_close + 1..]
        ));
    }

    // No settings block at all: create one just inside the widget open tag.
    let w_pos = widget_xml
        .find("<b:widget")
        .ok_or("no <b:widget> opening tag in block")?;
    let w_close = w_pos + widget_xml[w_pos..]
        .find('>')
        .ok_or("unclosed <b:widget> opening tag")?;
    if widget_xml[w_pos..w_close].ends_with('/') {
        return Err("self-closing <b:widget/> has no body to hold settings".into());
    }
    Ok(format!(
        "{}\n    <b:widget-settings>\n      {entry}\n    </b:widget-settings>{}",
        &widget_xml[..w_close + 1],
        &widget_xml[w_close + 1..]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDGET: &str = "<b:widget id='Blog1' locked='false' title='Blog Messages' type='Blog'>\n    <b:widget-settings>\n      <b:widget-setting name='showDateHeader'>true</b:widget-setting>\n      <b:widget-setting name='style.textcolor'>#ffffff</b:widget-setting>\n    </b:widget-settings>\n    <b:includable id='main' var='top'><![CDATA[if (a < b && c > d) {}]]></b:includable>\n  </b:widget>";

    #[test]
    fn replaces_existing_setting_and_leaves_cdata_alone() {
        let out = set_widget_setting(WIDGET, "showDateHeader", "false").unwrap();
        assert!(out.contains("<b:widget-setting name='showDateHeader'>false</b:widget-setting>"));
        assert!(out.contains("<![CDATA[if (a < b && c > d) {}]]>")); // untouched
        assert!(out.contains("name='style.textcolor'>#ffffff")); // sibling untouched
        assert_eq!(get_widget_setting(&out, "showDateHeader").unwrap(), "false");
    }

    #[test]
    fn inserts_missing_setting_into_existing_block() {
        let out = set_widget_setting(WIDGET, "showShareButtons", "true").unwrap();
        assert_eq!(get_widget_setting(&out, "showShareButtons").unwrap(), "true");
        // Existing entries still present exactly once.
        assert_eq!(out.matches("name='showDateHeader'").count(), 1);
    }

    #[test]
    fn creates_settings_block_when_widget_has_none() {
        let xml = "<b:widget id='HTML1' type='HTML'>\n    <b:includable id='main'>x</b:includable>\n  </b:widget>";
        let out = set_widget_setting(xml, "content", "hi").unwrap();
        assert!(out.contains("<b:widget-settings>"));
        assert_eq!(get_widget_setting(&out, "content").unwrap(), "hi");
        assert!(out.contains("<b:includable id='main'>x</b:includable>"));
    }

    #[test]
    fn settings_block_tag_is_not_mistaken_for_a_setting() {
        // A widget whose only "<b:widget-setting" hits are the block tags.
        let xml = "<b:widget id='X1' type='HTML'><b:widget-settings>\n</b:widget-settings><b:includable id='main'/></b:widget>";
        assert_eq!(get_widget_setting(xml, "anything"), None);
        let out = set_widget_setting(xml, "content", "v").unwrap();
        assert_eq!(get_widget_setting(&out, "content").unwrap(), "v");
    }

    #[test]
    fn escapes_on_write_and_unescapes_on_read() {
        let out = set_widget_setting(WIDGET, "commentLabel", "a & b <c>").unwrap();
        assert!(out.contains(">a &amp; b &lt;c&gt;</b:widget-setting>"));
        assert_eq!(get_widget_setting(&out, "commentLabel").unwrap(), "a & b <c>");
    }

    #[test]
    fn double_quoted_names_and_self_closing_entries_work() {
        let xml = "<b:widget id='B1' type='Blog'><b:widget-settings><b:widget-setting name=\"showAuthor\"/></b:widget-settings></b:widget>";
        assert_eq!(get_widget_setting(xml, "showAuthor").unwrap(), "");
        let out = set_widget_setting(xml, "showAuthor", "true").unwrap();
        assert_eq!(get_widget_setting(&out, "showAuthor").unwrap(), "true");
        assert_eq!(out.matches("showAuthor").count(), 1);
    }
}
