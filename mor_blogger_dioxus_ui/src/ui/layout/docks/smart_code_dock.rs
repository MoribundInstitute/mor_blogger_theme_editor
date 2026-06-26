use crate::app::state::LayoutState;
use crate::app::vfs::VfsDictionary;
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::layout::docks::code_nav_dock::{reveal_code_target, CODE_EDITOR_ID};
use dioxus::prelude::*;

#[component]
pub fn SmartCodeDock(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
    #[props(default)] active_xray_target: Option<Signal<Option<String>>>,
) -> Element {
    let layout = use_context::<LayoutState>();
    let mut is_takeover = use_signal(|| false);
    // Shared with the Code Nav dock: false = editable live TOML, true = compiled XML.
    let mut show_xml = layout.code_show_xml;
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

    // X-Ray targets config sections, so it always reveals in the editable TOML.
    if let Some(mut target_sig) = active_xray_target {
        use_effect(move || {
            if let Some(target_str) = target_sig() {
                show_xml.set(false);
                reveal_code_target(target_str);
                target_sig.set(None);
            }
        });
    }

    // The active editor (TOML editable + live reload, or compiled XML read-only).
    let editor = rsx! {
        if show_xml() {
            CodeEditor {
                id: Some(CODE_EDITOR_ID.to_string()),
                value: export_xml(),
                mode: "xml".to_string(),
                minimap_key: Some("code_editor_xml".to_string()),
                read_only: true,
                on_change: |_| {},
            }
        } else {
            CodeEditor {
                id: Some(CODE_EDITOR_ID.to_string()),
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
            // Editor fills the view; navigation lives in the Code Nav dock.
            div {
                class: "export-viewport",
                style: "display: flex; flex-direction: column; min-width: 0; height: 100%; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-base);",
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
