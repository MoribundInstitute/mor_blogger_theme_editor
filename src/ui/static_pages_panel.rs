use dioxus::prelude::*;

use crate::config::StaticPagesConfig;
use crate::ui::inputs::EditorCard;

#[component]
pub fn StaticPagesPanel(static_pages: Signal<StaticPagesConfig>) -> Element {
    rsx! {
        EditorCard {
            title: "Static Page Settings".to_string(),

            div {
                class: "editor-note",
                p {
                    class: "editor-note-body",
                    "These settings generate the HTML for your standalone Blogger 'Archive' and 'Categories' pages. By syncing with the global theme, they will automatically inherit your chosen colors, fonts, and borders without requiring extra CSS."
                }
            }

            div {
                class: "editor-field-group",
                label {
                    class: "editor-field-label",
                    style: "display: flex; align-items: center; gap: 8px; cursor: pointer; margin-top: 10px;",
                    input {
                        r#type: "checkbox",
                        checked: "{static_pages().sync_with_global_theme}",
                        onchange: move |evt| {
                            let mut config = static_pages();
                            config.sync_with_global_theme = evt.value() == "true";
                            static_pages.set(config);
                        }
                    }
                    "Sync styling with global theme"
                }
            }
        }

        EditorCard {
            title: "Archive Page Generator".to_string(),

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Kicker" }
                input {
                    class: "editor-field",
                    value: "{static_pages().archive.kicker}",
                    placeholder: "e.g., THE_MORIBUND_INSTITUTE // ARCHIVE",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        config.archive.kicker = evt.value();
                        static_pages.set(config);
                    }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Page Title" }
                input {
                    class: "editor-field",
                    value: "{static_pages().archive.title}",
                    placeholder: "e.g., Chronological Archive",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        config.archive.title = evt.value();
                        static_pages.set(config);
                    }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Description" }
                textarea {
                    class: "editor-textarea",
                    rows: "3",
                    value: "{static_pages().archive.description}",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        config.archive.description = evt.value();
                        static_pages.set(config);
                    }
                }
            }
        }

        EditorCard {
            title: "Categories Page Generator".to_string(),

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Kicker" }
                input {
                    class: "editor-field",
                    value: "{static_pages().categories.kicker}",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        config.categories.kicker = evt.value();
                        static_pages.set(config);
                    }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Page Title" }
                input {
                    class: "editor-field",
                    value: "{static_pages().categories.title}",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        config.categories.title = evt.value();
                        static_pages.set(config);
                    }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Description" }
                textarea {
                    class: "editor-textarea",
                    rows: "3",
                    value: "{static_pages().categories.description}",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        config.categories.description = evt.value();
                        static_pages.set(config);
                    }
                }
            }

            div {
                class: "editor-field-group",
                label { class: "editor-field-label", "Dewey Sections (One per line)" }
                div {
                    class: "editor-helper-text",
                    style: "margin-bottom: 4px;",
                    "Determines which category grids are rendered. Must match your actual Blogger labels."
                }
                textarea {
                    class: "editor-textarea",
                    rows: "7",
                    style: "font-family: var(--font-mono); font-size: 0.85rem;",
                    value: "{static_pages().categories.enabled_sections.join(\"\n\")}",
                    oninput: move |evt| {
                        let mut config = static_pages();
                        // Split by newline, trim whitespace, and filter out empty lines
                        config.categories.enabled_sections = evt.value()
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        static_pages.set(config);
                    }
                }
            }
        }
    }
}