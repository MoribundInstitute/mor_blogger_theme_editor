(() => {
  const init = () => {
    /* =========================================================
    07. Accessible Light/Dark Theme Toggler (Iframe Safe)
    ========================================================= */
    const themeToggleBtn = document.getElementById('mor-theme-toggle');
    const htmlEl = document.documentElement;
    const sysDarkQuery = window.matchMedia('(prefers-color-scheme: dark)');

    // Safe storage wrappers to prevent SecurityErrors in sandboxed iframes
    const getStorage = (key) => { try { return localStorage.getItem(key); } catch(e) { return null; } };
    const setStorage = (key, val) => { try { localStorage.setItem(key, val); } catch(e) {} };

    // 1. Determine initial state
    const savedTheme = getStorage('mor-theme');
    let isDark = savedTheme === 'dark' || (!savedTheme && sysDarkQuery.matches);

    // 2. Apply theme and update ARIA states
    const applyTheme = (dark) => {
      if (dark) {
        htmlEl.setAttribute('data-theme', 'dark');
      } else {
        htmlEl.removeAttribute('data-theme');
      }
      
      if (themeToggleBtn) {
        themeToggleBtn.setAttribute('aria-pressed', dark.toString());
        themeToggleBtn.setAttribute('aria-label', dark ? 'Switch to Light Mode' : 'Switch to Dark Mode');
      }
    };

    applyTheme(isDark);

    // 3. Handle manual clicks
    if (themeToggleBtn) {
      themeToggleBtn.addEventListener('click', (event) => {
        event.preventDefault();
        isDark = !isDark;
        setStorage('mor-theme', isDark ? 'dark' : 'light');
        applyTheme(isDark);
      });
    }

    // 4. Live-sync with OS changes
    sysDarkQuery.addEventListener('change', (e) => {
      if (!getStorage('mor-theme')) {
        isDark = e.matches;
        applyTheme(isDark);
      }
    });
  };

  // Run immediately if injected after load (Dioxus), otherwise wait for DOM.
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();