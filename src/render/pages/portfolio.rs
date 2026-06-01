use crate::config::pages::PortfolioPageConfig;
use crate::render::pages::escape_html;

/// Emits the Portfolio page as pure layout. Colors resolve from inherited theme
/// tokens; the `<style>` block carries structure only. For an offline color
/// override, wrap the output with `apply_stencil_colors` in the parent module.
pub fn generate_portfolio_html(config: &PortfolioPageConfig) -> String {
    let mut html = String::new();

    // Still a `format!` because `column-count` is data-driven (`{cols}`); all
    // other braces are escaped (`{{`/`}}`). Colors read from inherited tokens.
    html.push_str(&format!(
        r##"<style>
.mor-portfolio-section {{
  max-width: 1200px;
  margin: 0 auto;
  font-family: inherit;
  color: var(--fg-base, inherit);
}}
.mor-portfolio-intro {{
  margin-bottom: 40px;
  text-align: center;
  border-bottom: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  padding-bottom: 30px;
}}
.mor-portfolio-kicker {{
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 2px;
  color: var(--fg-dim, #888);
  margin-bottom: 8px;
}}
.mor-portfolio-title {{
  font-size: 2.5rem;
  color: var(--accent, #3b82f6);
  margin: 0 0 12px 0;
}}
.mor-portfolio-desc {{
  font-size: 1.1rem;
  line-height: 1.6;
  max-width: 600px;
  margin: 0 auto;
  color: var(--fg-dim, #888);
}}
.mor-masonry-grid {{
  column-count: {cols};
  column-gap: 16px;
}}
.mor-gallery-item {{
  break-inside: avoid;
  margin-bottom: 16px;
  border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  background: var(--bg-panel, rgba(128, 128, 128, 0.08));
  padding: 8px;
  border-radius: 4px;
  transition: transform 0.2s ease, border-color 0.2s ease;
}}
.mor-gallery-item:hover {{
  transform: translateY(-2px);
  border-color: var(--accent, #3b82f6);
}}
.mor-gallery-item img {{
  width: 100%;
  height: auto;
  display: block;
  border-radius: 2px;
}}
@media (max-width: 900px) {{
  .mor-masonry-grid {{ column-count: 2; }}
}}
@media (max-width: 600px) {{
  .mor-masonry-grid {{ column-count: 1; }}
}}
</style>
"##,
        cols = config.columns
    ));

    let mut grid_items = String::new();
    for url in &config.gallery_images {
        grid_items.push_str(&format!(
            r##"  <div class="mor-gallery-item">
    <img src="{}" alt="Portfolio artwork" loading="lazy" />
  </div>
"##,
            escape_html(url)
        ));
    }

    html.push_str(&format!(
        r##"<div class="mor-portfolio-section">
  <section class="mor-portfolio-intro">
    <div class="mor-portfolio-kicker">{kicker}</div>
    <h1 class="mor-portfolio-title">{title}</h1>
    <p class="mor-portfolio-desc">{desc}</p>
  </section>

  <div class="mor-masonry-grid">
{items}
  </div>
</div>
"##,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description),
        items = grid_items
    ));

    html
}