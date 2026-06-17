use crate::config::ThemeConfig;
use crate::render::tracking::stamp_all_widget_block_ids;
use crate::render::util::escape_attr;

pub fn render_widget_sockets(mut xml: String, config: &ThemeConfig) -> String {
    xml = stamp_all_widget_block_ids(xml);
    for (widget_id, title) in &config.template_pack.widget_titles {
        xml = patch_widget_title_attr(&xml, widget_id, title);
    }
    xml
}

fn patch_widget_title_attr(xml: &str, widget_id: &str, title: &str) -> String {
    let escaped = escape_attr(title);
    for marker in [format!("id='{widget_id}'"), format!("id=\"{widget_id}\"")] {
        let Some(id_pos) = xml.find(&marker) else { continue };
        let slice = &xml[id_pos..];
        if let Some(rel) = slice.find("title='") {
            let val_start = id_pos + rel + 7;
            let Some(rel_end) = xml[val_start..].find('\'') else { continue };
            let mut out = xml.to_string();
            out.replace_range(val_start..val_start + rel_end, &escaped);
            return out;
        }
        if let Some(rel) = slice.find("title=\"") {
            let val_start = id_pos + rel + 7;
            let Some(rel_end) = xml[val_start..].find('"') else { continue };
            let mut out = xml.to_string();
            out.replace_range(val_start..val_start + rel_end, &escaped);
            return out;
        }
    }
    xml.to_string()
}