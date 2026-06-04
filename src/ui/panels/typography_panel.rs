//! Typography panel.
//!
//! Font pickers use a static local registry: no font files, no network calls,
//! and no per-render option vector cloning. Custom entries are stored as raw
//! names/stacks and resolved by the XML generator during export.

use dioxus::prelude::*;

use crate::config::fonts::{FontPreset, FONT_REGISTRY, MONO_FONT_REGISTRY};
use crate::ui::components::inputs::{EditorCard, EditorInput};

const WEIGHT_OPTIONS: &[(&str, &str)] = &[
    ("Regular (400)", "400"),
    ("Medium (500)", "500"),
    ("Semibold (600)", "600"),
    ("Bold (700)", "700"),
];

const CUSTOM_SENTINEL: &str = "__custom__";

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
                input {
                    r#type: "text",
                    value: "{current}",
                    placeholder: "e.g. Fira Code, Noto Serif KR, or Georgia, serif",
                    class: "editor-field",
                    oninput: move |e| value.set(e.value()),
                }
                p {
                    class: "editor-mini-label",
                    "Type a Google Font family name, or paste a full CSS stack."
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
