use dioxus::html::HasFileData;
use dioxus::prelude::*;

use crate::ui::components::code_editor::CodeEditor;
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
                // Handle default prevention directly in Rust, not HTML strings
                ondragover: move |evt| evt.prevent_default(),
                ondrop: move |event| async move {
                    event.prevent_default();
                    for file in event.files() {
                        let file_name = file.name();
                        if !file_name.to_lowercase().ends_with(".js") {
                            log::warn!("Skipped non-JavaScript file: {}", file_name);
                            continue;
                        }

                        // Read raw bytes directly. Zero intermediate engine slop.
                        if let Ok(bytes) = file.read_bytes().await {
                            let contents = String::from_utf8_lossy(&bytes).into_owned();
                            custom_js.set(contents);
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

            div {
                style: "min-height: 180px; margin-top: 6px; border: 1px solid var(--editor-border-soft); border-radius: 4px; overflow: hidden;",
                CodeEditor {
                    value: (custom_js)(),
                    mode: "javascript".to_string(),
                    minimap_key: Some("plugins_custom_js".to_string()),
                    on_change: move |new_val| {
                        custom_js.set(new_val);
                    }
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
