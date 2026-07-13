use dioxus::prelude::*;

#[component]
pub fn EditorCard(title: String, children: Element) -> Element {
    rsx! {
        section {
            class: "editor-card",
            SectionTitle { title }
            {children}
        }
    }
}

#[component]
pub fn EditorInput(
    label: String,
    value: Signal<String>,
    input_type: String,
    placeholder: String,
) -> Element {
    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            input {
                r#type: "{input_type}",
                value: "{value}",
                placeholder: "{placeholder}",
                class: if input_type == "color" { "editor-field editor-color-field" } else { "editor-field" },
                oninput: move |e| value.set(e.value())
            }
        }
    }
}

#[component]
pub fn EditorSelect(
    label: String,
    value: String,
    options: Vec<(String, String)>,
    onchange: EventHandler<Event<FormData>>,
) -> Element {
    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            select {
                class: "editor-select",
                value: "{value}",
                onchange: move |e| onchange.call(e),

                for (opt_val, opt_label) in options {
                    option {
                        value: "{opt_val}",
                        selected: opt_val == value,
                        "{opt_label}"
                    }
                }
            }
        }
    }
}

/// Callback-flavored text field for state that lives inside a struct signal
/// (where [`EditorInput`]'s `Signal<String>` binding can't reach).
#[component]
pub fn EditorTextField(
    label: String,
    value: String,
    placeholder: String,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            input {
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                class: "editor-field",
                oninput: move |e| oninput.call(e.value())
            }
        }
    }
}

#[component]
pub fn EditorCheckbox(label: String, checked: bool, onchange: EventHandler<bool>) -> Element {
    rsx! {
        label {
            style: "display: flex; align-items: center; gap: 8px; font-size: 0.8rem; color: var(--fg-base); cursor: pointer;",
            input {
                r#type: "checkbox",
                checked: checked,
                onchange: move |e| onchange.call(e.checked()),
            }
            "{label}"
        }
    }
}

#[component]
pub fn SectionTitle(title: String) -> Element {
    rsx! {
        h3 {
            class: "editor-card-title",
            "{title}"
        }
    }
}

#[component]
pub fn PanelNote(title: String, body: String) -> Element {
    rsx! {
        div {
            class: "editor-note",

            h4 {
                class: "editor-note-title",
                "{title}"
            }

            p {
                class: "editor-note-body",
                "{body}"
            }
        }
    }
}
