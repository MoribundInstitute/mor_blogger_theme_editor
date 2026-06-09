<script>
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
</script>
