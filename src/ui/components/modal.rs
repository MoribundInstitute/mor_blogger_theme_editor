// src/modal.rs
// MOR modal component. Stripped default prop string allocation bloat.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub open: Signal<bool>,
    pub title: String,
    pub children: Element,
    #[props(default = None)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default = String::new())]
    pub style: String,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut open = props.open;
    let on_close = props.on_close;

    if !open() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            class: "mor-modal-backdrop",
            onclick: move |_| {
                if let Some(h) = on_close { h.call(()); }
                open.set(false);
            },

            div {
                class: "mor-modal",
                // Base width injected here. User style appends without heap allocation at mount.
                style: "min-width:380px; max-width:620px; {props.style}",
                onclick: |e| e.stop_propagation(),

                div { class: "mor-modal-header",
                    span { "{props.title}" }
                    div {
                        class: "mor-modal-close",
                        onclick: move |e| {
                            e.stop_propagation();
                            if let Some(h) = on_close { h.call(()); }
                            open.set(false);
                        },
                        "×"
                    }
                }

                div { class: "mor-modal-body",
                    {props.children}
                }
            }
        }
    }
}