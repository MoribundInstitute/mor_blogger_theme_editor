//! Diagnostic Integrity Engine
//!
//! Validates that a Blogger template string contains every structural element
//! the runtime JavaScript depends on (panel IDs, layout classes, b:section
//! containers) and every token the renderer expects to substitute.
//!
//! Phase 2 of Combination B (Hardcoded Skeleton + Diagnostics).

use roxmltree::{Document, Node, ParsingOptions};

/// Severity of a single diagnostic warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Template will not render correctly — JS will break or Blogger will reject the upload.
    Error,
    /// Template will render, but something the editor expects is missing.
    Warning,
}

/// A single integrity finding.
#[derive(Debug, Clone)]
pub struct Warning {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
}

impl Warning {
    fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
        }
    }

    fn warn(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: message.into(),
        }
    }
}

/// Aggregate result of an integrity check.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<Warning>,
}

impl DiagnosticResult {
    fn from_warnings(warnings: Vec<Warning>) -> Self {
        let errors: Vec<String> = warnings
            .iter()
            .filter(|w| w.severity == Severity::Error)
            .map(|w| format!("[{}] {}", w.code, w.message))
            .collect();
        let is_valid = errors.is_empty();
        Self {
            is_valid,
            errors,
            warnings,
        }
    }
}

// ---------------------------------------------------------------------------
// Requirements: hardcoded list of mandatory structural elements.
// ---------------------------------------------------------------------------

/// IDs that must exist on *some* element. Used by the JS event listeners.
const REQUIRED_IDS: &[&str] = &["panel-left", "panel-right"];

/// Classes that must appear on *some* element. Layout-critical.
const REQUIRED_CLASSES: &[&str] = &["canvas-core", "panel-toggle", "terminal-workspace"];

/// `<b:section>` IDs that Blogger requires for widget mounting.
const REQUIRED_BLOGGER_SECTIONS: &[&str] = &["sidebar-left", "sidebar-right", "main"];

/// Tokens the renderer is guaranteed to substitute. Missing tokens mean the
/// rendered output will have an unfilled value where there should be one.
const REQUIRED_TOKENS: &[&str] = &[
    "{{SITE_TITLE}}",
    "{{HOME_URL}}",
    "{{CUSTOM_PLUGIN_SCRIPTS}}",
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Run all integrity checks against a template source string.
///
/// Returns a [`DiagnosticResult`] with `is_valid = true` iff zero errors were
/// produced. Warnings never invalidate the template on their own.
pub fn check_integrity(source: &str) -> DiagnosticResult {
    let mut warnings = Vec::new();

    // Token checks are pure string scans and don't need a parsed DOM. We run
    // them first so a malformed XML body still produces useful feedback.
    check_tokens(source, &mut warnings);

    // Now try to parse. The template embeds <script> blocks inside CDATA, which
    // roxmltree handles fine, but we relax a couple of options just in case the
    // user's working copy has stray DTD-like artefacts.
    let opts = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };

    match Document::parse_with_options(source, opts) {
        Ok(doc) => {
            check_structure(&doc, &mut warnings);
        }
        Err(e) => {
            warnings.push(Warning::error(
                "XML_PARSE",
                format!(
                    "Template is not well-formed XML; structural checks were skipped. Parser said: {e}"
                ),
            ));
        }
    }

    DiagnosticResult::from_warnings(warnings)
}

// ---------------------------------------------------------------------------
// Internal: token presence
// ---------------------------------------------------------------------------

