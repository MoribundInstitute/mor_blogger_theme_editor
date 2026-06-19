use dioxus::prelude::*;

#[component]
pub fn EffectsPanel(glow_spread: Signal<String>, hover_scale: Signal<String>) -> Element {
    rsx! {
        div { class: "editor-panel",

            div {
                class: "editor-help-text",
                style: "margin-bottom: 16px;",
                "Control structural hover effects and neon glows."
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Neon Glow Spread" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 10px, 20px, 0",
                    value: "{glow_spread}",
                    oninput: move |evt| glow_spread.set(evt.value()),
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Hover Scale (Zoom)" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 1.02, 1.05, 1",
                    value: "{hover_scale}",
                    oninput: move |evt| hover_scale.set(evt.value()),
                }
            }
        }
    }
}
