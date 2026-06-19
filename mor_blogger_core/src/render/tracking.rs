//! Pure HTML tracking hooks for the headless render pipeline.
//! No runtime JavaScript — attributes only.

use super::util::{escape_attr, escape_html};

const WIDGET_OPEN: &str = "<b:widget ";

/// Stamp `data-block-id` on a single widget's outermost `<b:widget>` opening tag.
pub(super) fn stamp_widget_block_id(mut xml: String, widget_id: &str) -> String {
    if let Some(pos) = xml.find(WIDGET_OPEN) {
        xml.insert_str(
            pos + WIDGET_OPEN.len(),
            &format!("data-block-id='{widget_id}' "),
        );
    }
    xml
}

/// Walk assembled XML and stamp any `<b:widget>` missing `data-block-id`.
pub(super) fn stamp_all_widget_block_ids(mut xml: String) -> String {
    let mut scan_from = 0;
    while let Some(rel) = xml[scan_from..].find(WIDGET_OPEN) {
        let open_at = scan_from + rel;
        let body_at = open_at + WIDGET_OPEN.len();
        let Some(rel_end) = xml[body_at..].find('>') else {
            break;
        };
        let tag_end = body_at + rel_end;
        let tag_body = &xml[body_at..tag_end];

        if !tag_body.contains("data-block-id") {
            if let Some(id) = parse_widget_id(tag_body) {
                let tag_len = tag_body.len();
                let stamp = format!("data-block-id='{id}' ");
                let stamp_len = stamp.len();
                xml.insert_str(body_at, &stamp);
                scan_from = body_at + stamp_len + tag_len;
                continue;
            }
        }
        scan_from = tag_end;
    }
    xml
}

fn parse_widget_id(tag_body: &str) -> Option<&str> {
    for needle in ["id='", "id=\""] {
        let key_len = needle.len();
        let start = tag_body.find(needle)? + key_len;
        let quote = needle.as_bytes()[key_len - 1] as char;
        let end = tag_body[start..].find(quote)? + start;
        let id = &tag_body[start..end];
        if !id.is_empty() {
            return Some(id);
        }
    }
    None
}

/// Config field path for a widget's display title.
pub fn widget_title_field_path(widget_id: &str) -> String {
    format!("widget.{widget_id}.title")
}

/// Static HTML h2; editable text lives in the inner span.
pub fn widget_title_h2(widget_id: &str, title: &str) -> String {
    format!(
        r#"<h2 class="title"><span data-field-path="{path}">{title}</span></h2>"#,
        path = widget_title_field_path(widget_id),
        title = escape_html(title),
    )
}

/// Blogger XML h2; wraps dynamic `<data:title/>` for export.
pub fn widget_title_h2_xml(widget_id: &str) -> String {
    format!(
        "<h2 class='title'><span data-field-path='{path}'><data:title/></span></h2>",
        path = widget_title_field_path(widget_id),
    )
}

/// Menu anchor with a field-tagged label span.
pub fn menu_link_anchor(index: usize, url: &str, label: &str) -> String {
    format!(
        r#"<a href="{url}"><span data-field-path="menu_links.{index}.label">{label}</span></a>"#,
        url = escape_attr(url),
        index = index,
        label = escape_html(label),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamps_widget_block_id_from_existing_id() {
        let raw = "<b:widget id='Label1' type='Label'>";
        let out = stamp_widget_block_id(raw.to_string(), "Label1");
        assert!(out.contains("data-block-id='Label1'"));
    }

    #[test]
    fn stamps_all_inline_widgets() {
        let raw = "<b:widget id='Blog1' type='Blog'><b:widget id='Label2' type='Label'>";
        let out = stamp_all_widget_block_ids(raw.to_string());
        assert!(out.contains("data-block-id='Blog1'"));
        assert!(out.contains("data-block-id='Label2'"));
    }

    #[test]
    fn widget_title_h2_carries_field_path() {
        let h2 = widget_title_h2("Label1", "Labels");
        assert!(h2.contains(r#"data-field-path="widget.Label1.title""#));
        assert!(h2.contains(">Labels</span></h2>"));
    }

    #[test]
    fn menu_link_carries_field_path() {
        let link = menu_link_anchor(0, "/home", "Home &amp; Away");
        assert!(link.contains(r#"data-field-path="menu_links.0.label""#));
        assert!(link.contains("Home &amp;amp; Away"));
    }

    #[test]
    fn exported_theme_carries_tracking_hooks() {
        let xml = crate::render::render_theme(&crate::config::ThemeConfig::default());
        assert!(xml.contains("data-block-id='Label1'"));
        assert!(xml.contains("data-block-id='Blog1'"));
        assert!(xml.contains("data-field-path='site.site_title'"));
        assert!(xml.contains("data-field-path='footer.columns.0.heading'"));
        assert!(xml.contains("data-field-path='widget.Label1.title'"));
    }
}
