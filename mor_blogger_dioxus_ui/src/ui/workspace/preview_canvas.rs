use dioxus::prelude::*;

use crate::ui::workspace::layout::PreviewViewport;

const SCALER_JS: &str = r#"
(function() {
    function initScaler() {
        const wrapper = document.querySelector('.preview-scale-wrapper');
        const frame = document.getElementById('mor-preview-device-frame');
        if (!wrapper || !frame) return;

        let lastScale = null;
        function applyScale(scale) {
            // Only touch the DOM when the value actually changes. Writing the
            // same value back re-triggers the ResizeObserver and feeds a loop.
            if (scale === lastScale) return;
            lastScale = scale;
            wrapper.style.setProperty('--preview-scale', scale);
        }

        function scaleFrame() {
            if (frame.classList.contains('preview-device-frame-fit')) {
                applyScale(1);
                return;
            }

            const targetWidth = parseFloat(frame.style.width);
            if (!targetWidth) return;

            const availableWidth = wrapper.clientWidth - 48;

            if (targetWidth > availableWidth && availableWidth > 0) {
                applyScale(availableWidth / targetWidth);
            } else {
                applyScale(1);
            }
        }

        let rafPending = false;
        function scheduleScale() {
            // Defer the measure/write to the next frame so the ResizeObserver
            // callback returns immediately (no "undelivered notifications").
            if (rafPending) return;
            rafPending = true;
            requestAnimationFrame(function () {
                rafPending = false;
                scaleFrame();
            });
        }

        if (window.__morScalerObs) {
            window.__morScalerObs.disconnect();
        }
        window.__morScalerObs = new ResizeObserver(scheduleScale);
        window.__morScalerObs.observe(wrapper);
        scaleFrame();
    }

    initScaler();
    setTimeout(initScaler, 50);
})();
"#;

