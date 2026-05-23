//! Typography panel.
//!
//! Three font-stack pickers (body, heading, mono), each with a dropdown of
//! curated system stacks plus a "Custom…" option that reveals a text input.
//! Plus four numeric/text inputs for base size, scale ratio, line height,
//! and heading weight.

use dioxus::prelude::*;

use crate::ui::inputs::{EditorCard, EditorInput};

const STACK_MONO: &str = "'Courier New', Courier, monospace";
const STACK_SERIF: &str = "Georgia, 'Times New Roman', Times, serif";
const STACK_SANS: &str =
    "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif";
const STACK_NEWSPAPER: &str = "'Times New Roman', Times, Georgia, serif";
const STACK_SYSTEM_UI: &str = "system-ui, -apple-system, sans-serif";
const STACK_HELVETICA: &str = "Helvetica, Arial, sans-serif";

const BODY_OPTIONS: &[(&str, &str)] = &[
    ("Serif (Georgia)", STACK_SERIF),
    ("Sans (System)", STACK_SANS),
    ("Sans (Helvetica)", STACK_HELVETICA),
    ("Mono (Courier)", STACK_MONO),
    ("Newspaper (Times)", STACK_NEWSPAPER),
    ("System UI", STACK_SYSTEM_UI),
];

const HEADING_OPTIONS: &[(&str, &str)] = &[
    ("Match body", ""),
    ("Serif (Georgia)", STACK_SERIF),
    ("Sans (System)", STACK_SANS),
    ("Sans (Helvetica)", STACK_HELVETICA),
    ("Mono (Courier)", STACK_MONO),
    ("Newspaper (Times)", STACK_NEWSPAPER),
    ("System UI", STACK_SYSTEM_UI),
];

const MONO_OPTIONS: &[(&str, &str)] = &[
    ("Courier New", STACK_MONO),
    ("Menlo / Consolas", "Menlo, Consolas, 'Liberation Mono', monospace"),
    ("SF Mono", "'SF Mono', SFMono-Regular, ui-monospace, monospace"),
];

const WEIGHT_OPTIONS: &[(&str, &str)] = &[
    ("Regular (400)", "400"),
    ("Medium (500)", "500"),
    ("Semibold (600)", "600"),
    ("Bold (700)", "700"),
];

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

            FontStackPicker {
                label: "Body Font".to_string(),
                value: body_font_stack,
                options: BODY_OPTIONS.to_vec(),
            }

            FontStackPicker {
                label: "Heading Font".to_string(),
                value: heading_font_stack,
                options: HEADING_OPTIONS.to_vec(),
            }

            FontStackPicker {
                label: "Monospace Font".to_string(),
                value: mono_font_stack,
                options: MONO_OPTIONS.to_vec(),
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
                options: WEIGHT_OPTIONS.to_vec(),
            }
        }
    }
}

const CUSTOM_SENTINEL: &str = "__custom__";

#[component]
fn FontStackPicker(
    label: String,
    value: Signal<String>,
    options: Vec<(&'static str, &'static str)>,
) -> Element {
    let mut value = value;
    let opts = options.clone();

    let current = value.read().clone();
    let selected_key = opts
        .iter()
        .find(|(_, css)| **css == current)
        .map(|_| current.clone())
        .unwrap_or_else(|| CUSTOM_SENTINEL.to_string());

    let is_custom = selected_key == CUSTOM_SENTINEL;

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
                        let existing = value.read().clone();
                        let on_preset = opts.iter().any(|(_, css)| *css == existing);
                        if on_preset {
                            value.set(String::new());
                        }
                    } else {
                        value.set(chosen);
                    }
                },

                for (name, css) in opts.iter() {
                    option {
                        value: "{css}",
                        selected: *css == selected_key.as_str(),
                        "{name}"
                    }
                }
                option {
                    value: "{CUSTOM_SENTINEL}",
                    selected: is_custom,
                    "Custom…"
                }
            }

            if is_custom {
                input {
                    r#type: "text",
                    value: "{value}",
                    placeholder: "Georgia, serif",
                    class: "editor-field",
                    oninput: move |e| value.set(e.value()),
                }
            }
        }
    }
}

#[component]
fn SimpleSelect(
    label: String,
    value: Signal<String>,
    options: Vec<(&'static str, &'static str)>,
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
