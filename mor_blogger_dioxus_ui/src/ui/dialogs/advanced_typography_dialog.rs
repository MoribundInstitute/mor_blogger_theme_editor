use crate::app::state::ThemeState;
use crate::ui::components::inputs::EditorInput;
use crate::ui::components::modal::Modal;
use dioxus::html::HasFileData;
use dioxus::prelude::*;
use mor_blogger_core::config::fonts::{FontPreset, MONO_FONT_REGISTRY};
use std::path::Path;

const WEIGHT_OPTIONS: &[(&str, &str)] = &[
    ("Regular (400)", "400"),
    ("Medium (500)", "500"),
    ("Semibold (600)", "600"),
    ("Bold (700)", "700"),
];

/// Caveman metadata extraction. Avoids pulling in heavy ttf-parser crates.
/// Maps "FiraCode-Regular.ttf" -> "Fira Code"
fn parse_font_filename(filename: &str) -> String {
    let name = Path::new(filename)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Split camelCase boundaries if no spaces exist (e.g. FiraCode -> Fira Code)
    let mut spaced_name = String::new();
    let mut prev_is_lower = false;
    for c in name.chars() {
        if prev_is_lower && c.is_uppercase() {
            spaced_name.push(' ');
        }
        spaced_name.push(c);
        prev_is_lower = c.is_lowercase();
    }

    // Clean structural characters
    let cleaned = spaced_name.replace(['_', '-'], " ");

    // Strip common font weights from the end
    let mut final_name = cleaned.as_str();
    let dump_words = [
        " Regular",
        " Bold",
        " Italic",
        " Medium",
        " Light",
        " SemiBold",
        " Black",
        " Thin",
        " VariableFont",
    ];

    for w in dump_words {
        if final_name.ends_with(w) {
            final_name = final_name.trim_end_matches(w);
        }
    }

    final_name.trim().to_string()
}

#[component]
pub fn AdvancedTypographyDialog(mut open_signal: Signal<bool>) -> Element {
    let theme = use_context::<ThemeState>();
    let signals = theme.signals;

    rsx! {
        Modal {
            open: open_signal,
            title: "Advanced Typography",
            style: "width: 600px; max-height: 85vh; overflow-y: auto;".to_string(),
            on_close: move |_| open_signal.set(false),

            div { style: "padding: 20px; display: flex; flex-direction: column; gap: 20px;",

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid var(--editor-border-soft); padding-bottom: 8px;", "Mathematical Scaling" }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    EditorInput {
                        label: "Heading Scale Ratio".to_string(),
                        value: signals.scale_ratio,
                        input_type: "text".to_string(),
                        placeholder: "1.25".to_string(),
                    }
                    EditorInput {
                        label: "Body Line Height".to_string(),
                        value: signals.line_height,
                        input_type: "text".to_string(),
                        placeholder: "1.6".to_string(),
                    }
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 10px 0 0 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid var(--editor-border-soft); padding-bottom: 8px;", "Granular Font Selection" }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    FontStackPicker {
                        label: "Monospace Font".to_string(),
                        value: signals.mono_font_stack,
                        options: MONO_FONT_REGISTRY,
                    }
                    SimpleSelect {
                        label: "Heading Weight".to_string(),
                        value: signals.heading_weight,
                        options: WEIGHT_OPTIONS,
                    }
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 10px 0 0 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid var(--editor-border-soft); padding-bottom: 8px;", "Local Font Management" }

                div { style: "display: flex; flex-direction: column; gap: 16px;",
                    LocalFontUploader {
                        label: "Body Font Uploader".to_string(),
                        value: signals.body_font_stack,
                    }
                    LocalFontUploader {
                        label: "Heading Font Uploader".to_string(),
                        value: signals.heading_font_stack,
                    }
                    LocalFontUploader {
                        label: "Monospace Font Uploader".to_string(),
                        value: signals.mono_font_stack,
                    }
                }
            }
        }
    }
}