#[component]
pub fn PreviewCanvas(
    preview_viewport: Signal<PreviewViewport>,
    preview_width: Signal<u32>,
    preview_html: String,
    #[props(default)] on_navigate: Option<EventHandler<String>>,
    #[props(default)] on_select: Option<EventHandler<String>>,
    #[props(default)] on_icon_edit: Option<EventHandler<String>>,
) -> Element {
    let current_viewport = preview_viewport();
    let viewport_label = current_viewport.label();
    let viewport_meta = if current_viewport == PreviewViewport::Fit {
        "Fit · available width".to_string()
    } else {
        format!("{} · {}px wide", viewport_label, preview_width())
    };

    let device_class = if current_viewport == PreviewViewport::Fit {
        "preview-device-frame preview-device-frame-fit"
    } else {
        "preview-device-frame"
    };

    let device_style = if current_viewport == PreviewViewport::Fit {
        String::new()
    } else {
        format!("width: {}px;", preview_width())
    };

    rsx! {
        div {
            class: "preview-canvas",

            div {
                class: "preview-ruler",
                span {
                    class: "preview-ruler-label",
                    "{viewport_meta}"
                }
                div { class: "preview-ruler-line" }
            }

            div {
                class: "preview-scale-wrapper",

                div {
                    class: "{device_class}",
                    id: "mor-preview-device-frame",
                    style: "{device_style}",

                    pre {
                        id: "mor-preview-html-source",
                        style: "display: none;",
                        "{preview_html}"
                    }

                    iframe {
                        id: "mor-preview-frame",
                        class: "preview-iframe",
                        src: "about:blank",
                        onmounted: move |_| {
                            spawn(async move {
                                let mut eval = dioxus::document::eval(
                                    r#"
                                    (function installMorPreviewBridge() {
                                        const sourceId = "mor-preview-html-source";
                                        const frameId = "mor-preview-frame";

                                        function writePreview(source, frame) {
                                            const html = source.textContent || "";
                                            if (!html.trim()) return;
                                            if (source.__morLastPreviewHtml === html) return;
                                            source.__morLastPreviewHtml = html;

                                            const doc = frame.contentDocument ||
                                                (frame.contentWindow && frame.contentWindow.document);
                                            if (!doc) return;

                                            doc.open();
                                            doc.write(html);
                                            doc.close();

                                            // Add native tooltip to icons
                                            doc.querySelectorAll('[data-edit-target^="icons."]').forEach(function(el) {
                                                el.title = "Alt+Click to edit icon";
                                            });

                                            // Inject edit hover styles
                                            if (!doc.getElementById('mor-edit-styles')) {
                                                const style = doc.createElement('style');
                                                style.id = 'mor-edit-styles';
                                                style.textContent = `
                                                    [data-edit-target] { transition: outline 0.1s; outline: 2px solid transparent; outline-offset: 2px; }
                                                    [data-edit-target]:hover { outline: 2px dashed var(--accent, #3b82f6); cursor: pointer; }
                                                `;
                                                doc.head.appendChild(style);
                                            }

                                            doc.addEventListener('click', function(e) {
                                                const target = e.target && e.target.closest
                                                    ? e.target
                                                    : (e.target && e.target.parentElement);

                                                if (!target) return;

                                                const editTarget = target.closest('[data-edit-target]');
                                                if (editTarget) {
                                                    e.preventDefault();
                                                    e.stopPropagation();
                                                    
                                                    const targetId = editTarget.getAttribute('data-edit-target');

                                                    if (e.altKey && targetId.startsWith('icons.')) {
                                                        dioxus.send({
                                                            action: "ICON_EDIT",
                                                            target: targetId
                                                        });
                                                        return;
                                                    }

                                                    dioxus.send({
                                                        action: "SELECT",
                                                        target: targetId
                                                    });
                                                    return;
                                                }

                                                const anchor = target.closest('a');
                                                if (anchor) {
                                                    const href = anchor.getAttribute('href');
                                                    if (href && (href.startsWith('/') || href.startsWith('#'))) {
                                                        e.preventDefault();
                                                        dioxus.send({
                                                            action: "NAVIGATE",
                                                            target: href
                                                        });
                                                    }
                                                }
                                            });
                                        }

                                        function install(attempt) {
                                            const source = document.getElementById(sourceId);
                                            const frame = document.getElementById(frameId);

                                            if (!source || !frame) {
                                                if (attempt < 40) {
                                                    setTimeout(function () { install(attempt + 1); }, 25);
                                                }
                                                return;
                                            }

                                            if (source.__morPreviewObserver) {
                                                source.__morPreviewObserver.disconnect();
                                            }

                                            let writeTimer = null;
                                            const observer = new MutationObserver(function () {
                                                // Coalesce rapid edits (typing) into a single rewrite
                                                // so the main thread isn't saturated by back-to-back
                                                // full-document writes — that backlog is what delays
                                                // the alt+click IPC message reaching Rust.
                                                if (writeTimer) clearTimeout(writeTimer);
                                                writeTimer = setTimeout(function () {
                                                    writeTimer = null;
                                                    writePreview(source, frame);
                                                }, 80);
                                            });

                                            observer.observe(source, {
                                                childList: true,
                                                characterData: true,
                                                subtree: true
                                            });

                                            source.__morPreviewObserver = observer;
                                            // Initial paint stays immediate.
                                            writePreview(source, frame);
                                        }

                                        install(0);
                                    })();
                                    "#
                                );

                                while let Ok(json) = eval.recv::<serde_json::Value>().await {
                                    if let Some(action) = json.get("action").and_then(|a| a.as_str()) {
                                        if let Some(target) = json.get("target").and_then(|t| t.as_str()) {
                                            match action {
                                                "SELECT" => {
                                                    if let Some(handler) = on_select.as_ref() {
                                                        handler.call(target.to_string());
                                                    }
                                                }
                                                "NAVIGATE" => {
                                                    if let Some(handler) = on_navigate.as_ref() {
                                                        handler.call(target.to_string());
                                                    }
                                                }
                                                "ICON_EDIT" => {
                                                    if let Some(handler) = on_icon_edit.as_ref() {
                                                        handler.call(target.to_string());
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }

            script { dangerous_inner_html: "{SCALER_JS}" }
        }
    }
}