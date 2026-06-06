use roxmltree::{Document, Node, ParsingOptions};
use crate::config::TemplatePackConfig;
use super::{Severity, Warning};

/// Visual shell detected from the template body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TemplateProfile {
    TerminalWorkspace,
    FunctionalBloggerBase,
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

const REQUIRED_BLOG_INCLUDABLES: &[&str] = &[
    "main", "post", "postBody", "postTitle", "postCommentsAndAd", "postPagination",
];

const RECOMMENDED_BLOG_INCLUDABLES: &[&str] = &[
    "comments", "commentPicker", "threadedComments", "postFooter", "postLabels", "postTimestamp",
];

pub fn run_xml_checks(source: &str, active_variants: &TemplatePackConfig, out: &mut Vec<Warning>) {
    let opts = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };
    
    match Document::parse_with_options(source, opts) {
        Ok(doc) => {
            let facts = collect_facts(&doc);
            let profile = detect_profile(&facts);

            check_blogger_engine_layer(&facts, out);
            check_profile_structure(profile, &facts, out);
            check_panel_toggles(&doc, out);
            
            check_registry_dependencies(&doc, active_variants, source, out);
        }
        Err(e) => {
            out.push(Warning::error(
                "XML_PARSE",
                format!("Template is not well-formed XML; structural checks skipped. Parser said: {e}"),
            ));
        }
    }
}

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
            facts.has_blog_widget_settings = true;
        }
    }
    facts
}

