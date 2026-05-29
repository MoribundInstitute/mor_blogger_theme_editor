use dioxus::prelude::*;

use crate::rehydration::extract_and_decode;
use crate::ui::panels::presets_panel::ThemeSignals;

pub fn use_restore_drop_bridge(
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
) {
    use_effect(move || {
        let mut eval = eval(
            r#"
            (function () {
                if (window.__morRestoreXmlNativeDropInstalled) {
                    dioxus.send({
                        kind: "restore_drop_ready",
                        message: "Restore XML drop bridge already installed."
                    });
                    return;
                }

                window.__morRestoreXmlNativeDropInstalled = true;

                function hasFileDrag(event) {
                    if (!event.dataTransfer) return false;

                    if (event.dataTransfer.types) {
                        for (const item of event.dataTransfer.types) {
                            if (item === "Files") return true;
                        }
                    }

                    return event.dataTransfer.files && event.dataTransfer.files.length > 0;
                }

                function findXmlFile(files) {
                    for (const file of files) {
                        const name = (file.name || "").toLowerCase();
                        const type = (file.type || "").toLowerCase();

                        if (
                            name.endsWith(".xml") ||
                            type === "text/xml" ||
                            type === "application/xml"
                        ) {
                            return file;
                        }
                    }

                    return files.length > 0 ? files[0] : null;
                }

                window.addEventListener("dragover", function (event) {
                    if (!hasFileDrag(event)) return;

                    event.preventDefault();
                    if (event.dataTransfer) {
                        event.dataTransfer.dropEffect = "copy";
                    }
                }, true);

                window.addEventListener("dragenter", function (event) {
                    if (!hasFileDrag(event)) return;

                    event.preventDefault();
                    if (event.dataTransfer) {
                        event.dataTransfer.dropEffect = "copy";
                    }
                }, true);

                window.addEventListener("drop", async function (event) {
                    if (!hasFileDrag(event)) return;

                    event.preventDefault();

                    const files = Array.from(event.dataTransfer.files || []);
                    const file = findXmlFile(files);

                    if (!file) {
                        dioxus.send({
                            kind: "restore_drop_error",
                            message: "Drop detected, but no file was exposed by the desktop webview."
                        });
                        return;
                    }

                    try {
                        const text = await file.text();

                        dioxus.send({
                            kind: "restore_xml_drop",
                            name: file.name || "dropped XML",
                            text: text
                        });
                    } catch (error) {
                        dioxus.send({
                            kind: "restore_drop_error",
                            message: "Could not read dropped file: " + String(error)
                        });
                    }
                }, true);

                dioxus.send({
                    kind: "restore_drop_ready",
                    message: "Restore XML drop bridge installed."
                });
            })();
            "#,
        );

        let restore_signals = signals;
        let mut restored_active_preset = active_preset;

        spawn(async move {
            while let Ok(value) = eval.recv().await {
                let Some(kind) = value.get("kind").and_then(|v| v.as_str()) else {
                    continue;
                };

                match kind {
                    "restore_xml_drop" => {
                        let name = value
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("dropped XML");

                        let Some(xml_text) = value.get("text").and_then(|v| v.as_str()) else {
                            log::error!("Dropped XML event from desktop bridge did not include text.");
                            continue;
                        };

                        match extract_and_decode(xml_text) {
                            Ok(config) => {
                                restore_signals.apply_config(&config);
                                restored_active_preset.set(None);
                                log::info!("Workspace restored from dropped XML file: {}", name);
                            }
                            Err(err) => {
                                log::error!("Failed to restore dropped XML file {}: {}", name, err);
                            }
                        }
                    }
                    "restore_drop_error" => {
                        let message = value
                            .get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown restore drop error.");
                        log::error!("{}", message);
                    }
                    "restore_drop_ready" => {
                        let message = value
                            .get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Restore XML drop bridge ready.");
                        log::info!("{}", message);
                    }
                    _ => {}
                }
            }
        });
    });
}
