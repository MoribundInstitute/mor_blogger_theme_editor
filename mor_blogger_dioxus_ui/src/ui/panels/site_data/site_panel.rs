use dioxus::prelude::*;

use crate::ui::components::inputs::{EditorCard, EditorInput};

#[component]
pub fn SitePanel() -> Element {
    let site_state = use_context::<crate::app::state::SiteState>();
    rsx! {
        EditorCard {
            title: "Site Identity".to_string(),

            EditorInput {
                label: "Site Title".to_string(),
                value: site_state.site_title,
                input_type: "text".to_string(),
                placeholder: "My Blogger Site".to_string()
            }

            EditorInput {
                label: "Site Subtitle".to_string(),
                value: site_state.site_subtitle,
                input_type: "text".to_string(),
                placeholder: "A short tagline".to_string()
            }

            EditorInput {
                label: "Header Logo URL".to_string(),
                value: site_state.header_logo_url,
                input_type: "text".to_string(),
                placeholder: "https://example.com/logo.png".to_string()
            }

            EditorInput {
                label: "Home URL".to_string(),
                value: site_state.home_url,
                input_type: "text".to_string(),
                placeholder: "/".to_string()
            }
        }
    }
}
