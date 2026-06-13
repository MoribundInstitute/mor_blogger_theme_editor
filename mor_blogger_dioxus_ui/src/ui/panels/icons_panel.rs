use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

#[component]
pub fn SvgIconsPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let mut new_key = use_signal(String::new);

    rsx! {
        div { class: "editor-card", style: "padding: 16px; display: flex; flex-direction: column; gap: 16px;",
            
            // --- CORE THEME ICONS ---
            div { class: "editor-note",
                p { class: "editor-note-title", "Core Structural Icons" }
                p { class: "editor-note-body", "Shift+Click any of these inside the preview canvas to quickly swap them using the visual picker." }
            }

            div { class: "editor-field-group",
                div { style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 8px;",
                    div { class: "icon-preview-box", style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border: 1px solid var(--editor-border-soft); border-radius: 4px;", title: "Menu", 
                        span { style: "display: block; width: 20px; height: 20px; background-color: var(--fg-base); -webkit-mask-image: {current_config.icons.menu}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;" } 
                    }
                    div { class: "icon-preview-box", style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border: 1px solid var(--editor-border-soft); border-radius: 4px;", title: "Search", 
                        span { style: "display: block; width: 20px; height: 20px; background-color: var(--fg-base); -webkit-mask-image: {current_config.icons.search}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;" } 
                    }
                    div { class: "icon-preview-box", style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border: 1px solid var(--editor-border-soft); border-radius: 4px;", title: "Close", 
                        span { style: "display: block; width: 20px; height: 20px; background-color: var(--fg-base); -webkit-mask-image: {current_config.icons.panel_close}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;" } 
                    }
                    div { class: "icon-preview-box", style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border: 1px solid var(--editor-border-soft); border-radius: 4px;", title: "Left Sidebar", 
                        span { style: "display: block; width: 20px; height: 20px; background-color: var(--fg-base); -webkit-mask-image: {current_config.icons.sidebar_left}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;" } 
                    }
                    div { class: "icon-preview-box", style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border: 1px solid var(--editor-border-soft); border-radius: 4px;", title: "Right Sidebar", 
                        span { style: "display: block; width: 20px; height: 20px; background-color: var(--fg-base); -webkit-mask-image: {current_config.icons.sidebar_right}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;" } 
                    }
                }
            }

            div { style: "height: 1px; background: var(--editor-border-soft); margin: 8px 0;" }

            // --- QUARANTINED FEATURE ---
            div { class: "editor-note", style: "border-color: var(--editor-warning); background: rgba(210, 153, 34, 0.05);",
                p { class: "editor-note-title", style: "color: var(--editor-warning);", "🚧 Feature Marked For Repair 🚧" }
                p { class: "editor-note-body", style: "color: var(--editor-warning);", "Custom SVG Dictionary is temporarily disabled. Human Dev To-Do: Resolve strict OS namespace stripping and WYSIWYG coloring limitations before re-enabling." }
            }

            if !current_config.icons.custom_icons.is_empty() {
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    for (key, mask_uri) in current_config.icons.custom_icons.iter() {
                        div { style: "display: flex; align-items: center; justify-content: space-between; background: var(--bg-elevated); padding: 8px 12px; border: 1px solid var(--editor-border-soft); border-radius: 4px; opacity: 0.5;",
                            div { style: "display: flex; align-items: center; gap: 12px;",
                                span { style: "display: block; width: 16px; height: 16px; background-image: {mask_uri}; background-size: contain; background-repeat: no-repeat; background-position: center;" }
                                span { style: "font-family: var(--font-mono); font-size: 0.85em; color: var(--fg-base);", "{key} (Legacy)" }
                            }
                            button {
                                class: "editor-mini-button",
                                title: "Remove Icon",
                                onclick: {
                                    let mut cfg = current_config.clone();
                                    let k = key.clone();
                                    let apply = on_apply_theme.clone();
                                    move |_| {
                                        cfg.icons.custom_icons.remove(&k);
                                        apply.call(cfg.clone());
                                    }
                                },
                                "×"
                            }
                        }
                    }
                }
            }

            // Disabled form
            div { style: "display: flex; flex-direction: column; gap: 8px; background: rgba(0,0,0,0.1); padding: 12px; border-radius: 6px; border: 1px dashed var(--editor-border-soft); opacity: 0.5; pointer-events: none;",
                input {
                    class: "editor-input",
                    style: "width: 100%; box-sizing: border-box;",
                    placeholder: "Icon label (e.g. 'plank')",
                    value: "{new_key}",
                    disabled: true,
                }
                button {
                    class: "editor-button",
                    style: "width: 100%; justify-content: center;",
                    disabled: true,
                    "Browse OS for .svg... (Disabled)"
                }
            }
        }
    }
}