use dioxus::prelude::*;

use crate::app::state::ContextMenuPayload;
use crate::ui::workspace::layout::{clamp_preview_width, PreviewViewport};

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
    #[props(default)] xray_active: Option<Signal<bool>>,
    #[props(default)] on_navigate: Option<EventHandler<String>>,
    #[props(default)] on_select: Option<EventHandler<String>>,
    #[props(default)] on_icon_edit: Option<EventHandler<String>>,
    #[props(default)] on_icon_context_menu: Option<EventHandler<ContextMenuPayload>>,
    #[props(default)] on_update_value: Option<EventHandler<(String, String)>>,
    #[props(default)] on_move_widget: Option<EventHandler<(String, String)>>,
    #[props(default)] on_toggle_dark_mode: Option<EventHandler<()>>,
    #[props(default)] on_drop_svg: Option<EventHandler<(String, String)>>,
) -> Element {
    if let Some(xray_signal) = xray_active {
        use_effect(move || {
            let active = xray_signal();
            spawn(async move {
                let js = format!(
                    "window.__morXrayActive={active};if(window.morApplyXray)window.morApplyXray({active});"
                );
                let _ = dioxus::document::eval(&js);
            });
        });
    }

    let mut preview_width = preview_width;
    let mut preview_viewport = preview_viewport;
    // Drag-to-resize state: whether a drag is active, and (start cursor x, start width).
    let mut dragging = use_signal(|| false);
    let mut drag_start = use_signal(|| (0.0_f64, 0u32));

    let xray_on = xray_active.map(|s| s()).unwrap_or(false);
    let current_viewport = preview_viewport();
    let viewport_label = current_viewport.label();
    let viewport_meta = if current_viewport == PreviewViewport::Fit {
        "Fit · available width".to_string()
    } else {
        format!("{} · {}px wide", viewport_label, preview_width())
    };

    let device_class = if current_viewport == PreviewViewport::Fit {
        "preview-device-frame preview-device-frame-fit"
    } else if dragging() {
        // Kill the width transition mid-drag so the frame tracks the cursor 1:1.
        "preview-device-frame preview-device-frame-resizing"
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

            if xray_on {
                div {
                    class: "preview-xray-legend",
                    span { class: "preview-xray-legend-title", "X-Ray" }
                    span { class: "preview-xray-chip preview-xray-chip-layout", "Layout" }
                    span { class: "preview-xray-chip preview-xray-chip-widget", "Widget · drag" }
                    span { class: "preview-xray-chip preview-xray-chip-text", "Text · dbl-click" }
                    span { class: "preview-xray-chip preview-xray-chip-token", "Theme token" }
                    span { class: "preview-xray-chip preview-xray-chip-icon", "Icon · shift-click" }
                }
            }

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
                        key: "mor-preview-frame-stable",
                        id: "mor-preview-frame",
                        class: "preview-iframe",
                        src: "about:blank",
                        onmounted: move |_| {
                            spawn(async move {
                                let mut eval = dioxus::document::eval(
                                    r#"
                                    (function() {
                                        const SID = "mor-preview-html-source", FID = "mor-preview-frame";

                                        // Full document rewrite. Uses document.write (not innerHTML) so that
                                        // <script> tags (plugin JS) actually execute on (re)load.
                                        function reload(doc, html) {
                                            doc.open(); doc.write(html); doc.close();
                                            setTimeout(() => setup(doc), 50);
                                        }

                                        // Stable identity for an editable node, used to detect structural
                                        // reorders (a widget moving between regions) vs pure content edits.
                                        function keyOf(el) {
                                            return el.getAttribute('data-block-id')
                                                || el.getAttribute('data-field-path')
                                                || el.getAttribute('data-edit-target')
                                                || '';
                                        }

                                        function write(src, frm) {
                                            const html = src.textContent || "";
                                            if (!html.trim() || src._last === html) return;
                                            src._last = html;
                                            const doc = frm.contentDocument || frm.contentWindow.document;
                                            if (!doc) return;

                                            // First paint (blank iframe) or empty body.
                                            if (!doc.body || !doc.body.innerHTML.trim()) {
                                                reload(doc, html);
                                                return;
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

                                            // Match editable nodes by key AND verify identical order. If the
                                            // set or ordering changed (e.g. a widget moved regions), an in-place
                                            // innerHTML patch would scramble content across slots, so reload.
                                            let sameShape = (oT.length === nT.length);
                                            if (sameShape) {
                                                for (let i = 0; i < oT.length; i++) {
                                                    if (keyOf(oT[i]) !== keyOf(nT[i])) { sameShape = false; break; }
                                                }
                                            }
                                            if (!sameShape) { reload(doc, html); return; }

                                            // Same nodes, same order: only inner content can differ, so index
                                            // pairing is now provably safe. Skip a field the user is actively
                                            // editing so a background re-render doesn't wipe the caret / text.
                                            oT.forEach((el, i) => {
                                                if (el.isContentEditable && doc.activeElement === el) return;
                                                if (el.innerHTML !== nT[i].innerHTML) el.innerHTML = nT[i].innerHTML;
                                                if (el.hasAttribute('data-block-id')) el.draggable = true;
                                            });
                                        }
                                        const XRAY_CSS = `
html.mor-xray-on .main-header{outline:2px solid #f59e0b;outline-offset:-2px;background:rgba(245,158,11,.07)!important}
html.mor-xray-on .mor-workspace{outline:2px solid #6366f1;outline-offset:-2px;background:rgba(99,102,241,.05)!important}
html.mor-xray-on .mor-panel.panel-left{outline:2px solid #10b981;outline-offset:-2px;background:rgba(16,185,129,.07)!important}
html.mor-xray-on .mor-panel.panel-right{outline:2px solid #14b8a6;outline-offset:-2px;background:rgba(20,184,166,.07)!important}
html.mor-xray-on .canvas-core{outline:2px solid #8b5cf6;outline-offset:-2px;background:rgba(139,92,246,.06)!important}
html.mor-xray-on .mor-footer{outline:2px solid #eab308;outline-offset:-2px;background:rgba(234,179,8,.06)!important}
html.mor-xray-on .main-header::before,html.mor-xray-on .mor-workspace::before,html.mor-xray-on .mor-panel.panel-left::before,html.mor-xray-on .mor-panel.panel-right::before,html.mor-xray-on .canvas-core::before,html.mor-xray-on .mor-footer::before{position:absolute;top:4px;left:4px;z-index:10000;padding:2px 6px;border-radius:3px;font:600 10px/1.3 ui-monospace,monospace;letter-spacing:.03em;text-transform:uppercase;pointer-events:none}
html.mor-xray-on .main-header::before{content:"Header";background:#f59e0b;color:#451a03}
html.mor-xray-on .mor-workspace::before{content:"Workspace";background:#6366f1;color:#1e1b4b}
html.mor-xray-on .mor-panel.panel-left::before{content:"Left sidebar";background:#10b981;color:#052e16}
html.mor-xray-on .mor-panel.panel-right::before{content:"Right sidebar";background:#14b8a6;color:#042f2e}
html.mor-xray-on .canvas-core::before{content:"Main content";background:#8b5cf6;color:#2e1065}
html.mor-xray-on .mor-footer::before{content:"Footer";background:#eab308;color:#422006}
html.mor-xray-on .main-header,html.mor-xray-on .mor-workspace,html.mor-xray-on .mor-panel,html.mor-xray-on .canvas-core,html.mor-xray-on .mor-footer{position:relative}
html.mor-xray-on [data-block-id]{outline:2px solid #22c55e!important;outline-offset:2px;background:rgba(34,197,94,.09)!important;position:relative;cursor:grab}
html.mor-xray-on [data-block-id]::before{content:attr(data-xray-widget);position:absolute;top:0;right:0;z-index:10001;padding:2px 6px;font:600 10px/1.3 ui-monospace,monospace;background:#22c55e;color:#052e16;pointer-events:none;white-space:nowrap;max-width:100%;overflow:hidden;text-overflow:ellipsis}
html.mor-xray-on [data-field-path]{outline:2px dashed #3b82f6!important;outline-offset:2px;background:rgba(59,130,246,.08)!important;position:relative;cursor:text}
html.mor-xray-on [data-field-path]::after{content:attr(data-xray-hint);position:absolute;bottom:calc(100% + 4px);left:0;z-index:10002;padding:2px 6px;border-radius:3px;font:10px/1.3 ui-monospace,monospace;background:#1d4ed8;color:#eff6ff;pointer-events:none;white-space:nowrap;opacity:0;transition:opacity .12s ease}
html.mor-xray-on [data-field-path]:hover::after{opacity:1}
html.mor-xray-on [data-edit-target^="icons."]{outline:2px solid #f97316!important;outline-offset:2px;box-shadow:0 0 0 3px rgba(249,115,22,.18)!important;position:relative}
html.mor-xray-on [data-edit-target^="icons."]::after{content:attr(data-xray-hint);position:absolute;bottom:calc(100% + 4px);left:0;z-index:10002;padding:2px 6px;border-radius:3px;font:10px/1.3 ui-monospace,monospace;background:#c2410c;color:#fff7ed;pointer-events:none;white-space:nowrap;opacity:0;transition:opacity .12s ease}
html.mor-xray-on [data-edit-target^="icons."]:hover::after{opacity:1}
html.mor-xray-on [data-edit-target]:not([data-field-path]):not([data-block-id]):not([data-edit-target^="icons."]){outline:2px dotted #a855f7!important;outline-offset:2px;background:rgba(168,85,247,.06)!important;position:relative}
html.mor-xray-on [data-edit-target]:not([data-field-path]):not([data-block-id]):not([data-edit-target^="icons."])::after{content:attr(data-xray-hint);position:absolute;bottom:calc(100% + 4px);left:0;z-index:10002;padding:2px 6px;border-radius:3px;font:10px/1.3 ui-monospace,monospace;background:#7e22ce;color:#faf5ff;pointer-events:none;white-space:nowrap;max-width:min(280px,90vw);overflow:hidden;text-overflow:ellipsis;opacity:0;transition:opacity .12s ease}
html.mor-xray-on [data-edit-target]:not([data-field-path]):not([data-block-id]):not([data-edit-target^="icons."]):hover::after{opacity:1}`;

                                        function widgetRegion(el) {
                                            if (el.closest('.panel-left')) return 'Left';
                                            if (el.closest('.panel-right')) return 'Right';
                                            if (el.closest('.canvas-core')) return 'Main';
                                            return 'Layout';
                                        }

                                        function widgetType(el) {
                                            return [...el.classList].filter(c => c !== 'widget').join(' ') || 'Widget';
                                        }

                                        function clearXrayAnnotations(doc) {
                                            doc.querySelectorAll('[data-xray-widget],[data-xray-hint]').forEach(el => {
                                                el.removeAttribute('data-xray-widget');
                                                el.removeAttribute('data-xray-hint');
                                            });
                                        }

                                        function annotateXray(doc) {
                                            clearXrayAnnotations(doc);
                                            doc.querySelectorAll('[data-block-id]').forEach(el => {
                                                const id = el.getAttribute('data-block-id') || 'widget';
                                                el.setAttribute('data-xray-widget', widgetRegion(el) + ' · ' + id + ' (' + widgetType(el) + ')');
                                            });
                                            doc.querySelectorAll('[data-field-path]').forEach(el => {
                                                el.setAttribute('data-xray-hint', 'Dbl-click text · ' + el.getAttribute('data-field-path'));
                                            });
                                            doc.querySelectorAll('[data-edit-target^="icons."]').forEach(el => {
                                                el.setAttribute('data-xray-hint', 'Shift-click icon · ' + el.getAttribute('data-edit-target'));
                                            });
                                            doc.querySelectorAll('[data-edit-target]:not([data-field-path]):not([data-block-id])').forEach(el => {
                                                const target = el.getAttribute('data-edit-target') || '';
                                                if (target.startsWith('icons.')) return;
                                                el.setAttribute('data-xray-hint', 'Theme token · ' + target);
                                            });
                                        }

                                        function ensureXrayStyle(doc) {
                                            let style = doc.getElementById('mor-xray-style');
                                            if (!style) {
                                                style = doc.createElement('style');
                                                style.id = 'mor-xray-style';
                                                style.textContent = XRAY_CSS;
                                                doc.head.appendChild(style);
                                            }
                                        }

                                        function applyXray(doc, active) {
                                            ensureXrayStyle(doc);
                                            doc.documentElement.classList.toggle('mor-xray-on', !!active);
                                            if (active) annotateXray(doc);
                                            else clearXrayAnnotations(doc);
                                        }

                                        window.morApplyXray = function(active) {
                                            window.__morXrayActive = !!active;
                                            const frm = document.getElementById(FID);
                                            const doc = frm && (frm.contentDocument || frm.contentWindow.document);
                                            if (doc && doc.body) applyXray(doc, active);
                                        };

                                        function setup(doc) {
                                            if (doc._inst) return; doc._inst = true;
                                            const s = doc.createElement('style');
                                            s.id = 'mor-edit-style';
                                            s.textContent = `html:not(.mor-xray-on) [data-field-path]:hover{outline:2px dashed #3b82f6;cursor:text} [data-block-id]{cursor:grab;position:relative} .dragging{opacity:0.5} .drag-over{border-top:4px solid #3b82f6}`;
                                            doc.head.appendChild(s);
                                            doc.querySelectorAll('[data-block-id]').forEach(el => el.draggable = true);
                                            doc.addEventListener('dblclick', e => {
                                                const el = e.target.closest('[data-field-path]');
                                                if (el) { el.contentEditable = true; el.focus(); }
                                            });
                                            doc.addEventListener('contextmenu', e => {
                                                const targetEl = e.target.closest("[data-edit-target^='icons.']");
                                                const textTarget = e.target.closest('h1, h2, h3, h4, h5, h6, p, span, a');
                                                if (targetEl || textTarget) {
                                                    e.preventDefault();
                                                    e.stopPropagation();
                                                    const frm = document.getElementById(FID);
                                                    const rect = frm ? frm.getBoundingClientRect() : { left: 0, top: 0 };
                                                    const wrapper = document.querySelector('.preview-scale-wrapper');
                                                    const scale = wrapper ? parseFloat(getComputedStyle(wrapper).getPropertyValue('--preview-scale')) || 1 : 1;
                                                    const x = rect.left + e.clientX * scale;
                                                    const y = rect.top + e.clientY * scale;
                                                    if (targetEl) {
                                                        dioxus.send({
                                                            action: "svg_context_menu",
                                                            kind: "svg",
                                                            target_id: targetEl.getAttribute("data-edit-target"),
                                                            x: x,
                                                            y: y
                                                        });
                                                    } else if (textTarget) {
                                                        dioxus.send({
                                                            action: "svg_context_menu",
                                                            kind: "preview_typography",
                                                            target_id: textTarget.tagName.toLowerCase(),
                                                            x: x,
                                                            y: y
                                                        });
                                                    }
                                                }
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
                                                // 1. Icon Edit (Shift+Click)
                                                if (e.shiftKey) { 
                                                    const targetEl = e.target.closest("[data-edit-target^='icons.']"); 
                                                    if (targetEl) { 
                                                        e.preventDefault(); 
                                                        dioxus.send({action: "ICON_EDIT", target: targetEl.getAttribute("data-edit-target")}); 
                                                        return; 
                                                    } 
                                                }
                                                
                                                // 2. Theme Toggle
                                                const t = e.target, btn = t.closest('#mor-theme-toggle'), a = t.closest('a');
                                                if (btn) { e.preventDefault(); dioxus.send({action: "TOGGLE_DARK_MODE"}); return; }
                                                
                                                // 3. Navigation
                                                if (a && (a.getAttribute('href')||'').match(/^[\/#]/)) {
                                                    e.preventDefault(); dioxus.send({action: "NAVIGATE", target: a.getAttribute('href')});
                                                    return;
                                                }

                                                // 4. NEW: X-Ray Selection Bridge
                                                if (window.__morXrayActive) {
                                                    // Did they click a widget block or a theme token?
                                                    const selectable = e.target.closest('[data-block-id], [data-edit-target]');
                                                    if (selectable) {
                                                        e.preventDefault();
                                                        e.stopPropagation();
                                                        
                                                        // Prefer the edit target (specific token), fallback to block id (widget wrapper)
                                                        const targetId = selectable.getAttribute('data-edit-target') || selectable.getAttribute('data-block-id');
                                                        if (targetId) {
                                                            dioxus.send({action: "SELECT", target: targetId});
                                                        }
                                                    }
                                                }
                                            });
                                            applyXray(doc, window.__morXrayActive);
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
                                            "svg_context_menu" | "ICON_CONTEXT_MENU" => {
                                                if let (Some(target), Some(x_val), Some(y_val)) = (
                                                    json.get("target_id").or_else(|| json.get("target")).and_then(|t| t.as_str()),
                                                    json.get("x").and_then(|x| x.as_f64()),
                                                    json.get("y").and_then(|y| y.as_f64()),
                                                ) {
                                                    let kind_str = json.get("kind").and_then(|k| k.as_str()).unwrap_or("svg").to_string();
                                                    if let Some(handler) = on_icon_context_menu.as_ref() {
                                                        handler.call(ContextMenuPayload {
                                                            x: x_val,
                                                            y: y_val,
                                                            kind: kind_str,
                                                            target_id: target.to_string(),
                                                        });
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            });
                        }
                    }

                    // Drag-to-resize handle on the frame's right edge (fixed-width modes only).
                    if current_viewport != PreviewViewport::Fit {
                        div {
                            class: "preview-resize-handle",
                            title: "Drag to resize preview width",
                            onmousedown: move |e: MouseEvent| {
                                if e.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
                                    drag_start.set((e.client_coordinates().x, preview_width()));
                                    dragging.set(true);
                                }
                            },
                            div { class: "preview-resize-handle-grip" }
                        }
                    }
                }
            }

            // While dragging, an overlay above the iframe keeps mousemove/up flowing
            // (iframes swallow mouse events) so the drag never gets stuck.
            if dragging() {
                div {
                    class: "preview-resize-overlay",
                    onmousemove: move |e: MouseEvent| {
                        let (start_x, start_w) = drag_start();
                        let dx = e.client_coordinates().x - start_x;
                        // ponytail: 2x because the frame is center-anchored, so the right
                        // edge moves dx for every 2*dx of width. Ignores --preview-scale,
                        // which is only <1 when the frame is already wider than the viewport.
                        let new_w = clamp_preview_width((start_w as f64 + dx * 2.0).round() as u32);
                        preview_width.set(new_w);
                        preview_viewport.set(PreviewViewport::Custom);
                    },
                    onmouseup: move |_| dragging.set(false),
                    onmouseleave: move |_| dragging.set(false),
                }
            }

            script { dangerous_inner_html: "{SCALER_JS}" }
        }
    }
}
