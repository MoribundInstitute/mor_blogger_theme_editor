use crate::config::{MenuLink, ThemeConfig};

use super::super::util::{escape_attr, escape_html};

/// Render XML sockets owned by the header template part.
///
/// `template_parts/header.xml` owns the outer `<header class='main-header'>`
/// wrapper and this module only fills safe, config-driven header sockets.
///
/// The first safe socket is:
///
///   {{MAIN_NAV_LINKS}}
///
/// This keeps `template_parts/header.xml` Blogger-shaped while allowing Rust
/// to decide which user-configured nav links actually exist.
pub fn render_header_sockets(xml: String, config: &ThemeConfig) -> String {
    xml.replace("{{MAIN_NAV_LINKS}}", &render_main_nav_links(config))
}

fn render_main_nav_links(config: &ThemeConfig) -> String {
    config
        .menu_links
        .iter()
        .filter_map(render_menu_link)
        .collect::<Vec<_>>()
        .join("\n      ")
}

fn render_menu_link(link: &MenuLink) -> Option<String> {
    let label = link.label.trim();
    let url = link.url.trim();

    if label.is_empty() || url.is_empty() {
        return None;
    }

    // Use double-quoted XML attributes so apostrophes in user-entered URLs
    // do not break Blogger's XML parser. `escape_attr` handles XML-sensitive
    // characters such as ampersands and quotes.
    Some(format!(
        "<a href=\"{url}\">{label}</a>",
        url = escape_attr(url),
        label = escape_html(label),
    ))
}
