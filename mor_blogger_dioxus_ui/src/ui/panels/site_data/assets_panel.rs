use crate::ui::components::inputs::{EditorCard, EditorInput};
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

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

            div { class: "editor-help-text", style: "margin-top: 8px; line-height: 1.45;",
                "Paste a "
                b { "public image URL" }
                " (the app links to images, it does not host them). Both are baked into the theme XML as a visual override, so the favicon ships with the theme and needs no Blogger upload. If you also set a favicon in Blogger's dashboard, the dashboard one can win, so leave it blank there to let the theme drive it."
            }
        }
    }
}
