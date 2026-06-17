//! src/render/util.rs
//!
//! The Gatekeeper: This module ensures that every piece of data from your
//! Rust config is safe to be injected into the Blogger XML engine.
//! If data isn't escaped correctly, the Blogger parser dies silently.

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

/// Builds Google Fonts `<link>` tags from ThemeConfig typography stacks.
pub(super) fn build_google_fonts_link(stacks: &[&str]) -> String {
    crate::config::fonts::build_google_font_imports(stacks)
}

/// Returns the primary string if it contains text, otherwise returns the fallback.
pub fn first_non_empty<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if primary.trim().is_empty() {
        fallback
    } else {
        primary
    }
}