#[component]
fn FontStackPicker(
    label: String,
    value: Signal<String>,
    options: &'static [FontPreset],
) -> Element {
    let mut value = value;
    let current = value.read().clone();
    let current_trimmed = current.trim();

    let selected_key = options
        .iter()
        .find(|font| {
            font.name.eq_ignore_ascii_case(current_trimmed)
                || font.css_stack.eq_ignore_ascii_case(current_trimmed)
        })
        .map(|font| font.name.to_string())
        .unwrap_or_else(|| "__custom__".to_string());

    let is_custom = selected_key == "__custom__";

    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            select {
                class: "editor-select",
                value: "{selected_key}",
                onchange: move |e| {
                    let chosen = e.value();
                    if chosen == "__custom__" {
                        value.set(String::new());
                    } else {
                        value.set(chosen);
                    }
                },

                for font in options.iter() {
                    option {
                        value: "{font.name}",
                        selected: selected_key == font.name,
                        "{font.name} ({font.category})"
                    }
                }

                option {
                    value: "__custom__",
                    selected: is_custom,
                    "Custom / Google Font…"
                }
            }

            if is_custom {
                div {
                    style: "margin-top: 8px;",
                    input {
                        r#type: "text",
                        value: "{current}",
                        placeholder: "e.g. Courier New, monospace",
                        class: "editor-field",
                        style: "width: 100%;",
                        oninput: move |e| value.set(e.value()),
                    }
                }
            }
        }
    }
}

#[component]
fn SimpleSelect(
    label: String,
    value: Signal<String>,
    options: &'static [(&'static str, &'static str)],
) -> Element {
    let mut value = value;
    let current = value.read().clone();

    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            select {
                class: "editor-select",
                value: "{current}",
                onchange: move |e| value.set(e.value()),

                for (name, css) in options.iter() {
                    option {
                        value: "{css}",
                        selected: *css == current.as_str(),
                        "{name}"
                    }
                }
            }
        }
    }
}

#[component]
fn LocalFontUploader(label: String, mut value: Signal<String>) -> Element {
    let mut is_hovered = use_signal(|| false);
    let current = value.read().clone();

    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            div {
                style: if is_hovered() {
                    "border: 1px dashed var(--accent); padding: 12px; border-radius: 4px; background: color-mix(in srgb, var(--accent) 10%, transparent); transition: all 0.2s;"
                } else {
                    "border: 1px dashed var(--editor-border-soft, #3a3a3c); padding: 12px; border-radius: 4px; background: #2c2c2e; transition: all 0.2s;"
                },
                ondragover: move |evt| { evt.prevent_default(); is_hovered.set(true); },
                ondragenter: move |evt| { evt.prevent_default(); is_hovered.set(true); },
                ondragleave: move |_| is_hovered.set(false),
                ondrop: move |evt| {
                    evt.prevent_default();
                    is_hovered.set(false);
                    if let Some(file) = evt.files().first() {
                        let parsed = parse_font_filename(&file.name());
                        value.set(parsed);
                    }
                },

                div {
                    style: "display: flex; gap: 8px; align-items: center;",
                    input {
                        r#type: "text",
                        value: "{current}",
                        placeholder: "Type font family name or browse local file",
                        class: "editor-field",
                        style: "flex: 1; min-width: 0;",
                        oninput: move |e| value.set(e.value()),
                    }
                    button {
                        class: "editor-button",
                        style: "flex-shrink: 0; white-space: nowrap;",
                        onclick: move |_| {
                            spawn(async move {
                                if let Some(file) = rfd::AsyncFileDialog::new()
                                    .add_filter("Fonts", &["ttf", "woff", "woff2", "otf"])
                                    .pick_file()
                                    .await
                                {
                                    let parsed = parse_font_filename(&file.file_name());
                                    value.set(parsed);
                                }
                            });
                        },
                        "Browse..."
                    }
                }
                p {
                    class: "editor-mini-label",
                    style: "margin-top: 6px; margin-bottom: 0;",
                    "Drag a font file here or click Browse to load local TTF/WOFF."
                }
            }
        }
    }
}
