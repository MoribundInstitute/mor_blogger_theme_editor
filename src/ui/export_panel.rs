use dioxus::prelude::*;

use crate::clipboard::copy_to_clipboard;
use crate::config::ThemeConfig;
use crate::diagnostics::DiagnosticResult;
use crate::ui::layout::{
    apply_preview_viewport, clamp_preview_width, rotate_preview_width, set_workbench_layout,
    PreviewTemplateMode, PreviewViewport, WorkbenchLayout,
};

#[derive(Props, Clone, PartialEq)]
pub struct ExportPanelProps {
    pub workbench_layout: Signal<WorkbenchLayout>,
    pub preview_viewport: Signal<PreviewViewport>,
    pub preview_width: Signal<u32>,
    pub preview_template_mode: Signal<PreviewTemplateMode>,

    pub generated_xml: String,
    pub preview_html: String,
    pub show_preview: Signal<bool>,
    pub diag: Signal<DiagnosticResult>,

    /// The current theme state serialized to TOML.
    pub config_toml: String,

    /// Callback triggered when a user uploads a new .toml file.
    pub on_load_theme: EventHandler<String>,

    /// Callback triggered when a user pastes an exported Blogger XML template
    /// containing an embedded/recoverable workspace config.
    pub on_restore: EventHandler<ThemeConfig>,
}

