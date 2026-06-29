use dioxus::prelude::*;

use crate::ui::components::inputs::{EditorCard, EditorInput};

const BLOGGER_SETTINGS_URL: &str = "https://www.blogger.com/blog/settings/";

#[component]
pub fn SitePanel() -> Element {
    let theme_state = use_context::<crate::app::state::ThemeState>();
    rsx! {
        EditorCard {
            title: "Site Identity".to_string(),

            EditorInput {
                label: "Site Title".to_string(),
                value: theme_state.signals.site_title,
                input_type: "text".to_string(),
                placeholder: "My Blogger Site".to_string()
            }

            EditorInput {
                label: "Site Subtitle".to_string(),
                value: theme_state.signals.site_subtitle,
                input_type: "text".to_string(),
                placeholder: "A short tagline".to_string()
            }

            EditorInput {
                label: "Header Logo URL".to_string(),
                value: theme_state.signals.header_logo_url,
                input_type: "text".to_string(),
                placeholder: "https://example.com/logo.png".to_string()
            }

            EditorInput {
                label: "Home URL".to_string(),
                value: theme_state.signals.home_url,
                input_type: "text".to_string(),
                placeholder: "/".to_string()
            }
        }

        EditorCard {
            title: "Where these live".to_string(),
            div { class: "editor-help-text", style: "margin-bottom: 8px; line-height: 1.45;",
                "These fields are a "
                b { "visual override" }
                " baked into your theme XML. They change how the site looks, not your Blogger account. "
                "Your title, domain, HTTPS, comments, and search visibility are owned by the Blogger dashboard, which stays the source of truth for all of that."
            }
            div { class: "editor-help-text", style: "margin-bottom: 8px; line-height: 1.45;",
                b { "Note:" }
                " a blog must already be created and selected in Blogger before these settings can be edited there."
            }
            button {
                class: "editor-button editor-button-small",
                title: "Open Blogger Settings in your browser",
                onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(BLOGGER_SETTINGS_URL).spawn(); },
                "Open Blogger Settings ↗"
            }
        }
    }
}