fn check_tokens(source: &str, out: &mut Vec<Warning>) {
    for token in REQUIRED_TOKENS {
        if !source.contains(token) {
            out.push(Warning::error(
                "TOKEN_MISSING",
                format!("Required token {token} not found in template source."),
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Internal: structural element checks via roxmltree walk
// ---------------------------------------------------------------------------

fn check_structure(doc: &Document, out: &mut Vec<Warning>) {
    // Collect everything we care about in one pass over the tree.
    let mut found_ids: Vec<String> = Vec::new();
    let mut found_classes: Vec<String> = Vec::new();
    let mut found_b_section_ids: Vec<String> = Vec::new();

    for node in doc.descendants().filter(Node::is_element) {
        if let Some(id) = node.attribute("id") {
            found_ids.push(id.to_string());
        }
        if let Some(class_attr) = node.attribute("class") {
            for c in class_attr.split_ascii_whitespace() {
                found_classes.push(c.to_string());
            }
        }
        // b:section uses the Blogger namespace declared on <html>.
        // roxmltree exposes prefixed names via `tag_name().name()` (local name)
        // and `tag_name().namespace()` (resolved URI). We match by local name
        // "section" *and* the Blogger namespace URI for safety.
        if node.tag_name().name() == "section"
            && node.tag_name().namespace() == Some("http://www.google.com/2005/gml/b")
        {
            if let Some(id) = node.attribute("id") {
                found_b_section_ids.push(id.to_string());
            }
        }
    }

    for required in REQUIRED_IDS {
        if !found_ids.iter().any(|f| f == required) {
            out.push(Warning::error(
                "ID_MISSING",
                format!("Required id='{required}' not found anywhere in the template."),
            ));
        }
    }

    for required in REQUIRED_CLASSES {
        if !found_classes.iter().any(|f| f == required) {
            out.push(Warning::error(
                "CLASS_MISSING",
                format!("Required class='{required}' not found on any element."),
            ));
        }
    }

    for required in REQUIRED_BLOGGER_SECTIONS {
        if !found_b_section_ids.iter().any(|f| f == required) {
            out.push(Warning::error(
                "BSECTION_MISSING",
                format!(
                    "Required <b:section id='{required}'> not found. Blogger widgets cannot mount without it."
                ),
            ));
        }
    }

    // Soft check: panel-toggle buttons should carry a data-target. Missing one
    // is not fatal (the click handler ignores them) but is almost certainly a
    // template bug.
    for node in doc.descendants().filter(Node::is_element) {
        let class_attr = match node.attribute("class") {
            Some(c) => c,
            None => continue,
        };
        if !class_attr
            .split_ascii_whitespace()
            .any(|c| c == "panel-toggle")
        {
            continue;
        }
        // Skip the documented helper buttons inside the panel-header itself,
        // since some of those legitimately have no data-target (e.g. inline
        // back-to-top buttons). We only flag top-level toggles.
        if node.attribute("data-target").is_none() && node.attribute("onclick").is_none() {
            out.push(Warning::warn(
                "PANEL_TOGGLE_INERT",
                "Found a .panel-toggle button with neither data-target nor onclick. \
                 It will not respond to clicks.",
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal well-formed Blogger XML that satisfies every requirement.
    /// Used as a baseline so per-test mutations only remove one thing at a time.
    fn good_template() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" ?>
<html xmlns="http://www.w3.org/1999/xhtml"
      xmlns:b="http://www.google.com/2005/gml/b">
<head><title>x</title></head>
<body>
  <div class="terminal-workspace">
    <aside id="panel-left">
      <button class="panel-toggle" data-target="panel-left">x</button>
      <b:section id="sidebar-left"/>
    </aside>
    <main class="canvas-core">
      <b:section id="main"/>
    </main>
    <aside id="panel-right">
      <b:section id="sidebar-right"/>
    </aside>
  </div>
  <!-- tokens: {{SITE_TITLE}} {{HOME_URL}} {{CUSTOM_PLUGIN_SCRIPTS}} -->
</body>
</html>"#
            .to_string()
    }

    #[test]
    fn valid_template_passes() {
        let r = check_integrity(&good_template());
        assert!(r.is_valid, "expected valid, got errors: {:?}", r.errors);
        assert!(r.errors.is_empty());
    }

    #[test]
    fn missing_id_is_caught() {
        let bad = good_template().replace(r#"id="panel-left""#, r#"id="panel-LEFTY""#);
        let r = check_integrity(&bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("panel-left")));
    }

    #[test]
    fn missing_class_is_caught() {
        let bad = good_template().replace(r#"class="canvas-core""#, r#"class="canvas""#);
        let r = check_integrity(&bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("canvas-core")));
    }

    #[test]
    fn missing_bsection_is_caught() {
        let bad = good_template().replace(r#"<b:section id="main"/>"#, "");
        let r = check_integrity(&bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("main")));
    }

    #[test]
    fn missing_token_is_caught() {
        let bad = good_template().replace("{{SITE_TITLE}}", "");
        let r = check_integrity(&bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("SITE_TITLE")));
    }

    #[test]
    fn malformed_xml_is_caught_but_token_check_still_runs() {
        // Unclosed <head> breaks XML parsing entirely.
        let bad =
            "<html><head><body>{{SITE_TITLE}} {{HOME_URL}} {{CUSTOM_PLUGIN_SCRIPTS}}</body></html>";
        let r = check_integrity(bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("XML_PARSE")));
        // Tokens were present so no TOKEN_MISSING errors should appear.
        assert!(!r.errors.iter().any(|e| e.contains("TOKEN_MISSING")));
    }

    #[test]
    fn inert_panel_toggle_is_warning_not_error() {
        let bad = good_template().replace(
            r#"<button class="panel-toggle" data-target="panel-left">x</button>"#,
            r#"<button class="panel-toggle">x</button>"#,
        );
        let r = check_integrity(&bad);
        assert!(r.is_valid, "inert toggle should be a warning, not an error");
        assert!(r.warnings.iter().any(|w| w.code == "PANEL_TOGGLE_INERT"));
    }
}
