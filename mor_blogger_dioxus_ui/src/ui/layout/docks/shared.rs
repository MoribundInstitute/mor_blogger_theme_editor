//! Shared chrome for the floating side-panel docks.
//!
//! The theme-palette dock and the site-data dock both render the same
//! floating/resizable pane behaviour, so the CSS and the drag/resize scripts
//! live here once instead of being copy-pasted into each dock.

/// Positioning + resizer styling for both the left and right floating panes.
/// Floating width/height deliberately NOT !important: native CSS `resize`
/// writes inline width/height, which must win over the stylesheet defaults.
pub const PANE_CSS: &str = r#"
.mor_panel_left.is-floating, .mor_panel_right.is-floating {
    position: fixed !important;
    top: var(--left-pane-y, 80px) !important;
    right: auto !important;
    bottom: auto !important;
    margin: 0 !important;
    z-index: 100 !important;
    width: 340px;
    max-height: 85vh;
    min-width: 240px;
    min-height: 160px;
    resize: both;
    overflow: auto;
    border: 1px solid var(--editor-border);
    border-radius: var(--radius-md, 10px);
    box-shadow: 0 10px 40px rgba(0,0,0,0.5) !important;
}
.mor_panel_left.is-floating {
    left: var(--left-pane-x, 20px) !important;
}
.mor_panel_right.is-floating {
    left: var(--right-pane-x, calc(100vw - 360px)) !important;
    top: var(--right-pane-y, 80px) !important;
}
/* Dialog-like floating panes (e.g. Plugin Manager) open landscape instead of
   the tall sidebar default. Native resize still wins (inline width/height). */
.is-floating.floating-landscape {
    width: min(680px, 90vw);
    height: min(480px, 80vh);
}
/* Title bar stays pinned while the floating pane scrolls, and follows the
   pane's rounded corners. */
.is-floating .floating-editor-window-bar {
    position: sticky;
    top: 0;
    z-index: 30;
    border-radius: calc(var(--radius-md, 10px) - 1px) calc(var(--radius-md, 10px) - 1px) 0 0 !important;
}
/* Docked: the grid column (--left/right-pane-width) owns the width; the pane
   just fills its container so they can never drift apart. */
.mor_panel_left:not(.is-floating), .mor_panel_right:not(.is-floating) { width: 100% !important; position: relative; }
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

/// Drag a dock's title bar ([data-dock-id]) onto a side zone to move/tab it
/// there. Runs via document::eval (NOT a <script> tag) because it reports
/// drops back to Rust with dioxus.send({dock, zone}). Movement of floating
/// panes stays in PANE_DRAG_JS; this only does drop detection + hints.
pub const DOCK_TAB_DND_JS: &str = r#"
(function () {
    if (window.__morDockTabDndInstalled) return;
    window.__morDockTabDndInstalled = true;

    function zoneAt(x, y, dragged) {
        for (const el of document.elementsFromPoint(x, y)) {
            if (dragged && dragged.contains(el)) continue;
            if (!el.closest) continue;
            if (el.closest('.left-dock-container')) return 'left';
            if (el.closest('.right-dock-container')) return 'right';
        }
        // Edge bands so a dock can be dropped into a currently-empty zone.
        if (x < 90) return 'left';
        if (x > window.innerWidth - 90) return 'right';
        return null;
    }

    function hint(zone) {
        document.querySelectorAll('.left-dock-container, .right-dock-container')
            .forEach(c => c.classList.remove('mor-dock-drop-hint'));
        document.body.classList.toggle('mor-dock-drop-left', zone === 'left');
        document.body.classList.toggle('mor-dock-drop-right', zone === 'right');
        if (!zone) return;
        const c = document.querySelector(zone === 'left' ? '.left-dock-container' : '.right-dock-container');
        if (c) c.classList.add('mor-dock-drop-hint');
    }

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('[data-dock-id]');
        if (!bar) return;
        if (e.target.closest('button, input, a, select, textarea')) return;
        e.preventDefault(); // no text selection while dragging the bar
        const dock = bar.getAttribute('data-dock-id');
        const panel = bar.closest('.mor_panel_left, .mor_panel_right');
        const startX = e.clientX, startY = e.clientY;
        let engaged = false;

        const onMove = function (m) {
            if (!engaged && Math.hypot(m.clientX - startX, m.clientY - startY) < 8) return;
            engaged = true;
            document.body.classList.add('editor-floating-dragging');
            hint(zoneAt(m.clientX, m.clientY, panel));
        };
        const onUp = function (u) {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
            document.body.classList.remove('editor-floating-dragging');
            const drop = engaged ? zoneAt(u.clientX, u.clientY, panel) : null;
            hint(null);
            if (drop) dioxus.send({ dock: dock, zone: drop });
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
