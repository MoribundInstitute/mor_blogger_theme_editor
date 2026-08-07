//! Schema-driven property sheet for one `<b:widget>` block (C5). All XML
//! mutation goes through `mor_blogger_core::surgery` — this form never touches
//! the string itself. Local draft state; nothing propagates until Apply.

use dioxus::prelude::*;
use mor_blogger_core::schema::{SettingKind, WidgetSchema};
use mor_blogger_core::surgery;
use std::collections::HashMap;

use super::inputs::{ColorInput, EditorCheckbox, EditorSelect, EditorTextField};

/// Current values for every schema setting: what the XML holds, or the
/// schema default for settings the widget doesn't declare yet.
fn seed(schema: &WidgetSchema, xml: &str) -> HashMap<String, String> {
    schema
        .settings
        .iter()
        .map(|s| {
            let v = surgery::get_widget_setting(xml, &s.name).unwrap_or_else(|| s.default_text());
            (s.name.clone(), v)
        })
        .collect()
}

#[derive(Props, Clone, PartialEq)]
pub struct WidgetPropertyFormProps {
    pub schema: WidgetSchema,
    /// One widget block (`<b:widget ...>...</b:widget>`), not a whole document.
    pub xml: String,
    /// Fired with the patched block on Apply.
    pub on_apply: EventHandler<String>,
}

#[component]
pub fn WidgetPropertyForm(props: WidgetPropertyFormProps) -> Element {
    let init_schema = props.schema.clone();
    let init_xml = props.xml.clone();
    let mut values = use_signal(move || seed(&init_schema, &init_xml));
    // What the XML actually holds — Apply only writes settings that differ, so
    // untouched defaults never get inserted into the widget.
    let base_schema = props.schema.clone();
    let base_xml = props.xml.clone();
    let mut baseline = use_signal(move || seed(&base_schema, &base_xml));

    // Re-seed when the caller hands us a different widget (or the applied
    // block comes back), so the draft always starts from reality.
    let xml_dep = props.xml.clone();
    let schema_dep = props.schema.clone();
    use_effect(use_reactive!(|xml_dep, schema_dep| {
        let seeded = seed(&schema_dep, &xml_dep);
        baseline.set(seeded.clone());
        values.set(seeded);
    }));

    let dirty = values() != baseline();

    let apply_schema = props.schema.clone();
    let apply_xml = props.xml.clone();
    let on_apply = props.on_apply;
    let apply = move |_| {
        let mut xml = apply_xml.clone();
        let vals = values.peek();
        let base = baseline.peek();
        for s in &apply_schema.settings {
            let Some(v) = vals.get(&s.name) else { continue };
            if base.get(&s.name) == Some(v) {
                continue;
            }
            match surgery::set_widget_setting(&xml, &s.name, v) {
                Ok(patched) => xml = patched,
                Err(err) => log::error!("widget-setting surgery failed for '{}': {err}", s.name),
            }
        }
        on_apply.call(xml);
    };

    rsx! {
        div {
            class: "widget-property-form",
            style: "display: flex; flex-direction: column; gap: 6px; padding: 8px 10px;",

            for s in props.schema.settings.clone() {
                {
                    let name = s.name.clone();
                    let current = values().get(&name).cloned().unwrap_or_default();
                    match s.kind {
                        SettingKind::Boolean => rsx! {
                            EditorCheckbox {
                                label: s.label.clone(),
                                checked: current == "true",
                                onchange: move |on: bool| {
                                    values.write().insert(name.clone(), on.to_string());
                                },
                            }
                        },
                        SettingKind::Color => rsx! {
                            div {
                                class: "editor-field-group",
                                label { class: "editor-field-label", "{s.label}" }
                                ColorInput {
                                    value: current,
                                    swatches: false,
                                    oninput: move |v: String| {
                                        values.write().insert(name.clone(), v);
                                    },
                                }
                            }
                        },
                        SettingKind::Select => rsx! {
                            EditorSelect {
                                label: s.label.clone(),
                                value: current,
                                options: s.options.clone(),
                                onchange: move |e: Event<FormData>| {
                                    values.write().insert(name.clone(), e.value());
                                },
                            }
                        },
                        SettingKind::Text => rsx! {
                            EditorTextField {
                                label: s.label.clone(),
                                value: current,
                                placeholder: s.default_text(),
                                oninput: move |v: String| {
                                    values.write().insert(name.clone(), v);
                                },
                            }
                        },
                    }
                }
            }

            button {
                class: if dirty { "editor-button" } else { "editor-button editor-mini-button-disabled" },
                style: "align-self: flex-end; margin-top: 2px;",
                disabled: !dirty,
                onclick: apply,
                "Apply"
            }
        }
    }
}
