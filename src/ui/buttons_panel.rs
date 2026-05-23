use dioxus::prelude::*;

use crate::ui::inputs::EditorCard;

#[component]
pub fn ButtonsPanel(
    btn_radius: Signal<String>,
    btn_border_width: Signal<String>,
    btn_text_transform: Signal<String>,
) -> Element {
    rsx! {
        EditorCard {
            title: "Buttons & Shapes".to_string(),

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Button Shape" }
                select {
                    class: "editor-select",
                    value: "{btn_radius}",
                    onchange: move |e| btn_radius.set(e.value()),
                    option { value: "0px", selected: btn_radius() == "0px", "Square" }
                    option { value: "6px", selected: btn_radius() == "6px", "Rounded" }
                    option { value: "99px", selected: btn_radius() == "99px", "Pill" }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Border Thickness" }
                select {
                    class: "editor-select",
                    value: "{btn_border_width}",
                    onchange: move |e| btn_border_width.set(e.value()),
                    option { value: "0px", selected: btn_border_width() == "0px", "None" }
                    option { value: "1px", selected: btn_border_width() == "1px", "Thin (1px)" }
                    option { value: "2px", selected: btn_border_width() == "2px", "Thick (2px)" }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Text Style" }
                select {
                    class: "editor-select",
                    value: "{btn_text_transform}",
                    onchange: move |e| btn_text_transform.set(e.value()),
                    option { value: "none", selected: btn_text_transform() == "none", "Normal" }
                    option { value: "uppercase", selected: btn_text_transform() == "uppercase", "UPPERCASE" }
                    option { value: "lowercase", selected: btn_text_transform() == "lowercase", "lowercase" }
                }
            }
        }
    }
}
