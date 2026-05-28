use dioxus::prelude::*;

use crate::config::{BackgroundConfig, BackgroundMode};
use crate::ui::components::inputs::{EditorCard, EditorSelect, PanelNote};

#[component]
pub fn BackgroundPanel(mut background: Signal<BackgroundConfig>) -> Element {
    let mode_str = match &background.read().mode {
        BackgroundMode::Solid { .. } => "solid",
        BackgroundMode::Gradient { .. } => "gradient",
        BackgroundMode::Tile { .. } => "tile",
    };

    rsx! {
        EditorCard {
            title: "Page Background".to_string(),

            EditorSelect {
                label: "Background Mode".to_string(),
                value: mode_str.to_string(),
                options: vec![
                    ("solid".to_string(), "Solid Color".to_string()),
                    ("gradient".to_string(), "Linear Gradient".to_string()),
                    ("tile".to_string(), "Image Tile".to_string()),
                ],
                onchange: move |e: Event<FormData>| {
                    let mut bg = background.write();
                    match e.value().as_str() {
                        "solid" => bg.mode = BackgroundMode::Solid { color: "#0a0a0a".to_string() },
                        "gradient" => bg.mode = BackgroundMode::Gradient {
                            from: "#000000".to_string(),
                            to: "#333333".to_string(),
                            angle_deg: 135
                        },
                        "tile" => bg.mode = BackgroundMode::Tile { url: String::new() },
                        _ => {}
                    }
                }
            }

            match background.read().mode.clone() {
                BackgroundMode::Solid { color } => rsx! {
                    div {
                        class: "editor-field-group",
                        label { class: "editor-field-label", "Color" }
                        input {
                            r#type: "color",
                            value: "{color}",
                            class: "editor-field editor-color-field",
                            oninput: move |e| {
                                if matches!(background.read().mode, BackgroundMode::Solid { .. }) {
                                    background.write().mode = BackgroundMode::Solid { color: e.value() };
                                }
                            }
                        }
                    }
                },
                BackgroundMode::Gradient { from, to, angle_deg } => rsx! {
                    div {
                        class: "editor-row-stretch",

                        div {
                            class: "editor-flex-1 editor-stack",
                            label { class: "editor-mini-label", "From" }
                            input {
                                r#type: "color",
                                value: "{from}",
                                class: "editor-field editor-color-field",
                                oninput: move |e| {
                                    let state = match &background.read().mode {
                                        BackgroundMode::Gradient { to, angle_deg, .. } => Some((to.clone(), *angle_deg)),
                                        _ => None,
                                    };
                                    if let Some((to, angle_deg)) = state {
                                        background.write().mode = BackgroundMode::Gradient { from: e.value(), to, angle_deg };
                                    }
                                }
                            }
                        }

                        div {
                            class: "editor-flex-1 editor-stack",
                            label { class: "editor-mini-label", "To" }
                            input {
                                r#type: "color",
                                value: "{to}",
                                class: "editor-field editor-color-field",
                                oninput: move |e| {
                                    let state = match &background.read().mode {
                                        BackgroundMode::Gradient { from, angle_deg, .. } => Some((from.clone(), *angle_deg)),
                                        _ => None,
                                    };
                                    if let Some((from, angle_deg)) = state {
                                        background.write().mode = BackgroundMode::Gradient { from, to: e.value(), angle_deg };
                                    }
                                }
                            }
                        }

                        div {
                            class: "editor-w-70 editor-stack",
                            label { class: "editor-mini-label", "Angle°" }
                            input {
                                r#type: "number",
                                min: "0",
                                max: "360",
                                value: "{angle_deg}",
                                class: "editor-field",
                                oninput: move |e| {
                                    let state = match &background.read().mode {
                                        BackgroundMode::Gradient { from, to, .. } => Some((from.clone(), to.clone())),
                                        _ => None,
                                    };
                                    if let Some((from, to)) = state {
                                        let parsed_angle = e.value().parse().unwrap_or(0);
                                        background.write().mode = BackgroundMode::Gradient { from, to, angle_deg: parsed_angle };
                                    }
                                }
                            }
                        }
                    }
                },
                BackgroundMode::Tile { url } => rsx! {
                    div {
                        class: "editor-field-group",
                        label { class: "editor-field-label", "Tile Image URL" }
                        input {
                            r#type: "text",
                            value: "{url}",
                            placeholder: "https://...",
                            class: "editor-field",
                            oninput: move |e| {
                                if matches!(background.read().mode, BackgroundMode::Tile { .. }) {
                                    background.write().mode = BackgroundMode::Tile { url: e.value() };
                                }
                            }
                        }
                    }
                }
            }

            PanelNote {
                title: "Note".to_string(),
                body: "This configuration dictates the lowest layer of the website (the <body> element). Your panels and elevated containers will sit on top of it.".to_string()
            }
        }
    }
}