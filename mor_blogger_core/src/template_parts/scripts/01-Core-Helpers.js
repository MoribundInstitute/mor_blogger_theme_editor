(() => {
  document.addEventListener('DOMContentLoaded', () => {
    /* =========================================================
    01. Shared Helpers
    ========================================================= */
    const MOBILE_BREAKPOINT = 900;
    const isMobile = () => window.innerWidth <= MOBILE_BREAKPOINT;

    function isTypingInField() {
      const activeElement = document.activeElement;
      if (!activeElement) return false;
      const activeTag = activeElement.tagName ? activeElement.tagName.toLowerCase() : '';
      return activeTag === 'input' || activeTag === 'textarea' || activeTag === 'select' || activeElement.isContentEditable;
    }

    function safeClosest(target, selector) {
      if (!target || typeof target.closest !== 'function') return null;
      return target.closest(selector);
    }

    /* =========================================================
    02. Side Panels + Back to Top
    ========================================================= */
    const panelLeft = document.getElementById('panel-left');
    const panelRight = document.getElementById('panel-right');
    const canvasCore = document.querySelector('.canvas-core');
    const backToTopBtn = document.querySelector('.back-to-top-btn') || document.querySelector('.footer-sys-info button[onclick*="scrollTo"]');

    function scrollMainContentToTop() {
      const scrollOptions = { top: 0, behavior: 'smooth' };
      if (canvasCore && typeof canvasCore.scrollTo === 'function') canvasCore.scrollTo(scrollOptions);
      if (typeof window.scrollTo === 'function') window.scrollTo(scrollOptions);
      if (document.documentElement && typeof document.documentElement.scrollTo === 'function') document.documentElement.scrollTo(scrollOptions);
      if (document.body && typeof document.body.scrollTo === 'function') document.body.scrollTo(scrollOptions);
    }

    function setInitialPanelState() {
      if (!panelLeft || !panelRight) return;
      if (isMobile()) {
        panelLeft.classList.add('is-collapsed');
        panelRight.classList.add('is-collapsed');
      } else {
        panelLeft.classList.remove('is-collapsed');
        panelRight.classList.remove('is-collapsed');
      }
    }

    function closeMobilePanels() {
      if (!isMobile()) return;
      if (panelLeft) panelLeft.classList.add('is-collapsed');
      if (panelRight) panelRight.classList.add('is-collapsed');
    }

    setInitialPanelState();

    if (backToTopBtn) {
      backToTopBtn.removeAttribute('onclick');
      backToTopBtn.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();
        scrollMainContentToTop();
      });
    }

    /* =========================================================
    03. Catalog Mega Dropdown
    ========================================================= */
    const catalogMenu = document.querySelector('.mor-catalog-menu');
    let catalogTrigger = null;

    function openCatalogMenu() {
      if (!catalogMenu) return;
      catalogMenu.classList.add('is-open');
      if (catalogTrigger) catalogTrigger.setAttribute('aria-expanded', 'true');
    }

    function closeCatalogMenu() {
      if (!catalogMenu) return;
      catalogMenu.classList.remove('is-open');
      if (catalogTrigger) catalogTrigger.setAttribute('aria-expanded', 'false');
    }

    function toggleCatalogMenu() {
      if (!catalogMenu) return;
      if (catalogMenu.classList.contains('is-open')) {
        closeCatalogMenu();
      } else {
        openCatalogMenu();
      }
    }

    function setupCatalogMenu() {
      if (!catalogMenu) return;
      catalogTrigger = catalogMenu.querySelector('.mor-catalog-trigger');
      const categoryButtons = catalogMenu.querySelectorAll('.mor-catalog-category');
      const panels = catalogMenu.querySelectorAll('.mor-catalog-panel');

      function showPanel(panelName) {
        if (!panelName) return;
        categoryButtons.forEach((button) => {
          const isActive = button.getAttribute('data-catalog-panel') === panelName;
          button.classList.toggle('is-active', isActive);
        });
        panels.forEach((panel) => {
          const isActive = panel.getAttribute('data-catalog-panel-content') === panelName;
          panel.classList.toggle('is-active', isActive);
        });
      }

      if (catalogTrigger) {
        catalogTrigger.addEventListener('click', (event) => {
          event.preventDefault();
          event.stopPropagation();
          toggleCatalogMenu();
        });
      }

      categoryButtons.forEach((button) => {
        const panelName = button.getAttribute('data-catalog-panel');
        button.addEventListener('mouseenter', () => { showPanel(panelName); openCatalogMenu(); });
        button.addEventListener('focus', () => { showPanel(panelName); openCatalogMenu(); });
        button.addEventListener('click', (event) => {
          event.preventDefault();
          event.stopPropagation();
          showPanel(panelName);
          openCatalogMenu();
        });
      });
    }
    setupCatalogMenu();

    /* =========================================================
    04. Global Click Handling
    ========================================================= */
    document.addEventListener('click', (event) => {
      const toggleBtn = safeClosest(event.target, '.panel-toggle');
      if (toggleBtn) {
        const targetId = toggleBtn.getAttribute('data-target');
        if (targetId) {
          const targetPanel = document.getElementById(targetId);
          if (targetPanel) {
            event.preventDefault();
            targetPanel.classList.toggle('is-collapsed');
            
            // Accessibility update
            const isCollapsed = targetPanel.classList.contains('is-collapsed');
            toggleBtn.setAttribute('aria-expanded', (!isCollapsed).toString());
          }
        }
      }
      if (catalogMenu && !catalogMenu.contains(event.target)) {
        closeCatalogMenu();
      }
    });

    /* =========================================================
    05. Keyboard Shortcuts
    ========================================================= */
    document.addEventListener('keydown', (event) => {
      const key = event.key ? event.key.toLowerCase() : '';
      const typing = isTypingInField();

      if (event.ctrlKey && key === 'b') {
        event.preventDefault();
        if (panelLeft) panelLeft.classList.toggle('is-collapsed');
      }
      if (event.ctrlKey && key === 'e') {
        event.preventDefault();
        if (panelRight) panelRight.classList.toggle('is-collapsed');
      }
      if (key === 'home' && !typing) {
        event.preventDefault();
        scrollMainContentToTop();
      }
      if (key === 'escape') {
        if (catalogMenu && catalogMenu.classList.contains('is-open')) {
          closeCatalogMenu();
          if (catalogTrigger) catalogTrigger.focus();
        }
        closeMobilePanels();
      }
    });

    /* =========================================================
    06. Resize Handling
    ========================================================= */
    let lastMobileState = isMobile();
    window.addEventListener('resize', () => {
      const currentMobileState = isMobile();
      if (currentMobileState !== lastMobileState) {
        lastMobileState = currentMobileState;
        setInitialPanelState();
        closeCatalogMenu();
      }
    });
  });
})();