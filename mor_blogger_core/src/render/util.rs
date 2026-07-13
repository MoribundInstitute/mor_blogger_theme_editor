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

/// Reverses [`escape_attr`]/[`escape_html`] for use in a browser `<style>`.
///
/// The exported Blogger CSS is XML-escaped (`'` → `&#39;`, `"` → `&quot;`, …)
/// because `b:skin` is XML and Blogger decodes the entities before serving.
/// The in-editor preview, however, injects that same CSS straight into an HTML
/// `<style>` element, where character references are NOT decoded — so an escaped
/// `font-family: &#39;IM Fell English&#39;` is invalid and silently dropped.
/// Decode it back to raw CSS for the preview only. `&amp;` is decoded last so a
/// literal `&amp;#39;` isn't mangled into a quote.
pub fn unescape_for_style(s: &str) -> String {
    s.replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// Escapes a value for use inside a double-quoted JS string literal.
/// `/` becomes `\/` so a value containing `</script>` cannot close the tag.
pub(super) fn escape_js_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => out.push_str(r"\\"),
            '"' => out.push_str(r#"\""#),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '/' => out.push_str(r"\/"),
            _ => out.push(c),
        }
    }
    out
}

/// Returns the primary string if it contains text, otherwise returns the fallback.
pub fn first_non_empty<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if primary.trim().is_empty() {
        fallback
    } else {
        primary
    }
}
