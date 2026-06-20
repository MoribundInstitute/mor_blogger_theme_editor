//! src/render/template_resolver.rs
use crate::config::ThemeConfig;
use crate::render::css_builder::build_master_css;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentCategory {
    Header,
    Layout,
    Content,
    SidebarLeft,
    SidebarRight,
    Footer,
    Widget,
}

#[derive(Debug, Clone)]
pub struct ComponentManifest {
    pub id: &'static str,
    pub category: ComponentCategory,
    pub xml_content: &'static str,
    pub css_deps: &'static [&'static str],
    pub js_deps: &'static [&'static str],
}

impl ComponentManifest {
    pub fn inject_js(&self) -> Option<String> {
        if self.js_deps.is_empty() {
            return None;
        }
        let fragments: Vec<String> = self
            .js_deps
            .iter()
            .filter_map(|dep| {
                let src = fetch_js(dep);
                if src.is_empty() {
                    None
                } else {
                    Some(format!("/* --- [{}] {} --- */\n{}", self.id, dep, src))
                }
            })
            .collect();
        if fragments.is_empty() {
            None
        } else {
            Some(fragments.join("\n\n"))
        }
    }
}

// =====================================================================
// THE REGISTRIES
// =====================================================================

pub const HEADER_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "mor",
        category: ComponentCategory::Header,
        xml_content: include_str!("../template_parts/headers/mor_header_baseline.xml"),
        css_deps: &[
            "04-Main-Header.css",
            "05-Branding.css",
            "06-Main-Navigation.css",
            "07-Catalog-Mega-Dropdown.css",
            "08-Command-Line-Search.css",
        ],
        js_deps: &[],
    },
    ComponentManifest {
        id: "gtk_headerbar",
        category: ComponentCategory::Header,
        xml_content: include_str!("../template_parts/headers/gtk_headerbar.xml"),
        css_deps: &[
            "04-Main-Header.css",
            "05-Branding.css",
            "06-Main-Navigation.css",
        ],
        js_deps: &[],
    },
    ComponentManifest {
        id: "minimal",
        category: ComponentCategory::Header,
        xml_content: include_str!("../template_parts/headers/mor_header_minimal.xml"),
        css_deps: &[
            "04-Main-Header.css",
            "05-Branding.css",
            "06-Main-Navigation.css",
        ],
        js_deps: &[],
    },
];

pub const LAYOUT_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "sidebars",
        category: ComponentCategory::Layout,
        xml_content: include_str!("../template_parts/layouts/sidebars.xml"),
        css_deps: &[
            "09-Workspace-Layout.css",
            "10-Side-Panels.css",
            "11-Main-Canvas.css",
        ],
        js_deps: &[],
    },
    ComponentManifest {
        id: "single_column",
        category: ComponentCategory::Layout,
        xml_content: include_str!("../template_parts/layouts/single_column.xml"),
        css_deps: &["09-Workspace-Layout.css", "11-Main-Canvas.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "two_column_right",
        category: ComponentCategory::Layout,
        xml_content: include_str!("../template_parts/layouts/two_column_right.xml"),
        css_deps: &["09-Workspace-Layout.css", "11-Main-Canvas.css"],
        js_deps: &[],
    },
];

pub const CONTENT_REGISTRY: &[ComponentManifest] = &[
    ComponentManifest {
        id: "blog_standard",
        category: ComponentCategory::Content,
        xml_content: include_str!("../template_parts/content/blog_standard.xml"),
        css_deps: &[],
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
        xml_content: include_str!("../template_parts/sidebars/gtk_dock_left.xml"),
        css_deps: &["14-Widgets-Sidebars.css"],
        js_deps: &[],
    },
];

pub const SIDEBAR_RIGHT_REGISTRY: &[ComponentManifest] = &[ComponentManifest {
    id: "toc_right",
    category: ComponentCategory::SidebarRight,
    xml_content: include_str!("../template_parts/sidebars/toc_right.xml"),
    css_deps: &["16-Table-of-Contents.css"],
    js_deps: &[],
}];

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
        id: "mor",
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterMega.xml"),
        css_deps: &["18-Footer-Base.css", "18-Footer-Mega.css"],
        js_deps: &[],
    },
    ComponentManifest {
        id: "social",
        category: ComponentCategory::Footer,
        xml_content: include_str!("../template_parts/footers/MorFooterSocial.xml"),
        css_deps: &["18-Footer-Base.css"],
        js_deps: &[],
    },
];

// =====================================================================
// ASSET FETCHERS
// =====================================================================

