//! src/render/template_resolver.rs
use std::collections::HashSet;
use crate::config::ThemeConfig;
use crate::render::css_builder::build_master_css;

// =====================================================================
// 1. REGISTRY STRUCTS
// =====================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentCategory {
    Header, Layout, Content, SidebarLeft, SidebarRight, Footer, Widget
}

#[derive(Debug, Clone)]
pub struct ComponentManifest {
    pub id: &'static str,
    pub category: ComponentCategory,
    pub xml_content: &'static str,
    pub css_deps: &'static [&'static str],
    pub js_deps: &'static [&'static str],
}

// =====================================================================
// 2. COMPONENT DEFINITIONS (The Source of Truth)
// =====================================================================
pub const FOOTER_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "mor",
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterMega.xml"),
        // Notice it declares EXACTLY what it needs!
        css_deps: &["18-Footer-Base.css", "18-Footer-Mega.css"],
        js_deps: &["01-Core-Helpers.js"],
    },
];
// Add similar arrays for HEADER_REGISTRY, LAYOUT_REGISTRY, etc...

// =====================================================================
// 3. ASSET FETCHERS (Satisfies include_str! compile-time constraints)
// =====================================================================

// Hardcoded default fallback styles to guarantee compile success 
// without depending on external file existence before they are written.
const FOOTER_BASE_CSS_FALLBACK: &str = r#"
/* --- 18-Footer-Base.css --- */
.mor-footer {
    padding: 2rem 1rem;
    margin-top: auto;
    border-top: 1px solid var(--border-color, #333);
}
"#;

const FOOTER_MEGA_CSS_FALLBACK: &str = r#"
/* --- 18-Footer-Mega.css --- */
.mor-footer-mega {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
}
.footer-mega-grid {
    list-style: none;
    padding: 0;
}
"#;

fn fetch_css(filename: &str) -> &'static str {
    match filename {
        // If these base configuration files exist in your path, keep them:
        "00-Root-Section.css" => include_str!("../template_parts/base/skin/00-Root-Section.css"),
        "01-Reset-Base.css" => include_str!("../template_parts/base/skin/01-Reset-Base.css"),
        "02-Typography-Links.css" => include_str!("../template_parts/base/skin/02-Typography-Links.css"),
        
        // INTERIM FIX: Redirect missing files to compile-safe internal constants
        "18-Footer-Base.css" => FOOTER_BASE_CSS_FALLBACK, 
        "18-Footer-Mega.css" => FOOTER_MEGA_CSS_FALLBACK,
        _ => "", 
    }
}

fn fetch_js(filename: &str) -> &'static str {
    match filename {
        "01-Core-Helpers.js" => include_str!("../template_parts/scripts/01-Core-Helpers.js"),
        "07-Theme-Toggler.js" => include_str!("../template_parts/scripts/07-Theme-Toggler.js"),
        "08-Share-Actions.js" => include_str!("../template_parts/scripts/08-Share-Actions.js"),
        _ => "",
    }
}

// =====================================================================
// 4. THE DEPENDENCY RESOLVER
// =====================================================================
pub struct TemplateParts {
    pub meta: &'static str,
    pub css: String,
    pub header: &'static str,
    pub main: &'static str,
    pub content: &'static str,
    pub sidebar_left: &'static str,
    pub sidebar_right: &'static str,
    pub footer: &'static str,
    // CHANGED: Upgraded to String to support dynamic concatenation
    pub javascript: String,
}

pub fn resolve_template_parts(config: &ThemeConfig) -> TemplateParts {
    let pack = &config.template_pack;
    let get_comp = |registry: &[ComponentManifest], id: &str| -> ComponentManifest {
        registry.iter().find(|c| c.id == id).unwrap_or(&registry[0]).clone()
    };

    // 1. Gather active components from the user's workspace config
    let active_components = vec![
        get_comp(FOOTER_REGISTRY, &pack.footer_variant),
        // Add get_comp for Headers, Layouts, etc here...
    ];

    // 2. The Resolution Engine
    let mut unique_css = HashSet::new();
    let mut unique_js = HashSet::new();

    // Baseline required assets
    unique_css.insert("00-Root-Section.css");
    unique_css.insert("01-Reset-Base.css");
    unique_css.insert("02-Typography-Links.css");

    for comp in &active_components {
        for &css in comp.css_deps { unique_css.insert(css); }
        for &js in comp.js_deps { unique_js.insert(js); }
    }

    // 3. Sort alphanumerically to preserve CSS Cascade & JS Execution Order!
    let mut sorted_css: Vec<_> = unique_css.into_iter().collect();
    sorted_css.sort();

    let mut sorted_js: Vec<_> = unique_js.into_iter().collect();
    sorted_js.sort();

    // 4. Fetch & Concatenate
    let css_contents: Vec<&str> = sorted_css.iter().map(|f| fetch_css(f)).collect();
    let js_contents: Vec<String> = sorted_js.iter().map(|f| format!("/* --- {} --- */\n{}", f, fetch_js(f))).collect();

    TemplateParts {
        meta: include_str!("../template_parts/base/meta.xml"),
        css: build_master_css(&css_contents),
        javascript: js_contents.join("\n\n"),
        
        // Temporarily hardcoded until you map the rest of the registries
        header: include_str!("../template_parts/headers/mor_header_baseline.xml"),
        main: include_str!("../template_parts/layouts/sidebars.xml"),
        content: include_str!("../template_parts/content/blog_standard.xml"),
        sidebar_left: include_str!("../template_parts/sidebars/blogger_left.xml"),
        sidebar_right: include_str!("../template_parts/sidebars/toc_right.xml"),
        footer: get_comp(FOOTER_REGISTRY, &pack.footer_variant).xml_content,
    }
}