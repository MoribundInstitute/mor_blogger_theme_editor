use crate::config::{ColorConfig, StaticPagesConfig, ThemeConfig};

pub fn generate_archive_html(global_colors: &ColorConfig, pages_config: &StaticPagesConfig) -> String {
    let mut html = String::new();

    if !pages_config.sync_with_global_theme {
        let colors = pages_config.custom_colors.as_ref().unwrap_or(global_colors);
        html.push_str(&format!(
            r##"<style>
.mor-archive-section {{
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
    }

    html.push_str(&format!(
        r##"<div class="mor-archive-section">
  <section class="mor-archive-intro">
    <div class="mor-archive-kicker">{kicker}</div>
    <h2 class="mor-archive-title">{title}</h2>
    <p class="mor-archive-desc">{desc}</p>
  </section>

  <div id="archive-container">
    <div class="mor-archive-status">Loading archive...</div>
  </div>
</div>
"##,
        kicker = escape_html(&pages_config.archive.kicker),
        title = escape_html(&pages_config.archive.title),
        desc = escape_html(&pages_config.archive.description)
    ));

    let js_template = r##"<script>
(function () {
  async function loadStructuredArchive() {
    const container = document.getElementById("archive-container");
    if (!container) return;

    try {
      const response = await fetch("/feeds/posts/summary?alt=json&max-results={{MAX_RESULTS}}");

      if (!response.ok) {
        throw new Error("Feed request failed.");
      }

      const data = await response.json();
      const entries = data.feed && data.feed.entry ? data.feed.entry : [];

      if (!entries.length) {
        container.innerHTML = '<div class="mor-archive-empty">No archive entries found.</div>';
        return;
      }

      const grouped = {};

      entries.forEach(entry => {
        const title = entry.title && entry.title.$t ? entry.title.$t : "Untitled Post";
        const alternate = entry.link.find(link => link.rel === "alternate");
        const url = alternate ? alternate.href : "#";

        const rawSummary = entry.summary && entry.summary.$t ? entry.summary.$t : "";
        const summary = rawSummary
          .replace(/<[^>]*>/g, "")
          .replace(/\s+/g, " ")
          .trim();

        const cleanSnippet = summary
          ? summary.substring(0, 150) + (summary.length > 150 ? "..." : "")
          : "No summary available.";

        const date = new Date(entry.published.$t);
        const year = date.getFullYear();
        const monthIndex = date.getMonth();
        const month = date.toLocaleString("default", { month: "long" });
        const readableDate = date.toLocaleDateString("default", {
          year: "numeric",
          month: "short",
          day: "numeric"
        });

        const key = year + "-" + String(monthIndex).padStart(2, "0");

        if (!grouped[key]) {
          grouped[key] = {
            year,
            month,
            monthIndex,
            posts: []
          };
        }

        grouped[key].posts.push({
          title,
          url,
          cleanSnippet,
          readableDate,
          timestamp: date.getTime()
        });
      });

      const sortedKeys = Object.keys(grouped).sort((a, b) => {
        return grouped[b].year - grouped[a].year || grouped[b].monthIndex - grouped[a].monthIndex;
      });

      container.innerHTML = "";

      let lastYear = null;

      sortedKeys.forEach(key => {
        const group = grouped[key];

        if (group.year !== lastYear) {
          const yearEl = document.createElement("h2");
          yearEl.className = "archive-year";
          yearEl.textContent = group.year;
          container.appendChild(yearEl);
          lastYear = group.year;
        }

        const monthEl = document.createElement("h3");
        monthEl.className = "archive-month";
        monthEl.textContent = group.month;
        container.appendChild(monthEl);

        const grid = document.createElement("div");
        grid.className = "post-grid";

        group.posts
          .sort((a, b) => b.timestamp - a.timestamp)
          .forEach(post => {
            const snippet = document.createElement("a");
            snippet.className = "post-snippet";
            snippet.href = post.url;

            const title = escapeHtml(post.title);
            const date = escapeHtml(post.readableDate);
            const text = escapeHtml(post.cleanSnippet);

            snippet.innerHTML =
              '<div>' +
                '<span class="post-snippet-date">' + date + '</span>' +
                '<h3>' + title + '</h3>' +
              '</div>' +
              '<p>' + text + '</p>';

            grid.appendChild(snippet);
          });

        container.appendChild(grid);
      });
    } catch (error) {
      container.innerHTML =
        '<div class="mor-archive-error">Archive failed to load. Check the Blogger feed settings.</div>';
    }
  }

  function escapeHtml(value) {
    return String(value)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", loadStructuredArchive);
  } else {
    loadStructuredArchive();
  }
})();
</script>"##;

    let js_injected = js_template.replace(
        "{{MAX_RESULTS}}",
        &pages_config.archive.max_results.to_string(),
    );
    html.push_str(&js_injected);

    html
}

pub fn generate_categories_html(global_colors: &ColorConfig, pages_config: &StaticPagesConfig) -> String {
    let mut html = String::new();

    if !pages_config.sync_with_global_theme {
        let colors = pages_config.custom_colors.as_ref().unwrap_or(global_colors);
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
    }

    let mut topic_nav_links = String::new();
    let mut category_groups = String::new();

    for (i, section) in pages_config.categories.enabled_sections.iter().enumerate() {
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
        kicker = escape_html(&pages_config.categories.kicker),
        title = escape_html(&pages_config.categories.title),
        desc = escape_html(&pages_config.categories.description),
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