use dioxus::prelude::*;
use std::collections::HashMap;
use crate::app::layout_state::{AppLayoutState, CenterView, DockPosition};
use crate::app::state::ThemeAppState;
use crate::ui::components::code_editor::CodeEditor;
use mor_blogger_core::render::template_resolver::{
    fetch_default_css, ComponentManifest, CONTENT_REGISTRY, FOOTER_REGISTRY, HEADER_REGISTRY,
    LAYOUT_REGISTRY, SIDEBAR_LEFT_REGISTRY, SIDEBAR_RIGHT_REGISTRY,
};

#[derive(Clone, Copy)]
pub struct VfsDictionary(pub Signal<HashMap<String, String>>);

const EDITOR_DRAG_JS: &str = r#"
(function () {
    if (window.__morCssDragInstalled) return;
    window.__morCssDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.floating-editor-window-bar');
        if (!bar) return;
        const panel = bar.closest('.floating-css-editor');
        if (!panel) return;
        
        e.preventDefault();
        
        const rect = panel.getBoundingClientRect();
        const startX = e.clientX;
        const startY = e.clientY;
        const startLeft = rect.left;
        const startTop = rect.top;

        document.documentElement.style.setProperty('--css-dock-x', startLeft + 'px');
        document.documentElement.style.setProperty('--css-dock-y', startTop + 'px');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;
            document.documentElement.style.setProperty('--css-dock-x', Math.max(0, startLeft + dx) + 'px');
            document.documentElement.style.setProperty('--css-dock-y', Math.max(0, startTop + dy) + 'px');
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

const PANE_RESIZE_JS: &str = r#"
(function() {
    if (window.__morPaneResizeInstalled) return;
    window.__morPaneResizeInstalled = true;
    document.addEventListener('pointerdown', function(e) {
        const resizer = e.target.closest('.pane-resizer');
        if (!resizer) return;
        e.preventDefault();
        const isLeft = resizer.classList.contains('pane-resizer-right');
        const startX = e.clientX;
        const panel = resizer.closest('.editor-left-panel, .editor-right-panel');
        const startWidth = panel.getBoundingClientRect().width;
        
        const onMove = function(moveEvt) {
            const dx = moveEvt.clientX - startX;
            const newWidth = isLeft ? startWidth + dx : startWidth - dx;
            const clamped = Math.max(200, Math.min(newWidth, window.innerWidth / 2.5));
            const varName = isLeft ? '--left-pane-width' : '--right-pane-width';
            document.documentElement.style.setProperty(varName, clamped + 'px');
        };
        const onUp = function() {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
        };
        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

const EDITOR_CSS: &str = r#"
    .editor-left-panel.css-editor-mode,
    .editor-right-panel.css-editor-mode {
        padding: 0 !important;
        overflow: hidden !important;
        display: flex !important;
        flex-direction: column !important;
        width: 100% !important;
        height: 100% !important;
        box-sizing: border-box !important;
    }
    .floating-css-editor {
        position: fixed;
        left: var(--css-dock-x, 100px);
        top: var(--css-dock-y, 100px);
        width: 600px;
        height: 450px;
        background: var(--editor-bg);
        border: 1px solid var(--editor-border);
        border-radius: 8px;
        box-shadow: 0 10px 40px rgba(0,0,0,0.5);
        z-index: 4000;
        display: flex;
        flex-direction: column;
        resize: both;
        overflow: hidden;
        min-width: 400px;
        min-height: 250px;
    }
    .css-tab-bar {
        display: flex;
        overflow-x: auto;
        background: var(--editor-bg-deep);
        border-bottom: 1px solid var(--editor-border-soft);
    }
    .css-tab {
        padding: 8px 16px;
        font-family: var(--font-mono);
        font-size: 0.75rem;
        background: transparent;
        border: none;
        border-right: 1px solid var(--editor-border-soft);
        color: var(--editor-text-muted);
        cursor: pointer;
        transition: background 0.1s;
    }
    .css-tab:hover { background: rgba(255,255,255,0.05); }
    .css-tab.active {
        background: var(--editor-bg);
        color: var(--editor-accent);
        border-bottom: 2px solid var(--editor-accent);
    }
    .css-editor-textarea {
        flex-grow: 1;
        width: 100%;
        background: var(--bg-panel);
        color: var(--fg-base);
        font-family: var(--font-mono);
        font-size: 0.85rem;
        line-height: 1.5;
        border: none;
        padding: 16px;
        resize: none;
    }
    .editor-left-panel:not(.is-floating) { width: var(--left-pane-width, 320px) !important; position: relative; }
    .editor-right-panel:not(.is-floating) { width: var(--right-pane-width, 320px) !important; position: relative; }
    .pane-resizer { position: absolute; top: 0; bottom: 0; width: 6px; z-index: 999; cursor: ew-resize; background: transparent; transition: background 0.1s; }
    .pane-resizer:hover, .pane-resizer:active { background: var(--editor-accent, rgba(255,255,255,0.2)); }
    .pane-resizer-right { right: 0; }
    .pane-resizer-left { left: 0; }
    .floating-editor-window-bar {
        display: flex !important;
        align-items: center !important;
        justify-content: space-between !important;
        padding: 0 12px !important;
        background: var(--bg-elevated) !important;
        border-bottom: 1px solid var(--editor-border-soft) !important;
        cursor: move !important;
        box-sizing: border-box !important;
        width: 100% !important;
        flex: 0 0 44px !important; /* CRITICAL: Prevents vertical collapse */
        min-height: 44px !important;
        overflow: hidden !important;
    }
    .floating-editor-grip-group {
        display: flex !important;
        align-items: center !important;
        gap: 8px !important;
        flex-shrink: 0 !important;
    }
    .floating-editor-title {
        margin: 0 !important;
        font-size: 0.8rem !important;
        font-weight: 600 !important;
        text-transform: uppercase !important;
        letter-spacing: 0.05em !important;
        white-space: nowrap !important;
    }
    .floating-editor-window-actions {
        display: flex !important;
        align-items: center !important;
        gap: 6px !important;
    }
"#;

fn get_css_deps<'a>(registry: &'a [ComponentManifest], target_id: &str) -> Vec<&'a str> {
    registry
        .iter()
        .find(|c| c.id == target_id)
        .map(|c| c.css_deps.to_vec())
        .unwrap_or_default()
}

#[component]
pub fn CssEditorPanel() -> Element {
    let mut layout = use_context::<AppLayoutState>();
    let theme = use_context::<ThemeAppState>();
    let mut vfs = use_context::<VfsDictionary>().0;
    let mut last_workbench_key = use_signal(|| None::<String>);
    let mut active_tab = use_signal(|| "preset_css.css".to_string());
    
    let pos = (layout.css_editor_pos)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let view = (theme.center_view)();
    let config = (theme.current_config)();
    let pack = &config.template_pack;
    
    let mut available_files = vec!["preset_css.css"];

    if view == CenterView::ModuleWorkbench {
        if let Some(key) = (layout.active_workbench_module)() {
            let deps = match key {
                "header_variant" => {
                    let mut d = get_css_deps(HEADER_REGISTRY, &pack.header_variant);
                    if d.is_empty() {
                        d.push("header.css");
                    }
                    d
                }
                "main_variant" => {
                    let mut d = get_css_deps(LAYOUT_REGISTRY, &pack.main_variant);
                    if d.is_empty() {
                        d.push("layout.css");
                    }
                    d
                }
                "content_variant" => {
                    let mut d = get_css_deps(CONTENT_REGISTRY, &pack.content_variant);
                    if d.is_empty() {
                        d.push("content.css");
                    }
                    d
                }
                "left_sidebar_variant" | "right_sidebar_variant" => {
                    let mut d = if key == "left_sidebar_variant" {
                        get_css_deps(SIDEBAR_LEFT_REGISTRY, &pack.left_sidebar_variant)
                    } else {
                        get_css_deps(SIDEBAR_RIGHT_REGISTRY, &pack.right_sidebar_variant)
                    };
                    if d.is_empty() {
                        d.push("sidebar.css");
                    }
                    d
                }
                "footer_variant" => {
                    let mut d = get_css_deps(FOOTER_REGISTRY, &pack.footer_variant);
                    if d.is_empty() {
                        d.push("footer.css");
                    }
                    d
                }
                _ => vec![],
            };
            available_files.extend(deps);

            if let Some(current_key) = (layout.active_workbench_module)() {
                if last_workbench_key() != Some(current_key.to_string()) {
                    last_workbench_key.set(Some(current_key.to_string()));
                    if available_files.len() > 1 {
                        active_tab.set(available_files[1].to_string());
                    }
                }
            }
        }
    } else {
        last_workbench_key.set(None);
    }
    
    // BLOAT KILLER: Pure derived state. No use_effect watcher fighting the render cycle.
    let current_file = if available_files.contains(&active_tab().as_str()) {
        active_tab()
    } else {
        available_files[0].to_string()
    };

    let mut preset_css_signal = theme.signals.preset_css;
    
    let css_val = if current_file == "preset_css.css" {
        preset_css_signal.read().clone()
    } else {
        vfs.read().get(&current_file).cloned().unwrap_or_else(|| fetch_default_css(&current_file).to_string())
    };

    let mut last_file = use_signal(|| current_file.clone());
    let mut last_external_val = use_signal(|| css_val.clone());
    let mut current_editor_content = use_signal(|| css_val.clone());
    
    if last_file() != current_file {
        last_file.set(current_file.clone());
        current_editor_content.set(css_val.clone());
        last_external_val.set(css_val.clone());
    } else if last_external_val() != css_val {
        last_external_val.set(css_val.clone());
        current_editor_content.set(css_val.clone());
    }

    let is_floating = pos == DockPosition::Floating;

    let sync_vfs = vfs;

    let header_actions = rsx! {
        div { class: "floating-editor-window-actions",
            button {
                class: "editor-mini-button",
                style: "padding: 2px 6px; font-size: 0.75rem; border-radius: 4px; font-weight: 600;",
                onclick: move |_| {
                    spawn(async move {
                        let current_vfs = sync_vfs.read().clone();
                        for (filename, content) in current_vfs.iter() {
                            if filename == "preset_css.css" {
                                continue;
                            }
                            match mor_blogger_core::utils::fs_bridge::save_custom_css(filename, content) {
                                Ok(path) => log::info!("Successfully synced {} to OS at {}", filename, path.display()),
                                Err(e) => log::error!("Failed to sync {} to OS: {}", filename, e),
                            }
                        }
                    });
                },
                "Sync OS"
            }
            if pos != DockPosition::Left {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Left",
                    onclick: move |_| {
                        layout.request_exclusive_dock("css", DockPosition::Left);
                    },
                    IconDockLeft {}
                }
            }
            if pos != DockPosition::Right {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Right",
                    onclick: move |_| {
                        layout.request_exclusive_dock("css", DockPosition::Right);
                    },
                    IconDockRight {}
                }
            }
            if pos != DockPosition::Floating {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Float Window",
                    onclick: move |_| {
                        layout.css_editor_pos.set(DockPosition::Floating);
                    },
                    IconFloat {}
                }
            }
            button {
                class: "editor-mini-button",
                style: "display: flex; align-items: center; padding: 4px;",
                title: "Close",
                onclick: move |_| layout.css_editor_pos.set(DockPosition::Hidden),
                IconClose {}
            }
        }
    };

    let editor_body = rsx! {
        script { dangerous_inner_html: "{EDITOR_DRAG_JS}" }
        script { dangerous_inner_html: "{PANE_RESIZE_JS}" }
        style { dangerous_inner_html: "{EDITOR_CSS}" }

        if is_floating {
            div {
                class: "floating-editor-window-bar",
                div {
                    class: "floating-editor-grip-group",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", "⠿" }
                    span {
                        class: "floating-editor-title",
                        "CSS Editor"
                    }
                }
                {header_actions}
            }
        } else {
            div { class: "editor-panel-header", style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-bottom: 1px solid var(--editor-border-soft); box-sizing: border-box;",
                h2 { class: "editor-panel-title", style: "margin: 0; font-size: 0.9rem; font-weight: 600;", "CSS Editor" }
                {header_actions}
            }
        }



        div { class: "css-tab-bar",
            for file in available_files {
                button {
                    class: if current_file == file { "css-tab active" } else { "css-tab" },
                    onclick: {
                        let f = file.to_string();
                        move |_| active_tab.set(f.clone())
                    },
                    "{file}"
                }
            }
        }

        div {
            style: "display: flex; flex-direction: column; flex: 1 1 auto; width: 100%; min-height: 0;",
            CodeEditor {
                value: current_editor_content,
                mode: "css".to_string(),
                on_change: move |new_val: String| {
                    let file = current_file.clone();
                    if file == "preset_css.css" {
                        preset_css_signal.set(new_val.clone());
                    } else {
                        vfs.write().insert(file, new_val.clone());
                    }
                    last_external_val.set(new_val);
                }
            }
        }
    };

    match pos {
        DockPosition::Left => {
            rsx! {
                aside {
                    class: "editor-left-panel css-editor-mode",
                    {editor_body}
                    div { class: "pane-resizer pane-resizer-right" }
                }
            }
        }
        DockPosition::Right => {
            rsx! {
                aside {
                    class: "editor-right-panel css-editor-mode",
                    {editor_body}
                    div { class: "pane-resizer pane-resizer-left" }
                }
            }
        }
        DockPosition::Floating => {
            rsx! {
                div { class: "floating-css-editor",
                    {editor_body}
                }
            }
        }
        DockPosition::Hidden => {
            rsx! {}
        }
    }
}

#[component]
fn IconClose() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: "M4.5 4.5l7 7M11.5 4.5l-7 7" }
        }
    }
}

#[component]
fn IconFloat() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "1.5", width: "10", height: "8", rx: "1.5" }
            rect { x: "4.5", y: "6.5", width: "10", height: "8", rx: "1.5" }
        }
    }
}

#[component]
fn IconDockLeft() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M5.5 2.5v11" }
        }
    }
}

#[component]
fn IconDockRight() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M10.5 2.5v11" }
        }
    }
}

#[component]
fn IconSplit() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M8 2.5v11" }
        }
    }
}

#[component]
fn IconWide() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
        }
    }
}