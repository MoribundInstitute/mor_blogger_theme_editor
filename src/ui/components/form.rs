// src/form.rs
// Flat form controls. Native HTML elements wrapped in Dioxus. No bloat.

use dioxus::prelude::*;

#[component]
pub fn MorCheckbox(
    label: String,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        label {
            class: "mor-checkbox-wrapper",
            input {
                r#type: "checkbox",
                class: "mor-checkbox",
                checked: "{checked}",
                onchange: move |evt| onchange.call(evt.value() == "true")
            }
            span { class: "mor-checkbox-label", "{label}" }
        }
    }
}

#[component]
pub fn MorSelect(
    label: String,
    value: String,
    options: Vec<String>,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "mor-select-wrapper",
            div { class: "mor-select-label", "{label}" }
            select {
                class: "mor-select",
                value: "{value}",
                onchange: move |evt| onchange.call(evt.value()),
                for opt in &options {
                    option { value: "{opt}", "{opt}" }
                }
            }
        }
    }
}

#[component]
pub fn MorTextInput(
    label: String,
    value: String,
    #[props(default = false)] is_password: bool,
    onchange: EventHandler<String>,
) -> Element {
    let input_type = if is_password { "password" } else { "text" };
    
    rsx! {
        div { class: "mor-input-wrapper",
            div { class: "mor-input-label", "{label}" }
            input {
                class: "mor-input",
                r#type: "{input_type}",
                value: "{value}",
                oninput: move |evt| onchange.call(evt.value())
            }
        }
    }
}

#[component]
pub fn MorTabs(
    tabs: Vec<String>,
    active: String,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "mor-tabs",
            // Iterate over reference. Kills heap allocation bloat.
            for tab in &tabs {
                button {
                    class: if *tab == active { "mor-tab active" } else { "mor-tab" },
                    onclick: {
                        let tab_val = tab.clone();
                        move |_| onchange.call(tab_val.clone())
                    },
                    "{tab}"
                }
            }
        }
    }
}