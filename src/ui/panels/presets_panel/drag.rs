pub(crate) const PRESET_FLOATING_DRAG_JS: &str = r#"
(function () {
    if (window.__morPresetFloatingDragInstalled) return;
    window.__morPresetFloatingDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.preset-floating-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, textarea, select, a, label')) return;

        const panel = bar.closest('.preset-floating-window');
        if (!panel) return;

        e.preventDefault();

        const startX = e.clientX;
        const startY = e.clientY;
        const rect = panel.getBoundingClientRect();
        const startLeft = rect.left;
        const startTop = rect.top;

        document.body.classList.add('editor-floating-dragging');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;

            const maxLeft = Math.max(0, window.innerWidth - 160);
            const maxTop = Math.max(0, window.innerHeight - 80);

            const nextLeft = Math.max(0, Math.min(startLeft + dx, maxLeft));
            const nextTop = Math.max(0, Math.min(startTop + dy, maxTop));

            document.documentElement.style.setProperty('--preset-floating-x', nextLeft + 'px');
            document.documentElement.style.setProperty('--preset-floating-y', nextTop + 'px');
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
