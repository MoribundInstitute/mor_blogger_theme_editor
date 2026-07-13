//! In-editor preview HTML. Produces the HTML that gets shown in the
//! right-panel preview iframe. Distinct from `theme::render_theme`, which
//! produces uploadable Blogger XML.

use super::tracking::{menu_link_anchor, widget_title_h2};
use super::util::{escape_attr, escape_html, unescape_for_style};
use crate::config::{BackgroundMode, BlogPost, ThemeConfig};
use std::collections::HashMap;

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

fn get_default_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            title: "Welcome to Your Live Preview".to_string(),
            date: "24 Oct, 2026".to_string(),
            tags: vec!["Preview".to_string(), "Getting Started".to_string()],
            snippet: "This preview shows exactly how your blog will look once your theme is published to Blogger: the same layout, fonts, and colors your readers will see. No surprises after export.".to_string(),
            featured_image: None,
            body: "<p>This preview shows exactly how your blog will look once your theme is published to Blogger: the same layout, fonts, and colors your readers will see. No surprises after export.</p>\n<p><strong>Try it now:</strong> pick a different color or font in the side panels and watch this page update instantly. Nothing reloads and you never lose your place on the page.</p>\n<blockquote>\"What you see here is what your readers get. This preview is built from the very same theme file you upload to Blogger.\"</blockquote>\n<p>Handy shortcut: turn on <strong>X-Ray</strong> (the skeleton button in the toolbar), then click any text, background, or <code data-edit-target=\"typography.mono_font_stack\">code snippet</code> to jump straight to the setting that controls it. Hold <strong>Shift</strong> and click any icon to swap it.</p>".to_string(),
            url: "#".to_string(),
            author_name: "Moribund Engine".to_string(),
        }
    ]
}