fn detect_profile(facts: &StructuralFacts) -> TemplateProfile {
    if has_class(facts, "mor-workspace") || has_class(facts, "canvas-core") {
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

fn check_blogger_engine_layer(facts: &StructuralFacts, out: &mut Vec<Warning>) {
    if !has_main_blog_section(facts) {
        out.push(Warning::error("BSECTION_MISSING", "Required main Blog section not found. Expected either <b:section id='main'> or <b:section id='main-content'>."));
    }
    
    let has_single_sidebar = has_section(facts, "sidebar");
    let has_split_sidebars = has_section(facts, "sidebar-left") && has_section(facts, "sidebar-right");

    if !has_single_sidebar && !has_split_sidebars {
        out.push(Warning::error("BSECTION_MISSING", "No valid sidebar section set found. Expected either id='sidebar' or both id='sidebar-left' and id='sidebar-right'."));
    }

    if !has_widget(facts, "Blog1") {
        out.push(Warning::error("WIDGET_MISSING", "Required Blog widget id='Blog1' not found."));
        return;
    }

    if !facts.has_blog_widget_settings {
        out.push(Warning::warn(
            "BLOG_SETTINGS_MISSING",
            "Blog1 appears to lack <b:widget-settings>.",
        ));
    }

    for includable in REQUIRED_BLOG_INCLUDABLES {
        if !has_includable(facts, includable) {
            out.push(Warning::error("BLOG_INCLUDABLE_MISSING", format!("Blog1 engine layer incomplete: required <b:includable id='{includable}'> not found.")));
        }
    }

    for includable in RECOMMENDED_BLOG_INCLUDABLES {
        if !has_includable(facts, includable) {
            out.push(Warning::warn("BLOG_INCLUDABLE_RECOMMENDED_MISSING", format!("Recommended Blogger includable id='{includable}' not found.")));
        }
    }

    if !has_widget(facts, "Label1") {
        out.push(Warning::warn("WIDGET_OPTIONAL_MISSING", "Label1 not found. Label browsing will not render."));
    }

    if !has_widget(facts, "BlogArchive1") {
        out.push(Warning::warn("WIDGET_OPTIONAL_MISSING", "BlogArchive1 not found. Archive browsing will not render."));
    }
}

fn check_profile_structure(profile: TemplateProfile, facts: &StructuralFacts, out: &mut Vec<Warning>) {
    match profile {
        TemplateProfile::TerminalWorkspace => {
            require_id(facts, out, "panel-left", Severity::Error);
            require_id(facts, out, "panel-right", Severity::Error);
            require_class(facts, out, "canvas-core", Severity::Error);
            require_class(facts, out, "panel-toggle", Severity::Error);
            require_class(facts, out, "mor-workspace", Severity::Error);
        }
        TemplateProfile::FunctionalBloggerBase => {
            require_class(facts, out, "layout-container", Severity::Error);
            require_class(facts, out, "layout-main", Severity::Error);
            require_class(facts, out, "layout-sidebar", Severity::Warning);
            require_class(facts, out, "sidebar-section", Severity::Warning);
        }
        TemplateProfile::Unknown => {
            out.push(Warning::warn("PROFILE_UNKNOWN", "Template visual shell was not recognized. Core Blogger checks ran, shell checks skipped."));
        }
    }
}

fn check_panel_toggles(doc: &Document, out: &mut Vec<Warning>) {
    for node in doc.descendants().filter(Node::is_element) {
        let Some(class_attr) = node.attribute("class") else {
            continue;
        };
        
        if !class_attr
            .split_ascii_whitespace()
            .any(|class_name| class_name == "panel-toggle")
        {
            continue;
        }

        let has_target = node
            .attribute("data-target")
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
            
        let has_onclick = node
            .attribute("onclick")
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
            
        let has_href = node
            .attribute("href")
            .map(|value| {
                let value = value.trim();
                !value.is_empty() && value != "#"
            })
            .unwrap_or(false);

        if !has_target && !has_onclick && !has_href {
            let tag = node.tag_name().name();
            let id_str = node
                .attribute("id")
                .map(|id| format!(" id='{id}'"))
                .unwrap_or_default();

            out.push(Warning::error(
                "PANEL_TOGGLE_BROKEN",
                format!("<{tag}{id_str} class='panel-toggle'> is missing a data-target, onclick, or href attribute. It will do nothing."),
            ));
        }
    }
}

// --- UPDATED REGISTRY MAPPER ---
fn required_footer_css(footer_variant: &str) -> Option<&'static str> {
    match footer_variant {
        "mega" => Some("18-Footer-Mega.css"),
        "basic" | "compact" => Some("18-Footer-Base.css"),
        _ => None,
    }
}

fn check_registry_dependencies(
    doc: &Document,
    active_variants: &TemplatePackConfig,
    source: &str,
    warnings: &mut Vec<Warning>,
) {
    let Some(required_css) = required_footer_css(&active_variants.footer_variant) else {
        return;
    };

    if style_block_contains(source, doc, required_css) {
        return;
    }

    warnings.push(Warning::error(
        "REGISTRY_DEPENDENCY_MISSING",
        format!(
            "Missing Dependency: The active footer requires {} to display correctly. Layout may be broken.", required_css
        ),
    ));
}

fn style_block_contains(source: &str, doc: &Document, needle: &str) -> bool {
    let mut saw_style_element = false;

    for node in doc.descendants().filter(Node::is_element) {
        let tag_name = node.tag_name().name();

        if tag_name != "style" && tag_name != "skin" && tag_name != "template-skin" {
            continue;
        }

        saw_style_element = true;

        for text_node in node.children().filter(|n| n.is_text()) {
            if let Some(text) = text_node.text() {
                if text.contains(needle) {
                    return true;
                }
            }
        }
    }

    if !saw_style_element {
        return source.contains(needle);
    }

    false
}

fn require_id(facts: &StructuralFacts, out: &mut Vec<Warning>, id: &'static str, severity: Severity) {
    if !has_id(facts, id) {
        let warning = match severity {
            Severity::Error => Warning::error("ID_MISSING", format!("Required id='{id}' not found.")),
            Severity::Warning => Warning::warn("ID_OPTIONAL_MISSING", format!("Optional id='{id}' not found.")),
        };
        out.push(warning);
    }
}

fn require_class(facts: &StructuralFacts, out: &mut Vec<Warning>, class_name: &'static str, severity: Severity) {
    if !has_class(facts, class_name) {
        let warning = match severity {
            Severity::Error => Warning::error("CLASS_MISSING", format!("Required class='{class_name}' not found.")),
            Severity::Warning => Warning::warn("CLASS_OPTIONAL_MISSING", format!("Optional class='{class_name}' not found.")),
        };
        out.push(warning);
    }
}

fn has_id(facts: &StructuralFacts, id: &str) -> bool { facts.ids.iter().any(|f| f == id) }
fn has_class(facts: &StructuralFacts, class_name: &str) -> bool { facts.classes.iter().any(|f| f == class_name) }
fn has_main_blog_section(facts: &StructuralFacts) -> bool { has_section(facts, "main") || has_section(facts, "main-content") }
fn has_section(facts: &StructuralFacts, id: &str) -> bool { facts.b_section_ids.iter().any(|f| f == id) }
fn has_widget(facts: &StructuralFacts, id: &str) -> bool { facts.widget_ids.iter().any(|f| f == id) }
fn has_includable(facts: &StructuralFacts, id: &str) -> bool { facts.includable_ids.iter().any(|f| f == id) }