<script>
(function () {
  const config = window.morAboutConfig || {
    profile_image_url: "",
    kicker: "Hello, I'm",
    title: "Your Name",
    bio_text: "Write something meaningful about yourself here...",
    contact_email: "",
    social_links: []
  };

  function escapeHtml(unsafe) {
    return String(unsafe)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function generateAbout() {
    const container = document.getElementById("mor-about-container");
    if (!container) return;

    let avatarHTML = "";
    if (config.profile_image_url && config.profile_image_url.trim() !== "") {
      avatarHTML = `<img src="${escapeHtml(config.profile_image_url)}" alt="Profile avatar" class="mor-about-avatar" />`;
    }

    let linksHTML = "";
    if (config.contact_email && config.contact_email.trim() !== "") {
      linksHTML += `<a href="mailto:${escapeHtml(config.contact_email)}" class="mor-about-link">Email</a>`;
    }

    if (config.social_links && config.social_links.length > 0) {
      config.social_links.forEach(link => {
        linksHTML += `
          <a href="${escapeHtml(link.url)}" 
             class="mor-about-link" 
             target="_blank" 
             rel="noopener noreferrer">${escapeHtml(link.label)}</a>`;
      });
    }

    const bioHTML = escapeHtml(config.bio_text).replace(/\n/g, "<br/>\n");

    container.innerHTML = `
      <style>
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

      <div class="mor-about-section">
        <header class="mor-about-header">
          ${avatarHTML}
          <div class="mor-about-title-block">
            <div class="mor-about-kicker">${escapeHtml(config.kicker)}</div>
            <h1>${escapeHtml(config.title)}</h1>
          </div>
        </header>

        <div class="mor-about-bio">
          ${bioHTML}
        </div>

        <div class="mor-about-links">
          ${linksHTML || '<p style="margin:0;color:var(--fg-dim,#888);">No contact links configured.</p>'}
        </div>
      </div>
    `;
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", generateAbout);
  } else {
    generateAbout();
  }
})();
</script>
