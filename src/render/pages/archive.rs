use crate::config::{ArchivePageConfig, ColorConfig};

pub fn generate_archive_html(colors: &ColorConfig, config: &ArchivePageConfig) -> String {
    let mut html = String::new();

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
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description)
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

    let js_injected = js_template.replace("{{MAX_RESULTS}}", &config.max_results.to_string());
    html.push_str(&js_injected);

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
