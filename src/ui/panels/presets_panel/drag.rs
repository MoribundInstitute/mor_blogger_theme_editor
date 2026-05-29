use dioxus::prelude::*;
use dioxus_html::HasFileData;

use crate::config::ThemeConfig;
use crate::rehydration::extract_and_decode;

pub(crate) const PRESET_FLOATING_DRAG_JS: &str = r#"
(function () {
    if (window.__morPresetFloatingDragInstalled) return;
    window.__morPresetFloatingDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.preset-floating-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, textarea, select, a, label')) return;

        const panel = bar.closest('.preset-floating-window');
        if (!panel) return;

        e.preventDefault();

        const startX = e.clientX;
        const startY = e.clientY;
        const rect = panel.getBoundingClientRect();
        const startLeft = rect.left;
        const startTop = rect.top;

        document.body.classList.add('editor-floating-dragging');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;

            const maxLeft = Math.max(0, window.innerWidth - 160);
            const maxTop = Math.max(0, window.innerHeight - 80);

            const nextLeft = Math.max(0, Math.min(startLeft + dx, maxLeft));
            const nextTop = Math.max(0, Math.min(startTop + dy, maxTop));

            document.documentElement.style.setProperty('--preset-floating-x', nextLeft + 'px');
            document.documentElement.style.setProperty('--preset-floating-y', nextTop + 'px');
        };

        const onUp = function () {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
            document.body.classList.remove('editor-floating-dragging');
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

#[component]
pub fn ThemeRestoreDropZone(
    on_restore: EventHandler<ThemeConfig>,
    on_close: EventHandler<()>,
) -> Element {
    let mut is_hovered = use_signal(|| false);
    let mut import_text = use_signal(|| "".to_string());
    let mut import_status = use_signal(|| "".to_string());

    let restore_from_xml = move |contents: String, source_label: String| {
        import_text.set(contents.clone());

        match extract_and_decode(&contents) {
            Ok(config) => {
                on_restore.call(config);
                import_status.set(format!("Workspace restored successfully from {}!", source_label));
                import_text.set("".to_string());
            }
            Err(err) => {
                import_status.set(format!("Error restoring {}: {}", source_label, err));
            }
        }
    };

    rsx! {
        div {
            class: if is_hovered() { "restore-workspace-drawer hovered" } else { "restore-workspace-drawer" },

            // Dioxus handles browser-level prevention here. Do not call evt.prevent_default()
            // inside the Rust handlers; Event<DragData> does not expose that method in this app.
            prevent_default: "ondragover ondragenter ondrop",

            ondragover: move |_evt| {
                is_hovered.set(true);
                import_status.set("Drop the XML file to restore the workspace.".to_string());
            },
            ondragenter: move |_evt| {
                is_hovered.set(true);
            },
            ondragleave: move |_| {
                is_hovered.set(false);
            },
            ondrop: {
                let mut restore_from_xml = restore_from_xml.clone();

                move |evt| {
                    async move {
                        is_hovered.set(false);
                        import_status.set("Reading dropped XML file...".to_string());

                        let Some(file_engine) = evt.files() else {
                            import_status.set(
                                "Drop detected, but this desktop webview did not expose the file. Use Choose XML instead."
                                    .to_string(),
                            );
                            return;
                        };

                        let Some(file_name) = file_engine.files().first().cloned() else {
                            import_status.set(
                                "Drop detected, but no readable file was provided. Use Choose XML instead."
                                    .to_string(),
                            );
                            return;
                        };

                        let Some(contents) = file_engine.read_file_to_string(&file_name).await else {
                            import_status.set(format!(
                                "Could not read dropped file: {}. Use Choose XML instead.",
                                file_name
                            ));
                            return;
                        };

                        restore_from_xml(contents, file_name);
                    }
                }
            },

            div {
                class: "restore-header",
                h4 { class: "restore-title", "Restore Workspace from Blogger XML" }
                button {
                    class: "editor-mini-button",
                    onclick: move |_| { on_close.call(()); },
                    "Close"
                }
            }

            p {
                class: "restore-copy",
                "Paste a previously exported Blogger XML template here, drag and drop an .xml file directly into this box, or use Choose XML."
            }

            div {
                class: "editor-row-stretch",

                textarea {
                    class: "editor-textarea restore-textarea",
                    placeholder: "Paste Blogger XML here...",
                    value: "{import_text}",
                    oninput: move |evt| {
                        import_text.set(evt.value().clone());
                        import_status.set("".to_string());
                    },
                }

                div {
                    class: "restore-button-stack",

                    label {
                        class: "editor-button restore-button",
                        "Choose XML"
                        input {
                            r#type: "file",
                            accept: ".xml,text/xml,application/xml",
                            style: "display: none;",
                            onchange: {
                                let mut restore_from_xml = restore_from_xml.clone();

                                move |evt| {
                                    async move {
                                        import_status.set("Reading selected file...".to_string());

                                        let Some(file_engine) = evt.files() else {
                                            import_status.set(
                                                "Selected file did not include readable file data."
                                                    .to_string(),
                                            );
                                            return;
                                        };

                                        let Some(file_name) = file_engine.files().first().cloned() else {
                                            import_status.set("No file was selected.".to_string());
                                            return;
                                        };

                                        let Some(contents) = file_engine
                                            .read_file_to_string(&file_name)
                                            .await
                                        else {
                                            import_status.set(format!(
                                                "Could not read selected file: {}",
                                                file_name
                                            ));
                                            return;
                                        };

                                        restore_from_xml(contents, file_name);
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "editor-button restore-button",
                        onclick: {
                            let mut restore_from_xml = restore_from_xml.clone();

                            move |_| {
                                let pasted_xml = import_text();

                                if pasted_xml.trim().is_empty() {
                                    import_status.set(
                                        "Paste exported Blogger XML before rehydrating.".to_string(),
                                    );
                                    return;
                                }

                                restore_from_xml(pasted_xml, "pasted XML".to_string());
                            }
                        },
                        "Rehydrate"
                    }
                }
            }

            if !import_status().is_empty() {
                div { class: "restore-status", "{import_status}" }
            }
        }
    }
}
