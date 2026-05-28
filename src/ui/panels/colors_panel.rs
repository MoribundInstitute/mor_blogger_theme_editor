use dioxus::prelude::*;

use crate::config::{SurfaceFill, SurfaceMode};
use crate::ui::components::inputs::{EditorCard, EditorInput};

#[component]
pub fn ColorsPanel(
    bg_base: Signal<String>,
    bg_panel: Signal<SurfaceFill>,
    bg_elevated: Signal<SurfaceFill>,
    fg_base: Signal<String>,
    fg_muted: Signal<String>,
    accent: Signal<String>,
    border: Signal<String>,
) -> Element {
    rsx! {
        EditorCard {
            title: "Global Theme Palette".to_string(),

            EditorInput {
                label: "Base Background".to_string(),
                value: bg_base,
                input_type: "color".to_string(),
                placeholder: "#0a0a0a".to_string()
            }

            SurfaceFillEditor {
                label: "Panel Background".to_string(),
                value: bg_panel,
                default_color: "#111111".to_string(),
            }

            SurfaceFillEditor {
                label: "Elevated Background".to_string(),
                value: bg_elevated,
                default_color: "#1a1a1a".to_string(),
            }

            EditorInput {
                label: "Base Text".to_string(),
                value: fg_base,
                input_type: "color".to_string(),
                placeholder: "#e0e0e0".to_string()
            }

            EditorInput {
                label: "Muted Text".to_string(),
                value: fg_muted,
                input_type: "color".to_string(),
                placeholder: "#a8a8a8".to_string()
            }

            EditorInput {
                label: "Accent".to_string(),
                value: accent,
                input_type: "color".to_string(),
                placeholder: "#bc8d6b".to_string()
            }

            EditorInput {
                label: "Border".to_string(),
                value: border,
                input_type: "color".to_string(),
                placeholder: "#444444".to_string()
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SurfaceFillEditor — mode dropdown + conditional inputs
// ---------------------------------------------------------------------------

#[component]
fn SurfaceFillEditor(label: String, value: Signal<SurfaceFill>, default_color: String) -> Element {
    let mut value = value;
    let current = value.read().clone();
    let mode_str = match current.mode {
        SurfaceMode::Solid => "solid",
        SurfaceMode::Gradient => "gradient",
    };

    let on_mode_change = move |e: Event<FormData>| {
        let mut next = value.read().clone();
        next.mode = match e.value().as_str() {
            "gradient" => SurfaceMode::Gradient,
            _ => SurfaceMode::Solid,
        };
        value.set(next);
    };

    let on_solid_color = move |e: Event<FormData>| {
        let mut next = value.read().clone();
        next.color = e.value();
        value.set(next);
    };

    let on_grad_from = move |e: Event<FormData>| {
        let mut next = value.read().clone();
        next.gradient_from = e.value();
        value.set(next);
    };

    let on_grad_to = move |e: Event<FormData>| {
        let mut next = value.read().clone();
        next.gradient_to = e.value();
        value.set(next);
    };

    let on_angle = move |e: Event<FormData>| {
        let mut next = value.read().clone();
        if let Ok(parsed) = e.value().parse::<u16>() {
            next.gradient_angle_deg = parsed.min(360);
            value.set(next);
        }
    };

    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            select {
                class: "editor-select",
                value: "{mode_str}",
                onchange: on_mode_change,

                option { value: "solid", selected: mode_str == "solid", "Solid" }
                option { value: "gradient", selected: mode_str == "gradient", "Gradient" }
            }

            match current.mode {
                SurfaceMode::Solid => rsx! {
                    input {
                        r#type: "color",
                        value: "{current.color}",
                        placeholder: "{default_color}",
                        class: "editor-field editor-color-field",
                        oninput: on_solid_color,
                    }
                },
                SurfaceMode::Gradient => rsx! {
                    div {
                        class: "editor-row-stretch",

                        div {
                            class: "editor-flex-1 editor-stack",
                            label {
                                class: "editor-mini-label",
                                "From"
                            }
                            input {
                                r#type: "color",
                                value: "{current.gradient_from}",
                                class: "editor-field editor-color-field",
                                oninput: on_grad_from,
                            }
                        }

                        div {
                            class: "editor-flex-1 editor-stack",
                            label {
                                class: "editor-mini-label",
                                "To"
                            }
                            input {
                                r#type: "color",
                                value: "{current.gradient_to}",
                                class: "editor-field editor-color-field",
                                oninput: on_grad_to,
                            }
                        }

                        div {
                            class: "editor-w-70 editor-stack",
                            label {
                                class: "editor-mini-label",
                                "Angle°"
                            }
                            input {
                                r#type: "number",
                                min: "0",
                                max: "360",
                                value: "{current.gradient_angle_deg}",
                                class: "editor-field",
                                oninput: on_angle,
                            }
                        }
                    }
                },
            }
        }
    }
}