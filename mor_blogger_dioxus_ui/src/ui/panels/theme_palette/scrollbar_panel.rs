use dioxus::prelude::*;

use crate::app::state::ThemeState;
use crate::ui::components::inputs::EditorCard;

#[component]
pub fn ScrollbarPanel() -> Element {
    let mut state = consume_context::<ThemeState>();

    let raw = state.signals.scrollbar_width.read().clone();
    // ponytail: strip "px" to get numeric value for the range input
    let numeric: u32 = raw.trim_end_matches("px").parse().unwrap_or(6);

    rsx! {
        EditorCard {
            title: "Scrollbar".to_string(),

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Scrollbar Width ({numeric}px)" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "24",
                    value: "{numeric}",
                    style: "width: 100%;",
                    oninput: move |e| {
                        let v = e.value();
                        *state.signals.scrollbar_width.write() = format!("{v}px");
                    },
                }
            }

            div { class: "editor-row",
                div { class: "editor-field-group",
                    label { class: "editor-field-label", "Track Color" }
                    crate::ui::components::inputs::ColorInput {
                        value: state.signals.scrollbar_track_color.read().clone(),
                        oninput: move |v: String| {
                            *state.signals.scrollbar_track_color.write() = v;
                        },
                    }
                }
                div { class: "editor-field-group",
                    label { class: "editor-field-label", "Thumb Color" }
                    crate::ui::components::inputs::ColorInput {
                        value: state.signals.scrollbar_thumb_color.read().clone(),
                        oninput: move |v: String| {
                            *state.signals.scrollbar_thumb_color.write() = v;
                        },
                    }
                }
                div { class: "editor-field-group",
                    label { class: "editor-field-label", "Thumb Hover" }
                    crate::ui::components::inputs::ColorInput {
                        value: state.signals.scrollbar_thumb_hover_color.read().clone(),
                        oninput: move |v: String| {
                            *state.signals.scrollbar_thumb_hover_color.write() = v;
                        },
                    }
                }
            }
        }
    }
}
