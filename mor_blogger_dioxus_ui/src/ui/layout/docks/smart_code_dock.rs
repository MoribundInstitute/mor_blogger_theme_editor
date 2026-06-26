use crate::app::vfs::VfsDictionary;
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
    let mut is_takeover = use_signal(|| false);
    // false = editable live TOML, true = compiled (read-only) XML.
    let mut show_xml = use_signal(|| false);
    let vfs = use_context::<VfsDictionary>().0;

    // Compiled export XML, recomputed from the live config (mirrors the Export tab).
    let export_xml = use_memo(move || {
        match crate::app::services::workspace_service::build_fresh_export_xml(
            &config_toml(),
            &*vfs.read(),
        ) {
            Ok(xml) => xml,
            Err(err) => format!("Render failed: {}", err),
        }
    });

    // Save the live theme config buffer to disk (opens a save-as dialog).
    let save_config = move |_: Event<MouseData>| {
        crate::utils::io::save_toml(&config_toml());
    };

    let mut jump_to = move |target: &str| {
        // Jump links only exist in the TOML buffer — switch back to it first.
        show_xml.set(false);
        let target_str = target.to_string();
        active_target.set(Some(target_str.clone()));

        spawn(async move {
            let eval = dioxus::document::eval(
                r#"
                let target = await dioxus.recv();
                if (window.morCM) window.morCM.reveal("toml-editor-textarea", target);
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

    // The active editor (TOML editable + live reload, or compiled XML read-only).
    let editor = rsx! {
        if show_xml() {
            CodeEditor {
                value: export_xml(),
                mode: "xml".to_string(),
                minimap_key: Some("code_editor_xml".to_string()),
                read_only: true,
                on_change: |_| {},
            }
        } else {
            CodeEditor {
                id: Some("toml-editor-textarea".to_string()),
                value: (config_toml)(),
                mode: "toml".to_string(),
                on_change: move |new_val| { on_load_theme.call(new_val); }
            }
        }
    };

    // Header: TOML/XML toggle, Takeover, and (TOML only) Save.
    let header_controls = rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px;",
            button {
                class: if show_xml() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-active" },
                title: "Edit the live theme config",
                onclick: move |_| show_xml.set(false),
                "TOML"
            }
            button {
                class: if show_xml() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                title: "View the compiled Blogger XML (read-only)",
                onclick: move |_| show_xml.set(true),
                "XML"
            }
            div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 2px;" }
            button {
                class: "editor-mini-button",
                title: "Expand the editor to a full-viewport focused stage",
                onclick: move |_| is_takeover.set(true),
                "Takeover"
            }
            if !show_xml() {
                button {
                    class: "editor-mini-button",
                    title: "Save the live theme config to disk",
                    onclick: save_config,
                    "Save"
                }
            }
        }
    };

    let filename = if show_xml() { "exported_theme.xml" } else { "theme_config.toml" };
    let badge = if show_xml() { "Compiled · Read-only" } else { "Live Reload Active" };

    rsx! {
        if is_takeover() {
            // ── Editor Takeover ─ Full-viewport focused editor ────────────
            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",
                div {
                    style: "flex-shrink: 0; display: flex; align-items: center; gap: 8px; padding: 6px 12px; background: var(--bg-elevated); border-bottom: 1px solid var(--editor-border);",
                    span {
                        style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);",
                        "{filename}"
                    }
                    div { style: "flex: 1;" }
                    {header_controls}
                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 2px;" }
                    button {
                        class: "editor-mini-button",
                        onclick: move |_| is_takeover.set(false),
                        "Editor ×"
                    }
                }
                div {
                    style: "flex: 1; min-height: 0; display: flex; flex-direction: column;",
                    {editor.clone()}
                }
            }
        } else {
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
                            span { style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);", "{filename}" }
                            span {
                                style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                                "{badge}"
                            }
                        }
                        {header_controls}
                    }
                    div {
                        style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",
                        {editor}
                    }
                }
            }
        }
    }
}
