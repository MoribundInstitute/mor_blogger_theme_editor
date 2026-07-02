use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

#[component]
pub fn SvgFramesPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    // Slider position from the stored value's leading digits ("30%", "30",
    // "30px" all -> 30). Clamped to the slider range; defaults to 30.
    let slice_pct: u32 = current_config
        .svg_border_slice
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(30)
        .clamp(1, 50);
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

            p { class: "editor-mini-label", style: "margin: 0;",
                "Paste a frame image URL to turn the frame on, then choose where it applies below. Clear the URL to remove it."
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
                // Slider, not a text box: the right value depends on where the
                // ornament band sits in YOUR image, and it's near-impossible to
                // guess blind. Drag and watch the Preview Frame snap into place.
                // Percentage = resolution-independent (survives Blogger's =sNNNN
                // rescaling). Leading digits parsed so old px values still seed it.
                label { class: "editor-field-label", "9-Slice (% of image edge) — {slice_pct}%" }
                input {
                    class: "editor-input",
                    r#type: "range",
                    min: "1",
                    max: "50",
                    step: "1",
                    value: "{slice_pct}",
                    oninput: {
                        let mut cfg = current_config.clone();
                        let apply = on_apply_theme.clone();
                        move |evt| {
                            cfg.svg_border_slice = format!("{}%", evt.value());
                            apply.call(cfg.clone());
                        }
                    }
                }
                p { class: "editor-mini-label", style: "margin: 4px 0 0;",
                    "Low = thin outer edge only. Raise it until the corner ornaments look whole."
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
