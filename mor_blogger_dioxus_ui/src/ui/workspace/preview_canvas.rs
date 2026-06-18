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
    #[props(default)] on_update_value: Option<EventHandler<(String, String)>>,
    #[props(default)] on_move_widget: Option<EventHandler<(String, String)>>,
    #[props(default)] on_toggle_dark_mode: Option<EventHandler<()>>,
    #[props(default)] on_drop_svg: Option<EventHandler<(String, String)>>,
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
                                    (function() {
                                        const SID = "mor-preview-html-source", FID = "mor-preview-frame";
                                        function write(src, frm) {
                                            const html = src.textContent || "";
                                            if (!html.trim() || src._last === html) return;
                                            src._last = html;
                                            const doc = frm.contentDocument || frm.contentWindow.document;
                                            if (!doc) return;
                                            if (!doc.body || !doc.body.innerHTML.trim() || src._reload) {
                                                doc.open(); doc.write(html); doc.close(); src._reload = false;
                                                setTimeout(() => setup(doc), 50); return;
                                            }
                                            const nDoc = new DOMParser().parseFromString(html, 'text/html');
                                            const oCss = doc.getElementById('mor-true-css'), nCss = nDoc.getElementById('mor-true-css');
                                            if (oCss && nCss && oCss.textContent !== nCss.textContent) oCss.textContent = nCss.textContent;
                                            if (doc.body.style.cssText !== nDoc.body.style.cssText) doc.body.style.cssText = nDoc.body.style.cssText;
                                            if (doc.documentElement.style.cssText !== nDoc.documentElement.style.cssText) doc.documentElement.style.cssText = nDoc.documentElement.style.cssText;
                                            if (doc.documentElement.className !== nDoc.documentElement.className) doc.documentElement.className = nDoc.documentElement.className;
                                            if (doc.documentElement.getAttribute('data-theme') !== nDoc.documentElement.getAttribute('data-theme')) doc.documentElement.setAttribute('data-theme', nDoc.documentElement.getAttribute('data-theme') || "");
                                            if (doc.body.className !== nDoc.body.className) doc.body.className = nDoc.body.className;
                                            
                                            doc.querySelectorAll('link[href*="fonts.googleapis"], link[href*="fonts.gstatic"], style').forEach(el => {
                                                if (el.id !== 'mor-true-css' && !el.textContent.includes('[data-field-path]')) el.remove();
                                            });
                                            nDoc.querySelectorAll('link[href*="fonts.googleapis"], link[href*="fonts.gstatic"], style').forEach(el => { if (el.id !== 'mor-true-css') doc.head.appendChild(el.cloneNode(true)); });
                                            
                                            const oT = doc.querySelectorAll('[data-field-path], [data-block-id], [data-edit-target]');
                                            const nT = nDoc.querySelectorAll('[data-field-path], [data-block-id], [data-edit-target]');
                                            if (oT.length !== nT.length) { src._reload = true; write(src, frm); return; }
                                            oT.forEach((el, i) => { 
                                                if (el.innerHTML !== nT[i].innerHTML) el.innerHTML = nT[i].innerHTML; 
                                                if (el.hasAttribute('data-block-id')) el.draggable = true;
                                            });
                                        }
                                        function setup(doc) {
                                            if (doc._inst) return; doc._inst = true;
                                            const s = doc.createElement('style');
                                            s.textContent = `[data-field-path]:hover{outline:2px dashed #3b82f6;cursor:text} [data-block-id]{cursor:grab;position:relative} .dragging{opacity:0.5} .drag-over{border-top:4px solid #3b82f6}`;
                                            doc.head.appendChild(s);
                                            doc.querySelectorAll('[data-block-id]').forEach(el => el.draggable = true);
                                            doc.addEventListener('dblclick', e => {
                                                const el = e.target.closest('[data-field-path]');
                                                if (el) { el.contentEditable = true; el.focus(); }
                                            });
                                            doc.addEventListener('blur', e => {
                                                const el = e.target.closest('[data-field-path]');
                                                if (el && el.contentEditable === "true") {
                                                    el.contentEditable = false;
                                                    dioxus.send({action: "UPDATE_VALUE", target: el.getAttribute('data-field-path'), value: el.innerText});
                                                }
                                            }, true);
                                            let draggedId = null;
                                            doc.addEventListener('dragstart', e => {
                                                const el = e.target.closest('[data-block-id]');
                                                if (el) { draggedId = el.getAttribute('data-block-id'); e.dataTransfer.effectAllowed = 'move'; el.classList.add('dragging'); }
                                            });
                                            doc.addEventListener('dragend', e => e.target.closest('[data-block-id]')?.classList.remove('dragging'));
                                            doc.addEventListener('dragover', e => {
                                                if (e.dataTransfer.types.includes('Files')) { e.preventDefault(); e.dataTransfer.dropEffect = 'copy'; return; }
                                                e.preventDefault();
                                                const el = e.target.closest('[data-block-id]');
                                                if (el && el.getAttribute('data-block-id') !== draggedId) { el.classList.add('drag-over'); e.dataTransfer.dropEffect = 'move'; }
                                            });
                                            doc.addEventListener('dragleave', e => e.target.closest('[data-block-id]')?.classList.remove('drag-over'));
                                            doc.addEventListener('drop', e => {
                                                if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
                                                    e.preventDefault();
                                                    const file = e.dataTransfer.files[0];
                                                    if (file.type.includes("svg") || file.name.endsWith(".svg")) {
                                                        const targetEl = e.target.closest('[data-edit-target^="icons."]');
                                                        if (targetEl) {
                                                            const targetName = targetEl.getAttribute('data-edit-target');
                                                            const reader = new FileReader();
                                                            reader.onload = (event) => {
                                                                dioxus.send({ action: "DROP_SVG", target: targetName, content: event.target.result });
                                                            };
                                                            reader.readAsText(file);
                                                        }
                                                    }
                                                    return;
                                                }
                                                e.preventDefault();
                                                const el = e.target.closest('[data-block-id]');
                                                if (el) {
                                                    el.classList.remove('drag-over');
                                                    const destId = el.getAttribute('data-block-id');
                                                    if (draggedId && draggedId !== destId)
                                                        dioxus.send({action: "MOVE_WIDGET", id: draggedId, dest: destId});
                                                }
                                                draggedId = null;
                                            });
                                            doc.addEventListener('click', e => {
                                                const t = e.target, btn = t.closest('#mor-theme-toggle'), a = t.closest('a');
                                                if (btn) { e.preventDefault(); dioxus.send({action: "TOGGLE_DARK_MODE"}); }
                                                else if (a && (a.getAttribute('href')||'').match(/^[/|#]/)) {
                                                    e.preventDefault(); dioxus.send({action: "NAVIGATE", target: a.getAttribute('href')});
                                                }
                                            });
                                        }
                                        function install() {
                                            const src = document.getElementById(SID), frm = document.getElementById(FID);
                                            if (!src || !frm) return setTimeout(install, 50);
                                            new MutationObserver(() => write(src, frm)).observe(src, {childList:true, characterData:true, subtree:true});
                                            write(src, frm);
                                        }
                                        install();
                                    })();
                                    "#
                                );

                                while let Ok(json) = eval.recv::<serde_json::Value>().await {
                                    if let Some(action) = json.get("action").and_then(|a| a.as_str()) {
                                        match action {
                                            "SELECT" => {
                                                if let Some(target) = json.get("target").and_then(|t| t.as_str()) {
                                                    if let Some(handler) = on_select.as_ref() { handler.call(target.to_string()); }
                                                }
                                            }
                                            "NAVIGATE" => {
                                                if let Some(target) = json.get("target").and_then(|t| t.as_str()) {
                                                    if let Some(handler) = on_navigate.as_ref() { handler.call(target.to_string()); }
                                                }
                                            }
                                            "ICON_EDIT" => {
                                                if let Some(target) = json.get("target").and_then(|t| t.as_str()) {
                                                    if let Some(handler) = on_icon_edit.as_ref() { handler.call(target.to_string()); }
                                                }
                                            }
                                            "UPDATE_VALUE" => {
                                                if let (Some(target), Some(val)) = (json.get("target").and_then(|t| t.as_str()), json.get("value").and_then(|v| v.as_str())) {
                                                    if let Some(handler) = on_update_value.as_ref() { handler.call((target.to_string(), val.to_string())); }
                                                }
                                            }
                                            "MOVE_WIDGET" => {
                                                if let (Some(id), Some(dest)) = (json.get("id").and_then(|i| i.as_str()), json.get("dest").and_then(|d| d.as_str())) {
                                                    if let Some(handler) = on_move_widget.as_ref() { handler.call((id.to_string(), dest.to_string())); }
                                                }
                                            }
                                            "TOGGLE_DARK_MODE" => {
                                                if let Some(handler) = on_toggle_dark_mode.as_ref() { handler.call(()); }
                                            }
                                            "DROP_SVG" => {
                                                if let (Some(target), Some(content)) = (json.get("target").and_then(|t| t.as_str()), json.get("content").and_then(|c| c.as_str())) {
                                                    if let Some(handler) = on_drop_svg.as_ref() { handler.call((target.to_string(), content.to_string())); }
                                                }
                                            }
                                            _ => {}
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