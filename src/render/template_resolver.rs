use crate::config::ThemeConfig; 
use crate::render::css_builder::build_master_css;

pub struct TemplateParts {
    pub meta: &'static str,
    pub css: String,
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
        "mor" | _ => include_str!("../template_parts/footers/MorFooterBaseline.xml"),
    };

    // 7. Resolve Scripts
    let javascript = match pack.script_variant.as_str() {
        "minimal" => include_str!("../template_parts/scripts/minimal.xml"),
        "mor_panels" | _ => include_str!("../template_parts/scripts/mor_panels.xml"),
    };

    // Dynamically build the master CSS from the 23 modular skin files
    let master_css = build_master_css(&[
        include_str!("../template_parts/base/skin/00-Root-Section.css"),
        include_str!("../template_parts/base/skin/01-Reset-Base.css"),
        include_str!("../template_parts/base/skin/02-Typography-Links.css"),
        include_str!("../template_parts/base/skin/03-Buttons.css"),
        include_str!("../template_parts/base/skin/04-Main-Header.css"),
        include_str!("../template_parts/base/skin/05-Branding.css"),
        include_str!("../template_parts/base/skin/06-Main-Navigation.css"),
        include_str!("../template_parts/base/skin/07-Catalog-Mega-Dropdown.css"),
        include_str!("../template_parts/base/skin/08-Command-Line-Search.css"),
        include_str!("../template_parts/base/skin/09-Workspace-Layout.css"),
        include_str!("../template_parts/base/skin/10-Side-Panels.css"),
        include_str!("../template_parts/base/skin/11-Main-Canvas.css"),
        include_str!("../template_parts/base/skin/12-Terminal-Post-Styling.css"),
        include_str!("../template_parts/base/skin/13-Pagination.css"),
        include_str!("../template_parts/base/skin/14-Widgets-Sidebars.css"),
        include_str!("../template_parts/base/skin/15-Archive-Widget.css"),
        include_str!("../template_parts/base/skin/16-Table-of-Contents.css"),
        include_str!("../template_parts/base/skin/17-Scrollbars.css"),
        include_str!("../template_parts/base/skin/18-Footer.css"),
        include_str!("../template_parts/base/skin/19-Responsive-Mobile-Tablet.css"),
        include_str!("../template_parts/base/skin/20-Responsive-Very-Small-Screens.css"),
        include_str!("../template_parts/base/skin/21-Responsive-Desktop.css"),
        include_str!("../template_parts/base/skin/22-Export-Safety.css"),
    ]);

    TemplateParts {
        // Base elements are static and immutable for Blogger compatibility
        meta: include_str!("../template_parts/base/meta.xml"),
        css: master_css,
        header,
        main,
        content,
        sidebar_left,
        sidebar_right,
        footer,
        javascript,
    }
}