use dioxus::prelude::*;
use dioxus_html::HasFileData;

use crate::ui::components::inputs::EditorCard;

#[component]
pub fn PluginsPanel(mut custom_js: Signal<String>) -> Element {
    let has_js = !custom_js().trim().is_empty();

    rsx! {
        EditorCard {
            title: "Optional Plugins / JavaScript".to_string(),

            div {
                class: "editor-note",

                p {
                    class: "editor-note-body",
                    "Drop a .js file here, or paste JavaScript below. Plain JavaScript will be wrapped in Blogger CDATA before export. Already-wrapped <script> blocks are preserved."
                }
            }

            div {
                class: "editor-dropzone",
                prevent_default: "ondragover ondrop",

                ondragover: move |_| {},

                ondrop: move |event| async move {
                    if let Some(file_engine) = event.files() {
                        let file_names = file_engine.files();

                        for file_name in file_names {
                            if !file_name.to_lowercase().ends_with(".js") {
                                log::warn!("Skipped non-JavaScript file: {}", file_name);
                                continue;
                            }

                            match file_engine.read_file_to_string(&file_name).await {
                                Some(contents) => {
                                    custom_js.set(contents);
                                }
                                None => {
                                    log::warn!("Failed to read file: {}", file_name);
                                }
                            }
                        }
                    }
                },

                if has_js {
                    "JavaScript loaded. Drop another .js file to replace it."
                } else {
                    "Drop .js file here"
                }
            }

            label {
                class: "editor-field-label",
                "Custom JavaScript"
            }

            textarea {
                value: "{custom_js}",
                placeholder: "// Optional custom JavaScript",
                class: "editor-textarea",
                style: "min-height: 180px; resize: vertical; margin-top: 6px; font-family: 'Courier New', Courier, monospace;",
                oninput: move |event| {
                    custom_js.set(event.value());
                }
            }

            div {
                class: "editor-helper-row",

                span {
                    class: "editor-helper-text",
                    if has_js {
                        "Custom JavaScript will be inserted before </body> in the exported Blogger XML."
                    } else {
                        "No custom JavaScript will be injected."
                    }
                }

                button {
                    class: "editor-button editor-button-small editor-button-danger",
                    onclick: move |_| {
                        custom_js.set(String::new());
                    },
                    "Clear JavaScript"
                }
            }
        }
    }
}