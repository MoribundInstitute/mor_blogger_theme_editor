use crate::config::{CategoriesPageConfig, ColorConfig};

pub fn generate_categories_html(colors: &ColorConfig, config: &CategoriesPageConfig) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r##"<style>
.mor-category-section {{
  --bg-panel: {bg_panel};
  --fg-base: {fg_base};
  --fg-dim: {fg_muted};
  --border-color: {border};
  --accent: {accent};
}}
</style>
"##,
        bg_panel = colors.bg_panel.to_css(),
        fg_base = colors.fg_base,
        fg_muted = colors.fg_muted,
        border = colors.border,
        accent = colors.accent
    ));

    let mut topic_nav_links = String::new();
    let mut category_groups = String::new();

    for (i, section) in config.enabled_sections.iter().enumerate() {
        let section_id = format!("topic-sec-{}", i);
        let escaped_name = escape_html(section);
        let url_encoded = section.replace(" ", "%20");

        topic_nav_links.push_str(&format!(
            r##"      <a href="#{id}">{name}</a>
"##,
            id = section_id,
            name = escaped_name
        ));

        category_groups.push_str(&format!(
            r##"  <section class="mor-category-group" id="{id}">
    <h2 class="mor-category-heading">
      <a href="/search/label/{url}">{name}</a>
    </h2>
    <div class="mor-category-grid">
      <a href="/search/label/{url}">View all items labeled "{name}"</a>
    </div>
  </section>
"##,
            id = section_id,
            name = escaped_name,
            url = url_encoded
        ));
    }

    html.push_str(&format!(
        r##"<div class="mor-category-section">
  <section class="mor-category-intro">
    <div class="mor-category-kicker">{kicker}</div>
    <h1 class="mor-category-title">{title}</h1>
    <p class="mor-category-desc">{desc}</p>
  </section>

  <nav class="mor-category-nav" aria-label="Category navigation">
    <h2 class="mor-category-nav-title">Quick Filters</h2>

    <div class="mor-nav-buttons">
      <span class="mor-nav-label">A-Z:</span>
      <a href="/search/label/A">A</a> <a href="/search/label/B">B</a> <a href="/search/label/C">C</a> <a href="/search/label/D">D</a>
      <a href="/search/label/E">E</a> <a href="/search/label/F">F</a> <a href="/search/label/G">G</a> <a href="/search/label/H">H</a>
      <a href="/search/label/I">I</a> <a href="/search/label/J">J</a> <a href="/search/label/K">K</a> <a href="/search/label/L">L</a>
      <a href="/search/label/M">M</a> <a href="/search/label/N">N</a> <a href="/search/label/O">O</a> <a href="/search/label/P">P</a>
      <a href="/search/label/Q">Q</a> <a href="/search/label/R">R</a> <a href="/search/label/S">S</a> <a href="/search/label/T">T</a>
      <a href="/search/label/U">U</a> <a href="/search/label/V">V</a> <a href="/search/label/W">W</a> <a href="/search/label/X">X</a>
      <a href="/search/label/Y">Y</a> <a href="/search/label/Z">Z</a>
    </div>

    <div class="mor-nav-buttons">
      <span class="mor-nav-label">By Topic:</span>
{topic_navs}
    </div>
  </nav>

  <section class="mor-alpha-panel">
    <h2 id="authors">By Author</h2>
    <div class="mor-alpha-links" id="author-links"></div>
  </section>

  <section class="mor-alpha-panel">
    <h2 id="musicians">By Musical Artist</h2>
    <div class="mor-alpha-links" id="musician-links"></div>
  </section>

  <section class="mor-alpha-panel">
    <h2 id="painters">By Painter</h2>
    <div class="mor-alpha-links" id="painter-links"></div>
  </section>

  <section class="mor-alpha-panel">
    <h2 id="actors">By Actor</h2>
    <div class="mor-alpha-links" id="actor-links"></div>
  </section>

  <section class="mor-alpha-panel">
    <h2 id="animes">By Anime</h2>
    <div class="mor-alpha-links" id="anime-links"></div>
  </section>

  <section class="mor-alpha-panel">
    <h2 id="kdramas">By Korean Drama</h2>
    <div class="mor-alpha-links" id="kdrama-links"></div>
  </section>

  <section class="mor-alpha-panel">
    <h2 id="animals">By Animal</h2>
    <div class="mor-alpha-links" id="animal-links"></div>
  </section>

{category_groups}

  <p class="mor-page-note">
    Labels are intentionally broad. Use the topics for the main shelf, then use ordinary Blogger labels for narrower trails through the stacks.
  </p>
</div>
"##,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description),
        topic_navs = topic_nav_links,
        category_groups = category_groups
    ));

    let js_template = r##"<script>
(function () {
  const sections = [
    { id: "author-links", keyword: "Author" },
    { id: "musician-links", keyword: "Musician" },
    { id: "painter-links", keyword: "Painter" },
    { id: "actor-links", keyword: "Actor" },
    { id: "anime-links", keyword: "Anime" },
    { id: "kdrama-links", keyword: "Korean Drama" },
    { id: "animal-links", keyword: "Animal" }
  ];

  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

  sections.forEach(({ id, keyword }) => {
    const container = document.getElementById(id);
    if (!container) return;

    alphabet.forEach(letter => {
      const label = keyword + ": " + letter;
      const link = document.createElement("a");
      link.href = "/search/label/" + encodeURIComponent(label);
      link.textContent = letter;
      link.setAttribute("aria-label", keyword + " labels beginning with " + letter);
      container.appendChild(link);
    });
  });
})();
</script>"##;

    html.push_str(js_template);

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
