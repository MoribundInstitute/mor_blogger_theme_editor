use dioxus::prelude::*;

use crate::clipboard::copy_to_clipboard;
use crate::config::pages::StaticPagesConfig;
use crate::config::styling::ColorConfig;
use crate::render::pages::{
    generate_about_html, generate_archive_html, generate_categories_html,
    generate_course_catalog_html, generate_portfolio_html, generate_syllabus_html,
};
use crate::ui::presets_panel::ThemeSignals;

#[component]
pub fn StaticPagesPanel(signals: ThemeSignals) -> Element {
    let pages_config = signals.static_pages;
    let status_msg = use_signal(String::new);
    let mut active_tab = use_signal(|| "Archive");

    let current_colors = ColorConfig {
        bg_base: (signals.bg_base)(),
        bg_panel: (signals.bg_panel)(),
        bg_elevated: (signals.bg_elevated)(),
        fg_base: (signals.fg_base)(),
        fg_muted: (signals.fg_muted)(),
        accent: (signals.accent)(),
        border: (signals.border)(),
    };

    rsx! {
        div {
            class: "editor-panel",

            div {
                class: "editor-help-text",
                "Select a page template to generate its HTML. Paste this directly into Blogger's Pages editor (HTML View) to automatically match your active theme colors."
            }

            // Tab Navigation
            div {
                style: "display: flex; gap: 8px; margin: 20px 0; border-bottom: 1px solid var(--border-color); padding-bottom: 12px; overflow-x: auto;",
                button { class: "editor-button", onclick: move |_| active_tab.set("Archive"), "Archive" }
                button { class: "editor-button", onclick: move |_| active_tab.set("Directory"), "Directory" }
                button { class: "editor-button", onclick: move |_| active_tab.set("About"), "About Me" }
                button { class: "editor-button", onclick: move |_| active_tab.set("Portfolio"), "Portfolio" }
                button { class: "editor-button", onclick: move |_| active_tab.set("LMS"), "Courses" }
            }

            // Active Builder Canvas
            match active_tab() {
                "Archive" => rsx! { ArchiveBuilder { config: pages_config, colors: current_colors.clone(), status: status_msg } },
                "Directory" => rsx! { CategoriesBuilder { config: pages_config, colors: current_colors.clone(), status: status_msg } },
                "About" => rsx! { AboutBuilder { config: pages_config, colors: current_colors.clone(), status: status_msg } },
                "Portfolio" => rsx! { PortfolioBuilder { config: pages_config, colors: current_colors.clone(), status: status_msg } },
                "LMS" => rsx! { LmsBuilder { config: pages_config, colors: current_colors.clone(), status: status_msg } },
                _ => rsx! { div {} }
            }

            if !status_msg().is_empty() {
                div {
                    class: "export-status",
                    style: "margin-top: 15px; color: #3fb950; font-weight: bold;",
                    "{status_msg}"
                }
            }
        }
    }
}

// ---------------------------------------------------------
// Sub-Components for each Builder Type
// ---------------------------------------------------------

#[component]
fn ArchiveBuilder(
    config: Signal<StaticPagesConfig>,
    colors: ColorConfig,
    status: Signal<String>,
) -> Element {
    let mut config_sig = config;
    let html = generate_archive_html(&colors, &config_sig().archive);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Archive Page Settings" }
            label {
                span { class: "editor-label-text", "Title" }
                input {
                    class: "editor-input", r#type: "text", value: "{config_sig().archive.title}",
                    oninput: move |evt| {
                        let mut c = config_sig(); c.archive.title = evt.value().clone(); config_sig.set(c);
                    }
                }
            }
            button {
                class: "editor-button",
                onclick: move |_| {
                    copy_to_clipboard(html.clone());
                    status.set("Archive HTML copied to clipboard!".to_string());
                },
                "Copy Archive HTML"
            }
        }
    }
}

