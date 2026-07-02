use super::Warning;
use crate::config::TemplatePackConfig;
use crate::render::template_resolver::{
    ComponentManifest, CONTENT_REGISTRY, FOOTER_REGISTRY, HEADER_REGISTRY, LAYOUT_REGISTRY,
    SIDEBAR_LEFT_REGISTRY, SIDEBAR_RIGHT_REGISTRY,
};

use super::REQUIRED_BLOG_INCLUDABLES;

struct ActiveModule<'a> {
    slot: &'static str,
    variant_id: &'a str,
    xml: &'a str,
    display_path: String,
}

pub fn check_active_module_sources(pack: &TemplatePackConfig, out: &mut Vec<Warning>) {
    let modules = [
        active_module("header", &pack.header_variant, HEADER_REGISTRY),
        active_module("layout", &pack.main_variant, LAYOUT_REGISTRY),
        active_module("content", &pack.content_variant, CONTENT_REGISTRY),
        active_module(
            "left_sidebar",
            &pack.left_sidebar_variant,
            SIDEBAR_LEFT_REGISTRY,
        ),
        active_module(
            "right_sidebar",
            &pack.right_sidebar_variant,
            SIDEBAR_RIGHT_REGISTRY,
        ),
        active_module("footer", &pack.footer_variant, FOOTER_REGISTRY),
    ];

    for module in modules {
        check_module_source(module, out);
    }
}

fn active_module<'a>(
    slot: &'static str,
    variant_id: &'a str,
    registry: &'a [ComponentManifest],
) -> ActiveModule<'a> {
    let manifest = registry
        .iter()
        .find(|entry| entry.id == variant_id)
        .unwrap_or(&registry[0]);

    ActiveModule {
        slot,
        variant_id,
        xml: manifest.xml_content,
        display_path: module_display_path(slot, variant_id),
    }
}

fn module_display_path(slot: &str, variant_id: &str) -> String {
    let filename = match (slot, variant_id) {
        ("content", "standard_feed") => "blog_standard.xml".to_string(),
        ("header", "mor") => "mor_header_baseline.xml".to_string(),
        ("header", "minimal") => "mor_header_minimal.xml".to_string(),
        ("layout", "sidebars") => "sidebars.xml".to_string(),
        ("layout", "single_column") => "single_column.xml".to_string(),
        ("layout", "two_column_right") => "two_column_right.xml".to_string(),
        ("left_sidebar", "blogger_left") => "blogger_left.xml".to_string(),
        ("left_sidebar", "gtk_dock_left") => "gtk_dock_left.xml".to_string(),
        ("right_sidebar", "toc_right") => "toc_right.xml".to_string(),
        ("footer", "mega") => "MorFooterMega.xml".to_string(),
        ("footer", "mega-gno") => "MorFooterMegaGno.xml".to_string(),
        ("footer", "basic") => "MorFooterBasic.xml".to_string(),
        ("footer", "compact") => "MorFooterCompact.xml".to_string(),
        _ => format!("{variant_id}.xml"),
    };

    let category = match slot {
        "header" => "headers",
        "layout" => "layouts",
        "content" => "content",
        "left_sidebar" | "right_sidebar" => "sidebars",
        "footer" => "footers",
        _ => "modules",
    };

    if slot == "content" {
        format!(
            "templates/modules/content_variant.xml (compiled: template_parts/{category}/{filename})"
        )
    } else {
        format!("template_parts/{category}/{filename}")
    }
}

fn check_module_source(module: ActiveModule<'_>, out: &mut Vec<Warning>) {
    if has_unnamespaced_blogger_tags(module.xml) {
        out.push(Warning::error_in(
            "BLOGGER_NS_MISSING",
            format!(
                "Active {slot} module '{variant}' is missing required Blogger namespace tags. Expected elements such as <b:widget>, <b:includable>, and <b:section>.",
                slot = module.slot,
                variant = module.variant_id
            ),
            Some(module.display_path.clone()),
        ));
    }

    if module.slot != "content" || !module.xml.contains("id='Blog1'") {
        return;
    }

    for includable in REQUIRED_BLOG_INCLUDABLES {
        let namespaced = format!("<b:includable id='{includable}'");
        let broken = format!("<includable id='{includable}'");
        if !module.xml.contains(&namespaced) {
            if module.xml.contains(&broken) {
                out.push(Warning::error_in(
                    "BLOGGER_NS_MISSING",
                    format!(
                        "Required <b:includable id='{includable}'> is present but stripped of the Blogger namespace prefix."
                    ),
                    Some(module.display_path.clone()),
                ));
            } else {
                out.push(Warning::error_in(
                    "BLOG_INCLUDABLE_MISSING",
                    format!(
                        "Content module is missing required <b:includable id='{includable}'>."
                    ),
                    Some(module.display_path.clone()),
                ));
            }
        }
    }
}

fn has_unnamespaced_blogger_tags(xml: &str) -> bool {
    // Do not treat HTML <section> as a Blogger namespace violation.
    for tag in ["widget", "includable", "widget-settings", "widget-setting"] {
        if xml.contains(&format!("<{tag}")) && !xml.contains(&format!("<b:{tag}")) {
            return true;
        }
    }
    false
}