use crate::config::{AboutPageConfig, ColorConfig};

pub fn generate_about_html(colors: &ColorConfig, config: &AboutPageConfig) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r##"<style>
.mor-about-section {{
  --bg-panel: {bg_panel};
  --fg-base: {fg_base};
  --fg-dim: {fg_muted};
  --border-color: {border};
  --accent: {accent};
}}
.mor-about-section {{
  max-width: 800px;
  margin: 0 auto;
  font-family: inherit;
  color: var(--fg-base);
}}
.mor-about-header {{
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 32px;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 24px;
}}
.mor-about-avatar {{
  width: 120px;
  height: 120px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid var(--border-color);
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
}}
.mor-about-title-block h1 {{
  margin: 0 0 8px 0;
  color: var(--accent);
  font-size: 2rem;
}}
.mor-about-kicker {{
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--fg-dim);
  margin-bottom: 4px;
}}
.mor-about-bio {{
  line-height: 1.7;
  font-size: 1.1rem;
  margin-bottom: 40px;
}}
.mor-about-links {{
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  background: var(--bg-panel);
  padding: 20px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
}}
.mor-about-link {{
  background: transparent;
  color: var(--accent);
  border: 1px solid var(--border-color);
  padding: 8px 16px;
  text-decoration: none;
  border-radius: 4px;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}}
.mor-about-link:hover {{
  background: var(--accent);
  color: var(--bg-panel);
}}
@media (max-width: 600px) {{
  .mor-about-header {{
    flex-direction: column;
    text-align: center;
  }}
}}
</style>
"##,
        bg_panel = colors.bg_panel.to_css(),
        fg_base = colors.fg_base,
        fg_muted = colors.fg_muted,
        border = colors.border,
        accent = colors.accent
    ));

    let avatar_html = if !config.profile_image_url.trim().is_empty() {
        format!(
            r##"<img src="{}" alt="Profile avatar" class="mor-about-avatar" />"##,
            escape_html(&config.profile_image_url)
        )
    } else {
        String::new()
    };

    let mut links_html = String::new();
    if !config.contact_email.trim().is_empty() {
        links_html.push_str(&format!(
            r##"<a href="mailto:{}" class="mor-about-link">Email</a>"##,
            escape_html(&config.contact_email)
        ));
    }

    for link in &config.social_links {
        links_html.push_str(&format!(
            r##"<a href="{}" class="mor-about-link" target="_blank" rel="noopener noreferrer">{}</a>"##,
            escape_html(&link.url),
            escape_html(&link.label)
        ));
    }

    html.push_str(&format!(
        r##"<div class="mor-about-section">
  <header class="mor-about-header">
    {avatar}
    <div class="mor-about-title-block">
      <div class="mor-about-kicker">{kicker}</div>
      <h1>{title}</h1>
    </div>
  </header>

  <div class="mor-about-bio">
    {bio}
  </div>

  <div class="mor-about-links">
    {links}
  </div>
</div>
"##,
        avatar = avatar_html,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        bio = escape_html(&config.bio_text).replace('\n', "<br/>\n"),
        links = links_html
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
