use dioxus::prelude::*;

use crate::app::state::ThemeState;
use crate::ui::components::inputs::EditorCard;

/// Size presets map to padding + font-size on the ButtonConfig.
fn apply_size(b: &mut mor_blogger_core::config::ButtonConfig, size: &str) {
    let (px, py, fs) = match size {
        "sm" => ("10px", "3px", "0.8rem"),
        "lg" => ("20px", "10px", "1.05rem"),
        _ => ("14px", "6px", ""), // md
    };
    b.padding_x = px.to_string();
    b.padding_y = py.to_string();
    b.font_size = fs.to_string();
}

fn current_size(b: &mor_blogger_core::config::ButtonConfig) -> &'static str {
    match (b.padding_x.as_str(), b.padding_y.as_str()) {
        ("10px", "3px") => "sm",
        ("20px", "10px") => "lg",
        _ => "md",
    }
}

#[component]
pub fn ButtonsPanel() -> Element {
    let mut state = consume_context::<ThemeState>();
    let mut buttons = state.signals.buttons;
    let b = buttons.read().clone();
    let size = current_size(&b);

    rsx! {
        EditorCard {
            title: "Buttons & Shapes".to_string(),

            button {
                class: "mor-btn-secondary",
                style: "width: 100%; margin-bottom: 12px;",
                onclick: move |_| state.show_advanced_buttons.set(true),
                "⚙ Advanced Buttons"
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Button Shape" }
                select {
                    class: "editor-select",
                    value: "{b.radius}",
                    onchange: move |e| { buttons.write().radius = e.value(); },
                    option { value: "0px", selected: b.radius == "0px", "Square" }
                    option { value: "6px", selected: b.radius == "6px", "Rounded" }
                    option { value: "99px", selected: b.radius == "99px", "Pill" }
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Size" }
                select {
                    class: "editor-select",
                    value: "{size}",
                    onchange: move |e| { apply_size(&mut buttons.write(), &e.value()); },
                    option { value: "sm", selected: size == "sm", "Small" }
                    option { value: "md", selected: size == "md", "Medium" }
                    option { value: "lg", selected: size == "lg", "Large" }
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Fill Style" }
                select {
                    class: "editor-select",
                    value: "{b.fill}",
                    onchange: move |e| { buttons.write().fill = e.value(); },
                    option { value: "outline", selected: b.fill == "outline", "Outline" }
                    option { value: "solid", selected: b.fill == "solid", "Solid" }
                    option { value: "soft", selected: b.fill == "soft", "Soft" }
                    option { value: "ghost", selected: b.fill == "ghost", "Ghost" }
                    option { value: "glass", selected: b.fill == "glass", "Glass" }
                    option { value: "neon", selected: b.fill == "neon", "Neon" }
                    option { value: "glossy", selected: b.fill == "glossy", "Glossy" }
                    option { value: "gradient", selected: b.fill == "gradient", "Gradient" }
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Text Style" }
                select {
                    class: "editor-select",
                    value: "{b.text_transform}",
                    onchange: move |e| { buttons.write().text_transform = e.value(); },
                    option { value: "none", selected: b.text_transform == "none", "Normal" }
                    option { value: "uppercase", selected: b.text_transform == "uppercase", "UPPERCASE" }
                    option { value: "lowercase", selected: b.text_transform == "lowercase", "lowercase" }
                }
            }
        }
    }
}
