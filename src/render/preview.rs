//! In-editor preview HTML. Produces the HTML that gets shown in the
//! right-panel preview iframe. Distinct from `theme::render_theme`, which
//! produces uploadable Blogger XML (different output format, different
//! consumer, share only the helpers in `util`).

use crate::config::{BackgroundMode, ThemeConfig};
use crate::ui::workspace::layout::PreviewTemplateMode;

use super::util::{build_google_fonts_link, escape_attr, escape_html};

pub fn render_preview_html(config: &ThemeConfig, preview_mode: PreviewTemplateMode) -> String {
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

    let heading_stack = if config.typography.heading_font_stack.trim().is_empty() {
        config.typography.body_font_stack.clone()
    } else {
        config.typography.heading_font_stack.clone()
    };

    let google_fonts_link = build_google_fonts_link(&[
        &config.typography.body_font_stack,
        &heading_stack,
        &config.typography.mono_font_stack,
    ]);

    let menu_links = config
        .menu_links
        .iter()
        .filter(|link| !link.label.trim().is_empty())
        .map(|link| {
            format!(
                r#"<a href="{}">{}</a>"#,
                escape_attr(&link.url),
                escape_html(&link.label)
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let plugin_note = if config.plugins.custom_js.trim().is_empty() {
        "No optional JavaScript plugin is currently included."
    } else {
        "Optional JavaScript plugin will be included in the exported Blogger XML."
    };

    // NOTE: Each preview-* element carries a second class corresponding to
    // the production-template class name (e.g. preview-hero-card -> mor-post).
    // The preview's own CSS in this file matches the `preview-*` selectors;
    // preset CSS files (src/presets/css/*.css) target the production names.
    // This lets one preset CSS work for both the live preview AND the
    // exported Blogger theme without duplicating selectors.
    let modern_body = format!(
        r##"<div class="preview-shell preview-shell-modern">
    <header class="preview-site-header mor-main-header">
      <div class="preview-brand branding">
        <p class="preview-kicker">Blogger Theme Preview</p>
        <h1 class="preview-site-title institute-title">{site_title}</h1>
        <p class="preview-site-subtitle">{site_subtitle}</p>
      </div>
      <nav class="preview-nav mor-nav">{menu_links}</nav>
    </header>

    <main>
      <section class="preview-hero">
        <article class="preview-hero-card mor-post">
          <p class="preview-kicker post-kicker">Featured Post</p>
          <h2 class="post-title">Designing a better weblog shell</h2>
          <div class="post-body">
            <p>This modern preview uses your current Blogger theme settings while keeping the exported Blogger XML separate.</p>
            <div class="preview-hero-actions">
              <a class="preview-button pager-btn" href="#">Read Latest</a>
              <a class="preview-button pager-btn" href="#">Archive</a>
            </div>
          </div>
        </article>

        <aside class="preview-side-card runelite-panel sidebar-section">
          <p class="preview-kicker">Site Notes</p>
          <h3>Preview status</h3>
          <p>{plugin_note}</p>
          <dl class="preview-meta-list">
            <div class="preview-meta-row"><dt>Template</dt><dd>Modern Editorial</dd></div>
            <div class="preview-meta-row"><dt>Menu</dt><dd>{menu_count} links</dd></div>
            <div class="preview-meta-row"><dt>Export</dt><dd>Blogger XML</dd></div>
          </dl>
        </aside>
      </section>

      <section class="preview-card-grid">
        <article class="preview-card mor-post">
          <p class="preview-kicker">Recent Notes</p>
          <h3 class="post-title">Readable post cards</h3>
          <div class="post-body">
            <p>Use this space to judge contrast, line-height, borders, and card surfaces.</p>
          </div>
        </article>
        <article class="preview-card mor-post">
          <p class="preview-kicker">Typography</p>
          <h3 class="post-title">Font stack test</h3>
          <div class="post-body">
            <p>Body, heading, and <code>mono</code> font choices render directly in this preview.</p>
          </div>
        </article>
        <article class="preview-card mor-post">
          <p class="preview-kicker">About</p>
          <h3 class="post-title">Browser-safe shell</h3>
          <div class="post-body">
            <p>The exported Blogger XML remains separate; this is a friendlier visual preview.</p>
          </div>
        </article>
      </section>
    </main>

    <footer class="preview-footer mor-footer">
      <span>{footer_text}</span>
      <span>{site_title}</span>
    </footer>
  </div>"##,
        site_title = escape_html(&config.site.site_title),
        site_subtitle = escape_html(&config.site.site_subtitle),
        menu_links = menu_links,
        plugin_note = escape_html(plugin_note),
        menu_count = config
            .menu_links
            .iter()
            .filter(|link| !link.label.trim().is_empty())
            .count(),
        footer_text = escape_html(&config.footer.footer_text),
    );

    let sidebars_body = format!(
        r##"<div class="preview-shell preview-shell-sidebars">
    <header class="preview-site-header preview-site-header-with-toggles mor-main-header">
      <button type="button" class="preview-panel-toggle panel-toggle" data-target="left" aria-label="Browse" title="Browse">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="9" y1="3" x2="9" y2="21"></line>
        </svg>
      </button>

      <div class="preview-brand preview-brand-centered branding">
        <p class="preview-kicker">Blogger Theme Preview</p>
        <h1 class="preview-site-title institute-title">{site_title}</h1>
        <p class="preview-site-subtitle">{site_subtitle}</p>
        <nav class="preview-nav preview-nav-centered mor-nav">{menu_links}</nav>
      </div>

      <button type="button" class="preview-panel-toggle panel-toggle" data-target="right" aria-label="Contents" title="Contents">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="15" y1="3" x2="15" y2="21"></line>
        </svg>
      </button>
    </header>

    <main class="preview-sidebar-layout mor-workspace" id="preview-layout">
      <aside class="preview-sidebar runelite-panel panel-left sidebar-section" data-side="left">
        <div class="preview-sidebar-header panel-header">
          <strong>Browse</strong>
          <button type="button" class="preview-sidebar-close panel-toggle" data-target="left">×</button>
        </div>
        <div class="panel-content">
          <p>Use this panel for labels, archive links, profile widgets, blogrolls, or navigation blocks.</p>
          <ul>
            <li>Latest posts</li>
            <li>Archive</li>
            <li>Labels</li>
            <li>Profile</li>
          </ul>
        </div>
      </aside>

      <article class="preview-hero-card preview-sidebar-main-card mor-post">
        <p class="preview-kicker post-kicker">Featured Post</p>
        <h2 class="post-title">Modern layout with closeable sidebars</h2>
        <div class="post-body">
          <p>This preview keeps the clean modern look but restores the useful Blogger-style side panels on either end.</p>
          <p>Click <strong>Browse</strong>, <strong>Contents</strong>, or either × button to test collapsed sidebars.</p>
          <div class="preview-hero-actions">
            <a class="preview-button pager-btn" href="#">Read Latest</a>
            <a class="preview-button pager-btn" href="#">Archive</a>
          </div>
        </div>
      </article>

      <aside class="preview-sidebar runelite-panel panel-right sidebar-section" data-side="right">
        <div class="preview-sidebar-header panel-header">
          <strong>Contents</strong>
          <button type="button" class="preview-sidebar-close panel-toggle" data-target="right">×</button>
        </div>
        <div class="panel-content">
          <p>{plugin_note}</p>
          <ol>
            <li>Featured post</li>
            <li>Recent notes</li>
            <li>Typography check</li>
            <li>Footer</li>
          </ol>
        </div>
      </aside>
    </main>

    <footer class="preview-footer mor-footer">
      <span>{footer_text}</span>
      <span>{site_title}</span>
    </footer>
  </div>"##,
        site_title = escape_html(&config.site.site_title),
        site_subtitle = escape_html(&config.site.site_subtitle),
        menu_links = menu_links,
        plugin_note = escape_html(plugin_note),
        footer_text = escape_html(&config.footer.footer_text),
    );

    let body_markup = match preview_mode {
        PreviewTemplateMode::Modern => modern_body,
        PreviewTemplateMode::Sidebars => sidebars_body,
        PreviewTemplateMode::StaticArchive => format!(
            "<div style='padding: 40px; color: {};'><h2>Archive Preview coming soon...</h2></div>", 
            escape_attr(&config.colors.fg_base)
        ),
        PreviewTemplateMode::StaticCategories => format!(
            "<div style='padding: 40px; color: {};'><h2>Categories Preview coming soon...</h2></div>",
            escape_attr(&config.colors.fg_base)
        ),
    };

    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{site_title}</title>
{google_fonts_link}
<style>
:root {{
  --bg-base: {bg_base};
  --bg-panel: {bg_panel};
  --bg-elevated: {bg_elevated};
  --fg-base: {fg_base};
  --fg-muted: {fg_muted};
  --accent: {accent};
  --border: {border};
  --btn-radius: {btn_radius};
  --btn-border: {btn_border};
  --btn-transform: {btn_transform};

  --font-body: {font_body};
  --font-heading: {font_heading};
  --font-mono: {font_mono};
  --base-size: {base_size};
  --scale: {scale_ratio};
  --line-height: {line_height};
  --heading-weight: {heading_weight};
}}

* {{ box-sizing: border-box; }}
html {{ font-size: var(--base-size); }}

body {{
  margin: 0;
  min-height: 100vh;
  background: var(--bg-base);
  color: var(--fg-base);
  font-family: var(--font-body);
  line-height: var(--line-height);
  {background_tile_css}
}}

body::before {{
  content: "";
  position: fixed;
  inset: 0;
  pointer-events: none;
  background:
    radial-gradient(circle at 16% 8%, color-mix(in srgb, var(--accent) 18%, transparent), transparent 34rem),
    radial-gradient(circle at 88% 12%, color-mix(in srgb, var(--fg-muted) 12%, transparent), transparent 28rem);
  opacity: 0.9;
}}

h1, h2, h3, h4, h5, h6 {{
  font-family: var(--font-heading);
  font-weight: var(--heading-weight);
  line-height: 1.08;
}}

h1 {{ font-size: clamp(2.4rem, 7vw, calc(1rem * var(--scale) * var(--scale) * var(--scale) * var(--scale))); }}
h2 {{ font-size: clamp(1.6rem, 3vw, calc(1rem * var(--scale) * var(--scale) * var(--scale))); }}
h3 {{ font-size: calc(1rem * var(--scale)); }}
code, pre, kbd, samp {{ font-family: var(--font-mono); }}

.preview-shell {{
  position: relative;
  z-index: 1;
  width: min(1120px, calc(100% - 32px));
  margin: 0 auto;
  padding: 22px 0 28px;
}}

.preview-shell-sidebars {{ width: min(1280px, calc(100% - 32px)); }}

.preview-site-header {{
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  padding: 14px 0 22px;
}}

.preview-site-header-with-toggles {{ align-items: stretch; }}
.preview-brand {{ min-width: 0; }}
.preview-brand-centered {{ text-align: center; flex: 1; }}

.preview-kicker {{
  margin: 0 0 8px;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.78rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}}

.preview-site-title {{ margin: 0; color: var(--fg-base); font-size: clamp(1.2rem, 2.4vw, 1.9rem); }}
.preview-site-subtitle {{ margin: 5px 0 0; color: var(--fg-muted); max-width: 62ch; }}
.preview-brand-centered .preview-site-subtitle {{ margin-left: auto; margin-right: auto; }}

.preview-nav {{ display: flex; gap: 10px; flex-wrap: wrap; justify-content: flex-end; }}
.preview-nav-centered {{ justify-content: center; margin-top: 12px; }}

.preview-nav a,
.preview-button,
.preview-panel-toggle,
.preview-sidebar-close {{
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--fg-base);
  background: color-mix(in srgb, var(--bg-panel) 88%, transparent);
  text-decoration: none;
  border-style: solid;
  border-color: var(--border);
  border-width: var(--btn-border);
  border-radius: var(--btn-radius);
  text-transform: var(--btn-transform);
  font-family: var(--font-body);
  cursor: pointer;
}}

.preview-nav a,
.preview-button {{ padding: 0.58rem 0.8rem; }}
.preview-panel-toggle {{ padding: 8px; width: 44px; height: 44px; }}
.preview-panel-toggle svg {{ width: 100%; height: 100%; display: block; }}
.preview-sidebar-close {{ padding: 0.2rem 0.45rem; font-family: var(--font-mono); }}

.preview-nav a:hover,
.preview-button:hover,
.preview-panel-toggle:hover,
.preview-sidebar-close:hover {{
  color: var(--bg-base);
  background: var(--accent);
}}

.preview-hero {{
  display: grid;
  grid-template-columns: minmax(0, 1.5fr) minmax(280px, 0.75fr);
  gap: 18px;
}}

.preview-hero-card,
.preview-side-card,
.preview-card,
.preview-footer,
.preview-sidebar {{
  background: var(--bg-panel);
  border: 1px solid var(--border);
  box-shadow: 0 24px 80px rgba(0, 0, 0, 0.22);
}}

.preview-hero-card {{ padding: clamp(24px, 5vw, 56px); background: var(--bg-elevated); }}
.preview-hero-card h2 {{ margin: 0; color: var(--fg-base); }}
.preview-hero-card p {{ color: var(--fg-muted); max-width: 68ch; }}
.preview-hero-actions {{ display: flex; gap: 10px; flex-wrap: wrap; margin-top: 22px; }}

.preview-side-card {{ padding: 24px; align-self: stretch; }}
.preview-side-card h3 {{ margin: 0; color: var(--accent); }}
.preview-side-card p {{ margin: 0; color: var(--fg-muted); }}

.preview-meta-list {{ margin: 18px 0 0; }}
.preview-meta-row {{ display: flex; justify-content: space-between; gap: 16px; padding: 8px 0; border-top: 1px solid var(--border); color: var(--fg-muted); }}
.preview-meta-row dt, .preview-meta-row dd {{ margin: 0; }}
.preview-meta-row strong {{ color: var(--fg-base); font-weight: var(--heading-weight); }}

.preview-card-grid {{ display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 18px; margin-top: 18px; }}
.preview-card {{ padding: 22px; }}
.preview-card h3 {{ margin: 0; color: var(--fg-base); }}
.preview-card p {{ color: var(--fg-muted); }}
.preview-card code {{ color: var(--accent); }}

.preview-sidebar-layout {{
  display: grid;
  grid-template-columns: minmax(180px, 240px) minmax(0, 1fr) minmax(180px, 240px);
  gap: 16px;
  align-items: start;
}}

.preview-sidebar-layout.left-collapsed {{ grid-template-columns: 0 minmax(0, 1fr) minmax(180px, 240px); }}
.preview-sidebar-layout.right-collapsed {{ grid-template-columns: minmax(180px, 240px) minmax(0, 1fr) 0; }}
.preview-sidebar-layout.left-collapsed.right-collapsed {{ grid-template-columns: 0 minmax(0, 1fr) 0; }}

.preview-sidebar {{
  padding: 18px;
  overflow: hidden;
  transition: opacity 0.15s ease, padding 0.2s ease, border-width 0.2s ease;
}}

.preview-sidebar.is-collapsed {{
  opacity: 0;
  pointer-events: none;
  padding: 0;
  border-width: 0;
}}

.preview-sidebar-header {{ display: flex; align-items: center; justify-content: space-between; gap: 8px; margin-bottom: 10px; }}
.preview-sidebar-header strong {{ color: var(--accent); }}
.preview-sidebar p, .preview-sidebar li {{ color: var(--fg-muted); }}
.preview-sidebar-main-card {{ min-height: 420px; }}

.preview-plugin-note {{ margin-top: 12px; font-size: 0.85rem; color: var(--fg-muted); }}
.preview-footer {{ margin-top: 18px; padding: 18px 22px; display: flex; justify-content: space-between; gap: 16px; color: var(--fg-muted); }}

@media (max-width: 880px) {{
  .preview-site-header,
  .preview-hero,
  .preview-card-grid,
  .preview-sidebar-layout {{ grid-template-columns: 1fr; flex-direction: column; }}
  .preview-site-header {{ align-items: flex-start; }}
  .preview-nav {{ justify-content: flex-start; }}
  .preview-brand-centered {{ text-align: left; }}
  .preview-nav-centered {{ justify-content: flex-start; }}
}}
/* ===== Active preset CSS ===== */
{preset_css}
</style>
</head>
<body>
  {body_markup}
  <script>
    (function () {{
      function setSide(side) {{
        var layout = document.getElementById('preview-layout');
        var panel = document.querySelector('[data-side="' + side + '"]');
        if (!layout || !panel) return;
        var cls = side + '-collapsed';
        layout.classList.toggle(cls);
        panel.classList.toggle('is-collapsed');
      }}

      document.addEventListener('click', function (event) {{
        var button = event.target.closest('[data-target]');
        if (!button) return;
        setSide(button.getAttribute('data-target'));
      }});
    }})();
  </script>
</body>
</html>"#,
        site_title = escape_html(&config.site.site_title),
        bg_base = config.colors.bg_base,
        bg_panel = config.colors.bg_panel.to_css(),
        bg_elevated = config.colors.bg_elevated.to_css(),
        fg_base = config.colors.fg_base,
        fg_muted = config.colors.fg_muted,
        accent = config.colors.accent,
        border = config.colors.border,
        btn_radius = config.buttons.radius,
        btn_border = config.buttons.border_width,
        btn_transform = config.buttons.text_transform,
        font_body = config.typography.body_font_stack,
        font_heading = heading_stack,
        font_mono = config.typography.mono_font_stack,
        base_size = config.typography.base_size,
        scale_ratio = config.typography.scale_ratio,
        line_height = config.typography.line_height,
        heading_weight = config.typography.heading_weight,
        background_tile_css = background_tile_css,
        body_markup = body_markup,
        google_fonts_link = google_fonts_link,
        preset_css = config.preset_css,
    )
}