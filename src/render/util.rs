//! src/render/util.rs
//! 
//! The Gatekeeper: This module ensures that every piece of data from your
//! Rust config is safe to be injected into the Blogger XML engine.
//! If data isn't escaped correctly, the Blogger parser dies silently.

use crate::config::{MenuLink, ThemeConfig};

/// Escapes content for use inside standard HTML elements.
/// Crucial for Blogger: If an apostrophe or bracket slips through, 
/// the entire XML file becomes unparseable.
pub(super) fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escapes content for use inside HTML attributes (like `content="..."`).
/// This is the most dangerous area; a single unescaped quote here 
/// causes the SAXParseException you experienced.
pub(super) fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Safely retrieves a menu link, ensuring we don't return an error
/// if a slot in the UI is empty.
pub(super) fn menu_link_or_empty(config: &ThemeConfig, index: usize) -> MenuLink {
    config
        .menu_links
        .get(index)
        .cloned()
        .unwrap_or_else(|| MenuLink {
            label: String::new(),
            url: String::new(),
        })
}

/// Wraps custom JS snippets in Blogger's specific CDATA wrapper.
/// Without the CDATA/CDATA wrapper, scripts often fail to load.
pub(super) fn render_custom_plugin_scripts(js: &str) -> String {
    if js.trim().is_empty() {
        return String::new();
    }
    format!(
        "<script type=\"text/javascript\">\n//<![CDATA[\n{}\n//]]>\n</script>",
        js
    )
}

/// Dynamically builds the Google Fonts link based on the user's font stack.
pub(super) fn build_google_fonts_link(stacks: &[&str]) -> String {
    let mut families = Vec::new();
    let web_safe = ["Georgia", "Times", "Courier", "monospace", "serif", "sans-serif"];

    for stack in stacks {
        let first_font = stack.split(',').next().unwrap_or("").trim().trim_matches('\'').trim_matches('\"');

        if !first_font.is_empty() && !web_safe.contains(&first_font) {
            let family = first_font.replace(' ', "+");
            if !families.contains(&family) {
                families.push(family);
            }
        }
    }

    if families.is_empty() {
        return String::new();
    }

    // Google Fonts URL format: family=Font+Name:wght@400;700
    let families_param = families.join("&amp;family=");
    format!(
        "<link href=\"https://fonts.googleapis.com/css2?family={}:wght@400;700&amp;display=swap\" rel=\"stylesheet\"/>",
        families_param
    )
}