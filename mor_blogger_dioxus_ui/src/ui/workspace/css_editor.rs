use dioxus::prelude::*;
use std::collections::HashMap;
use crate::app::layout_state::{AppLayoutState, CenterView, DockPosition};
use crate::app::state::ThemeAppState;
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

const EDITOR_CSS: &str = r#"
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
    
    if (layout.css_editor_pos)() == DockPosition::Hidden {
        return rsx! {};
    }

    let view = (theme.center_view)();
    let config = (theme.current_config)();
    let pack = &config.template_pack;
    
    let mut available_files = vec!["preset_css.css"];

    if view == CenterView::ModuleWorkbench {
        if let Some(key) = (layout.active_workbench_module)() {
            let deps = match key {
                "header_variant" => get_css_deps(HEADER_REGISTRY, &pack.header_variant),
                "main_variant" => get_css_deps(LAYOUT_REGISTRY, &pack.main_variant),
                "content_variant" => get_css_deps(CONTENT_REGISTRY, &pack.content_variant),
                "left_sidebar_variant" => get_css_deps(SIDEBAR_LEFT_REGISTRY, &pack.left_sidebar_variant),
                "right_sidebar_variant" => get_css_deps(SIDEBAR_RIGHT_REGISTRY, &pack.right_sidebar_variant),
                "footer_variant" => get_css_deps(FOOTER_REGISTRY, &pack.footer_variant),
                _ => vec![],
            };
            available_files.extend(deps);
        }
    }

    let mut active_tab = use_signal(|| "preset_css.css".to_string());
    
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

    rsx! {
        script { dangerous_inner_html: "{EDITOR_DRAG_JS}" }
        style { dangerous_inner_html: "{EDITOR_CSS}" }

        div { class: "floating-css-editor",
            div { class: "floating-editor-window-bar", style: "cursor: move;",
                span { class: "floating-editor-grip", style: "display: flex; align-items: center;", "⠿" }
                span { class: "floating-editor-title", "CSS Architect" }
                div { class: "floating-editor-window-actions",
                    button {
                        class: "editor-mini-button",
                        onclick: move |_| layout.css_editor_pos.set(DockPosition::Hidden),
                        "×"
                    }
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

            textarea {
                key: "{current_file}",
                class: "css-editor-textarea",
                value: "{css_val}",
                oninput: move |evt| { 
                    let file = current_file.clone();
                    if file == "preset_css.css" {
                        preset_css_signal.set(evt.value()); 
                    } else {
                        vfs.write().insert(file, evt.value());
                    }
                }
            }
        }
    }
}