#[component]
fn CategoriesBuilder(
    config: Signal<StaticPagesConfig>,
    colors: ColorConfig,
    status: Signal<String>,
) -> Element {
    let mut config_sig = config;
    let html = generate_categories_html(&colors, &config_sig().categories);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Directory Settings" }
            label {
                span { class: "editor-label-text", "Title" }
                input {
                    class: "editor-input", r#type: "text", value: "{config_sig().categories.title}",
                    oninput: move |evt| {
                        let mut c = config_sig(); c.categories.title = evt.value().clone(); config_sig.set(c);
                    }
                }
            }
            button {
                class: "editor-button",
                onclick: move |_| {
                    copy_to_clipboard(html.clone());
                    status.set("Directory HTML copied to clipboard!".to_string());
                },
                "Copy Directory HTML"
            }
        }
    }
}

#[component]
fn AboutBuilder(
    config: Signal<StaticPagesConfig>,
    colors: ColorConfig,
    status: Signal<String>,
) -> Element {
    let mut config_sig = config;
    let html = generate_about_html(&colors, &config_sig().about);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Profile & About Settings" }
            label {
                span { class: "editor-label-text", "Profile Image URL" }
                input {
                    class: "editor-input", r#type: "text", value: "{config_sig().about.profile_image_url}",
                    oninput: move |evt| {
                        let mut c = config_sig(); c.about.profile_image_url = evt.value().clone(); config_sig.set(c);
                    }
                }
            }
            label {
                span { class: "editor-label-text", "Biography" }
                textarea {
                    class: "editor-textarea", rows: 4, value: "{config_sig().about.bio_text}",
                    oninput: move |evt| {
                        let mut c = config_sig(); c.about.bio_text = evt.value().clone(); config_sig.set(c);
                    }
                }
            }
            button {
                class: "editor-button",
                onclick: move |_| {
                    copy_to_clipboard(html.clone());
                    status.set("About HTML copied to clipboard!".to_string());
                },
                "Copy About HTML"
            }
        }
    }
}

#[component]
fn PortfolioBuilder(
    config: Signal<StaticPagesConfig>,
    colors: ColorConfig,
    status: Signal<String>,
) -> Element {
    let mut config_sig = config;
    let html = generate_portfolio_html(&colors, &config_sig().portfolio);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Art Portfolio Settings" }
            label {
                span { class: "editor-label-text", "Gallery Title" }
                input {
                    class: "editor-input", r#type: "text", value: "{config_sig().portfolio.title}",
                    oninput: move |evt| {
                        let mut c = config_sig(); c.portfolio.title = evt.value().clone(); config_sig.set(c);
                    }
                }
            }
            button {
                class: "editor-button",
                onclick: move |_| {
                    copy_to_clipboard(html.clone());
                    status.set("Portfolio HTML copied to clipboard!".to_string());
                },
                "Copy Portfolio HTML"
            }
        }
    }
}

#[component]
fn LmsBuilder(
    config: Signal<StaticPagesConfig>,
    colors: ColorConfig,
    status: Signal<String>,
) -> Element {
    let mut config_sig = config;
    let catalog_html = generate_course_catalog_html(&colors, &config_sig().lms);
    let syllabus_html = generate_syllabus_html(&colors, &config_sig().lms);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Learning Management System" }
            label {
                span { class: "editor-label-text", "Course Title" }
                input {
                    class: "editor-input", r#type: "text", value: "{config_sig().lms.course_title}",
                    oninput: move |evt| {
                        let mut c = config_sig(); c.lms.course_title = evt.value().clone(); config_sig.set(c);
                    }
                }
            }
            div {
                style: "display: flex; gap: 12px; margin-top: 16px;",
                button {
                    class: "editor-button",
                    onclick: move |_| {
                        copy_to_clipboard(catalog_html.clone());
                        status.set("Course Catalog HTML copied to clipboard!".to_string());
                    },
                    "Copy Master Catalog"
                }
                button {
                    class: "editor-button",
                    onclick: move |_| {
                        copy_to_clipboard(syllabus_html.clone());
                        status.set("Course Syllabus HTML copied to clipboard!".to_string());
                    },
                    "Copy Syllabus Page"
                }
            }
        }
    }
}