fn format_posts_for_preview(posts: &[BlogPost], content_variant: &str) -> String {
    let default_posts = get_default_posts();
    let posts_to_render = if posts.is_empty() {
        &default_posts
    } else {
        posts
    };

    match content_variant {
        "mor_magazine" => {
            let mut html = String::new();
            html.push_str("<div class=\"mor-magazine-feed\">");
            for (i, post) in posts_to_render.iter().enumerate() {
                let class_name = if i == 0 {
                    "mor-post hero-post"
                } else {
                    "mor-post grid-post"
                };
                let img_html = match &post.featured_image {
                    Some(img) if !img.is_empty() => format!(
                        "<img class=\"post-thumbnail\" alt=\"{}\" src=\"{}\"/>",
                        escape_attr(&post.title),
                        escape_attr(img)
                    ),
                    _ => String::new(),
                };
                html.push_str(&format!(
                    r##"<article class="{class_name}" data-edit-target="colors.bg_panel">
                        {img_html}
                        <h2 class="post-title" data-edit-target="typography.heading_font_stack"><a href="{url}">{title}</a></h2>
                        <div class="post-meta" data-edit-target="typography.mono_font_stack">
                            <span class="sys-date">{date}</span>
                            {author_span}
                        </div>
                        <div class="post-body" data-edit-target="typography.body_font_stack">
                            {snippet}
                        </div>
                    </article>"##,
                    class_name = class_name,
                    img_html = img_html,
                    url = escape_attr(&post.url),
                    title = escape_html(&post.title),
                    date = escape_html(&post.date),
                    author_span = if i == 0 { format!(" | <span class=\"post-author\">{}</span>", escape_html(&post.author_name)) } else { String::new() },
                    snippet = if i == 0 { &post.body } else { &post.snippet }
                ));
            }
            html.push_str("</div>");
            html
        }
        "mor_masonry" => {
            let mut html = String::new();
            html.push_str("<div class=\"mor-masonry-feed\">");
            for post in posts_to_render {
                let img_html = match &post.featured_image {
                    Some(img) if !img.is_empty() => format!(
                        "<img class=\"post-thumbnail\" alt=\"{}\" src=\"{}\"/>",
                        escape_attr(&post.title),
                        escape_attr(img)
                    ),
                    _ => String::new(),
                };
                html.push_str(&format!(
                    r##"<article class="mor-post masonry-card" data-edit-target="colors.bg_panel">
                        {img_html}
                        <h2 class="post-title" data-edit-target="typography.heading_font_stack"><a href="{url}">{title}</a></h2>
                        <div class="post-meta" data-edit-target="typography.mono_font_stack">
                            {date}
                        </div>
                        <div class="post-body" data-edit-target="typography.body_font_stack">
                            {snippet}
                        </div>
                    </article>"##,
                    img_html = img_html,
                    url = escape_attr(&post.url),
                    title = escape_html(&post.title),
                    date = escape_html(&post.date),
                    snippet = escape_html(&post.snippet)
                ));
            }
            html.push_str("</div>");
            html
        }
        "mor_minimal" => {
            let mut html = String::new();
            html.push_str("<div class=\"mor-minimal-feed\">");
            for post in posts_to_render {
                let tags_html = if post.tags.is_empty() {
                    String::new()
                } else {
                    let mut links = String::new();
                    links.push_str("<div class=\"post-tags\">");
                    for tag in &post.tags {
                        links.push_str(&format!(
                            "<a class=\"minimal-tag\" href=\"#\">#{}</a> ",
                            escape_html(tag)
                        ));
                    }
                    links.push_str("</div>");
                    links
                };
                html.push_str(&format!(
                    r##"<article class="mor-post minimal-row" data-edit-target="colors.bg_panel">
                        <div class="post-date">{date}</div>
                        <h2 class="post-title" data-edit-target="typography.heading_font_stack"><a href="{url}">{title}</a></h2>
                        {tags_html}
                    </article>"##,
                    date = escape_html(&post.date),
                    url = escape_attr(&post.url),
                    title = escape_html(&post.title),
                    tags_html = tags_html
                ));
            }
            html.push_str("</div>");
            html
        }
        _ => {
            // Standard feed layout (blog_standard)
            let mut html = String::new();
            for post in posts_to_render {
                let tags_html = if post.tags.is_empty() {
                    String::new()
                } else {
                    let links = post
                        .tags
                        .iter()
                        .map(|t| format!("<a href='#'>{}</a>", escape_html(t)))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(" <span class=\"sys-tags\">Tags: {}</span>", links)
                };
                let img_html = match &post.featured_image {
                    Some(img) if !img.is_empty() => format!(
                        "<img class=\"post-thumbnail\" alt=\"{}\" src=\"{}\"/>",
                        escape_attr(&post.title),
                        escape_attr(img)
                    ),
                    _ => String::new(),
                };
                html.push_str(&format!(
                    r##"<article class="mor-post" data-edit-target="colors.bg_panel">
                        {img_html}
                        <h2 class="post-title" data-edit-target="typography.heading_font_stack"><a href="{url}">{title}</a></h2>
                        <div class="post-meta" data-edit-target="typography.mono_font_stack">
                            <span class="sys-date">[{date}]</span>
                            {tags_html}
                        </div>
                        <div class="post-body" data-edit-target="typography.body_font_stack">
                            {body}
                        </div>
                        <div class="mor-pager" style="margin-top: 20px;">
                            <button class="pager-btn" data-edit-target="buttons.radius">Read More</button>
                        </div>
                    </article>"##,
                    img_html = img_html,
                    url = escape_attr(&post.url),
                    title = escape_html(&post.title),
                    date = escape_html(&post.date),
                    tags_html = tags_html,
                    body = post.body
                ));
            }
            html
        }
    }
}

// ── Shared preview section builders ─────────────────────────────────────────
//
// The live preview and the per-module workbench preview render the SAME markup by
// calling these, so they can never drift apart. (Both still rely on the real
// resolved CSS via `render_css_sockets`.)

