//! In-editor preview HTML. Produces the HTML that gets shown in the
//! right-panel preview iframe. Distinct from `theme::render_theme`, which
//! produces uploadable Blogger XML.

use crate::config::{BackgroundMode, ThemeConfig};
use crate::config::prefs::RenderPrefs;
use super::tracking::{menu_link_anchor, widget_title_h2};
use super::util::{build_google_fonts_link, escape_attr, escape_html};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewTemplateMode {
    #[default]
    Modern,
    Sidebars,
    StaticArchive,
    StaticCategories,
}

impl PreviewTemplateMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Modern => "Modern",
            Self::Sidebars => "Sidebars",
            Self::StaticArchive => "Static Archive",
            Self::StaticCategories => "Static Categories",
        }
    }
}

pub fn render_preview_html(config: &ThemeConfig, _preview_mode: PreviewTemplateMode, is_dark: bool) -> String {
    let data_theme = if is_dark { "dark" } else { "light" };
    let background_tile_css = match &config.background.mode {
        BackgroundMode::Solid { color } => format!("background-color: {};", escape_attr(color)),
        BackgroundMode::Gradient { from, to, angle_deg } => format!(
            "background: linear-gradient({}deg, {}, {});",
            angle_deg, escape_attr(from), escape_attr(to)
        ),
        BackgroundMode::Tile { url } if url.trim().is_empty() => String::new(),
        BackgroundMode::Tile { url } => format!(
            "background-image: url('{}'); background-repeat: repeat;",
            escape_attr(url)
        ),
    };

    let google_fonts_link = build_google_fonts_link(&[
        &config.typography.body_font_stack,
        &config.typography.heading_font_stack,
        &config.typography.mono_font_stack,
    ]);

    let menu_links = config
        .menu_links
        .iter()
        .enumerate()
        .filter(|(_, link)| !link.label.trim().is_empty())
        .map(|(index, link)| menu_link_anchor(index, &link.url, &link.label))
        .collect::<Vec<_>>()
        .join("");

    let site_title = escape_html(&config.site.site_title);
    let site_subtitle = escape_html(&config.site.site_subtitle);
    let footer_text = escape_html(&config.footer.footer_text);
    let label_title = widget_title_h2("Label1", config.template_pack.widget_title("Label1", "Labels"));
    let archive_title = widget_title_h2("BlogArchive1", config.template_pack.widget_title("BlogArchive1", "Archive"));
    let toc_title = widget_title_h2("HTML1", config.template_pack.widget_title("HTML1", "Table of Contents"));

    // Fetch the TRUE CSS that will be injected into the final Blogger XML
    let mut parts = crate::render::template_resolver::resolve_template_parts(config);
    let true_css = crate::render::xml_parts::css_generator::render_css_sockets(parts.css, config);

    // Wire up the Plugin Pipeline for the Preview
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
    for plugin in active_plugins {
        if let Some(js) = plugin.inject_js() {
            plugin_javascript.push_str(js);
            plugin_javascript.push('\n');
        }
    }

    parts.javascript.push('\n');
    parts.javascript.push_str(&plugin_javascript);
    
    // Securely wrap the aggregated JS for the iframe DOM
    let true_js = crate::render::xml_parts::javascript_generator::render_javascript_sockets(parts.javascript, config);

    let body_markup = format!(
        r##"
<header class="main-header" data-edit-target="colors.bg_elevated">
    <div class="header-top-row">
        <div class="header-side-controls left-controls">
            <button class="panel-toggle header-panel-toggle header-panel-toggle-left" id="mor-dock-left-toggle" data-target="panel-left"><span class="visually-hidden">Browse</span></button>
        </div>
        <a class="branding branding-link">
            <span class="institute-title" data-field-path="site.site_title">{site_title}</span>
        </a>
        <div class="header-side-controls right-controls">
            <button class="header-panel-toggle theme-toggle-btn" id="mor-theme-toggle" title="Toggle Light/Dark Mode (Use Editor UI to switch)" data-edit-target="colors.accent">
               <svg class='theme-toggle-sun' fill='currentColor' height='18' viewBox='0 0 24 24' width='18' xmlns='http://www.w3.org/2000/svg'><path d='M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0-5c.55 0 1 .45 1 1v2c0 .55-.45 1-1 1s-1-.45-1-1V3c0-.55.45-1 1-1zm0 18c.55 0 1 .45 1 1v2c0 .55-.45 1-1 1s-1-.45-1-1v-2c0-.55.45-1 1-1zM3 11h2c.55 0 1 .45 1 1s-.45 1-1 1H3c-.55 0-1-.45-1-1s.45-1 1-1zm16 0h2c.55 0 1 .45 1 1s-.45 1-1 1h-2c-.55 0-1-.45-1-1s.45-1 1-1zM5.64 4.22l1.42 1.42c.39.39.39 1.02 0 1.41s-1.02.39-1.41 0L4.22 5.64c-.39-.39-.39-1.02 0-1.41s1.03-.4 1.42-.01zm12.02 12.02l1.42 1.42c.39.39.39 1.02 0 1.41s-1.02.39-1.41 0l-1.42-1.42c-.39-.39-.39-1.02 0-1.41s1.02-.39 1.41 0zm1.42-12.02c.39.39.39 1.02 0 1.41l-1.42 1.42c-.39.39-1.02.39-1.41 0s-.39-1.02 0-1.41l1.42-1.42c.38-.39 1.02-.39 1.41 0zM5.64 17.66c.39.39.39 1.02 0 1.41l-1.42 1.42c-.39.39-1.02.39-1.41 0s-.39-1.02 0-1.41l1.42-1.42c.39-.39 1.02-.39 1.41 0z' /></svg>
               <svg class='theme-toggle-moon' fill='currentColor' height='18' viewBox='0 0 24 24' width='18' xmlns='http://www.w3.org/2000/svg'><path d='M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z' /></svg>
            </button>
            <button class="panel-toggle header-panel-toggle header-panel-toggle-right" id="mor-dock-right-toggle" data-target="panel-right"><span class="visually-hidden">Contents</span></button>
        </div>
    </div>
    <div class="header-bottom-row">
        <nav class="mor-nav">{menu_links}</nav>
        <div class="mor-search">
            <form><span class="prompt" data-edit-target="icons.search">root@moribund:~$</span><input type="text" placeholder="Search..."><button type="button" class="icon-search-btn" data-edit-target="buttons.radius" aria-label="Search"></button></form>
        </div>
    </div>
</header>

<div class="mor-workspace" data-edit-target="colors.bg_base">
    <aside class="runelite-panel panel-left" id="panel-left" data-edit-target="colors.bg_panel">
        <div class="panel-header">
            <span data-edit-target="colors.accent">Browse</span>
            <button class="panel-toggle" data-target="panel-left" data-edit-target="icons.panel_close"><span class="visually-hidden">Close</span></button>
        </div>
        <div class="panel-content sidebar-section">
            <div class="widget Label" id="Label1" data-block-id="Label1">{label_title}<div class="widget-content Label" data-edit-target="typography.body_font_stack"><ul><li><a href="#">Typography</a></li><li><a href="#">Design</a></li><li><a href="#">Dev</a></li></ul></div></div>
            <div class="widget BlogArchive" id="BlogArchive1" data-block-id="BlogArchive1">{archive_title}<div class="widget-content" data-field-path="site.site_subtitle">{site_subtitle}</div></div>
        </div>
    </aside>

    <main class="canvas-core">
        <div class="canvas-content">
            <article class="mor-post" data-edit-target="colors.bg_panel">
                <h2 class="post-title" data-edit-target="typography.heading_font_stack"><a href="#">WYSIWYG: The Tier 3 Architecture</a></h2>
                <div class="post-meta" data-edit-target="typography.mono_font_stack">
                    <span class="sys-date">[24 Oct, 2026]</span>
                    <span class="sys-tags">Tags: <a href="#">Preview</a>, <a href="#">Architecture</a></span>
                </div>
                <div class="post-body" data-edit-target="typography.body_font_stack">
                    <p>This is a 100% accurate representation of your exported Blogger XML. It maps the <strong>exact CSS class hooks, variables, and DOM structures</strong> used by the Blogger engine, completely eliminating visual guesswork.</p>
                    <p>Furthermore, Dioxus now runs a <strong>Two-Way DOM Morpher</strong> inside the iframe. Modifying colors, fonts, and text fields in the left/right docks will update the preview instantly without causing destructive iframe reloads or scroll-jumping.</p>
                    <blockquote>"WYSIWYG means What You See Is What You Get. No more making up shite."</blockquote>
                    <p>Shift+Click on any text, background, or <code data-edit-target="typography.mono_font_stack">code block</code> to instantly jump to the relevant editor panel via the JS interop bridge.</p>
                </div>
                <div class="mor-pager" style="margin-top: 20px;">
                    <button class="pager-btn" data-edit-target="buttons.radius">Read More</button>
                    <button class="pager-btn" data-edit-target="buttons.border_width">Share</button>
                </div>
            </article>
        </div>
        <footer class="mor-footer" data-edit-target="colors.bg_elevated">
            <div class="footer-sys-info">
                <p class="footer-copyright" data-field-path="footer.footer_text">{footer_text}</p>
                <div class="footer-legal-links">
                    <a href="#">Privacy policy</a> | <a href="#">Terms of use</a>
                </div>
                <button class="back-to-top-btn" type="button" data-edit-target="buttons.text_transform">Back to Top</button>
            </div>
        </footer>
    </main>

    <aside class="runelite-panel panel-right" id="panel-right" data-edit-target="colors.bg_panel">
        <div class="panel-header">
            <span data-edit-target="colors.accent">Contents</span>
            <button class="panel-toggle" data-target="panel-right" data-edit-target="icons.panel_close"><span class="visually-hidden">Close</span></button>
        </div>
        <div class="panel-content sidebar-section">
            <div class="widget HTML" id="HTML1" data-block-id="HTML1">{toc_title}<div class="widget-content" data-edit-target="typography.body_font_stack"><ul><li><a href="#">WYSIWYG: The Tier 3 Architecture</a></li><li><a href="#">Hot-swapping Variables</a></li></ul></div></div>
        </div>
    </aside>
</div>"##,
        site_title = site_title,
        site_subtitle = site_subtitle,
        menu_links = menu_links,
        footer_text = footer_text,
        label_title = label_title,
        archive_title = archive_title,
        toc_title = toc_title,
    );

    format!(
        r#"<!doctype html>
<html lang="en" data-theme="{data_theme}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{site_title}</title>
{google_fonts_link}
<style id="mor-true-css">
{true_css}
/* Minimal overrides to prevent absolute iframe bleeding */
html, body {{ overflow: hidden; }}
.canvas-core {{ overflow-y: auto; overflow-x: hidden; }}
</style>
</head>
<body style="{background_tile_css}">
    {body_markup}
    {true_js}
</body>
</html>"#,
        data_theme = data_theme,
        site_title = site_title,
        google_fonts_link = google_fonts_link,
        true_css = true_css,
        background_tile_css = background_tile_css,
        body_markup = body_markup,
        true_js = true_js
    )
}