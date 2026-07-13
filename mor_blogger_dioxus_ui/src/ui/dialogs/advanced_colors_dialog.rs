use crate::app::state::ThemeState;
use crate::ui::dialogs::modal::Modal;
use crate::ui::panels::theme_palette::presets::importers::parse_theme_text;
use dioxus::prelude::*;
use mor_blogger_core::config::{SurfaceFill, SurfaceMode};

#[component]
pub fn AdvancedColorsDialog(mut open_signal: Signal<bool>) -> Element {
    let theme = use_context::<ThemeState>();
    let mut signals = theme.signals;

    rsx! {
        Modal {
            open: open_signal,
            title: "Advanced Color Routing",
            style: "width: 600px; max-height: 80vh; overflow-y: auto;".to_string(),
            on_close: move |_| open_signal.set(false),

            div { style: "padding: 20px; display: flex; flex-direction: column; gap: 20px;",

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 0; font-size: 16px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Gradient Configurations" }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    div {
                        h4 { style: "color: var(--fg-muted); margin: 0 0 8px 0; font-size: 13px;", "Panel Background" }
                        SurfaceFillEditor {
                            value: signals.bg_panel,
                            default_color: "#111111".to_string(),
                        }
                    }
                    div {
                        h4 { style: "color: var(--fg-muted); margin: 0 0 8px 0; font-size: 13px;", "Elevated Background" }
                        SurfaceFillEditor {
                            value: signals.bg_elevated,
                            default_color: "#1a1a1a".to_string(),
                        }
                    }
                    div {
                        h4 { style: "color: var(--fg-muted); margin: 0 0 8px 0; font-size: 13px;", "Header Background" }
                        SurfaceFillEditor {
                            value: signals.header_fill,
                            default_color: "#141414".to_string(),
                        }
                    }
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 10px 0 0 0; font-size: 16px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Granular Overrides" }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        h4 { style: "color: var(--fg-muted); margin: 0; font-size: 13px;", "Scrollbar Customization" }

                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 11px; color: #8e8e93;", "Track Color" }
                            input {
                                r#type: "color",
                                value: "{signals.scrollbar_track_color}",
                                style: "width: 100%; height: 28px; background: #2c2c2e; border: 1px solid #3a3a3c; border-radius: 4px; cursor: pointer; padding: 2px;",
                                oninput: move |e| signals.scrollbar_track_color.set(e.value()),
                            }
                        }
                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 11px; color: #8e8e93;", "Thumb Color" }
                            input {
                                r#type: "color",
                                value: "{signals.scrollbar_thumb_color}",
                                style: "width: 100%; height: 28px; background: #2c2c2e; border: 1px solid #3a3a3c; border-radius: 4px; cursor: pointer; padding: 2px;",
                                oninput: move |e| signals.scrollbar_thumb_color.set(e.value()),
                            }
                        }
                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 11px; color: #8e8e93;", "Thumb Hover Color" }
                            input {
                                r#type: "color",
                                value: "{signals.scrollbar_thumb_hover_color}",
                                style: "width: 100%; height: 28px; background: #2c2c2e; border: 1px solid #3a3a3c; border-radius: 4px; cursor: pointer; padding: 2px;",
                                oninput: move |e| signals.scrollbar_thumb_hover_color.set(e.value()),
                            }
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        h4 { style: "color: var(--fg-muted); margin: 0; font-size: 13px;", "Interactive States & Outlines" }

                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 11px; color: #8e8e93;", "Glow Color Override" }
                            input {
                                r#type: "text",
                                placeholder: "e.g. #bc8d6b or empty",
                                value: "{signals.glow_color}",
                                style: "width: 100%; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; padding: 6px; border-radius: 4px; font-size: 12px;",
                                oninput: move |e| signals.glow_color.set(e.value()),
                            }
                        }

                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 11px; color: #8e8e93;", "Border Radius (Buttons)" }
                            input {
                                r#type: "text",
                                placeholder: "e.g. 4px",
                                value: "{signals.buttons.read().radius}",
                                style: "width: 100%; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; padding: 6px; border-radius: 4px; font-size: 12px;",
                                oninput: move |e| { signals.buttons.write().radius = e.value(); },
                            }
                        }
                    }
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 10px 0 0 0; font-size: 16px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Palette Tools" }

                div { style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 12px; color: var(--fg-muted);", "Load complete theme configuration from a JSON palette file." }
                    button {
                        class: "mor-btn",
                        onclick: move |_| {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Import JSON Palette")
                                .add_filter("JSON Theme / Palette", &["json"])
                                .pick_file()
                            {
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    match parse_theme_text(&content) {
                                        Ok(config) => {
                                            signals.apply_config(&config);
                                            theme.commit();
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to parse theme: {}", e);
                                        }
                                    }
                                }
                            }
                        },
                        "Import JSON Palette"
                    }
                }
            }
        }
    }
}

#[component]
fn SurfaceFillEditor(value: Signal<SurfaceFill>, default_color: String) -> Element {
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
            style: "display: flex; flex-direction: column; gap: 8px; padding: 10px; background: #1C1C1E; border: 1px solid #2C2C2E; border-radius: 6px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                label { style: "font-size: 11px; color: #8e8e93;", "Fill Mode" }
                select {
                    style: "background: #2c2c2e; border: 1px solid #3a3a3c; color: #e5e5ea; border-radius: 4px; padding: 2px 4px; font-size: 12px;",
                    value: "{mode_str}",
                    onchange: on_mode_change,

                    option { value: "solid", "Solid" }
                    option { value: "gradient", "Gradient" }
                }
            }

            match current.mode {
                SurfaceMode::Solid => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 11px; color: #8e8e93;", "Color" }
                        input {
                            r#type: "color",
                            value: "{current.color}",
                            placeholder: "{default_color}",
                            style: "width: 100%; height: 28px; background: #2c2c2e; border: 1px solid #3a3a3c; border-radius: 4px; cursor: pointer; padding: 2px;",
                            oninput: on_solid_color,
                        }
                    }
                },
                SurfaceMode::Gradient => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                label { style: "font-size: 10px; color: #8e8e93;", "From" }
                                input {
                                    r#type: "color",
                                    value: "{current.gradient_from}",
                                    style: "width: 100%; height: 28px; background: #2c2c2e; border: 1px solid #3a3a3c; border-radius: 4px; cursor: pointer; padding: 2px;",
                                    oninput: on_grad_from,
                                }
                            }
                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                label { style: "font-size: 10px; color: #8e8e93;", "To" }
                                input {
                                    r#type: "color",
                                    value: "{current.gradient_to}",
                                    style: "width: 100%; height: 28px; background: #2c2c2e; border: 1px solid #3a3a3c; border-radius: 4px; cursor: pointer; padding: 2px;",
                                    oninput: on_grad_to,
                                }
                            }
                        }
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center;",
                            label { style: "font-size: 10px; color: #8e8e93;", "Angle" }
                            input {
                                r#type: "number",
                                min: "0",
                                max: "360",
                                value: "{current.gradient_angle_deg}",
                                style: "width: 60px; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; padding: 2px 4px; border-radius: 4px; font-size: 11px;",
                                oninput: on_angle,
                            }
                        }
                    }
                }
            }
        }
    }
}
