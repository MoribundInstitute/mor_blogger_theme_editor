use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

#[component]
pub fn SvgFramesPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    rsx! {
        div { class: "editor-card", style: "padding: 16px; display: flex; flex-direction: column; gap: 16px;",

            div { class: "editor-field-group", style: "margin-bottom: 16px;",
                label { class: "editor-field-label", "Fallback Border Width" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 1px, 2px, 0",
                    value: "{current_config.colors.panel_border_width}",
                    oninput: {
                        let mut cfg = current_config.clone();
                        let apply = on_apply_theme.clone();
                        move |evt| {
                            cfg.colors.panel_border_width = evt.value();
                            apply.call(cfg.clone());
                        }
                    }
                }
            }

            // Toggle Switch / Row
            div { style: "display: flex; align-items: center; justify-content: space-between;",
                label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; font-size: 13px;",
                    input {
                        r#type: "checkbox",
                        checked: current_config.enable_image_borders,
                        onchange: {
                            let mut cfg = current_config.clone();
                            let apply = on_apply_theme.clone();
                            move |_| {
                                cfg.enable_image_borders = !cfg.enable_image_borders;
                                apply.call(cfg.clone());
                            }
                        }
                    }
                    "Enable Custom Image Borders"
                }
            }

            // Hosted URL Input Section
            div { class: "editor-field-group",
                label { class: "editor-field-label", "Image URL (e.g., hosted on Blogger/Imgur)" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "https://.../frame.png",
                    value: "{current_config.custom_border_url.clone().unwrap_or_default()}",
                    oninput: {
                        let mut cfg = current_config.clone();
                        let apply = on_apply_theme.clone();
                        move |evt| {
                            let val = evt.value();
                            cfg.custom_border_url = if val.trim().is_empty() { None } else { Some(val) };
                            apply.call(cfg.clone());
                        }
                    }
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Image Frame Width" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 20px, 30px",
                    value: "{current_config.image_border_width}",
                    oninput: {
                        let mut cfg = current_config.clone();
                        let apply = on_apply_theme.clone();
                        move |evt| {
                            cfg.image_border_width = evt.value();
                            apply.call(cfg.clone());
                        }
                    }
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "9-Slice Value" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 30, 30%",
                    value: "{current_config.svg_border_slice}",
                    oninput: {
                        let mut cfg = current_config.clone();
                        let apply = on_apply_theme.clone();
                        move |evt| {
                            cfg.svg_border_slice = evt.value();
                            apply.call(cfg.clone());
                        }
                    }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; font-size: 13px;",
                    input {
                        r#type: "checkbox",
                        checked: current_config.target_sidebars,
                        onchange: {
                            let mut cfg = current_config.clone();
                            let apply = on_apply_theme.clone();
                            move |_| {
                                cfg.target_sidebars = !cfg.target_sidebars;
                                apply.call(cfg.clone());
                            }
                        }
                    }
                    "Apply to Sidebars"
                }
                label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; font-size: 13px;",
                    input {
                        r#type: "checkbox",
                        checked: current_config.target_canvas,
                        onchange: {
                            let mut cfg = current_config.clone();
                            let apply = on_apply_theme.clone();
                            move |_| {
                                cfg.target_canvas = !cfg.target_canvas;
                                apply.call(cfg.clone());
                            }
                        }
                    }
                    "Apply to Main Canvas"
                }
            }

            // Visual Preview Box
            div { style: "display: flex; flex-direction: column; gap: 8px;",
                span { class: "editor-field-label", "Preview Frame" }
                if let Some(ref url) = current_config.custom_border_url {
                    div {
                        style: "height: 120px; background: var(--bg-elevated); display: flex; align-items: center; justify-content: center; box-sizing: border-box; border-style: solid; border-width: {current_config.image_border_width}; border-image-source: url(\"{url}\"); border-image-slice: {current_config.svg_border_slice}; border-image-repeat: round;",
                        span { style: "color: var(--editor-text); font-size: 0.9em; background: rgba(0,0,0,0.5); padding: 4px 8px; border-radius: 4px;", "Border Frame Active" }
                    }
                } else {
                    div {
                        style: "height: 120px; background: var(--bg-elevated); border: 1px dashed var(--editor-border-soft); border-radius: 4px; display: flex; align-items: center; justify-content: center;",
                        span { style: "color: var(--editor-text-muted); font-size: 0.9em;", "No Custom Border Image Loaded" }
                    }
                }
            }
        }
    }
}
