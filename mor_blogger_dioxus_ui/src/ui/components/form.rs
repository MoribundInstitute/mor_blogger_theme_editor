// src/form.rs
// Flat form controls. Native HTML elements wrapped in Dioxus. No bloat.

use dioxus::prelude::*;

#[component]
pub fn MorCheckbox(label: String, checked: bool, onchange: EventHandler<bool>) -> Element {
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