#[component]
pub fn ExportPanel(props: ExportPanelProps) -> Element {
    let ExportPanelProps {
        workbench_layout,
        preview_viewport,
        preview_width,
        preview_template_mode,
        generated_xml,
        preview_html,
        show_preview,
        diag,
        config_toml,
        on_load_theme,
        on_restore,
    } = props;

    let export_xml = crate::rehydration::inject_state(&generated_xml, &config_toml);
    let copy_xml = export_xml.clone();
    let save_toml_data = config_toml.clone();

    let is_valid = diag.read().is_valid;
    let error_count = diag.read().errors.len();

    let mut status_msg = use_signal(|| "".to_string());
    let mut import_text = use_signal(|| "".to_string());
    let mut import_status = use_signal(|| "".to_string());
    let mut show_restore = use_signal(|| false);

    let current_viewport = preview_viewport();
    let viewport_label = current_viewport.label();
    let viewport_meta = if current_viewport == PreviewViewport::Fit {
        "Fit · available width".to_string()
    } else {
        format!("{} · {}px wide", viewport_label, preview_width())
    };

    let device_class = if current_viewport == PreviewViewport::Fit {
        "preview-device-frame preview-device-frame-fit"
    } else {
        "preview-device-frame"
    };

    let device_style = if current_viewport == PreviewViewport::Fit {
        "width: 100%;".to_string()
    } else {
        format!("width: {}px;", preview_width())
    };

    rsx! {
        div {
            class: "export-panel",

            div {
                class: "export-panel-header",

                div {
                    class: "export-panel-title-block",

                    h3 {
                        class: "export-panel-title",
                        "Generated Blogger Theme"
                    }

                    span {
                        class: "preview-device-meta",
                        "{viewport_meta}"
                    }
                }

                div {
                    class: "export-toolbar export-toolbar-primary",

                    div {
                        class: "preview-toolbar-group",

                        button {
                            class: if preview_viewport() == PreviewViewport::Desktop {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Desktop);
                                apply_preview_viewport(PreviewViewport::Desktop, preview_width);
                            },
                            "Desktop"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Laptop {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Laptop);
                                apply_preview_viewport(PreviewViewport::Laptop, preview_width);
                            },
                            "Laptop"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Tablet {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Tablet);
                                apply_preview_viewport(PreviewViewport::Tablet, preview_width);
                            },
                            "Tablet"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Phone {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Phone);
                                apply_preview_viewport(PreviewViewport::Phone, preview_width);
                            },
                            "Phone"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Fit {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Fit);
                                apply_preview_viewport(PreviewViewport::Fit, preview_width);
                            },
                            "Fit"
                        }

                        button {
                            class: if preview_viewport().is_rotatable() {
                                "editor-mini-button"
                            } else {
                                "editor-mini-button editor-mini-button-disabled"
                            },
                            title: "Rotate tablet, phone, or custom preview width",
                            onclick: move |_| {
                                if preview_viewport().is_rotatable() {
                                    let next_width = rotate_preview_width(
                                        preview_viewport(),
                                        preview_width(),
                                    );
                                    let mut width = preview_width;
                                    width.set(next_width);
                                }
                            },
                            "Rotate"
                        }

                        label {
                            class: "preview-width-control",

                            span {
                                class: "preview-width-label",
                                "Width"
                            }

                            input {
                                class: "preview-width-input",
                                r#type: "number",
                                min: "240",
                                max: "2400",
                                step: "10",
                                value: "{preview_width()}",
                                oninput: move |evt| {
                                    if let Ok(width_value) = evt.value().parse::<u32>() {
                                        let mut width = preview_width;
                                        let mut viewport = preview_viewport;
                                        width.set(clamp_preview_width(width_value));
                                        viewport.set(PreviewViewport::Custom);
                                    }
                                },
                            }
                        }
                    }

                    div {
                        class: "preview-toolbar-group preview-template-mode-group",

                        span {
                            class: "preview-width-label",
                            "Layout"
                        }

                        button {
                            class: if preview_template_mode() == PreviewTemplateMode::Modern {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut mode = preview_template_mode;
                                mode.set(PreviewTemplateMode::Modern);
                            },
                            "Modern"
                        }

                        button {
                            class: if preview_template_mode() == PreviewTemplateMode::Sidebars {
                                "editor-mini-button editor-mini-button-active"
                            } else {
                                "editor-mini-button"
                            },
                            onclick: move |_| {
                                let mut mode = preview_template_mode;
                                mode.set(PreviewTemplateMode::Sidebars);
                            },
                            "Sidebars"
                        }
                    }
                }
            }

            div {
                class: "export-action-bar",

                div {
                    class: "export-action-group",

                    if workbench_layout() == WorkbenchLayout::PreviewTakeover {
                        button {
                            class: "editor-button",
                            onclick: move |_| {
                                set_workbench_layout(workbench_layout, WorkbenchLayout::Split);
                            },
                            "Open Editor"
                        }
                    } else {
                        button {
                            class: "editor-button",
                            onclick: move |_| {
                                set_workbench_layout(workbench_layout, WorkbenchLayout::PreviewTakeover);
                            },
                            "Takeover Preview"
                        }
                    }

                    button {
                        class: "editor-button",
                        onclick: move |_| {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::WideEditor);
                        },
                        "Wide Editor"
                    }
                }

                div {
                    class: "export-action-group",

                    label {
                        class: "editor-button",
                        "Load .toml"

                        input {
                            r#type: "file",
                            accept: ".toml",
                            style: "display: none;",
                            onchange: {
                                let on_load_theme = on_load_theme.clone();

                                move |evt| {
                                    let on_load_theme = on_load_theme.clone();

                                    async move {
                                        if let Some(file_engine) = evt.files() {
                                            if let Some(file_name) = file_engine.files().first() {
                                                if let Some(contents) = file_engine.read_file_to_string(file_name).await {
                                                    on_load_theme.call(contents);
                                                    status_msg.set(format!("Loaded: {}", file_name));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "editor-button",
                        onclick: {
                            let save_toml_data = save_toml_data.clone();

                            move |_| {
                                let toml_data = save_toml_data.clone();

                                async move {
                                    let eval = eval(
                                        r#"
                                        let text = await dioxus.recv();
                                        let blob = new Blob([text], { type: 'text/plain' });
                                        let url = URL.createObjectURL(blob);
                                        let a = document.createElement('a');
                                        a.href = url;
                                        a.download = 'my_theme.toml';
                                        document.body.appendChild(a);
                                        a.click();
                                        URL.revokeObjectURL(url);
                                        document.body.removeChild(a);
                                        "#
                                    );

                                    let _ = eval.send(toml_data.into());
                                    status_msg.set("Theme saved!".to_string());
                                }
                            }
                        },
                        "Save .toml"
                    }

                    button {
                        class: if show_restore() {
                            "editor-button editor-button-active"
                        } else {
                            "editor-button"
                        },
                        onclick: move |_| {
                            show_restore.set(!show_restore());
                        },
                        "Import XML"
                    }
                }

                div {
                    class: "export-action-group",

                    button {
                        class: "editor-button",
                        onclick: move |_| {
                            let mut sp = show_preview;
                            sp.set(!sp());
                        },

                        if show_preview() {
                            "Show XML"
                        } else {
                            "Show Preview"
                        }
                    }

                    if is_valid {
                        button {
                            class: "editor-button editor-button-good",
                            onclick: {
                                let copy_xml = copy_xml.clone();

                                move |_| {
                                    copy_to_clipboard(copy_xml.clone());
                                    status_msg.set("XML copied to clipboard!".to_string());
                                }
                            },
                            "Copy XML"
                        }

                        button {
                            class: "editor-button editor-button-good",
                            onclick: {
                                let export_xml = export_xml.clone();

                                move |_| {
                                    match crate::render::save_xml_to_disk(
                                        &export_xml,
                                        "Moribund_Institute",
                                    ) {
                                        Ok(msg) => {
                                            println!("{}", msg);
                                            status_msg.set(msg);
                                        }
                                        Err(err) => {
                                            let msg = format!("Export failed: {}", err);
                                            eprintln!("{}", msg);
                                            status_msg.set(msg);
                                        }
                                    }
                                }
                            },
                            "Export Theme to Disk"
                        }
                    } else {
                        button {
                            class: "editor-button editor-button-disabled",
                            title: "Fix template integrity errors before exporting",
                            "Copy XML"
                        }

                        button {
                            class: "editor-button editor-button-disabled",
                            title: "Fix template integrity errors before exporting",
                            "Export Theme to Disk"
                        }
                    }
                }
            }

            if !status_msg().is_empty() {
                div {
                    class: "export-status",
                    "{status_msg}"
                }
            }

            if !is_valid {
                div {
                    class: "export-error-banner",

                    span {
                        style: "flex-shrink: 0;",
                        "⚠"
                    }

                    span {
                        "Export disabled — {error_count} integrity error(s). Fix the template skeleton before copying."
                    }
                }
            }

            if show_preview() {
                div {
                    class: "preview-canvas",

                    div {
                        class: "preview-ruler",

                        span {
                            class: "preview-ruler-label",
                            "{viewport_meta}"
                        }

                        div {
                            class: "preview-ruler-line"
                        }
                    }

                    div {
                        class: "{device_class}",
                        style: "{device_style}",

                        // Hidden source of truth for the generated preview HTML.
                        // Dioxus updates this text node when presets/settings change;
                        // the MutationObserver below rewrites the iframe document.
                        pre {
                            id: "mor-preview-html-source",
                            style: "display: none;",
                            "{preview_html}"
                        }

                        iframe {
                            id: "mor-preview-frame",
                            class: "preview-iframe",
                            src: "about:blank",
                            onmounted: move |_| {
                                spawn(async move {
                                    let _ = eval(
                                        r#"
                                        (function installMorPreviewBridge() {
                                            const sourceId = "mor-preview-html-source";
                                            const frameId = "mor-preview-frame";

                                            function writePreview(source, frame) {
                                                const html = source.textContent || "";
                                                if (!html.trim()) return;
                                                if (source.__morLastPreviewHtml === html) return;
                                                source.__morLastPreviewHtml = html;

                                                const doc = frame.contentDocument ||
                                                    (frame.contentWindow && frame.contentWindow.document);
                                                if (!doc) return;

                                                doc.open();
                                                doc.write(html);
                                                doc.close();
                                            }

                                            function install(attempt) {
                                                const source = document.getElementById(sourceId);
                                                const frame = document.getElementById(frameId);

                                                if (!source || !frame) {
                                                    if (attempt < 40) {
                                                        setTimeout(function () { install(attempt + 1); }, 25);
                                                    }
                                                    return;
                                                }

                                                if (source.__morPreviewObserver) {
                                                    source.__morPreviewObserver.disconnect();
                                                }

                                                const observer = new MutationObserver(function () {
                                                    writePreview(source, frame);
                                                });

                                                observer.observe(source, {
                                                    childList: true,
                                                    characterData: true,
                                                    subtree: true
                                                });

                                                source.__morPreviewObserver = observer;
                                                writePreview(source, frame);
                                            }

                                            install(0);
                                        })();
                                        "#
                                    );
                                });
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "export-viewport",

                    textarea {
                        class: "export-xml-textarea",
                        readonly: true,
                        value: "{export_xml}"
                    }
                }
            }

            if show_restore() {
                div {
                    class: "restore-workspace restore-workspace-drawer",

                    div {
                        class: "restore-header",

                        h4 {
                            class: "restore-title",
                            "Restore Workspace from Blogger XML"
                        }

                        button {
                            class: "editor-mini-button",
                            onclick: move |_| {
                                show_restore.set(false);
                            },
                            "Close"
                        }
                    }

                    p {
                        class: "restore-copy",
                        "Paste a previously exported Blogger XML template here to restore your GUI state."
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

                        button {
                            class: "editor-button restore-button",
                            onclick: {
                                let on_restore = on_restore.clone();

                                move |_| {
                                    let pasted_xml = import_text();

                                    if pasted_xml.trim().is_empty() {
                                        import_status.set("Paste exported Blogger XML before rehydrating.".to_string());
                                        return;
                                    }

                                    match crate::rehydration::extract_and_decode(&pasted_xml) {
                                        Ok(config) => {
                                            on_restore.call(config);
                                            import_status.set("Workspace restored successfully.".to_string());
                                            import_text.set("".to_string());
                                        }
                                        Err(err) => {
                                            import_status.set(format!("Error: {}", err));
                                        }
                                    }
                                }
                            },
                            "Rehydrate"
                        }
                    }

                    if !import_status().is_empty() {
                        div {
                            class: "restore-status",
                            "{import_status}"
                        }
                    }
                }
            }
        }
    }
}
