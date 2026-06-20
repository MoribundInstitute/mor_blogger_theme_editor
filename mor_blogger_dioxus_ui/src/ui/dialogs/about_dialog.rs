use crate::ui::components::modal::Modal;
use dioxus::prelude::*;

#[component]
pub fn AboutDialog(mut open: Signal<bool>) -> Element {
    rsx! {
        Modal {
            title: "About MorBlogger".to_string(),
            open: open,
            on_close: move |_| open.set(false),
            div { class: "editor-note",
                p { class: "editor-note-title", "Version 0.1.0" }
                p { class: "editor-note-body", "Frugal desktop theme engine for Blogger." }
            }
        }
    }
}
