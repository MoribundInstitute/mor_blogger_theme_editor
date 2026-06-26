//! Shared chrome for the floating side-panel docks.
//!
//! The theme-palette dock and the site-data dock both render the same
//! floating/resizable pane behaviour, so the CSS and the drag/resize scripts
//! live here once instead of being copy-pasted into each dock.

/// Positioning + resizer styling for both the left and right floating panes.
pub const PANE_CSS: &str = r#"
.mor_panel_left.is-floating {
    position: fixed !important;
    left: var(--left-pane-x, 20px) !important;
    top: var(--left-pane-y, 80px) !important;
    right: auto !important;
    bottom: auto !important;
    margin: 0 !important;
    z-index: 100 !important;
    width: 320px !important;
    max-height: 85vh !important;
    box-shadow: 0 10px 40px rgba(0,0,0,0.5) !important;
}
.mor_panel_right.is-floating {
    position: fixed !important;
    left: var(--right-pane-x, calc(100vw - 340px)) !important;
    top: var(--right-pane-y, 80px) !important;
    right: auto !important;
    bottom: auto !important;
    margin: 0 !important;
    z-index: 100 !important;
    width: 320px !important;
    max-height: 85vh !important;
    box-shadow: 0 10px 40px rgba(0,0,0,0.5) !important;
}
.mor_panel_left:not(.is-floating) { width: var(--left-pane-width, 320px) !important; position: relative; }
.mor_panel_right:not(.is-floating) { width: var(--right-pane-width, 320px) !important; position: relative; }
.pane-resizer { position: absolute; top: 0; bottom: 0; width: 6px; z-index: 999; cursor: ew-resize; background: transparent; transition: background 0.1s; }
.pane-resizer:hover, .pane-resizer:active { background: var(--editor-accent, rgba(255,255,255,0.2)); }
.pane-resizer-right { right: 0; }
.pane-resizer-left { left: 0; }
"#;

/// Title-bar drag handler that repositions a floating pane.
pub const PANE_DRAG_JS: &str = r#"
(function () {
    if (window.__morCorePaneDragInstalled) return;
    window.__morCorePaneDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.floating-editor-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, a, select, textarea')) return;

        const panel = bar.closest('.mor_panel_left, .mor_panel_right');
        if (!panel) return;

        if (window.getComputedStyle(panel).position !== 'fixed' && window.getComputedStyle(panel).position !== 'absolute') return;

        e.preventDefault();

        const isLeft = panel.classList.contains('mor_panel_left');
        const varX = isLeft ? '--left-pane-x' : '--right-pane-x';
        const varY = isLeft ? '--left-pane-y' : '--right-pane-y';

        const rect = panel.getBoundingClientRect();
        const startX = e.clientX;
        const startY = e.clientY;
        const startLeft = rect.left;
        const startTop = rect.top;

        document.documentElement.style.setProperty(varX, startLeft + 'px');
        document.documentElement.style.setProperty(varY, startTop + 'px');
        document.body.classList.add('editor-floating-dragging');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;

            const nextLeft = Math.max(0, Math.min(startLeft + dx, window.innerWidth - 100));
            const nextTop = Math.max(0, Math.min(startTop + dy, window.innerHeight - 40));

            document.documentElement.style.setProperty(varX, nextLeft + 'px');
            document.documentElement.style.setProperty(varY, nextTop + 'px');
        };

        const onUp = function () {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
            document.body.classList.remove('editor-floating-dragging');
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

/// Edge-resizer handler that adjusts a pane's width.
pub const PANE_RESIZE_JS: &str = r#"
(function () {
    if (window.__morCorePaneResizeInstalled) return;
    window.__morCorePaneResizeInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const resizer = e.target.closest('.pane-resizer');
        if (!resizer) return;

        const panel = resizer.closest('.mor_panel_left, .mor_panel_right');
        if (!panel) return;

        e.preventDefault();

        const isLeft = panel.classList.contains('mor_panel_left');
        const varWidth = isLeft ? '--left-pane-width' : '--right-pane-width';
        const startX = e.clientX;
        const startWidth = panel.getBoundingClientRect().width;

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const newWidth = isLeft ? (startWidth + dx) : (startWidth - dx);
            const clamped = Math.max(220, Math.min(newWidth, 600));
            document.documentElement.style.setProperty(varWidth, clamped + 'px');
        };

        const onUp = function () {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;
