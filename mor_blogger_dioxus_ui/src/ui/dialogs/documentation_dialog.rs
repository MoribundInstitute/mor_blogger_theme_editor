use crate::ui::dialogs::modal::Modal;
use dioxus::prelude::*;

#[component]
pub fn DocumentationDialog(mut open: Signal<bool>) -> Element {
    rsx! {
        Modal {
            title: "Documentation".to_string(),
            open: open,
            on_close: move |_| open.set(false),
            div { class: "editor-note",
                p { class: "editor-note-title", "Online Resources" }
                p { class: "editor-note-body", "Read the architecture and integration guides in the MOR_PLAN.md at the repository root." }
            }
        }
    }
}
