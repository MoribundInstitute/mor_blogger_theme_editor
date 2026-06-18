//! Low-level Blogger XML generator.
//! Orchestrator. Delegates rendering to component modules.

use crate::config::prefs::RenderPrefs;
use crate::config::ThemeConfig;
use crate::render::template_resolver::resolve_template_parts;
use crate::render::util::{escape_attr, escape_html, first_non_empty};

use super::xml_parts::{
    content_generator::render_content_sockets,
    css_generator::render_css_sockets,
    footer_generator::render_footer_sockets,
    header_generator::render_header_sockets,
    javascript_generator::render_javascript_sockets,
    layout_compiler::render_layout_blocks,
    meta_generator::render_meta_sockets,
    sidebar_generator::render_sidebar_sockets,
    widget_generator::render_widget_sockets,
};

#[derive(Debug, Clone)]
pub struct XmlNode {
    pub target_socket: &'static str,
    pub content: String,
}

impl XmlNode {
    pub fn new(target_socket: &'static str, content: &str) -> Self {
        Self {
            target_socket,
            content: content.to_string(),
        }
    }
}

fn assemble_template(
    meta: &str,
    css: &str,
    header: &str,
    sidebar_left: &str,
    main_layout: &str,
    sidebar_right: &str,
    javascript: &str,
    ads_consent: &str,
    ads_runtime: &str,
) -> String {
    format!(
        "{meta}\n<b:skin><![CDATA[\n{css}\n]]></b:skin>\n<b:template-skin><![CDATA[]]></b:template-skin>\n{{{{PLUGIN_HEAD_XML}}}}\n</head>\n<body>\n{{{{PLUGIN_BODY_TOP}}}}\n{header}\n<div class='mor-workspace'>\n{sidebar_left}\n{main_layout}\n{sidebar_right}\n</div>\n{ads_consent}\n{ads_runtime}\n{javascript}\n</body>\n</html>"
    )
}

pub(super) fn render_template(config: &ThemeConfig) -> String {
    let mut parts = resolve_template_parts(config);

    let mut active_plugins: Vec<Box<dyn crate::render::plugins::MorBloggerPlugin>> = Vec::new();
    if let Ok(toml_str) = std::fs::read_to_string(crate::config::prefs::editor_prefs_path()) {
        if let Ok(prefs) = toml::from_str::<RenderPrefs>(&toml_str) {
            for p in prefs.plugins {
                if p.enabled {
                    match p.id.as_str() {
                        "os_chameleon" => active_plugins.push(Box::new(crate::render::plugins::OsChameleonPlugin)),
                        "dewey_indexer" => active_plugins.push(Box::new(crate::render::plugins::DeweyIndexerPlugin)),
                        "workspace_docks" => active_plugins.push(Box::new(crate::render::plugins::WorkspaceDocksPlugin)),
                        _ => {}
                    }
                }
            }
        }
    }

    let mut plugin_javascript = String::new();
    let mut plugin_custom_css = String::new();
    let mut plugin_widgets: std::collections::HashMap<&str, String> = std::collections::HashMap::new();

    for plugin in active_plugins {
        if let Some(js) = plugin.inject_js() {
            plugin_javascript.push_str(&js);
            plugin_javascript.push('\n');
        }
        if let Some(css) = plugin.inject_css() {
            plugin_custom_css.push_str(&css);
            plugin_custom_css.push('\n');
        }
        if let Some(widgets) = plugin.inject_xml_widgets() {
            for widget in widgets {
                let current = plugin_widgets.entry(widget.target_socket).or_insert_with(String::new);
                current.push_str(&widget.content);
                current.push('\n');
            }
        }
    }

    if !plugin_custom_css.is_empty() {
        parts.css.push_str("\n/* ===== Dynamic Plugin CSS ===== */\n");
        parts.css.push_str(&plugin_custom_css);
    }

    parts.javascript.push('\n');
    parts.javascript.push_str(&plugin_javascript);

    let meta = render_meta_sockets(parts.meta.to_string(), config);
    let css = render_css_sockets(parts.css, config);
    let header = render_header_sockets(parts.header, config);
    let left_sidebar = render_widget_sockets(
        render_sidebar_sockets(parts.sidebar_left, config, "LEFT"),
        config,
    );
    let right_sidebar = render_widget_sockets(
        render_sidebar_sockets(parts.sidebar_right, config, "RIGHT"),
        config,
    );

    let content = render_widget_sockets(render_content_sockets(parts.content, config), config);
    let footer = render_footer_sockets(parts.footer, config);
    
    let main_layout = render_layout_blocks(
        parts
            .main
            .replace("{{MAIN_CONTENT_MODULE}}", &content)
            .replace("{{FOOTER_MODULE}}", &footer),
        config,
    );
        
    let scripts = render_javascript_sockets(parts.javascript, config);

    let ads_consent_banner = crate::render::ads::render_ads_consent_banner(&config.ads);
    let ads_runtime_script = crate::render::ads::render_ads_runtime_script(&config.ads);

    let mut final_xml = assemble_template(
        &meta,
        &css,
        &header,
        &left_sidebar,
        &main_layout,
        &right_sidebar,
        &scripts,
        &ads_consent_banner,
        &ads_runtime_script,
    );

    // Apply plugin injections
    for (socket, content) in plugin_widgets {
        final_xml = final_xml.replace(socket, &content);
    }

    // --- GLOBAL IDENTITY SOCKETS ---
    // These must be applied to the fully assembled template because they appear across 
    // multiple domains (SEO Meta, Header Branding, Content Schema, User Plugins).
    let site_home_url = first_non_empty(&config.site.home_url, "/");
    final_xml = final_xml.replace("{{SITE_TITLE}}", &escape_html(&config.site.site_title));
    final_xml = final_xml.replace("{{SITE_TITLE_ATTR}}", &escape_attr(&config.site.site_title));
    final_xml = final_xml.replace("{{SITE_SUBTITLE}}", &escape_html(&config.site.site_subtitle));
    final_xml = final_xml.replace("{{SITE_SUBTITLE_ATTR}}", &escape_attr(&config.site.site_subtitle));
    final_xml = final_xml.replace("{{HEADER_LOGO_URL}}", &escape_attr(&config.site.header_logo_url));
    final_xml = final_xml.replace("{{HEADER_LOGO_URL_ATTR}}", &escape_attr(&config.site.header_logo_url));
    final_xml = final_xml.replace("{{SITE_HOME_URL}}", &escape_attr(site_home_url));
    final_xml = final_xml.replace("{{SITE_HOME_URL_ATTR}}", &escape_attr(site_home_url));
    final_xml = final_xml.replace("{{HOME_URL}}", &escape_attr(site_home_url));

    // Cleanup empty plugin sockets
    final_xml = final_xml.replace("{{PLUGIN_WIDGET_SIDEBAR_RIGHT}}", "");
    final_xml = final_xml.replace("{{PLUGIN_WIDGET_SIDEBAR_LEFT}}", "");
    final_xml = final_xml.replace("{{PLUGIN_WIDGET_HEADER}}", "");
    final_xml = final_xml.replace("{{PLUGIN_WIDGET_FOOTER}}", "");
    final_xml = final_xml.replace("{{PLUGIN_HEAD_XML}}", "");
    final_xml = final_xml.replace("{{PLUGIN_BODY_TOP}}", "");

    crate::render::tracking::stamp_all_widget_block_ids(final_xml)
}