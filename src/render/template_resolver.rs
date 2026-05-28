use crate::config::ThemeConfig; // Adjust import path if needed

pub struct TemplateParts {
    pub meta: &'static str,
    pub css: &'static str,
    pub header: &'static str,
    pub main: &'static str,
    pub content: &'static str,
    pub sidebar_left: &'static str,
    pub sidebar_right: &'static str,
    pub footer: &'static str,
    pub javascript: &'static str,
}

pub fn resolve_template_parts(config: &ThemeConfig) -> TemplateParts {
    let pack = &config.template_pack;

    // 1. Resolve Headers
    let header = match pack.header_variant.as_str() {
        "gtk_headerbar" => include_str!("../template_parts/headers/gtk_headerbar.xml"),
        "mor" | _ => include_str!("../template_parts/headers/mor.xml"),
    };

    // 2. Resolve Main Layouts
    let main = match pack.main_variant.as_str() {
        // "single_column" => include_str!("../template_parts/layouts/single_column.xml"), // Uncomment when created
        "sidebars" | _ => include_str!("../template_parts/layouts/sidebars.xml"),
    };

    // 3. Resolve Content
    let content = match pack.content_variant.as_str() {
        "blog_standard" | _ => include_str!("../template_parts/content/blog_standard.xml"),
    };

    // 4. Resolve Left Sidebars
    let sidebar_left = match pack.left_sidebar_variant.as_str() {
        // "gtk_dock" => include_str!("../template_parts/sidebars/gtk_dock.xml"), // Uncomment when created
        "blogger_left" | _ => include_str!("../template_parts/sidebars/blogger_left.xml"),
    };

    // 5. Resolve Right Sidebars
    let sidebar_right = match pack.right_sidebar_variant.as_str() {
        "toc_right" | _ => include_str!("../template_parts/sidebars/toc_right.xml"),
    };

    // 6. Resolve Footers
    let footer = match pack.footer_variant.as_str() {
        "mor" | _ => include_str!("../template_parts/footers/mor.xml"),
    };

    // 7. Resolve Scripts
    let javascript = match pack.script_variant.as_str() {
        "minimal" => include_str!("../template_parts/scripts/minimal.xml"),
        "mor_panels" | _ => include_str!("../template_parts/scripts/mor_panels.xml"),
    };

    TemplateParts {
        // Base elements are static and immutable for Blogger compatibility
        meta: include_str!("../template_parts/base/meta.xml"),
        css: include_str!("../template_parts/base/css.xml"),
        header,
        main,
        content,
        sidebar_left,
        sidebar_right,
        footer,
        javascript,
    }
}