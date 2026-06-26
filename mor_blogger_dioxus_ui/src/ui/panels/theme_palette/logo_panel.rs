use dioxus::prelude::*;

use crate::ui::components::inputs::{EditorCard, EditorInput, PanelNote};

#[component]
pub fn LogoPanel(header_logo_url: Signal<String>) -> Element {
    let url = header_logo_url.read().clone();

    rsx! {
        EditorCard {
            title: "Logo".to_string(),

            EditorInput {
                label: "Logo Image URL".to_string(),
                value: header_logo_url,
                input_type: "text".to_string(),
                placeholder: "https://example.com/logo.png".to_string()
            }

            if !url.trim().is_empty() {
                div {
                    class: "editor-field-group",
                    label { class: "editor-field-label", "Preview" }
                    img {
                        src: "{url}",
                        alt: "Logo preview",
                        style: "max-width: 100%; max-height: 80px; object-fit: contain;"
                    }
                }
            }

            PanelNote {
                title: "Note".to_string(),
                body: "Paste a direct image URL (PNG, SVG, JPG). It replaces the site title text in the header.".to_string()
            }
        }
    }
}
