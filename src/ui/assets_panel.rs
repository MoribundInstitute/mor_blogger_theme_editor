use dioxus::prelude::*;

use crate::ui::inputs::{EditorCard, EditorInput};

#[component]
pub fn AssetsPanel(favicon_url: Signal<String>, social_card_image_url: Signal<String>) -> Element {
    rsx! {
        EditorCard {
            title: "Media / Assets".to_string(),

            EditorInput {
                label: "Favicon URL".to_string(),
                value: favicon_url,
                input_type: "text".to_string(),
                placeholder: "https://example.com/favicon.png".to_string()
            }

            EditorInput {
                label: "Social Card Image URL".to_string(),
                value: social_card_image_url,
                input_type: "text".to_string(),
                placeholder: "https://example.com/social-card.png".to_string()
            }
        }
    }
}
