use crate::app::shell_layout::render_named_svg;
use crate::app::state::LayoutState;
use dioxus::prelude::*;

const INPUT_STYLE: &str = "flex:1; padding:6px 8px; background:var(--editor-bg); color:var(--editor-text); border:1px solid var(--editor-border); border-radius:4px; font-size:14px;";

/// Modal for customizing a dock's activity-bar icon: emoji, a built-in SVG, or
/// pasted custom SVG. Driven by `layout.active_activity_icon_picker` (the dock id).
#[component]
pub fn ActivityIconPicker() -> Element {
    let layout = use_context::<LayoutState>();
    let mut picker = layout.active_activity_icon_picker;
    let Some(dock_id) = picker() else {
        return rsx! {};
    };

    let mut emoji_input = use_signal(String::new);
    let mut raw_input = use_signal(String::new);
    let mut err = use_signal(String::new);

    let svgs = ["palette", "site_data", "xml", "plugin", "preset", "bug", "code"];
    let close = move |_| picker.set(None);

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 10000; background: rgba(0,0,0,0.5); display:flex; align-items:center; justify-content:center;",
            onclick: close,
            div {
                style: "width: 380px; max-width: 90vw; background:var(--editor-panel); border:1px solid var(--editor-border); border-radius:10px; box-shadow:0 20px 60px rgba(0,0,0,0.6); padding:18px; display:flex; flex-direction:column; gap:14px;",
                onclick: move |e| e.stop_propagation(),

                div {
                    style: "display:flex; justify-content:space-between; align-items:center;",
                    h3 { style: "margin:0; font-size:1rem; color:var(--fg-base);", "Change icon — {dock_id}" }
                    button { class: "editor-mini-button", onclick: close, "×" }
                }

                if !err().is_empty() {
                    div { style: "color:var(--editor-warning,#d29922); font-size:0.8rem;", "{err}" }
                }

                // Emoji
                div {
                    style: "display:flex; flex-direction:column; gap:6px;",
                    label { style: "font-size:0.75rem; text-transform:uppercase; letter-spacing:0.05em; color:var(--fg-muted);", "Emoji" }
                    div {
                        style: "display:flex; gap:8px;",
                        input {
                            style: "{INPUT_STYLE}",
                            placeholder: "type or paste an emoji",
                            value: "{emoji_input}",
                            oninput: move |e| emoji_input.set(e.value()),
                        }
                        button {
                            class: "editor-button",
                            onclick: {
                                let id = dock_id.clone();
                                move |_| {
                                    let v = emoji_input().trim().to_string();
                                    if v.is_empty() { err.set("Enter an emoji first.".into()); return; }
                                    layout.set_activity_icon(&id, Some(format!("emoji:{v}")));
                                    picker.set(None);
                                }
                            },
                            "Use emoji"
                        }
                    }
                }

                // Built-in SVG gallery
                div {
                    style: "display:flex; flex-direction:column; gap:6px;",
                    label { style: "font-size:0.75rem; text-transform:uppercase; letter-spacing:0.05em; color:var(--fg-muted);", "Built-in icons" }
                    div {
                        style: "display:flex; gap:8px; flex-wrap:wrap;",
                        for name in svgs {
                            button {
                                class: "editor-mini-button",
                                style: "width:34px; height:34px; display:flex; align-items:center; justify-content:center;",
                                title: "{name}",
                                onclick: {
                                    let id = dock_id.clone();
                                    move |_| { layout.set_activity_icon(&id, Some(format!("svg:{name}"))); picker.set(None); }
                                },
                                {render_named_svg(name)}
                            }
                        }
                    }
                }

                // Custom SVG
                div {
                    style: "display:flex; flex-direction:column; gap:6px;",
                    label { style: "font-size:0.75rem; text-transform:uppercase; letter-spacing:0.05em; color:var(--fg-muted);", "Custom SVG" }
                    textarea {
                        style: "min-height:70px; padding:6px 8px; background:var(--editor-bg); color:var(--editor-text); border:1px solid var(--editor-border); border-radius:4px; font-family:monospace; font-size:11px; resize:vertical;",
                        placeholder: "<svg ...>...</svg>",
                        value: "{raw_input}",
                        oninput: move |e| raw_input.set(e.value()),
                    }
                    button {
                        class: "editor-button",
                        onclick: {
                            let id = dock_id.clone();
                            move |_| {
                                match sanitize_svg(&raw_input()) {
                                    Some(clean) => { layout.set_activity_icon(&id, Some(format!("raw:{clean}"))); picker.set(None); }
                                    None => err.set("Not a valid <svg> (or it contained scripts).".into()),
                                }
                            }
                        },
                        "Use custom SVG"
                    }
                }

                hr { style: "border:0; border-top:1px solid var(--border-color); margin:2px 0;" }
                button {
                    class: "editor-mini-button",
                    onclick: {
                        let id = dock_id.clone();
                        move |_| { layout.set_activity_icon(&id, None); picker.set(None); }
                    },
                    "Reset to default"
                }
            }
        }
    }
}

/// Minimal SVG sanitize: require an `<svg>` root and reject obvious script vectors.
/// ponytail: crude substring filter, not a real sanitizer — fine for a single-user
/// desktop tool; swap for a proper sanitizer if SVGs ever come from untrusted sources.
fn sanitize_svg(input: &str) -> Option<String> {
    let s = input.trim();
    let lower = s.to_ascii_lowercase();
    if !lower.contains("<svg") || !lower.contains("</svg>") {
        return None;
    }
    if lower.contains("<script") || lower.contains("javascript:") || lower.contains("onload=") {
        return None;
    }
    Some(s.to_string())
}
