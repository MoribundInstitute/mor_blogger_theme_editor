use dioxus::prelude::*;

#[component]
pub fn EffectsPanel(
    glow_spread: Signal<String>,
    hover_scale: Signal<String>,
) -> Element {
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

#[component]
pub fn BordersPanel(
    panel_border_width: Signal<String>,
    panel_border_image_url: Signal<String>,
    panel_border_image_slice: Signal<String>,
    panel_border_image_repeat: Signal<String>,
) -> Element {
    rsx! {
        div { class: "editor-panel",

            div {
                class: "editor-help-text",
                style: "margin-bottom: 16px;",
                "Control master container borders and image-frame borders."
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

            div {
                class: "editor-help-text",
                style: "margin: 24px 0 16px 0;",
                "Image Borders (overrides the visual solid border when a URL is provided)"
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Border Image Source (URL)" }
                if panel_border_image_url().starts_with("data:image/svg+xml") {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        div {
                            class: "editor-row-stretch",
                            style: "align-items: center; justify-content: space-between; background: rgba(255, 255, 255, 0.05); padding: 6px 12px; border-radius: var(--radius-sm); border: 1px dashed var(--editor-border-soft);",
                            span {
                                style: "font-family: monospace; font-size: 0.85rem; color: var(--editor-accent);",
                                "[Embedded SVG Frame Asset]"
                            }
                            button {
                                class: "editor-mini-button",
                                style: "color: #ff7b72; border-color: rgba(218, 54, 51, 0.3); padding: 2px 8px; min-height: 22px;",
                                onclick: move |_| {
                                    panel_border_image_url.set(String::new());
                                },
                                "Clear ×"
                            }
                        }
                        button {
                            class: "editor-button",
                            style: "width: 100%;",
                            onclick: move |_| {
                                spawn(async move {
                                    if let Some(handle) = rfd::AsyncFileDialog::new().add_filter("SVG", &["svg"]).pick_file().await {
                                        let bytes = handle.read().await;
                                        if let Ok(svg_str) = String::from_utf8(bytes) {
                                            panel_border_image_url.set(mor_blogger_core::utils::svg_icons::svg_to_data_uri(&svg_str));
                                        }
                                    }
                                });
                            },
                            "Browse SVG..."
                        }
                    }
                } else {
                    div { class: "editor-row-stretch",
                        input {
                            class: "editor-input",
                            style: "flex: 1;",
                            r#type: "text",
                            placeholder: "e.g. https://.../frame.png",
                            value: "{panel_border_image_url}",
                            oninput: move |evt| panel_border_image_url.set(evt.value()),
                        }
                        button {
                            class: "editor-button",
                            onclick: move |_| {
                                spawn(async move {
                                    if let Some(handle) = rfd::AsyncFileDialog::new().add_filter("SVG", &["svg"]).pick_file().await {
                                        let bytes = handle.read().await;
                                        if let Ok(svg_str) = String::from_utf8(bytes) {
                                            panel_border_image_url.set(mor_blogger_core::utils::svg_icons::svg_to_data_uri(&svg_str));
                                        }
                                    }
                                });
                            },
                            "Browse SVG..."
                        }
                    }
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
