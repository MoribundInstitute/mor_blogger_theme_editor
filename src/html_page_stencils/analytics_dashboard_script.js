<script>
(function () {
  const config = window.morAnalyticsDashboardConfig || {};
  const maxResults = Number(config.maxResults || "{{MAX_RESULTS}}" || 150);

  function setText(id, value) {
    const element = document.getElementById(id);
    if (element) element.textContent = value;
  }

  function formatNumber(value) {
    const number = Number(value || 0);
    return Number.isFinite(number) ? number.toLocaleString() : String(value || "0");
  }

  function getTotalResults(feed) {
    return feed &&
      feed.openSearch$totalResults &&
      feed.openSearch$totalResults.$t
      ? feed.openSearch$totalResults.$t
      : "0";
  }

  async function fetchJson(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error("Feed request failed: " + url);
    return response.json();
  }

  function renderBars(containerId, rows) {
    const container = document.getElementById(containerId);
    if (!container) return;

    if (!rows.length) {
      container.innerHTML = '<div class="mor-analytics-placeholder">No data found.</div>';
      return;
    }

    const max = Math.max.apply(null, rows.map(row => row.value)) || 1;

    const bars = rows.map(row => {
      const height = Math.max(6, Math.round((row.value / max) * 100));
      return '<span class="mor-analytics-bar" style="height:' + height + '%" title="' +
        escapeHtml(row.label + ': ' + row.value) + '"></span>';
    }).join("");

    const labels = rows.map(row => '<span>' + escapeHtml(row.label) + '</span>').join("");

    container.innerHTML =
      '<div class="mor-analytics-bars">' + bars + '</div>' +
      '<div class="mor-analytics-bar-labels">' + labels + '</div>';
  }

  function renderCategories(entries) {
    const counts = {};

    entries.forEach(entry => {
      const categories = entry.category || [];
      categories.forEach(category => {
        const term = category.term || "Unlabeled";
        counts[term] = (counts[term] || 0) + 1;
      });
    });

    const rows = Object.keys(counts)
      .map(name => ({ name, value: counts[name] }))
      .sort((a, b) => b.value - a.value)
      .slice(0, 8);

    const legend = document.getElementById("mor-analytics-category-legend");
    if (!legend) return;

    if (!rows.length) {
      legend.innerHTML = "<li>No labels found.</li>";
      return;
    }

    const total = rows.reduce((sum, row) => sum + row.value, 0) || 1;

    legend.innerHTML = rows.map(row => {
      const percent = Math.round((row.value / total) * 100);
      return '<li><span>' + escapeHtml(row.name) + '</span><strong>' +
        formatNumber(row.value) + ' · ' + percent + '%</strong></li>';
    }).join("");
  }

  function renderPostsOverTime(entries) {
    const monthly = {};

    entries.forEach(entry => {
      if (!entry.published || !entry.published.$t) return;

      const date = new Date(entry.published.$t);
      if (Number.isNaN(date.getTime())) return;

      const key = date.getFullYear() + "-" + String(date.getMonth() + 1).padStart(2, "0");
      monthly[key] = (monthly[key] || 0) + 1;
    });

    const rows = Object.keys(monthly)
      .sort()
      .slice(-10)
      .map(key => ({ label: key.slice(5), value: monthly[key] }));

    renderBars("mor-analytics-posts-chart", rows);
  }

  async function loadAnalyticsDashboard() {
    setText("mor-analytics-followers", config.manualFollowers || "Manual");
    setText("mor-analytics-views-30", config.manualThirtyDayViews || "Manual");
    setText("mor-analytics-total-views", config.manualTotalViews || "Manual");
    setText("mor-analytics-storage", config.manualStorageUsed || "Manual");

    try {
      const posts = await fetchJson("/feeds/posts/summary?alt=json&max-results=" + encodeURIComponent(maxResults));
      const entries = posts.feed && posts.feed.entry ? posts.feed.entry : [];

      setText("mor-analytics-total-posts", formatNumber(getTotalResults(posts.feed)));
      renderCategories(entries);
      renderPostsOverTime(entries);
    } catch (error) {
      setText("mor-analytics-total-posts", "Unavailable");
      const legend = document.getElementById("mor-analytics-category-legend");
      if (legend) legend.innerHTML = "<li>Could not load post labels.</li>";
      const chart = document.getElementById("mor-analytics-posts-chart");
      if (chart) chart.innerHTML = '<div class="mor-analytics-placeholder">Could not load post feed.</div>';
    }

    try {
      const comments = await fetchJson("/feeds/comments/summary?alt=json&max-results=1");
      setText("mor-analytics-total-comments", formatNumber(getTotalResults(comments.feed)));
    } catch (error) {
      setText("mor-analytics-total-comments", "Unavailable");
    }

    try {
      const pages = await fetchJson("/feeds/pages/default?alt=json&max-results=1");
      setText("mor-analytics-total-pages", formatNumber(getTotalResults(pages.feed)));
    } catch (error) {
      setText("mor-analytics-total-pages", "Manual");
    }

    setText("mor-analytics-last-updated", new Date().toLocaleString());
  }

  function enableButtonRows() {
    document.querySelectorAll(".mor-analytics-button-row").forEach(row => {
      row.querySelectorAll(".mor-analytics-button").forEach(button => {
        button.addEventListener("click", () => {
          row.querySelectorAll(".mor-analytics-button").forEach(item => {
            item.classList.remove("is-active");
          });
          button.classList.add("is-active");
        });
      });
    });
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
    document.addEventListener("DOMContentLoaded", function () {
      enableButtonRows();
      loadAnalyticsDashboard();
    });
  } else {
    enableButtonRows();
    loadAnalyticsDashboard();
  }
})();
</script>
