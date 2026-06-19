//! Typography panel.
//!
//! Font pickers use a static local registry: no font files, no network calls,
//! and no per-render option vector cloning. Custom entries are stored as raw
//! names/stacks and resolved by the XML generator during export.

use dioxus::html::HasFileData;
use dioxus::prelude::*;
use std::path::Path;

use crate::ui::components::inputs::{EditorCard, EditorInput};
use mor_blogger_core::config::fonts::{FontPreset, FONT_REGISTRY, MONO_FONT_REGISTRY};

const WEIGHT_OPTIONS: &[(&str, &str)] = &[
    ("Regular (400)", "400"),
    ("Medium (500)", "500"),
    ("Semibold (600)", "600"),
    ("Bold (700)", "700"),
];

const CUSTOM_SENTINEL: &str = "__custom__";

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
pub fn TypographyPanel(
    body_font_stack: Signal<String>,
    heading_font_stack: Signal<String>,
    mono_font_stack: Signal<String>,
    base_size: Signal<String>,
    scale_ratio: Signal<String>,
    line_height: Signal<String>,
    heading_weight: Signal<String>,
) -> Element {
    rsx! {
        EditorCard {
            title: "Typography".to_string(),

            p {
                class: "editor-mini-label",
                title: "The app stays offline. Exported Blogger XML gets Google Fonts links for active Google font names.",
                "Offline preview uses local fallbacks. Export loads Google Fonts on live Blogger. ⓘ"
            }

            FontStackPicker {
                label: "Body Font".to_string(),
                value: body_font_stack,
                options: FONT_REGISTRY,
                include_match_body: false,
            }

            FontStackPicker {
                label: "Heading Font".to_string(),
                value: heading_font_stack,
                options: FONT_REGISTRY,
                include_match_body: true,
            }

            FontStackPicker {
                label: "Monospace Font".to_string(),
                value: mono_font_stack,
                options: MONO_FONT_REGISTRY,
                include_match_body: false,
            }

            EditorInput {
                label: "Base Font Size".to_string(),
                value: base_size,
                input_type: "text".to_string(),
                placeholder: "16px".to_string(),
            }

            EditorInput {
                label: "Heading Scale Ratio".to_string(),
                value: scale_ratio,
                input_type: "text".to_string(),
                placeholder: "1.25".to_string(),
            }

            EditorInput {
                label: "Body Line Height".to_string(),
                value: line_height,
                input_type: "text".to_string(),
                placeholder: "1.6".to_string(),
            }

            SimpleSelect {
                label: "Heading Weight".to_string(),
                value: heading_weight,
                options: WEIGHT_OPTIONS,
            }
        }
    }
}

#[component]
fn FontStackPicker(
    label: String,
    value: Signal<String>,
    options: &'static [FontPreset],
    include_match_body: bool,
) -> Element {
    let mut value = value;
    let mut is_hovered = use_signal(|| false);

    let current = value.read().clone();
    let current_trimmed = current.trim();

    let selected_key = if include_match_body && current_trimmed.is_empty() {
        "__match_body__".to_string()
    } else {
        options
            .iter()
            .find(|font| {
                font.name.eq_ignore_ascii_case(current_trimmed)
                    || font.css_stack.eq_ignore_ascii_case(current_trimmed)
            })
            .map(|font| font.name.to_string())
            .unwrap_or_else(|| CUSTOM_SENTINEL.to_string())
    };

    let is_custom = selected_key == CUSTOM_SENTINEL;
    let current_is_preset = !is_custom;

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

                    if chosen == CUSTOM_SENTINEL {
                        if current_is_preset {
                            value.set(String::new());
                        }
                    } else if chosen == "__match_body__" {
                        value.set(String::new());
                    } else {
                        value.set(chosen);
                    }
                },

                if include_match_body {
                    option {
                        value: "__match_body__",
                        selected: selected_key == "__match_body__",
                        "Match body"
                    }
                }

                for font in options.iter() {
                    option {
                        value: "{font.name}",
                        selected: selected_key == font.name,
                        "{font.name} ({font.category})"
                    }
                }

                option {
                    value: "{CUSTOM_SENTINEL}",
                    selected: is_custom,
                    "Custom / Google Font…"
                }
            }

            if is_custom {
                div {
                    style: if is_hovered() {
                        "margin-top: 8px; border: 1px dashed var(--accent); padding: 8px; border-radius: 4px; background: color-mix(in srgb, var(--accent) 10%, transparent); transition: all 0.2s;"
                    } else {
                        "margin-top: 8px; border: 1px dashed transparent; padding: 0 0 8px 0; transition: all 0.2s;"
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
                            placeholder: "e.g. Fira Code, Noto Serif KR, or Georgia, serif",
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
                            "Browse Local Font..."
                        }
                    }
                    p {
                        class: "editor-mini-label",
                        style: "margin-top: 6px;",
                        "Type Google Font family, browse a local font file, or drag a .ttf/.woff file here."
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
