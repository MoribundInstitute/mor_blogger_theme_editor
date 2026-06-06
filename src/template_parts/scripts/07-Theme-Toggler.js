(() => {
  document.addEventListener('DOMContentLoaded', () => {
    /* =========================================================
    07. Light/Dark Theme Toggler
    ========================================================= */
    const themeToggleBtn = document.getElementById('mor-theme-toggle');
    const htmlEl = document.documentElement;

    const savedTheme = localStorage.getItem('mor-theme');
    const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

    if (savedTheme === 'dark' || (!savedTheme && systemDark)) {
      htmlEl.setAttribute('data-theme', 'dark');
    }

    if (themeToggleBtn) {
      themeToggleBtn.addEventListener('click', (event) => {
        event.preventDefault();
        if (htmlEl.getAttribute('data-theme') === 'dark') {
          htmlEl.removeAttribute('data-theme');
          localStorage.setItem('mor-theme', 'light');
        } else {
          htmlEl.setAttribute('data-theme', 'dark');
          localStorage.setItem('mor-theme', 'dark');
        }
      });
    }
  });
})();