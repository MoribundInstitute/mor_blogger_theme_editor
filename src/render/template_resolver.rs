//! src/render/template_resolver.rs
use std::collections::HashSet;
use crate::config::ThemeConfig;
use crate::render::css_builder::build_master_css;

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
// THE REGISTRIES (Now mapped to your actual files!)
// =====================================================================

pub const HEADER_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "mor",
        category: ComponentCategory::Header,
        xml_content: include_str!("../template_parts/headers/mor_header_baseline.xml"),
        css_deps: &["04-Main-Header.css", "05-Branding.css", "06-Main-Navigation.css", "07-Catalog-Mega-Dropdown.css", "08-Command-Line-Search.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "gtk_headerbar",
        category: ComponentCategory::Header,
        xml_content: include_str!("../template_parts/headers/gtk_headerbar.xml"), // Mapped to real file!
        css_deps: &["04-Main-Header.css", "05-Branding.css", "06-Main-Navigation.css"], // Dropped mega-dropdown for GTK
        js_deps: &[],
    },
];

pub const LAYOUT_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "sidebars",
        category: ComponentCategory::Layout,
        xml_content: include_str!("../template_parts/layouts/sidebars.xml"),
        css_deps: &["09-Workspace-Layout.css", "10-Side-Panels.css", "11-Main-Canvas.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "single_column",
        category: ComponentCategory::Layout,
        xml_content: include_str!("../template_parts/layouts/single_column.xml"), // Mapped to real file!
        // Notice: Single column deliberately drops "10-Side-Panels.css"!
        css_deps: &["09-Workspace-Layout.css", "11-Main-Canvas.css"], 
        js_deps: &[],
    },
];

pub const CONTENT_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "blog_standard",
        category: ComponentCategory::Content,
        xml_content: include_str!("../template_parts/content/blog_standard.xml"),
        css_deps: &[], // Relies on baseline CSS
        js_deps: &[],
    },
    ComponentManifest {
        id: "mor_magazine",
        category: ComponentCategory::Content,
        xml_content: include_str!("../template_parts/content/mor_magazine.xml"),
        css_deps: &["30-Content-Magazine.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "mor_masonry",
        category: ComponentCategory::Content,
        xml_content: include_str!("../template_parts/content/mor_masonry.xml"),
        css_deps: &["31-Content-Masonry.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "mor_minimal",
        category: ComponentCategory::Content,
        xml_content: include_str!("../template_parts/content/mor_minimal.xml"),
        css_deps: &["32-Content-Minimal.css"],
        js_deps: &[],
    },
];

pub const SIDEBAR_LEFT_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "blogger_left",
        category: ComponentCategory::SidebarLeft,
        xml_content: include_str!("../template_parts/sidebars/blogger_left.xml"),
        css_deps: &["14-Widgets-Sidebars.css", "15-Archive-Widget.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "gtk_dock_left",
        category: ComponentCategory::SidebarLeft,
        xml_content: include_str!("../template_parts/sidebars/gtk_dock_left.xml"), // Mapped to real file!
        css_deps: &["14-Widgets-Sidebars.css"],
        js_deps: &[],
    },
];

pub const SIDEBAR_RIGHT_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "toc_right",
        category: ComponentCategory::SidebarRight,
        xml_content: include_str!("../template_parts/sidebars/toc_right.xml"),
        css_deps: &["16-Table-of-Contents.css"],
        js_deps: &[],
    },
];

pub const FOOTER_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "mega",
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterMega.xml"),
        css_deps: &["18-Footer-Base.css", "18-Footer-Mega.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "basic",
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterBasic.xml"),
        css_deps: &["18-Footer-Base.css"], 
        js_deps: &[],
    },
    ComponentManifest {
        id: "compact",
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterCompact.xml"),
        css_deps: &["18-Footer-Base.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "mor", // Fallback
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterMega.xml"),
        css_deps: &["18-Footer-Base.css", "18-Footer-Mega.css"],
        js_deps: &[],
    },
];

// =====================================================================
// ASSET FETCHERS (Compile-time embedded)
// =====================================================================

