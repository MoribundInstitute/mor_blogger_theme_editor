use dioxus::prelude::*;

use crate::ui::inputs::{EditorInput, SectionTitle};

#[component]
pub fn MenuPanel(
    menu_1_label: Signal<String>,
    menu_1_url: Signal<String>,
    menu_2_label: Signal<String>,
    menu_2_url: Signal<String>,
    menu_3_label: Signal<String>,
    menu_3_url: Signal<String>,
    menu_4_label: Signal<String>,
    menu_4_url: Signal<String>,
) -> Element {
    rsx! {
        SectionTitle { title: "Menu Builder".to_string() }

        MenuLinkEditor {
            title: "Menu Link 1".to_string(),
            label_signal: menu_1_label,
            url_signal: menu_1_url,
            label_placeholder: "Home".to_string(),
            url_placeholder: "/".to_string(),
        }

        MenuLinkEditor {
            title: "Menu Link 2".to_string(),
            label_signal: menu_2_label,
            url_signal: menu_2_url,
            label_placeholder: "Archive".to_string(),
            url_placeholder: "/p/archive.html".to_string(),
        }

        MenuLinkEditor {
            title: "Menu Link 3".to_string(),
            label_signal: menu_3_label,
            url_signal: menu_3_url,
            label_placeholder: "About".to_string(),
            url_placeholder: "/p/about.html".to_string(),
        }

        MenuLinkEditor {
            title: "Menu Link 4".to_string(),
            label_signal: menu_4_label,
            url_signal: menu_4_url,
            label_placeholder: "Contact".to_string(),
            url_placeholder: "/p/contact.html".to_string(),
        }
    }
}

#[component]
fn MenuLinkEditor(
    title: String,
    label_signal: Signal<String>,
    url_signal: Signal<String>,
    label_placeholder: String,
    url_placeholder: String,
) -> Element {
    rsx! {
        div {
            style: "padding: 10px; background: #242424; border: 1px solid #383838; margin-bottom: 10px;",

            h4 {
                style: "margin: 0 0 8px 0;",
                "{title}"
            }

            EditorInput {
                label: "Label".to_string(),
                value: label_signal,
                input_type: "text".to_string(),
                placeholder: label_placeholder
            }

            EditorInput {
                label: "URL".to_string(),
                value: url_signal,
                input_type: "text".to_string(),
                placeholder: url_placeholder
            }
        }
    }
}