/// The preview `<header>` (branding, theme toggle, nav, search).
pub fn preview_header_html(config: &ThemeConfig) -> String {
    let header_extra_class = if config.template_pack.header_variant == "mor_search_center" {
        " search-centered"
    } else {
        ""
    };
    let site_title = escape_html(&config.site.site_title);
    let branding_inner = if config.site.header_logo_url.trim().is_empty() {
        format!(r#"<span class="institute-title" data-field-path="site.site_title">{site_title}</span>"#)
    } else {
        format!(
            r#"<img alt="{} logo" class="institute-logo" src="{}"/>"#,
            escape_attr(&config.site.site_title),
            escape_attr(&config.site.header_logo_url)
        )
    };
    let menu_links = config
        .menu_links
        .iter()
        .enumerate()
        .filter(|(_, link)| !link.label.trim().is_empty())
        .map(|(index, link)| menu_link_anchor(index, &link.url, &link.label))
        .collect::<Vec<_>>()
        .join("");

    // ponytail: preview mirror of the self-contained bell + glowing title baked
    // into mor_header_search.xml. The live preview builds its own header and
    // can't run the module's b:section / inline <style>/<script>, so we restate
    // them here (newest post = first sample post). Keep in sync with the module.
    let (bell_html, bell_assets) = if header_extra_class == " search-centered" {
        let sample = get_default_posts();
        let (title, snippet, url) = sample
            .first()
            .map(|p| {
                (
                    escape_html(&p.title),
                    escape_html(&p.snippet),
                    escape_attr(&p.url),
                )
            })
            .unwrap_or_default();
        let bell = format!(
            r##"<nav class="mor-bell">
              <span class="mor-bell-btn" role="button" tabindex="0" aria-label="Newest post">
                <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/><path d="M13.73 21a2 2 0 0 1-3.46 0"/></svg>
              </span>
              <div class="mor-bell-panel">
                <a class="mor-bell-link" href="{url}">
                  <span class="mor-bell-eyebrow">Latest</span>
                  <h3 class="mor-bell-title">{title}</h3>
                  <p class="mor-bell-snippet">{snippet}</p>
                </a>
              </div>
            </nav>"##
        );
        let assets = r##"<style>
.search-centered .branding { justify-content: center; }
.search-centered .institute-title { font-family: var(--font-heading, inherit); font-size: clamp(2rem, 6vw, 4rem); line-height: 1.1; color: var(--fg-base, #ececeb); text-shadow: 0 0 1rem var(--glow, rgba(255,255,255,.35)), 0 0 2rem var(--glow, rgba(255,255,255,.22)); transition: color .3s ease, text-shadow .3s ease; }
.search-centered .branding-link:hover .institute-title { color: var(--accent, #fff); text-shadow: 0 0 1.5rem var(--glow, rgba(255,255,255,.6)), 0 0 3rem var(--glow, rgba(255,255,255,.4)); }
.mor-bell { position: relative; display: inline-flex; align-items: center; }
.mor-bell-btn { display: inline-flex; cursor: pointer; color: var(--fg-base, #ececeb); transition: color .2s ease, filter .2s ease; }
.mor-bell-btn:hover { color: var(--accent, #fff); filter: drop-shadow(0 0 6px var(--glow, rgba(255,255,255,.5))); }
/* The bell swings when you reach for it. */
.mor-bell:hover .mor-bell-btn svg { animation: morBellRing .6s ease; transform-origin: 50% 3px; }
@keyframes morBellRing { 0%,100% { transform: rotate(0); } 20% { transform: rotate(15deg); } 40% { transform: rotate(-11deg); } 60% { transform: rotate(7deg); } 80% { transform: rotate(-4deg); } }
.mor-bell-panel { opacity: 0; visibility: hidden; transform: translateY(-8px) scale(.96); transform-origin: top right; transition: opacity .2s ease, transform .26s cubic-bezier(.34,1.32,.64,1), visibility .26s; position: absolute; top: calc(100% + 12px); right: 0; width: 320px; max-width: min(320px, 84vw); text-align: left; background: var(--bg-panel, #1c1c1e); color: var(--fg-base, #ececeb); border: 1px solid color-mix(in srgb, var(--accent, #8aa) 30%, transparent); border-radius: 14px; padding: 16px; box-shadow: 0 20px 48px -16px rgba(0,0,0,.75), 0 0 28px -10px var(--glow, transparent); -webkit-backdrop-filter: blur(10px); backdrop-filter: blur(10px); z-index: 9999; }
/* preview-only: the iframe morpher doesn't run the toggle script, so open on
   hover/focus too. The exported header keeps the real click-to-toggle JS. */
.mor-bell.open .mor-bell-panel, .mor-bell:hover .mor-bell-panel, .mor-bell:focus-within .mor-bell-panel { opacity: 1; visibility: visible; transform: translateY(0) scale(1); }
.mor-bell-panel::after { content: ""; position: absolute; left: 16px; right: 16px; top: 0; height: 2px; background: linear-gradient(90deg, transparent, var(--accent, #6cf), transparent); opacity: .85; }
.mor-bell-panel::before { content: ""; position: absolute; top: -6px; right: 18px; width: 12px; height: 12px; background: var(--bg-panel, #1c1c1e); border-left: 1px solid color-mix(in srgb, var(--accent, #8aa) 30%, transparent); border-top: 1px solid color-mix(in srgb, var(--accent, #8aa) 30%, transparent); transform: rotate(45deg); }
.mor-bell-link { display: block; text-decoration: none; color: inherit; }
.mor-bell-eyebrow { display: inline-flex; align-items: center; gap: 7px; margin-bottom: 10px; font-size: .62rem; font-weight: 700; letter-spacing: .2em; text-transform: uppercase; color: var(--accent, #6cf); }
.mor-bell-eyebrow::before { content: ""; width: 6px; height: 6px; border-radius: 50%; background: var(--accent, #6cf); box-shadow: 0 0 6px var(--accent, #6cf); animation: morDotPulse 1.8s ease-in-out infinite; }
@keyframes morDotPulse { 0%,100% { box-shadow: 0 0 4px var(--accent, #6cf); opacity: .8; } 50% { box-shadow: 0 0 12px var(--accent, #6cf); opacity: 1; } }
.mor-bell-title { display: block; margin: 0 0 7px; font-size: 1.05rem; line-height: 1.25; font-weight: 700; font-family: var(--font-heading, inherit); color: var(--fg-base, #fff); transition: color .2s ease, text-shadow .2s ease; }
.mor-bell-link:hover .mor-bell-title { color: var(--accent, #fff); text-shadow: 0 0 10px var(--glow, rgba(255,255,255,.45)); }
@media (prefers-reduced-motion: reduce) { .mor-bell-panel { transition: opacity .15s ease, visibility .15s; transform: none; } .mor-bell.open .mor-bell-panel, .mor-bell:hover .mor-bell-panel, .mor-bell:focus-within .mor-bell-panel { transform: none; } .mor-bell:hover .mor-bell-btn svg { animation: none; } .mor-bell-eyebrow::before { animation: none; } }
.mor-bell-snippet { display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden; margin: 0; font-size: .85rem; line-height: 1.5; color: var(--fg-muted, #b9b9b8); }
</style>
<script>
(function () {
  if (window.__morHeaderBellInit) return;
  window.__morHeaderBellInit = true;
  document.addEventListener('click', function (e) {
    var bell = document.querySelector('.mor-bell'); if (!bell) return;
    if (e.target.closest('.mor-bell-btn')) { e.preventDefault(); e.stopPropagation(); bell.classList.toggle('open'); return; }
    if (!e.target.closest('.mor-bell-panel')) bell.classList.remove('open');
  });
  document.addEventListener('keydown', function (e) { if (e.key === 'Escape') { var m = document.querySelector('.mor-bell.open'); if (m) m.classList.remove('open'); } });
})();
</script>"##
        .to_string();
        (bell, assets)
    } else {
        (String::new(), String::new())
    };

    format!(
        r##"<header class="main-header{header_extra_class}" data-edit-target="colors.bg_elevated">
    <div class="header-top-row">
        <div class="header-side-controls left-controls">
            <button class="panel-toggle header-panel-toggle header-panel-toggle-left" id="mor-dock-left-toggle" data-target="panel-left" data-edit-target="icons.sidebar_left"><span class="visually-hidden">Browse</span></button>
        </div>
        <a class="branding branding-link">
            {branding_inner}
        </a>
        <div class="header-side-controls right-controls">
            <button class="header-panel-toggle theme-toggle-btn" id="mor-theme-toggle" title="Toggle Light/Dark Mode (Use Editor UI to switch)" data-edit-target="colors.accent">
               <svg class='theme-toggle-sun' fill='currentColor' height='18' viewBox='0 0 24 24' width='18' xmlns='http://www.w3.org/2000/svg'><path d='M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0-5c.55 0 1 .45 1 1v2c0 .55-.45 1-1 1s-1-.45-1-1V3c0-.55.45-1 1-1zm0 18c.55 0 1 .45 1 1v2c0 .55-.45 1-1 1s-1-.45-1-1v-2c0-.55.45-1 1-1zM3 11h2c.55 0 1 .45 1 1s-.45 1-1 1H3c-.55 0-1-.45-1-1s.45-1 1-1zm16 0h2c.55 0 1 .45 1 1s-.45 1-1 1h-2c-.55 0-1-.45-1-1s.45-1 1-1zM5.64 4.22l1.42 1.42c.39.39.39 1.02 0 1.41s-1.02.39-1.41 0L4.22 5.64c-.39-.39-.39-1.02 0-1.41s1.03-.4 1.42-.01zm12.02 12.02l1.42 1.42c.39.39.39 1.02 0 1.41s-1.02.39-1.41 0l-1.42-1.42c-.39-.39-.39-1.02 0-1.41s1.02-.39 1.41 0zm1.42-12.02c.39.39.39 1.02 0 1.41l-1.42 1.42c-.39.39-1.02.39-1.41 0s-.39-1.02 0-1.41l1.42-1.42c.38-.39 1.02-.39 1.41 0zM5.64 17.66c.39.39.39 1.02 0 1.41l-1.42 1.42c-.39.39-1.02.39-1.41 0s-.39-1.02 0-1.41l1.42-1.42c.39-.39 1.02-.39 1.41 0z' /></svg>
               <svg class='theme-toggle-moon' fill='currentColor' height='18' viewBox='0 0 24 24' width='18' xmlns='http://www.w3.org/2000/svg'><path d='M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z' /></svg>
            </button>
            {bell_html}
            <button class="panel-toggle header-panel-toggle header-panel-toggle-right" id="mor-dock-right-toggle" data-target="panel-right" data-edit-target="icons.sidebar_right"><span class="visually-hidden">Contents</span></button>
        </div>
    </div>
    <div class="header-bottom-row">
        <nav class="mor-nav">{menu_links}</nav>
        <div class="mor-search">
            <form><span class="prompt">root@moribund:~$</span><input type="text" placeholder="Search..."><button type="button" class="icon-search-btn" data-edit-target="icons.search" aria-label="Search"></button></form>
        </div>
    </div>
    {bell_assets}
</header>"##
    )
}

/// The left sidebar panel (Labels + Archive sample widgets).
pub fn preview_left_panel_html(config: &ThemeConfig) -> String {
    let site_subtitle = escape_html(&config.site.site_subtitle);
    let label_title = widget_title_h2("Label1", config.template_pack.widget_title("Label1", "Labels"));
    let archive_title = widget_title_h2(
        "BlogArchive1",
        config.template_pack.widget_title("BlogArchive1", "Archive"),
    );
    format!(
        r##"<aside class="mor-panel panel-left" id="panel-left" data-edit-target="colors.bg_panel">
        <div class="panel-header">
            <span data-edit-target="colors.accent">Browse</span>
            <button class="panel-toggle" data-target="panel-left" data-edit-target="icons.panel_close"><span class="visually-hidden">Close</span></button>
        </div>
        <div class="panel-content sidebar-section">
            <div class="widget Label" id="Label1" data-block-id="Label1">{label_title}<div class="widget-content Label" data-edit-target="typography.body_font_stack"><ul><li><a href="#">Typography</a></li><li><a href="#">Design</a></li><li><a href="#">Dev</a></li></ul></div></div>
            <div class="widget BlogArchive" id="BlogArchive1" data-block-id="BlogArchive1">{archive_title}<div class="widget-content" data-field-path="site.site_subtitle">{site_subtitle}</div></div>
        </div>
    </aside>"##
    )
}

/// The right sidebar panel (Table of Contents sample widget).
pub fn preview_right_panel_html(config: &ThemeConfig) -> String {
    let toc_title = widget_title_h2(
        "HTML1",
        config.template_pack.widget_title("HTML1", "Table of Contents"),
    );
    format!(
        r##"<aside class="mor-panel panel-right" id="panel-right" data-edit-target="colors.bg_panel">
        <div class="panel-header">
            <span data-edit-target="colors.accent">Contents</span>
            <button class="panel-toggle" data-target="panel-right" data-edit-target="icons.panel_close"><span class="visually-hidden">Close</span></button>
        </div>
        <div class="panel-content sidebar-section">
            <div class="widget HTML" id="HTML1" data-block-id="HTML1">{toc_title}<div class="widget-content" data-edit-target="typography.body_font_stack"><ul><li><a href="#">Welcome to Your Live Preview</a></li><li><a href="#">Editing with Shift+Click</a></li></ul></div></div>
        </div>
    </aside>"##
    )
}

/// The post feed (`canvas-content`) for the active content variant.
pub fn preview_content_html(config: &ThemeConfig, posts: &[BlogPost]) -> String {
    let posts_html = format_posts_for_preview(posts, &config.template_pack.content_variant);
    format!(r##"<div class="canvas-content">
            {posts_html}
        </div>"##)
}

/// The simple site footer (copyright, legal links, back-to-top), preceded by the
/// optional footer gadget area (widgets dropped into the `{{SOCKET_FOOTER}}` slot).
pub fn preview_footer_html(config: &ThemeConfig) -> String {
    let footer_text = escape_html(&config.footer.footer_text);
    let gadgets: String = config
        .template_pack
        .widget_map
        .get("footer")
        .map(|ids| {
            ids.iter()
                .map(|id| {
                    let title = config.template_pack.widget_title(id, id);
                    format!(
                        r#"<div class="widget" data-block-id="{}" style="padding:6px 12px;">{}</div>"#,
                        escape_attr(id),
                        escape_html(title)
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default();
    let gadget_area = if gadgets.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="footer-gadget-area" style="display:flex;flex-wrap:wrap;gap:12px;justify-content:center;padding:8px 0;">{gadgets}</div>"#
        )
    };
    format!(
        r##"<footer class="mor-footer" data-edit-target="colors.bg_elevated">
            {gadget_area}
            <div class="footer-sys-info">
                <p class="footer-copyright" data-field-path="footer.footer_text">{footer_text}</p>
                <div class="footer-legal-links">
                    <a href="#">Privacy policy</a> | <a href="#">Terms of use</a>
                </div>
                <button class="back-to-top-btn" type="button" data-edit-target="buttons.text_transform">Back to Top</button>
            </div>
        </footer>"##
    )
}

/// Render a SINGLE widget, by Blogger type, with representative dummy data,
/// wrapped in the theme's widget markup so it picks up the live CSS. Used by the
/// Widget Workbench's master-canvas preview for dynamic widgets that have no
/// static HTML of their own. Unknown types fall back to a generic gadget card.
pub fn preview_widget_html(
    config: &ThemeConfig,
    w_type: &str,
    title: &str,
    posts: &[BlogPost],
) -> String {
    // Content-area widgets render inside the main canvas with real post cards.
    if matches!(w_type, "Blog" | "FeaturedPost") {
        return format!(
            r#"<main class="canvas-core" style="max-width:760px;margin:0 auto;">{}</main>"#,
            preview_content_html(config, posts)
        );
    }

    let id = format!("{w_type}1");
    let label = if title.trim().is_empty() {
        match w_type {
            "BlogArchive" => "Archive",
            "Label" | "Labels" => "Labels",
            "PageList" => "Pages",
            "Profile" => "About",
            "Subscribe" => "Subscribe To",
            _ => w_type,
        }
    } else {
        title
    };
    let head = widget_title_h2(&id, label);
    let content = match w_type {
        "BlogArchive" => r##"<ul class="archive-list"><li><a href="#">October 2026 <span class="post-count">(12)</span></a></li><li><a href="#">September 2026 <span class="post-count">(8)</span></a></li><li><a href="#">August 2026 <span class="post-count">(5)</span></a></li></ul>"##.to_string(),
        "Label" | "Labels" => r##"<ul class="label-list"><li><a href="#">Typography</a></li><li><a href="#">Design</a></li><li><a href="#">Dev</a></li><li><a href="#">Architecture</a></li></ul>"##.to_string(),
        "PageList" => r##"<ul class="page-list"><li><a href="#">Home</a></li><li><a href="#">About</a></li><li><a href="#">Archive</a></li><li><a href="#">Contact</a></li></ul>"##.to_string(),
        "Profile" => r#"<p>A short author bio renders here on the live blog, pulled from your Blogger profile.</p>"#.to_string(),
        "Subscribe" => r##"<ul class="subscribe-list" style="list-style:none;padding:0;margin:0;display:flex;flex-direction:column;gap:6px;"><li class="subscribe-wrapper" style="display:flex;align-items:center;gap:8px;"><span class="feed-icon">📡</span> Posts <a href="#">Atom</a></li><li class="subscribe-wrapper" style="display:flex;align-items:center;gap:8px;"><span class="feed-icon">💬</span> Comments <a href="#">Atom</a></li></ul>"##.to_string(),
        "Wikipedia" => r#"<form class="wikipedia-search-form" onsubmit="return false;" style="display:flex;gap:6px;align-items:center;"><img class="wikipedia-icon" src="https://resources.blogblog.com/img/widgets/icon_wikipedia_w.png" style="width:24px;height:24px;flex:0 0 auto;"/><input class="wikipedia-search-input" type="text" placeholder="Search Wikipedia…" style="flex:1;min-width:0;"/><input class="wikipedia-search-button" type="submit" value="Go"/></form>"#.to_string(),
        "Translate" => r#"<div id="google_translate_element"><label style="display:flex;gap:6px;align-items:center;">🌐 <select><option>Select language</option><option>English</option><option>Español</option><option>Français</option><option>Deutsch</option></select></label></div>"#.to_string(),
        "ContactForm" => r#"<form><input type="text" placeholder="Name"><input type="email" placeholder="Email"><textarea placeholder="Message"></textarea><button type="button">Send</button></form>"#.to_string(),
        "HTML" => r#"<p>Custom HTML gadget. Add markup in the Code tab to see it rendered here.</p>"#.to_string(),
        _ => format!("<p>{} widget renders live on Blogger.</p>", escape_html(w_type)),
    };

    format!(
        r#"<div class="sidebar-section" style="max-width:340px;margin:0 auto;"><div class="widget {wt}" id="{id}" data-block-id="{id}">{head}<div class="widget-content">{content}</div></div></div>"#,
        wt = escape_attr(w_type),
        id = escape_attr(&id),
    )
}

/// The full workspace shell: left panel + main (content + footer) + right panel.
pub fn preview_workspace_html(config: &ThemeConfig, posts: &[BlogPost]) -> String {
    format!(
        r##"<div class="mor-workspace" data-edit-target="colors.bg_base">
    {left}
    <main class="canvas-core">
        {content}
        {footer}
    </main>
    {right}
</div>"##,
        left = preview_left_panel_html(config),
        content = preview_content_html(config, posts),
        footer = preview_footer_html(config),
        right = preview_right_panel_html(config),
    )
}

pub fn render_preview_html(
    config: &ThemeConfig,
    posts: &[BlogPost],
    _preview_mode: PreviewTemplateMode,
    is_dark: bool,
    vfs: &HashMap<String, String>,
) -> String {
    let data_theme = if is_dark { "dark" } else { "light" };
    let background_tile_css = match &config.background.mode {
        BackgroundMode::Solid { color } => format!("background-color: {};", escape_attr(color)),
        BackgroundMode::Gradient {
            from,
            to,
            angle_deg,
        } => format!(
            "background: linear-gradient({}deg, {}, {});",
            angle_deg,
            escape_attr(from),
            escape_attr(to)
        ),
        BackgroundMode::Tile { url } if url.trim().is_empty() => String::new(),
        BackgroundMode::Tile { url } => format!(
            "background-image: url('{}'); background-repeat: repeat;",
            escape_attr(url)
        ),
    };

    let google_fonts_link = crate::config::fonts::build_google_font_imports(&[
        &config.typography.body_font_stack,
        &config.typography.heading_font_stack,
        &config.typography.mono_font_stack,
    ]);

    let site_title = escape_html(&config.site.site_title);

    // Fetch the TRUE CSS that will be injected into the final Blogger XML, then
    // decode its XML entities: the exported CSS is escaped for Blogger's b:skin
    // (XML), but here it goes into a browser <style> where entities are NOT
    // decoded — leaving `font-family: &#39;…&#39;` invalid and dropped.
    let mut parts = crate::render::template_resolver::resolve_template_parts(config, vfs);
    let true_css = unescape_for_style(
        &crate::render::xml_parts::css_generator::render_css_sockets(parts.css, config),
    );

    // Wire up the Plugin Pipeline for the Preview
    let active_plugins = crate::render::plugins::load_active_plugins();

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
    let true_js = crate::render::xml_parts::javascript_generator::render_javascript_sockets(
        parts.javascript,
        config,
    );

    // The "Mor — Centered Search" header variant is a CSS modifier on the same
    // .main-header markup, so the preview can reflect it by toggling the class.
    let header_extra_class = if config.template_pack.header_variant == "mor_search_center" {
        " search-centered"
    } else {
        ""
    };

    // Compose from the shared section builders so the per-module workbench preview
    // renders byte-identical markup.
    let body_markup = format!(
        "{header}\n{workspace}",
        header = preview_header_html(config),
        workspace = preview_workspace_html(config, posts),
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
<body class="{header_extra_class}" style="{background_tile_css}">
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glow_hover_reaches_preview_with_matching_target() {
        let vfs = HashMap::new();
        let mut config = ThemeConfig::default();
        config.colors.glow_title = true; // hover-only is the default trigger
        let html = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        // The hover glow rule is injected into the preview CSS...
        assert!(html.contains(".post-title:hover"));
        // ...and the element it targets actually exists in the preview DOM, so
        // :hover fires. (If these drift apart, hover glow silently no-ops.)
        assert!(html.contains(r#"class="post-title"#));
        // Default trigger is hover-only: no resting glow on the title.
        assert!(!html.contains(".post-title a, .post-title { text-shadow"));

        // Switch to always-on: a resting rule appears.
        config.colors.glow_hover = false;
        let always = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        assert!(always.contains(".post-title a, .post-title { text-shadow"));
    }

    #[test]
    fn preview_widget_renders_by_type() {
        let config = ThemeConfig::default();
        // Content widget → main canvas with post markup.
        let blog = preview_widget_html(&config, "Blog", "", &[]);
        assert!(blog.contains("canvas-core"));
        // Sidebar widget → themed gadget block with a type-specific list.
        let archive = preview_widget_html(&config, "BlogArchive", "", &[]);
        assert!(archive.contains(r#"class="widget BlogArchive""#));
        assert!(archive.contains("archive-list"));
        // Unknown type → generic gadget card, never empty.
        let unknown = preview_widget_html(&config, "Weird", "", &[]);
        assert!(unknown.contains("renders live on Blogger"));
    }

    // The preview header used to be hardcoded to the site title and ignored
    // header_logo_url; this guards that a set logo renders an <img>, mirroring export.
    #[test]
    fn logo_url_renders_img_in_preview_header() {
        let vfs = HashMap::new();
        let mut config = ThemeConfig::default();

        // Empty logo -> title text, no logo img.
        config.site.header_logo_url = String::new();
        let html = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        assert!(html.contains(r#"class="institute-title""#));
        // `.institute-logo` appears in the injected CSS regardless; the <img>
        // is what we must not emit, so match the class *attribute* form.
        assert!(!html.contains(r#"class="institute-logo""#));

        // Set logo -> img with the url, title text dropped from the brand.
        config.site.header_logo_url = "https://example.com/logo.png".to_string();
        let html = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        assert!(html.contains(r#"class="institute-logo""#));
        assert!(html.contains("https://example.com/logo.png"));
    }

    #[test]
    fn unescape_for_style_reverses_attr_escaping() {
        let escaped = "font-family: &#39;IM Fell English&#39;, serif; content: &quot;x&quot;;";
        assert_eq!(
            super::unescape_for_style(escaped),
            "font-family: 'IM Fell English', serif; content: \"x\";"
        );
    }

    // The preview reuses the Blogger-escaped CSS; if it isn't decoded, the font
    // stack lands in the <style> as `&#39;…&#39;` (invalid) and the chosen font
    // never applies in the canvas. Guard that the preview CSS is raw.
    #[test]
    fn preview_css_font_family_is_unescaped() {
        let vfs = HashMap::new();
        let mut config = ThemeConfig::default();
        config.typography.body_font_stack = "IM Fell English".to_string();
        let html = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        assert!(html.contains("'IM Fell English'"));
        assert!(!html.contains("&#39;IM Fell English&#39;"));
    }

    // The "Mor — Centered Search" header variant is a CSS modifier on the
    // preview's .main-header. Selecting it must add the class AND bundle the
    // centering CSS, otherwise the preview won't reflect the variant.
    #[test]
    fn centered_search_variant_reflects_in_preview() {
        let vfs = HashMap::new();
        let mut config = ThemeConfig::default();

        // Default header: no modifier class.
        let html = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        assert!(html.contains(r#"class="main-header""#));
        assert!(!html.contains("main-header search-centered"));
        // No bell on other variants.
        assert!(!html.contains("mor-bell"));

        // Centered-search variant: modifier class on the header + centering CSS,
        // plus the mirrored notification bell + glowing-title assets.
        config.template_pack.header_variant = "mor_search_center".to_string();
        let html = render_preview_html(&config, &[], PreviewTemplateMode::default(), true, &vfs);
        assert!(html.contains("main-header search-centered"));
        assert!(html.contains(".search-centered .header-bottom-row"));
        assert!(html.contains(r#"class="mor-bell""#));
        assert!(html.contains(".search-centered .institute-title"));
    }

    // Every shipped preset must define a custom cursor that survives into the
    // preview CSS (preset_css is appended last in build_master_css). Guards the
    // theme_presets/*.toml cursor blocks end-to-end.
    #[test]
    fn shipped_presets_define_preview_cursors() {
        let presets = crate::presets::all_presets();
        if presets.is_empty() {
            return; // preset dir not resolvable from this cwd; nothing to check.
        }
        let vfs = HashMap::new();
        for p in &presets {
            let mut cfg = ThemeConfig::default();
            cfg.preset_css = p.preset_css.to_string();
            let html =
                render_preview_html(&cfg, &[], PreviewTemplateMode::default(), true, &vfs);
            assert!(
                html.contains("cursor: url("),
                "preset '{}' has no custom cursor in preview output",
                p.name
            );
        }
    }
}
