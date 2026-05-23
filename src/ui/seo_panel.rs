use dioxus::prelude::*;

use crate::ui::inputs::{EditorInput, SectionTitle, PanelNote};

#[component]
pub fn SeoPanel(
    meta_description: Signal<String>,
    meta_keywords: Signal<String>,
    custom_robots: Signal<String>,
    author_name: Signal<String>,
    license_url: Signal<String>,
) -> Element {
    rsx! {
        SectionTitle { title: "SEO & Site Identity".to_string() }

        EditorInput {
            label: "Meta Description".to_string(),
            value: meta_description,
            input_type: "text".to_string(),
            placeholder: "Short site description".to_string()
        }

        EditorInput {
            label: "Keywords (Comma separated)".to_string(),
            value: meta_keywords,
            input_type: "text".to_string(),
            placeholder: "blog, writing, technology".to_string()
        }

        EditorInput {
            label: "Robots (Search Engine Rules)".to_string(),
            value: custom_robots,
            input_type: "text".to_string(),
            placeholder: "index, follow".to_string()
        }

        EditorInput {
            label: "Author Name".to_string(),
            value: author_name,
            input_type: "text".to_string(),
            placeholder: "Author name".to_string()
        }

        EditorInput {
            label: "License URL".to_string(),
            value: license_url,
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