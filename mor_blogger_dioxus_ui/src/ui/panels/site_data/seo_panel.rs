use dioxus::prelude::*;

use crate::ui::components::inputs::{EditorInput, PanelNote, SectionTitle};

#[component]
pub fn SeoPanel() -> Element {
    let site_state = use_context::<crate::app::state::SiteState>();
    rsx! {
        SectionTitle { title: "SEO & Site Identity".to_string() }

        EditorInput {
            label: "Meta Description".to_string(),
            value: site_state.meta_description,
            input_type: "text".to_string(),
            placeholder: "Short site description".to_string()
        }

        EditorInput {
            label: "Keywords (Comma separated)".to_string(),
            value: site_state.meta_keywords,
            input_type: "text".to_string(),
            placeholder: "blog, writing, technology".to_string()
        }

        EditorInput {
            label: "Robots (Search Engine Rules)".to_string(),
            value: site_state.custom_robots,
            input_type: "text".to_string(),
            placeholder: "index, follow".to_string()
        }

        EditorInput {
            label: "Author Name".to_string(),
            value: site_state.author_name,
            input_type: "text".to_string(),
            placeholder: "Author name".to_string()
        }

        EditorInput {
            label: "License URL".to_string(),
            value: site_state.license_url,
            input_type: "text".to_string(),
            placeholder: "https://example.com/license".to_string()
        }

        PanelNote {
            title: "SEO Note".to_string(),
            body: "Leave Robots as 'index, follow' unless you explicitly want to hide this blog from search engines (use 'noindex, nofollow').".to_string()
        }
    }
}

#[component]
pub fn FooterPanel(
    footer_text: Signal<String>,
    footer_license_label: Signal<String>,
    footer_license_url: Signal<String>,
) -> Element {
    rsx! {
        SectionTitle { title: "Footer".to_string() }

        EditorInput {
            label: "Footer Text".to_string(),
            value: footer_text,
            input_type: "text".to_string(),
            placeholder: "Powered by Blogger.".to_string()
        }

        EditorInput {
            label: "Footer License Label".to_string(),
            value: footer_license_label,
            input_type: "text".to_string(),
            placeholder: "License".to_string()
        }

        EditorInput {
            label: "Footer License URL".to_string(),
            value: footer_license_url,
            input_type: "text".to_string(),
            placeholder: "https://example.com/license".to_string()
        }
    }
}
