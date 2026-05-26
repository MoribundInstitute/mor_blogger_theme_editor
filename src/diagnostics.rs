//! Diagnostic Integrity Engine
//!
//! Validates Blogger template strings without assuming that every template uses
//! the same visual shell.
//!
//! Important distinction:
//! - The Blogger engine layer is required: Blog1, working b:section mounts,
//!   and the full expanded V2 widget includables.
//! - The visual shell is profile-specific: Terminal shell uses panel-left,
//!   panel-right, canvas-core, terminal-workspace, etc.; the functional
//!   compendium/base shell uses layout-container, layout-main, layout-sidebar,
//!   and a single sidebar section.
//!
//! This file intentionally accepts both shapes.

use roxmltree::{Document, Node, ParsingOptions};

/// Severity of a single diagnostic warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Template will not render correctly, or Blogger will probably reject it.
    Error,
    /// Template can render, but something expected by a specific shell/profile is missing.
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

        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

/// Visual shell detected from the template body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TemplateProfile {
    /// Your older RuneLite/terminal shell.
    TerminalWorkspace,
    /// The known-good Blogger XML converted to GUI-token form.
    FunctionalBloggerBase,
    /// Unknown shell. We still validate the Blogger engine layer.
    Unknown,
}

#[derive(Debug, Default)]
struct StructuralFacts {
    ids: Vec<String>,
    classes: Vec<String>,
    b_section_ids: Vec<String>,
    widget_ids: Vec<String>,
    includable_ids: Vec<String>,
    has_blog_widget_settings: bool,
}

// ---------------------------------------------------------------------------
// Rendered XML token checks
// ---------------------------------------------------------------------------
//
// This diagnostics module is used after rendering/export, so tokens should
// be gone. Missing {{SITE_TITLE}} is not an error in final XML; leftover
// {{SOMETHING}} is the real error.

const ALLOWED_LITERAL_DOUBLE_BRACES: &[&str] = &[
    // Keep this empty unless you intentionally place literal moustache text in
    // the final Blogger XML.
];

// Required Blogger engine pieces for the known-good expanded Blog widget.
const REQUIRED_BLOG_INCLUDABLES: &[&str] = &[
    "main",
    "post",
    "postBody",
    "postTitle",
    "postCommentsAndAd",
    "postPagination",
];