fn fetch_css(filename: &str) -> &'static str {
    match filename {
        "00-Root-Section.css" => include_str!("../template_parts/base/skin/00-Root-Section.css"),
        "01-Reset-Base.css" => include_str!("../template_parts/base/skin/01-Reset-Base.css"),
        "02-Typography-Links.css" => include_str!("../template_parts/base/skin/02-Typography-Links.css"),
        "03-Buttons.css" => include_str!("../template_parts/base/skin/03-Buttons.css"),
        "04-Main-Header.css" => include_str!("../template_parts/base/skin/04-Main-Header.css"),
        "05-Branding.css" => include_str!("../template_parts/base/skin/05-Branding.css"),
        "06-Main-Navigation.css" => include_str!("../template_parts/base/skin/06-Main-Navigation.css"),
        "07-Catalog-Mega-Dropdown.css" => include_str!("../template_parts/base/skin/07-Catalog-Mega-Dropdown.css"),
        "08-Command-Line-Search.css" => include_str!("../template_parts/base/skin/08-Command-Line-Search.css"),
        "09-Workspace-Layout.css" => include_str!("../template_parts/base/skin/09-Workspace-Layout.css"),
        "10-Side-Panels.css" => include_str!("../template_parts/base/skin/10-Side-Panels.css"),
        "11-Main-Canvas.css" => include_str!("../template_parts/base/skin/11-Main-Canvas.css"),
        "12-Terminal-Post-Styling.css" => include_str!("../template_parts/base/skin/12-Terminal-Post-Styling.css"),
        "13-Pagination.css" => include_str!("../template_parts/base/skin/13-Pagination.css"),
        "14-Widgets-Sidebars.css" => include_str!("../template_parts/base/skin/14-Widgets-Sidebars.css"),
        "15-Archive-Widget.css" => include_str!("../template_parts/base/skin/15-Archive-Widget.css"),
        "16-Table-of-Contents.css" => include_str!("../template_parts/base/skin/16-Table-of-Contents.css"),
        "17-Scrollbars.css" => include_str!("../template_parts/base/skin/17-Scrollbars.css"),
        "18-Footer-Base.css" => include_str!("../template_parts/base/skin/18-Footer-Base.css"),
        "18-Footer-Mega.css" => include_str!("../template_parts/base/skin/18-Footer-Mega.css"),
        "19-Responsive-Mobile-Tablet.css" => include_str!("../template_parts/base/skin/19-Responsive-Mobile-Tablet.css"),
        "20-Responsive-Very-Small-Screens.css" => include_str!("../template_parts/base/skin/20-Responsive-Very-Small-Screens.css"),
        "21-Responsive-Desktop.css" => include_str!("../template_parts/base/skin/21-Responsive-Desktop.css"),
        "22-Export-Safety.css" => include_str!("../template_parts/base/skin/22-Export-Safety.css"),
        "23-Comments.css" => include_str!("../template_parts/base/skin/23-Comments.css"),
        "24-Author-Profile.css" => include_str!("../template_parts/base/skin/24-Author-Profile.css"),
        "25-Share-Menu.css" => include_str!("../template_parts/base/skin/25-Share-Menu.css"),
        "26-Analytics-Dashboard.css" => include_str!("../template_parts/base/skin/26-Analytics-Dashboard.css"),
        "30-Content-Magazine.css" => include_str!("../template_parts/base/skin/30-Content-Magazine.css"),
        // Hardcoded fallbacks until Claude and Qwen build them:
        "31-Content-Masonry.css" => "/* Masonry Placeholder */",
        "32-Content-Minimal.css" => "/* Minimal Placeholder */",
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
// DEPENDENCY RESOLVER
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
    pub javascript: String,
}

pub fn resolve_template_parts(config: &ThemeConfig) -> TemplateParts {
    let pack = &config.template_pack;
    
    let get_comp = |registry: &[ComponentManifest], id: &str| -> ComponentManifest {
        registry.iter().find(|c| c.id == id).unwrap_or(&registry[0]).clone()
    };

    let active_components = vec![
        get_comp(HEADER_REGISTRY, &pack.header_variant),
        get_comp(LAYOUT_REGISTRY, &pack.main_variant),
        get_comp(CONTENT_REGISTRY, &pack.content_variant), // <-- NEW
        get_comp(SIDEBAR_LEFT_REGISTRY, &pack.left_sidebar_variant),
        get_comp(SIDEBAR_RIGHT_REGISTRY, &pack.right_sidebar_variant),
        get_comp(FOOTER_REGISTRY, &pack.footer_variant),
    ];

    let mut unique_css = HashSet::new();
    let mut unique_js = HashSet::new();

    // 🌟 GLOBAL BASELINE CSS
    let global_css = [
        "00-Root-Section.css", "01-Reset-Base.css", "02-Typography-Links.css",
        "03-Buttons.css", "12-Terminal-Post-Styling.css", "13-Pagination.css",
        "17-Scrollbars.css", "19-Responsive-Mobile-Tablet.css", "20-Responsive-Very-Small-Screens.css",
        "21-Responsive-Desktop.css", "22-Export-Safety.css", "23-Comments.css",
        "24-Author-Profile.css", "25-Share-Menu.css", "26-Analytics-Dashboard.css",
    ];

    for css in global_css { unique_css.insert(css); }

    // 🌟 DYNAMIC JS BASED ON CONFIG
    if pack.script_variant == "mor_panels" {
        unique_js.insert("01-Core-Helpers.js");
        unique_js.insert("07-Theme-Toggler.js");
        unique_js.insert("08-Share-Actions.js");
    }

    // Add component dependencies
    for comp in &active_components {
        for &css in comp.css_deps { unique_css.insert(css); }
        for &js in comp.js_deps { unique_js.insert(js); }
    }

    let mut sorted_css: Vec<_> = unique_css.into_iter().collect();
    sorted_css.sort();

    let mut sorted_js: Vec<_> = unique_js.into_iter().collect();
    sorted_js.sort();

    let css_contents: Vec<&str> = sorted_css.iter().map(|f| fetch_css(f)).collect();
    let js_contents: Vec<String> = sorted_js.iter().map(|f| format!("/* --- {} --- */\n{}", f, fetch_js(f))).collect();

    TemplateParts {
        meta: include_str!("../template_parts/base/meta.xml"),
        css: build_master_css(&css_contents, config),
        javascript: js_contents.join("\n\n"),
        
        header: get_comp(HEADER_REGISTRY, &pack.header_variant).xml_content,
        main: get_comp(LAYOUT_REGISTRY, &pack.main_variant).xml_content,
        sidebar_left: get_comp(SIDEBAR_LEFT_REGISTRY, &pack.left_sidebar_variant).xml_content,
        sidebar_right: get_comp(SIDEBAR_RIGHT_REGISTRY, &pack.right_sidebar_variant).xml_content,
        footer: get_comp(FOOTER_REGISTRY, &pack.footer_variant).xml_content,
        
        content: get_comp(CONTENT_REGISTRY, &pack.content_variant).xml_content,
    }
}