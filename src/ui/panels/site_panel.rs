use dioxus::prelude::*;

use crate::ui::components::inputs::{EditorCard, EditorInput};

#[component]
pub fn SitePanel(
    site_title: Signal<String>,
    site_subtitle: Signal<String>,
    header_logo_url: Signal<String>,
    home_url: Signal<String>,
) -> Element {
    rsx! {
        EditorCard {
            title: "Site Identity".to_string(),

            EditorInput {
                label: "Site Title".to_string(),
                value: site_title,
                input_type: "text".to_string(),
                placeholder: "My Blogger Site".to_string()
            }

            EditorInput {
                label: "Site Subtitle".to_string(),
                value: site_subtitle,
                input_type: "text".to_string(),
                placeholder: "A short tagline".to_string()
            }

            EditorInput {
                label: "Header Logo URL".to_string(),
                value: header_logo_url,
                input_type: "text".to_string(),
                placeholder: "https://example.com/logo.png".to_string()
            }

            EditorInput {
                label: "Home URL".to_string(),
                value: home_url,
                input_type: "text".to_string(),
                placeholder: "/".to_string()
            }
        }
    }
}