pub fn fetch_default_css(filename: &str) -> &'static str {
    match filename {
        // Core
        "00-Root-Section.css" => include_str!("../template_parts/css/core/00-Root-Section.css"),
        "01-Reset-Base.css" => include_str!("../template_parts/css/core/01-Reset-Base.css"),
        "02-Typography-Links.css" => {
            include_str!("../template_parts/css/core/02-Typography-Links.css")
        }
        "03-Buttons.css" => include_str!("../template_parts/css/core/03-Buttons.css"),
        "17-Scrollbars.css" => include_str!("../template_parts/css/core/17-Scrollbars.css"),
        "19-Responsive-Mobile-Tablet.css" => {
            include_str!("../template_parts/css/core/19-Responsive-Mobile-Tablet.css")
        }
        "20-Responsive-Very-Small-Screens.css" => {
            include_str!("../template_parts/css/core/20-Responsive-Very-Small-Screens.css")
        }
        "21-Responsive-Desktop.css" => {
            include_str!("../template_parts/css/core/21-Responsive-Desktop.css")
        }
        "22-Export-Safety.css" => include_str!("../template_parts/css/core/22-Export-Safety.css"),

        // Headers
        "04-Main-Header.css" => include_str!("../template_parts/css/headers/04-Main-Header.css"),
        "05-Branding.css" => include_str!("../template_parts/css/headers/05-Branding.css"),
        "06-Main-Navigation.css" => {
            include_str!("../template_parts/css/headers/06-Main-Navigation.css")
        }
        "07-Catalog-Mega-Dropdown.css" => {
            include_str!("../template_parts/css/headers/07-Catalog-Mega-Dropdown.css")
        }
        "08-Command-Line-Search.css" => {
            include_str!("../template_parts/css/headers/08-Command-Line-Search.css")
        }

        // Layouts
        "09-Workspace-Layout.css" => {
            include_str!("../template_parts/css/layouts/09-Workspace-Layout.css")
        }
        "10-Side-Panels.css" => include_str!("../template_parts/css/layouts/10-Side-Panels.css"),
        "11-Main-Canvas.css" => include_str!("../template_parts/css/layouts/11-Main-Canvas.css"),

        // Content
        "12-Terminal-Post-Styling.css" => {
            include_str!("../template_parts/css/content/12-Terminal-Post-Styling.css")
        }
        "13-Pagination.css" => include_str!("../template_parts/css/content/13-Pagination.css"),
        "23-Comments.css" => include_str!("../template_parts/css/content/23-Comments.css"),
        "24-Author-Profile.css" => {
            include_str!("../template_parts/css/content/24-Author-Profile.css")
        }
        "25-Share-Menu.css" => include_str!("../template_parts/css/content/25-Share-Menu.css"),
        "30-Content-Magazine.css" => {
            include_str!("../template_parts/css/content/30-Content-Magazine.css")
        }
        "31-Content-Masonry.css" => "/* Masonry Placeholder */",
        "32-Content-Minimal.css" => "/* Minimal Placeholder */",

        // Sidebars
        "14-Widgets-Sidebars.css" => {
            include_str!("../template_parts/css/sidebars/14-Widgets-Sidebars.css")
        }
        "15-Archive-Widget.css" => {
            include_str!("../template_parts/css/sidebars/15-Archive-Widget.css")
        }
        "16-Table-of-Contents.css" => {
            include_str!("../template_parts/css/sidebars/16-Table-of-Contents.css")
        }

        // Footers
        "18-Footer-Base.css" => include_str!("../template_parts/css/footers/18-Footer-Base.css"),
        "18-Footer-Mega.css" => include_str!("../template_parts/css/footers/18-Footer-Mega.css"),

        // Features
        "26-Analytics-Dashboard.css" => {
            include_str!("../template_parts/css/features/26-Analytics-Dashboard.css")
        }

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
// WIDGET INJECTION
// =====================================================================

pub const WIDGET_REGISTRY: &[(&str, &str)] = &[
    ("Blog", include_str!("../template_parts/widgets/blog1.xml")),
    (
        "BlogArchive",
        include_str!("../template_parts/widgets/blogarchive1.xml"),
    ),
    ("HTML", include_str!("../template_parts/widgets/html1.xml")),
    (
        "Label",
        include_str!("../template_parts/widgets/label1.xml"),
    ),
];

pub(crate) fn generate_widget_xml(widget_id: &str) -> String {
    let w_type = widget_id.trim_end_matches(char::is_numeric);
    let w_type = if w_type.is_empty() { "HTML" } else { w_type };

    let template = WIDGET_REGISTRY
        .iter()
        .find(|(id, _)| *id == w_type)
        .map(|(_, xml)| *xml)
        .unwrap_or(&"");

    let xml = if template.is_empty() {
        format!(
            "<b:widget data-block-id='{widget_id}' id='{widget_id}' locked='false' title='{widget_id}' type='{w_type}' visible='true'>\n  <b:includable id='main'>\n  </b:includable>\n</b:widget>",
            widget_id = widget_id,
            w_type = w_type,
        )
    } else {
        let base_id = format!("{}1", w_type);
        template.replace(&base_id, widget_id)
    };

    crate::render::tracking::stamp_widget_block_id(xml, widget_id)
}

fn inject_widgets(
    template: &str,
    socket_id: &str,
    pack: &crate::config::TemplatePackConfig,
) -> String {
    let mut template = template.to_string();
    let placeholder = format!(
        "{{{{SOCKET_{}}}}}",
        socket_id.to_uppercase().replace("-", "_")
    );

    if template.contains(&placeholder) {
        if let Some(widget_ids) = pack.widget_map.get(socket_id) {
            let mut widgets_xml = String::new();
            for wid in widget_ids {
                widgets_xml.push_str(&generate_widget_xml(wid));
                widgets_xml.push('\n');
            }
            template = template.replace(&placeholder, &widgets_xml);
        } else {
            template = template.replace(&placeholder, "");
        }
    }
    template
}

// =====================================================================
// RESOLVER
// =====================================================================

pub struct TemplateParts {
    pub meta: &'static str,
    pub css: String,
    pub header: String,
    pub main: String,
    pub content: String,
    pub sidebar_left: String,
    pub sidebar_right: String,
    pub footer: String,
    pub javascript: String,
}

pub fn resolve_template_parts(
    config: &ThemeConfig,
    vfs: &HashMap<String, String>,
) -> TemplateParts {
    let pack = &config.template_pack;

    let get_comp = |registry: &[ComponentManifest], id: &str| -> ComponentManifest {
        registry
            .iter()
            .find(|c| c.id == id)
            .unwrap_or(&registry[0])
            .clone()
    };

    let active_components = vec![
        get_comp(HEADER_REGISTRY, &pack.header_variant),
        get_comp(LAYOUT_REGISTRY, &pack.main_variant),
        get_comp(CONTENT_REGISTRY, &pack.content_variant),
        get_comp(SIDEBAR_LEFT_REGISTRY, &pack.left_sidebar_variant),
        get_comp(SIDEBAR_RIGHT_REGISTRY, &pack.right_sidebar_variant),
        get_comp(FOOTER_REGISTRY, &pack.footer_variant),
    ];

    let mut unique_css = HashSet::new();
    let mut unique_js = HashSet::new();

    let global_css = [
        "00-Root-Section.css",
        "01-Reset-Base.css",
        "02-Typography-Links.css",
        "03-Buttons.css",
        "12-Terminal-Post-Styling.css",
        "13-Pagination.css",
        "17-Scrollbars.css",
        "19-Responsive-Mobile-Tablet.css",
        "20-Responsive-Very-Small-Screens.css",
        "21-Responsive-Desktop.css",
        "22-Export-Safety.css",
        "23-Comments.css",
        "24-Author-Profile.css",
        "25-Share-Menu.css",
        "26-Analytics-Dashboard.css",
    ];

    for css in global_css {
        unique_css.insert(css);
    }

    if pack.script_variant == "mor_panels" {
        unique_js.insert("01-Core-Helpers.js");
        unique_js.insert("07-Theme-Toggler.js");
        unique_js.insert("08-Share-Actions.js");
    }

    for comp in &active_components {
        for &css in comp.css_deps {
            unique_css.insert(css);
        }
        for &js in comp.js_deps {
            unique_js.insert(js);
        }
    }

    let mut sorted_css: Vec<_> = unique_css.into_iter().collect();
    sorted_css.sort();

    let mut js_sections: Vec<String> = Vec::new();
    let mut sorted_js: Vec<_> = unique_js.into_iter().collect();
    sorted_js.sort();

    for filename in &sorted_js {
        let src = fetch_js(filename);
        if !src.is_empty() {
            js_sections.push(format!("/* --- {} --- */\n{}", filename, src));
        }
    }

    for comp in &active_components {
        if let Some(plugin_js) = comp.inject_js() {
            js_sections.push(plugin_js);
        }
    }

    let aggregated_js = js_sections.join("\n\n");

    let css_contents: Vec<Cow<str>> = sorted_css
        .iter()
        .map(|f| {
            if let Some(custom) = vfs.get(*f) {
                Cow::Owned(custom.clone())
            } else {
                Cow::Borrowed(fetch_default_css(f))
            }
        })
        .collect();

    let css_refs: Vec<&str> = css_contents.iter().map(|c| c.as_ref()).collect();

    let header_raw = get_comp(HEADER_REGISTRY, &pack.header_variant).xml_content;
    let main_raw = get_comp(LAYOUT_REGISTRY, &pack.main_variant).xml_content;
    let content_raw = get_comp(CONTENT_REGISTRY, &pack.content_variant).xml_content;
    let sidebar_left_raw = get_comp(SIDEBAR_LEFT_REGISTRY, &pack.left_sidebar_variant).xml_content;
    let sidebar_right_raw =
        get_comp(SIDEBAR_RIGHT_REGISTRY, &pack.right_sidebar_variant).xml_content;
    let footer_raw = get_comp(FOOTER_REGISTRY, &pack.footer_variant).xml_content;

    TemplateParts {
        meta: include_str!("../template_parts/meta/meta.xml"),
        css: build_master_css(&css_refs, config),
        javascript: aggregated_js,
        header: inject_widgets(header_raw, "header", pack),
        main: inject_widgets(main_raw, "main", pack),
        content: inject_widgets(content_raw, "content", pack),
        sidebar_left: inject_widgets(sidebar_left_raw, "sidebar-left", pack),
        sidebar_right: inject_widgets(sidebar_right_raw, "sidebar-right", pack),
        footer: inject_widgets(footer_raw, "footer", pack),
    }
}