const RECOMMENDED_BLOG_INCLUDABLES: &[&str] = &[
    "comments",
    "commentPicker",
    "threadedComments",
    "postFooter",
    "postLabels",
    "postTimestamp",
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Run all integrity checks against a Blogger template source string.
///
/// `is_valid = true` means the core Blogger engine layer appears intact.
/// Profile-specific missing UI pieces are warnings unless the template clearly
/// declares that profile.
pub fn check_integrity(source: &str) -> DiagnosticResult {
    let mut warnings = Vec::new();

    check_tokens(source, &mut warnings);

    let opts = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };

    match Document::parse_with_options(source, opts) {
        Ok(doc) => {
            let facts = collect_facts(&doc);
            let profile = detect_profile(&facts);

            check_blogger_engine_layer(source, &facts, &mut warnings);
            check_profile_structure(profile, &facts, &mut warnings);
            check_optional_tokens(source, &mut warnings);
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
// Token checks
// ---------------------------------------------------------------------------

fn check_tokens(source: &str, out: &mut Vec<Warning>) {
    let mut scrubbed = source.to_string();

    for allowed in ALLOWED_LITERAL_DOUBLE_BRACES {
        scrubbed = scrubbed.replace(allowed, "");
    }

    if scrubbed.contains("{{") || scrubbed.contains("}}") {
        let sample = unresolved_token_sample(&scrubbed)
            .unwrap_or_else(|| "unknown unresolved token".to_string());

        out.push(Warning::error(
            "UNRESOLVED_TOKEN",
            format!(
                "Rendered XML still contains unresolved template placeholder(s), for example: {sample}"
            ),
        ));
    }
}

fn check_optional_tokens(_source: &str, _out: &mut Vec<Warning>) {
    // No-op for rendered XML. Optional GUI tokens are allowed to disappear
    // because the renderer should replace them before upload.
}

fn unresolved_token_sample(source: &str) -> Option<String> {
    let start = source.find("{{")?;
    let after_start = &source[start..];
    let end_rel = after_start.find("}}")?;
    Some(after_start[..end_rel + 2].to_string())
}

// ---------------------------------------------------------------------------
// Structure collection
// ---------------------------------------------------------------------------

fn collect_facts(doc: &Document) -> StructuralFacts {
    let mut facts = StructuralFacts::default();

    for node in doc.descendants().filter(Node::is_element) {
        if let Some(id) = node.attribute("id") {
            facts.ids.push(id.to_string());
        }

        if let Some(class_attr) = node.attribute("class") {
            for c in class_attr.split_ascii_whitespace() {
                facts.classes.push(c.to_string());
            }
        }

        let is_blogger_ns = node.tag_name().namespace() == Some("http://www.google.com/2005/gml/b");

        if is_blogger_ns && node.tag_name().name() == "section" {
            if let Some(id) = node.attribute("id") {
                facts.b_section_ids.push(id.to_string());
            }
        }

        if is_blogger_ns && node.tag_name().name() == "widget" {
            if let Some(id) = node.attribute("id") {
                facts.widget_ids.push(id.to_string());
            }
        }

        if is_blogger_ns && node.tag_name().name() == "includable" {
            if let Some(id) = node.attribute("id") {
                facts.includable_ids.push(id.to_string());
            }
        }

        if is_blogger_ns && node.tag_name().name() == "widget-settings" {
            // This is a broad signal, but in the functional base the important
            // one is inside Blog1. The source string check below reinforces it.
            facts.has_blog_widget_settings = true;
        }
    }

    facts
}

fn detect_profile(facts: &StructuralFacts) -> TemplateProfile {
    if has_class(facts, "terminal-workspace") || has_class(facts, "canvas-core") {
        TemplateProfile::TerminalWorkspace
    } else if has_class(facts, "layout-container")
        || has_class(facts, "layout-main")
        || has_class(facts, "layout-sidebar")
        || has_section(facts, "sidebar")
    {
        TemplateProfile::FunctionalBloggerBase
    } else {
        TemplateProfile::Unknown
    }
}

// ---------------------------------------------------------------------------
// Core Blogger engine checks
// ---------------------------------------------------------------------------

fn check_blogger_engine_layer(source: &str, facts: &StructuralFacts, out: &mut Vec<Warning>) {
    // Every viable Blogger theme needs a main Blog section. This project now
    // uses id='main-content' inside the terminal canvas, while older templates
    // and some Blogger bases use id='main'.
    if !has_main_blog_section(facts) {
        out.push(Warning::error(
            "BSECTION_MISSING",
            "Required main Blog section not found. Expected either <b:section id='main'> or <b:section id='main-content'>.",
        ));
    }

    // Accept either the functional single-sidebar model or the older split-sidebar model.
    let has_single_sidebar = has_section(facts, "sidebar");
    let has_split_sidebars = has_section(facts, "sidebar-left") && has_section(facts, "sidebar-right");

    if !has_single_sidebar && !has_split_sidebars {
        out.push(Warning::error(
            "BSECTION_MISSING",
            "No valid sidebar section set found. Expected either id='sidebar' or both id='sidebar-left' and id='sidebar-right'.",
        ));
    }

    if !has_widget(facts, "Blog1") {
        out.push(Warning::error(
            "WIDGET_MISSING",
            "Required Blog widget id='Blog1' not found.",
        ));
        return;
    }

    if !source.contains("<b:widget-settings>") && !source.contains("<b:widget-settings ") {
        out.push(Warning::warn(
            "BLOG_SETTINGS_MISSING",
            "Blog1 appears to lack <b:widget-settings>. The template may still render, but the known-good base keeps these settings.",
        ));
    }

    for includable in REQUIRED_BLOG_INCLUDABLES {
        if !has_includable(facts, includable) {
            out.push(Warning::error(
                "BLOG_INCLUDABLE_MISSING",
                format!(
                    "Blog1 engine layer is incomplete: required <b:includable id='{includable}'> not found."
                ),
            ));
        }
    }

    for includable in RECOMMENDED_BLOG_INCLUDABLES {
        if !has_includable(facts, includable) {
            out.push(Warning::warn(
                "BLOG_INCLUDABLE_RECOMMENDED_MISSING",
                format!(
                    "Recommended Blogger includable id='{includable}' not found. Keep the known-good full widget body unless intentionally simplifying."
                ),
            ));
        }
    }

    // Side widgets are not always mandatory, but this app expects them in the functional base.
    if !has_widget(facts, "Label1") {
        out.push(Warning::warn(
            "WIDGET_OPTIONAL_MISSING",
            "Label1 not found. Label browsing will not render.",
        ));
    }

    if !has_widget(facts, "BlogArchive1") {
        out.push(Warning::warn(
            "WIDGET_OPTIONAL_MISSING",
            "BlogArchive1 not found. Archive browsing will not render.",
        ));
    }
}

// ---------------------------------------------------------------------------
// Profile-specific checks
// ---------------------------------------------------------------------------

fn check_profile_structure(profile: TemplateProfile, facts: &StructuralFacts, out: &mut Vec<Warning>) {
    match profile {
        TemplateProfile::TerminalWorkspace => {
            require_id(facts, out, "panel-left", Severity::Error);
            require_id(facts, out, "panel-right", Severity::Error);
            require_class(facts, out, "canvas-core", Severity::Error);
            require_class(facts, out, "panel-toggle", Severity::Error);
            require_class(facts, out, "terminal-workspace", Severity::Error);
        }
        TemplateProfile::FunctionalBloggerBase => {
            require_class(facts, out, "layout-container", Severity::Error);
            require_class(facts, out, "layout-main", Severity::Error);
            require_class(facts, out, "layout-sidebar", Severity::Warning);
            require_class(facts, out, "sidebar-section", Severity::Warning);
        }
        TemplateProfile::Unknown => {
            out.push(Warning::warn(
                "PROFILE_UNKNOWN",
                "Template visual shell was not recognized. Core Blogger checks still ran, but shell-specific checks were skipped.",
            ));
        }
    }

    // Generic warning for inert terminal toggles, but only if they exist.
    // Functional base does not use panel-toggle at all.
    if has_class(facts, "panel-toggle") {
        // Detailed per-node data would require keeping nodes around; the old
        // implementation did this. The simplified warning is enough for now.
        out.push(Warning::warn(
            "PANEL_TOGGLE_CHECK_SKIPPED",
            "Template contains .panel-toggle. Manually verify each toggle has data-target or an intentional onclick.",
        ));
    }
}

fn require_id(facts: &StructuralFacts, out: &mut Vec<Warning>, id: &'static str, severity: Severity) {
    if !has_id(facts, id) {
        let warning = match severity {
            Severity::Error => Warning::error(
                "ID_MISSING",
                format!("Required id='{id}' not found for this template profile."),
            ),
            Severity::Warning => Warning::warn(
                "ID_OPTIONAL_MISSING",
                format!("Optional id='{id}' not found for this template profile."),
            ),
        };
        out.push(warning);
    }
}

fn require_class(
    facts: &StructuralFacts,
    out: &mut Vec<Warning>,
    class_name: &'static str,
    severity: Severity,
) {
    if !has_class(facts, class_name) {
        let warning = match severity {
            Severity::Error => Warning::error(
                "CLASS_MISSING",
                format!("Required class='{class_name}' not found for this template profile."),
            ),
            Severity::Warning => Warning::warn(
                "CLASS_OPTIONAL_MISSING",
                format!("Optional class='{class_name}' not found for this template profile."),
            ),
        };
        out.push(warning);
    }
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn has_id(facts: &StructuralFacts, id: &str) -> bool {
    facts.ids.iter().any(|f| f == id)
}

fn has_class(facts: &StructuralFacts, class_name: &str) -> bool {
    facts.classes.iter().any(|f| f == class_name)
}

fn has_main_blog_section(facts: &StructuralFacts) -> bool {
    has_section(facts, "main") || has_section(facts, "main-content")
}

fn has_section(facts: &StructuralFacts, id: &str) -> bool {
    facts.b_section_ids.iter().any(|f| f == id)
}

fn has_widget(facts: &StructuralFacts, id: &str) -> bool {
    facts.widget_ids.iter().any(|f| f == id)
}

fn has_includable(facts: &StructuralFacts, id: &str) -> bool {
    facts.includable_ids.iter().any(|f| f == id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn terminal_template() -> String {
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
      <b:section id="main-content">
        <b:widget id="Blog1" type="Blog" version="2">
          <b:widget-settings><b:widget-setting name="showLabels">true</b:widget-setting></b:widget-settings>
          <b:includable id="main"/>
          <b:includable id="post"/>
          <b:includable id="postBody"/>
          <b:includable id="postTitle"/>
          <b:includable id="postCommentsAndAd"/>
          <b:includable id="postPagination"/>
        </b:widget>
      </b:section>
    </main>
    <aside id="panel-right">
      <b:section id="sidebar-right"/>
    </aside>
  </div>

</body>
</html>"#
            .to_string()
    }

    fn functional_base_template() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" ?>
<html xmlns="http://www.w3.org/1999/xhtml"
      xmlns:b="http://www.google.com/2005/gml/b">
<head><title>Rendered title</title></head>
<body>
  <div class="layout-container">
    <main class="layout-main">
      <b:section id="main">
        <b:widget id="Blog1" type="Blog" version="2">
          <b:widget-settings><b:widget-setting name="showLabels">true</b:widget-setting></b:widget-settings>
          <b:includable id="main"/>
          <b:includable id="post"/>
          <b:includable id="postBody"/>
          <b:includable id="postTitle"/>
          <b:includable id="postCommentsAndAd"/>
          <b:includable id="postPagination"/>
        </b:widget>
      </b:section>
    </main>
    <aside class="layout-sidebar">
      <b:section class="sidebar-section" id="sidebar">
        <b:widget id="Label1" type="Label" version="2"><b:includable id="content"/></b:widget>
        <b:widget id="BlogArchive1" type="BlogArchive" version="2"><b:includable id="hierarchy"/></b:widget>
      </b:section>
    </aside>
  </div>
</body>
</html>"#
            .to_string()
    }

    #[test]
    fn terminal_template_passes() {
        let r = check_integrity(&terminal_template());
        assert!(r.is_valid, "expected valid, got errors: {:?}", r.errors);
    }

    #[test]
    fn functional_base_template_passes_without_terminal_shell() {
        let r = check_integrity(&functional_base_template());
        assert!(r.is_valid, "expected valid, got errors: {:?}", r.errors);
        assert!(!r.errors.iter().any(|e| e.contains("panel-left")));
        assert!(!r.errors.iter().any(|e| e.contains("sidebar-left")));
        assert!(!r.errors.iter().any(|e| e.contains("HOME_URL")));
    }

    #[test]
    fn missing_blog_engine_is_error() {
        let bad = functional_base_template().replace(r#"<b:includable id="postBody"/>"#, "");
        let r = check_integrity(&bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("postBody")));
    }

    #[test]
    fn unresolved_token_is_error() {
        let bad = functional_base_template().replace("Rendered title", "{{SITE_TITLE}}");
        let r = check_integrity(&bad);
        assert!(!r.is_valid);
        assert!(r.errors.iter().any(|e| e.contains("UNRESOLVED_TOKEN")));
    }
}
