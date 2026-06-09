use dioxus::prelude::*;

#[component]
pub fn EditorAccordion(
    id: &'static str,
    title: &'static str,
    active: Signal<&'static str>,
    children: Element,
) -> Element {
    let mut open = active;
    let is_open = open() == id;

    rsx! {
        div {
            class: "editor-accordion",

            button {
                class: if is_open { "editor-accordion-header is-open" } else { "editor-accordion-header" },
                onclick: move |_| {
                    if is_open {
                        open.set("");
                    } else {
                        open.set(id);
                    }
                },

                span {
                    class: "editor-accordion-title",
                    "{title}"
                }

                span {
                    class: "editor-accordion-icon",
                    if is_open { "−" } else { "+" }
                }
            }

            if is_open {
                div {
                    class: "editor-accordion-body",
                    {children}
                }
            }
        }
    }
}
