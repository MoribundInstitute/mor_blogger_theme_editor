use dioxus::prelude::*;

#[component]
pub fn EffectsPanel(
    panel_border_width: Signal<String>,
    glow_spread: Signal<String>,
    hover_scale: Signal<String>,
    panel_border_image_url: Signal<String>,
    panel_border_image_slice: Signal<String>,
    panel_border_image_repeat: Signal<String>,
) -> Element {
    rsx! {
        div { class: "editor-panel",

            div {
                class: "editor-help-text",
                style: "margin-bottom: 16px;",
                "Control structural hover effects, neon glows, master container borders, and image-frame borders."
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Panel Border Width" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 1px, 2px, 0",
                    value: "{panel_border_width}",
                    oninput: move |evt| panel_border_width.set(evt.value()),
                }
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

            div {
                class: "editor-help-text",
                style: "margin: 24px 0 16px 0;",
                "Image Borders (overrides the visual solid border when a URL is provided)"
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Border Image Source (URL)" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. https://.../frame.png",
                    value: "{panel_border_image_url}",
                    oninput: move |evt| panel_border_image_url.set(evt.value()),
                }
            }

            div { class: "editor-row-stretch",
                div { class: "editor-field-group", style: "flex: 1;",
                    label { class: "editor-field-label", "Slice" }
                    input {
                        class: "editor-input",
                        r#type: "text",
                        placeholder: "e.g. 30%, 20px",
                        value: "{panel_border_image_slice}",
                        oninput: move |evt| panel_border_image_slice.set(evt.value()),
                    }
                }

                div { class: "editor-field-group", style: "flex: 1;",
                    label { class: "editor-field-label", "Repeat Mode" }
                    select {
                        class: "editor-input",
                        value: "{panel_border_image_repeat}",
                        onchange: move |evt| panel_border_image_repeat.set(evt.value()),
                        option { value: "stretch", "Stretch" }
                        option { value: "repeat", "Repeat" }
                        option { value: "round", "Round" }
                        option { value: "space", "Space" }
                    }
                }
            }
        }
    }
}
