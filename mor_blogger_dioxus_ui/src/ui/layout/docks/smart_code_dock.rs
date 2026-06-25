use crate::ui::components::code_editor::CodeEditor;
use dioxus::prelude::*;

const TEMPLATE_LAYOUTS: &[(&str, &str)] = &[
    ("Header Variant", "header_variant"),
    ("Main Canvas Variant", "main_variant"),
    ("Content Feed", "content_variant"),
    ("Left Sidebar", "left_sidebar_variant"),
    ("Right Sidebar", "right_sidebar_variant"),
    ("Footer Grid", "footer_variant"),
];

const CORE_IDENTITY: &[(&str, &str)] = &[
    ("Site Information", "[site]"),
    ("Color Palette", "[colors]"),
    ("Typography", "[typography]"),
];

#[component]
pub fn SmartCodeDock(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
    #[props(default)] active_xray_target: Option<Signal<Option<String>>>,
) -> Element {
    let mut active_target = use_signal(|| None::<String>);

    let mut jump_to = move |target: &str| {
        let target_str = target.to_string();
        active_target.set(Some(target_str.clone()));

        spawn(async move {
            let eval = dioxus::document::eval(
                r#"
                let target = await dioxus.recv();
                let el = document.getElementById("toml-editor-textarea");
                if (el) {
                    let idx = el.value.indexOf(target);
                    if (idx !== -1) {
                        el.focus();
                        el.setSelectionRange(idx, idx + target.length);

                        const linesBefore = el.value.substring(0, idx).split('\n').length;
                        const lineHeight = parseFloat(window.getComputedStyle(el).lineHeight) || 19;
                        el.scrollTop = Math.max(0, (linesBefore * lineHeight) - (el.clientHeight / 2));
                    }
                }
            "#,
            );
            let _ = eval.send(target_str);
        });
    };

    if let Some(mut target_sig) = active_xray_target {
        use_effect(move || {
            if let Some(target_str) = target_sig() {
                jump_to(&target_str);
                target_sig.set(None);
            }
        });
    }

    rsx! {
        div {
            class: "export-viewport",
            style: "display: flex; flex-direction: row; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel);",

            div {
                style: "width: 220px; border-right: 1px solid var(--editor-border); display: flex; flex-direction: column;",

                div {
                    style: "padding: 12px; border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                    span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Template Layouts" }
                }
                div {
                    style: "padding: 12px; display: flex; flex-direction: column; gap: 8px; overflow-y: auto;",
                    for (label, search_key) in TEMPLATE_LAYOUTS {
                        button {
                            class: if active_target() == Some(search_key.to_string()) { "editor-button editor-button-active" } else { "editor-button" },
                            style: "text-align: left; font-size: 0.85rem;",
                            onclick: move |_| jump_to(search_key),
                            "{label}"
                        }
                    }
                }

                div {
                    style: "padding: 12px; border-top: 1px solid var(--editor-border-soft); border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                    span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Core Identity" }
                }
                div {
                    style: "padding: 12px; display: flex; flex-direction: column; gap: 8px; overflow-y: auto;",
                    for (label, search_key) in CORE_IDENTITY {
                        button {
                            class: if active_target() == Some(search_key.to_string()) { "editor-button editor-button-active" } else { "editor-button" },
                            style: "text-align: left; font-size: 0.85rem;",
                            onclick: move |_| jump_to(search_key),
                            "{label}"
                        }
                    }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: var(--bg-base); border-radius: 6px; overflow: hidden; border: 1px solid var(--border-color);",
                div {
                    class: "editor-pane-header",
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: rgba(0,0,0,0.2); border-bottom: 1px solid var(--border-color); flex-shrink: 0;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);", "theme_config.toml" }
                        span {
                            style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                            "Live Reload Active"
                        }
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",
                    CodeEditor {
                        id: Some("toml-editor-textarea".to_string()),
                        value: (config_toml)(),
                        mode: "toml".to_string(),
                        on_change: move |new_val| {
                            on_load_theme.call(new_val);
                        }
                    }
                }
            }
        }
    }
}