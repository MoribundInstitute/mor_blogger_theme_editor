// src/slider.rs
// MOR range slider component. Stripped internal memory. Pure controlled component.

use dioxus::prelude::*;

#[component]
pub fn Slider(
    #[props(default = None)] label: Option<&'static str>,
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = true)] show_value: bool,
    #[props(default = 0)] precision: usize,
    #[props(default = false)] disabled: bool,
    #[props(default = None)] oninput: Option<EventHandler<f64>>,
) -> Element {
    let display = format!("{:.prec$}", value, prec = precision);

    rsx! {
        div {
            class: if disabled { "mor-slider-shell disabled" } else { "mor-slider-shell" },

            if label.is_some() || show_value {
                div { class: "mor-slider-header",
                    if let Some(lbl) = label {
                        span { class: "mor-slider-label", "{lbl}" }
                    }
                    if show_value {
                        span { class: "mor-slider-value", "{display}" }
                    }
                }
            }

            input {
                class: "mor-slider-input",
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                disabled: disabled,
                oninput: move |evt: Event<FormData>| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        if let Some(h) = oninput {
                            h.call(v);
                        }
                    }
                },
            }
        }
    }
}