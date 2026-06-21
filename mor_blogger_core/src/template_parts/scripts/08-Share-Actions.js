(() => {
  document.addEventListener('DOMContentLoaded', () => {
    /* =========================================================
    08. Share Menu Toggler & Actions
    ========================================================= */
    const enableShare = typeof _MOR_CONFIG !== 'undefined' ? _MOR_CONFIG.SHARE_ACTIONS : true;
    if (!enableShare) {
      document.querySelectorAll('.sharing-button, .post-share-buttons').forEach(el => el.style.display = 'none');
      return;
    }

    const shareButtons = document.querySelectorAll('.sharing-button');

    // 1. Handle opening/closing the dropdown
    shareButtons.forEach(button => {
      button.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();

        const targetId = button.getAttribute('aria-controls');
        if (targetId) {
          const targetMenu = document.getElementById(targetId);
          if (targetMenu) {
            // Close other open menus first
            document.querySelectorAll('.share-buttons').forEach(menu => {
              if (menu !== targetMenu && !menu.classList.contains('hidden')) {
                menu.classList.add('hidden');
                menu.setAttribute('aria-hidden', 'true');
              }
            });

            targetMenu.classList.toggle('hidden');
            const isHidden = targetMenu.classList.contains('hidden');
            targetMenu.setAttribute('aria-hidden', isHidden.toString());
          }
        }
      });
    });

    // 2. Handle clicking the actual platforms
    const platformButtons = document.querySelectorAll('.sharing-platform-button');
    platformButtons.forEach(btn => {
      btn.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();

        const shareHref = btn.getAttribute('data-href');
        const shareUrl = btn.getAttribute('data-url');
        const isCopyLink = btn.classList.contains('sharing-element-link');

        // Action A: "Get Link" (Copy to Clipboard)
        if (isCopyLink && shareUrl) {
          if (navigator.clipboard) {
            navigator.clipboard.writeText(shareUrl).then(() => {
              // Visual feedback
              const textSpan = btn.querySelector('.platform-sharing-text');
              if (textSpan) {
                const originalText = textSpan.innerText;
                textSpan.innerText = 'Link Copied!';
                setTimeout(() => { textSpan.innerText = originalText; }, 2000);
              }
            });
          } else {
            // Fallback for older browsers
            prompt("Copy to clipboard: Ctrl+C, Enter", shareUrl);
          }
        } 
        // Action B: Social Platforms (Popup Window)
        else if (shareHref) {
          window.open(shareHref, 'shareWindow', 'width=600,height=500,scrollbars=yes,resizable=yes');
        }

        // Close the menu after clicking
        const parentMenu = btn.closest('.share-buttons');
        if (parentMenu) {
          parentMenu.classList.add('hidden');
          parentMenu.setAttribute('aria-hidden', 'true');
        }
      });
    });

    // 3. Close the menu if clicking off-target
    document.addEventListener('click', (event) => {
      if (!event.target.closest('.post-share-buttons')) {
        const allMenus = document.querySelectorAll('.share-buttons');
        allMenus.forEach(menu => {
          if (!menu.classList.contains('hidden')) {
            menu.classList.add('hidden');
            menu.setAttribute('aria-hidden', 'true');
          }
        });
      }
    });
  });
})();
