use crate::config::pages::LmsConfig;
use crate::config::styling::ColorConfig;

pub fn generate_course_catalog_html(colors: &ColorConfig, config: &LmsConfig) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r##"<style>
.mor-catalog-section {{
  --bg-panel: {bg_panel};
  --fg-base: {fg_base};
  --fg-dim: {fg_muted};
  --border-color: {border};
  --accent: {accent};
}}
.mor-catalog-section {{
  max-width: 1000px;
  margin: 0 auto;
  font-family: inherit;
  color: var(--fg-base);
}}
.mor-catalog-header {{
  text-align: center;
  margin-bottom: 48px;
  border-bottom: 1px dashed var(--border-color);
  padding-bottom: 32px;
}}
.mor-catalog-kicker {{
  font-family: monospace;
  font-size: 0.9rem;
  color: var(--fg-dim);
  margin-bottom: 12px;
}}
.mor-catalog-title {{
  font-size: 2.5rem;
  color: var(--accent);
  margin: 0 0 16px 0;
}}
.mor-course-grid {{
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 24px;
}}
.mor-course-card {{
  background: var(--bg-panel);
  border: 2px solid var(--border-color);
  border-radius: 6px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s ease, border-color 0.2s ease;
}}
.mor-course-card:hover {{
  transform: translateY(-4px);
  border-color: var(--accent);
}}
.mor-course-card h2 {{
  margin: 0 0 12px 0;
  font-size: 1.4rem;
  color: var(--fg-base);
}}
.mor-course-card p {{
  color: var(--fg-dim);
  line-height: 1.5;
  margin-bottom: 24px;
  flex-grow: 1;
}}
.mor-course-card a {{
  display: inline-block;
  text-align: center;
  background: transparent;
  color: var(--accent);
  border: 1px solid var(--border-color);
  padding: 10px 16px;
  text-decoration: none;
  font-weight: bold;
  border-radius: 4px;
  transition: all 0.2s ease;
}}
.mor-course-card a:hover {{
  background: var(--accent);
  color: var(--bg-panel);
}}
</style>
"##,
        bg_panel = colors.bg_panel.to_css(),
        fg_base = colors.fg_base,
        fg_muted = colors.fg_muted,
        border = colors.border,
        accent = colors.accent
    ));

    let course_cards = format!(
        r##"  <article class="mor-course-card">
    <h2>{course_title}</h2>
    <p>{course_desc}</p>
    <a href="/p/course-syllabus.html">View Syllabus &raquo;</a>
  </article>
"##,
        course_title = escape_html(&config.course_title),
        course_desc = escape_html(&config.course_description)
    );

    html.push_str(&format!(
        r##"<div class="mor-catalog-section">
  <header class="mor-catalog-header">
    <div class="mor-catalog-kicker">{kicker}</div>
    <h1 class="mor-catalog-title">{title}</h1>
    <p>{desc}</p>
  </header>

  <div class="mor-course-grid">
{cards}
  </div>
</div>
"##,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description),
        cards = course_cards
    ));

    html
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
