use dioxus::prelude::*;

use crate::app::state::ThemeState;
use crate::ui::components::inputs::{EditorCard, EditorInput};
use mor_blogger_core::config::SurfaceFill;

#[component]
pub fn ColorsPanel(
    bg_base: Signal<String>,
    bg_panel: Signal<SurfaceFill>,
    bg_elevated: Signal<SurfaceFill>,
    header_fill: Signal<SurfaceFill>,
    fg_base: Signal<String>,
    fg_muted: Signal<String>,
    accent: Signal<String>,
    border: Signal<String>,
) -> Element {
    let mut app_state = use_context::<ThemeState>();

    rsx! {
        EditorCard {
            title: "Global Theme Palette".to_string(),

            button {
                class: "mor-btn-secondary",
                style: "width: 100%; margin-bottom: 12px;",
                onclick: move |_| app_state.show_advanced_colors.set(true),
                "⚙ Advanced Color Routing"
            }

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

            SurfaceFillEditor {
                label: "Header Background".to_string(),
                value: header_fill,
                default_color: "#141414".to_string(),
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
// SurfaceFillEditor — simplified solid color editor (gradient edits moved to Advanced dialog)
// ---------------------------------------------------------------------------

#[component]
fn SurfaceFillEditor(label: String, value: Signal<SurfaceFill>, default_color: String) -> Element {
    let mut value = value;
    let current = value.read().clone();

    let on_solid_color = move |e: Event<FormData>| {
        let mut next = value.read().clone();
        next.color = e.value();
        value.set(next);
    };

    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            input {
                r#type: "color",
                value: "{current.color}",
                placeholder: "{default_color}",
                class: "editor-field editor-color-field",
                oninput: on_solid_color,
            }
        }
    }
}
