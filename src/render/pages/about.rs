use crate::config::AboutPageConfig;
use crate::render::pages::escape_html;

/// Emits the About page as pure layout. Colors resolve from the inherited theme
/// tokens (`--accent`, `--bg-panel`, ...) at render time; the `<style>` block
/// below carries structure only and reads every color from a token with a
/// context-neutral fallback. For an offline color override, wrap the output with
/// `apply_stencil_colors` in the parent module rather than baking colors in here.
pub fn generate_about_html(config: &AboutPageConfig) -> String {
    let mut html = String::new();

    // No interpolation in this block, so it's a plain literal (single braces,
    // no escaping). Every color reads from an inherited theme token with a
    // context-neutral fallback for when the stencil lands outside a Mor theme.
    html.push_str(
        r##"<style>
.mor-about-section {
  max-width: 800px;
  margin: 0 auto;
  font-family: inherit;
  color: var(--fg-base, inherit);
}
.mor-about-header {
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 32px;
  border-bottom: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  padding-bottom: 24px;
}
.mor-about-avatar {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid var(--border-color, rgba(128, 128, 128, 0.3));
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
}
.mor-about-title-block h1 {
  margin: 0 0 8px 0;
  color: var(--accent, #3b82f6);
  font-size: 2rem;
}
.mor-about-kicker {
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--fg-dim, #888);
  margin-bottom: 4px;
}
.mor-about-bio {
  line-height: 1.7;
  font-size: 1.1rem;
  margin-bottom: 40px;
}
.mor-about-links {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  background: var(--bg-panel, rgba(128, 128, 128, 0.08));
  padding: 20px;
  border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  border-radius: 8px;
}
.mor-about-link {
  background: transparent;
  color: var(--accent, #3b82f6);
  border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  padding: 8px 16px;
  text-decoration: none;
  border-radius: 4px;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}
.mor-about-link:hover {
  background: var(--accent, #3b82f6);
  color: var(--bg-panel, #fff);
}
@media (max-width: 600px) {
  .mor-about-header {
    flex-direction: column;
    text-align: center;
  }
}
</style>
"##,
    );

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