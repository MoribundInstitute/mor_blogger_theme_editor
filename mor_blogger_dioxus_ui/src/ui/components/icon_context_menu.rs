use crate::app::state::{ContextMenuPayload, LayoutState};
use dioxus::prelude::*;

#[component]
pub fn IconContextMenu(payload: ContextMenuPayload) -> Element {
    let mut layout = use_context::<LayoutState>();
    let target_id = payload.target_id.clone();

    let friendly_name = target_id
        .strip_prefix("icons.")
        .unwrap_or(&target_id)
        .replace('_', " ");

    let mut active_context_menu = layout.active_context_menu;

    rsx! {
        // Overlay to close when clicking outside
        div {
            style: "position: fixed; inset: 0; z-index: 9998; background: transparent;",
            onclick: move |_| { active_context_menu.set(None); },
            oncontextmenu: move |e| {
                e.prevent_default();
                active_context_menu.set(None);
            }
        }

        // Custom context menu container
        div {
            style: "position: fixed; left: {payload.x}px; top: {payload.y}px; z-index: 9999; background: #16140f; border: 1px solid #3c382f; border-radius: 8px; box-shadow: 0 10px 30px rgba(0,0,0,0.6); padding: 12px; width: 220px; display: flex; flex-direction: column; gap: 8px; user-select: none;",
            onclick: move |e| { e.stop_propagation(); },
            oncontextmenu: move |e| { e.prevent_default(); e.stop_propagation(); },

            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                span {
                    style: "font-size: 10px; font-weight: bold; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;",
                    match payload.kind.as_str() {
                        "svg" => "Icon Context Menu",
                        "ui_typography" => "UI Context Menu",
                        "preview_typography" => "Preview Context Menu",
                        _ => "Context Menu"
                    }
                }
                span {
                    style: "font-size: 12px; font-weight: 600; color: var(--accent); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{friendly_name}"
                }
            }

            hr { style: "border: 0; border-top: 1px solid var(--border-color); margin: 2px 0 4px 0;" }

            match payload.kind.as_str() {
                "svg" => rsx! {
                    button {
                        class: "editor-mini-button",
                        style: "text-align: left; padding: 8px 12px; font-size: 12px; cursor: pointer; width: 100%; display: flex; align-items: center; gap: 8px; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5; transition: background 0.15s ease;",
                        onclick: move |_| {
                            layout.active_icon_picker.set(Some(target_id.clone()));
                            active_context_menu.set(None);
                        },
                        span { style: "font-size: 12px;", "✨" }
                        span { "Swap Icon..." }
                    }
                },
                "ui_typography" => rsx! {
                    button {
                        class: "editor-mini-button",
                        style: "text-align: left; padding: 8px 12px; font-size: 12px; cursor: pointer; width: 100%; display: flex; align-items: center; gap: 8px; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5; transition: background 0.15s ease;",
                        onclick: move |_| {
                            println!("TODO: Route to Editor UI Settings");
                            active_context_menu.set(None);
                        },
                        span { style: "font-size: 12px;", "⚙️" }
                        span { "Editor UI Settings" }
                    }
                },
                "preview_typography" => rsx! {
                    button {
                        class: "editor-mini-button",
                        style: "text-align: left; padding: 8px 12px; font-size: 12px; cursor: pointer; width: 100%; display: flex; align-items: center; gap: 8px; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5; transition: background 0.15s ease;",
                        onclick: move |_| {
                            println!("TODO: Open Typography Panel in Theme Palette");
                            active_context_menu.set(None);
                        },
                        span { style: "font-size: 12px;", "✍️" }
                        span { "Edit Theme Typography" }
                    }
                },
                _ => rsx! {
                    div {
                        style: "font-size: 11px; color: var(--fg-muted); padding: 4px;",
                        "Unknown Context"
                    }
                }
            }
        }
    }
}
