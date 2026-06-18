(() => {
  // Skip when running inside the editor preview iframe
  if (window.self !== window.top) return;

  const K = 'mor-theme';
  const html = document.documentElement;

  // Apply stored preference on load
  const stored = localStorage.getItem(K);
  if (stored === 'dark' || stored === 'light') {
    html.setAttribute('data-theme', stored);
  }

  // Toggle on button click
  document.addEventListener('click', e => {
    if (!e.target.closest('#mor-theme-toggle')) return;
    const next = html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
    html.setAttribute('data-theme', next);
    try { localStorage.setItem(K, next); } catch (_) {}
  });
})();
