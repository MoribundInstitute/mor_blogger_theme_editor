use dioxus::prelude::*;
use crate::config::ThemeConfig;
use crate::ui::components::inputs::{EditorCard, EditorInput};

#[component]
pub fn AssetsPanel(
    favicon_url: Signal<String>,
    social_card_image_url: Signal<String>,
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let _ = current_config;
    let _ = on_apply_theme;
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