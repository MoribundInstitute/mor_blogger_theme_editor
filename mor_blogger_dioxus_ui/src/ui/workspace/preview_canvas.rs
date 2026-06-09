use dioxus::prelude::*;

use crate::ui::workspace::layout::PreviewViewport;

const SCALER_JS: &str = r#"
(function() {
    function initScaler() {
        const wrapper = document.querySelector('.preview-scale-wrapper');
        const frame = document.getElementById('mor-preview-device-frame');
        if (!wrapper || !frame) return;

        function scaleFrame() {
            if (frame.classList.contains('preview-device-frame-fit')) {
                wrapper.style.setProperty('--preview-scale', 1);
                return;
            }

            const targetWidth = parseFloat(frame.style.width);
            if (!targetWidth) return;

            const availableWidth = wrapper.clientWidth - 48; 
            
            if (targetWidth > availableWidth && availableWidth > 0) {
                const scale = availableWidth / targetWidth;
                wrapper.style.setProperty('--preview-scale', scale);
            } else {
                wrapper.style.setProperty('--preview-scale', 1);
            }
        }

        if (window.__morScalerObs) {
            window.__morScalerObs.disconnect();
        }
        window.__morScalerObs = new ResizeObserver(scaleFrame);
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
    #[props(default)] on_context_menu: Option<EventHandler<String>>,
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
                                                    dioxus.send({
                                                        action: "SELECT",
                                                        target: editTarget.getAttribute('data-edit-target')
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

                                            // NEW: Catch right-clicks for the icon visual picker
                                            doc.addEventListener('contextmenu', function(e) {
                                                const target = e.target && e.target.closest
                                                    ? e.target
                                                    : (e.target && e.target.parentElement);

                                                if (!target) return;

                                                const editTarget = target.closest('[data-edit-target]');
                                                if (editTarget && editTarget.getAttribute('data-edit-target').startsWith('icons.')) {
                                                    e.preventDefault(); // Stop normal browser menu
                                                    e.stopPropagation();
                                                    dioxus.send({
                                                        action: "CONTEXT_MENU",
                                                        target: editTarget.getAttribute('data-edit-target')
                                                    });
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

                                            const observer = new MutationObserver(function () {
                                                writePreview(source, frame);
                                            });

                                            observer.observe(source, {
                                                childList: true,
                                                characterData: true,
                                                subtree: true
                                            });

                                            source.__morPreviewObserver = observer;
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
                                                "CONTEXT_MENU" => {
                                                    if let Some(handler) = on_context_menu.as_ref() {
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