<script>
(function () {
  const config = window.morPortfolioConfig || {
    columns: 3,
    kicker: "Selected Works",
    title: "Portfolio",
    description: "A collection of my creative work across various mediums.",
    gallery_images: []
  };

  function escapeHtml(unsafe) {
    return String(unsafe)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function generatePortfolio() {
    const container = document.getElementById("mor-portfolio-container");
    if (!container) return;

    const cols = Math.max(1, Math.min(5, Number(config.columns) || 3));

    let itemsHTML = "";
    if (config.gallery_images && config.gallery_images.length > 0) {
      itemsHTML = config.gallery_images.map(url => `
        <div class="mor-gallery-item">
          <img src="${escapeHtml(url)}" alt="Portfolio artwork" loading="lazy" />
        </div>
      `).join("");
    } else {
      itemsHTML = `<div class="mor-portfolio-placeholder">No images configured yet.</div>`;
    }

    container.innerHTML = `
      <style>
        .mor-portfolio-section {
          max-width: 1200px;
          margin: 0 auto;
          font-family: inherit;
          color: var(--fg-base, inherit);
        }
        .mor-portfolio-intro {
          margin-bottom: 40px;
          text-align: center;
          border-bottom: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
          padding-bottom: 30px;
        }
        .mor-portfolio-kicker {
          font-size: 0.85rem;
          text-transform: uppercase;
          letter-spacing: 2px;
          color: var(--fg-dim, #888);
          margin-bottom: 8px;
        }
        .mor-portfolio-title {
          font-size: 2.5rem;
          color: var(--accent, #3b82f6);
          margin: 0 0 12px 0;
        }
        .mor-portfolio-desc {
          font-size: 1.1rem;
          line-height: 1.6;
          max-width: 600px;
          margin: 0 auto;
          color: var(--fg-dim, #888);
        }
        .mor-masonry-grid {
          column-count: ${cols};
          column-gap: 16px;
        }
        .mor-gallery-item {
          break-inside: avoid;
          margin-bottom: 16px;
          border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
          background: var(--bg-panel, rgba(128, 128, 128, 0.08));
          padding: 8px;
          border-radius: 4px;
          transition: transform 0.2s ease, border-color 0.2s ease;
        }
        .mor-gallery-item:hover {
          transform: translateY(-2px);
          border-color: var(--accent, #3b82f6);
        }
        .mor-gallery-item img {
          width: 100%;
          height: auto;
          display: block;
          border-radius: 2px;
        }
        @media (max-width: 900px) {
          .mor-masonry-grid { column-count: 2; }
        }
        @media (max-width: 600px) {
          .mor-masonry-grid { column-count: 1; }
        }
        .mor-portfolio-placeholder {
          text-align: center;
          padding: 60px 20px;
          color: var(--fg-dim, #888);
          font-style: italic;
        }
      </style>

      <div class="mor-portfolio-section">
        <section class="mor-portfolio-intro">
          <div class="mor-portfolio-kicker">${escapeHtml(config.kicker)}</div>
          <h1 class="mor-portfolio-title">${escapeHtml(config.title)}</h1>
          <p class="mor-portfolio-desc">${escapeHtml(config.description)}</p>
        </section>

        <div class="mor-masonry-grid">
          ${itemsHTML}
        </div>
      </div>
    `;
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", generatePortfolio);
  } else {
    generatePortfolio();
  }
})();
</script>
