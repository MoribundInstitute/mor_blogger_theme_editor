use dioxus::prelude::*;
use crate::clipboard::copy_to_clipboard;
use crate::config::ThemeConfig;
use crate::diagnostics::DiagnosticResult;
use crate::ui::workspace::layout::{
    apply_preview_viewport, clamp_preview_width, rotate_preview_width, PreviewTemplateMode,
    PreviewViewport,
};
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use crate::ui::panels::presets_panel::ThemeRestoreDropZone;

#[component]
pub fn CenterWorkspacePanel(
    preview_viewport: Signal<PreviewViewport>,
    preview_width: Signal<u32>,
    preview_template_mode: Signal<PreviewTemplateMode>,

    generated_xml: ReadSignal<String>, // Accepts Memos or Signals.
    preview_html: ReadSignal<String>,  // Accepts Memos or Signals.
    show_preview: Signal<bool>,
    diag: Signal<DiagnosticResult>,

    config_toml: ReadSignal<String>, // Accepts Memos or Signals.
    active_preset: Signal<Option<&'static str>>,
    on_load_theme: EventHandler<String>,
    on_restore: EventHandler<ThemeConfig>,
    on_load_hotswap: EventHandler<String>,
    #[props(default)] on_navigate: Option<EventHandler<String>>,
) -> Element {
    let is_valid = diag.read().is_valid;
    let error_count = diag.read().errors.len();

    let mut status_msg = use_signal(String::new);
    let mut show_restore = use_signal(|| false);

    // Derived signal. Computes only when xml or toml signals change.
    // Zero manual clone bloat.
    let export_xml = use_memo(move || {
        match toml::from_str::<ThemeConfig>(&config_toml()) {
            Ok(config) => crate::rehydration::inject_state(&generated_xml(), &config)
                .unwrap_or_else(|err| {
                    log::error!("Failed to inject state: {}", err);
                    generated_xml()
                }),
            Err(err) => {
                log::error!("Failed to parse config for state injection: {}", err);
                generated_xml()
            }
        }
    });

    rsx! {
        div {
            class: "editor-center-workspace",
            style: "flex: 1 1 auto; min-width: 0; min-height: 0; display: flex; flex-direction: column; padding: 24px; overflow: hidden;",

            div {
                class: "export-panel-header",

                div {
                    class: "export-panel-title-block",
                    h3 { class: "export-panel-title", "Generated Blogger Theme" }
                }

                div {
                    class: "export-toolbar export-toolbar-primary",

                    div {
                        class: "preview-toolbar-group",

                        button {
                            class: if preview_viewport() == PreviewViewport::Desktop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                preview_viewport.set(PreviewViewport::Desktop);
                                apply_preview_viewport(PreviewViewport::Desktop, preview_width);
                            },
                            "Desktop"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                preview_viewport.set(PreviewViewport::Laptop);
                                apply_preview_viewport(PreviewViewport::Laptop, preview_width);
                            },
                            "Laptop"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                preview_viewport.set(PreviewViewport::Tablet);
                                apply_preview_viewport(PreviewViewport::Tablet, preview_width);
                            },
                            "Tablet"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                preview_viewport.set(PreviewViewport::Phone);
                                apply_preview_viewport(PreviewViewport::Phone, preview_width);
                            },
                            "Phone"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                preview_viewport.set(PreviewViewport::Fit);
                                apply_preview_viewport(PreviewViewport::Fit, preview_width);
                            },
                            "Fit"
                        }

                        button {
                            class: if preview_viewport().is_rotatable() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                            title: "Rotate tablet, phone, or custom preview width",
                            onclick: move |_| {
                                if preview_viewport().is_rotatable() {
                                    preview_width.set(rotate_preview_width(preview_viewport(), preview_width()));
                                }
                            },
                            "Rotate"
                        }

                        label {
                            class: "preview-width-control",
                            span { class: "preview-width-label", "Width" }
                            input {
                                class: "preview-width-input", r#type: "number", min: "240", max: "2400", step: "10", value: "{preview_width()}",
                                oninput: move |evt| {
                                    if let Ok(width_value) = evt.value().parse::<u32>() {
                                        preview_width.set(clamp_preview_width(width_value));
                                        preview_viewport.set(PreviewViewport::Custom);
                                    }
                                },
                            }
                        }
                    }

                    div {
                        class: "preview-toolbar-group preview-template-mode-group",
                        span { class: "preview-width-label", "Layout" }
                        button {
                            class: if preview_template_mode() == PreviewTemplateMode::Modern { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| { preview_template_mode.set(PreviewTemplateMode::Modern); },
                            "Modern"
                        }
                        button {
                            class: if preview_template_mode() == PreviewTemplateMode::Sidebars { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| { preview_template_mode.set(PreviewTemplateMode::Sidebars); },
                            "Sidebars"
                        }
                    }
                }
            }

            div {
                class: "export-action-bar",

                div {
                    class: "export-action-group",
                    button {
                        class: "editor-button",
                        title: "Copies your finished theme as Blogger XML to the clipboard. In Blogger, open Theme → Edit HTML, select all, and paste.",
                        onclick: move |_| {
                            copy_to_clipboard(export_xml());
                            status_msg.set("XML copied to clipboard!".to_string());
                        },
                        "Copy Theme Code"
                    }
                    button {
                        class: "editor-button",
                        title: "Saves your theme as an .xml file. In Blogger, go to Theme → Restore to upload it.",
                        onclick: move |_| {
                            async move {
                                let mut eval = dioxus::document::eval(r#"
                                    let text = await dioxus.recv();
                                    let blob = new Blob([text], { type: 'text/xml' });
                                    let url = URL.createObjectURL(blob);
                                    let a = document.createElement('a'); a.href = url; a.download = 'theme.xml';
                                    document.body.appendChild(a); a.click(); URL.revokeObjectURL(url); document.body.removeChild(a);
                                    dioxus.send("done");
                                "#);
                                let _ = eval.send(export_xml());
                                let _ = eval.recv::<serde_json::Value>().await; 
                            }
                        },
                        "Download Theme"
                    }
                }

                div {
                    class: "export-action-group",

                    label {
                        class: "editor-button",
                        title: "Reopens a saved editor project — a .toml file holding all your settings — so you can keep editing.",
                        "Open Project"
                        input {
                            r#type: "file", accept: ".toml", style: "display: none;",
                            onchange: move |evt| {
                                let on_load = on_load_theme.clone();
                                async move {
                                    if let Some(file) = evt.files().first() {
                                        if let Ok(bytes) = file.read_bytes().await {
                                            let contents = String::from_utf8_lossy(&bytes).into_owned();
                                            on_load.call(contents);
                                            status_msg.set("Theme loaded successfully.".to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "editor-button",
                        title: "Saves your editor settings as a .toml project file you can reopen later. This is your working file, not the Blogger theme.",
                        onclick: move |_| {
                            async move {
                                let mut eval = dioxus::document::eval(r#"
                                    let text = await dioxus.recv();
                                    let blob = new Blob([text], { type: 'text/plain' });
                                    let url = URL.createObjectURL(blob);
                                    let a = document.createElement('a'); a.href = url; a.download = 'my_theme.toml';
                                    document.body.appendChild(a); a.click(); URL.revokeObjectURL(url); document.body.removeChild(a);
                                    dioxus.send("done");
                                "#);
                                let _ = eval.send(config_toml());
                                let _ = eval.recv::<serde_json::Value>().await;
                                status_msg.set("Theme saved!".to_string());
                            }
                        },
                        "Save Project"
                    }
                }

                div {
                    class: "export-action-group",
                    button {
                        class: "editor-button", style: "color: var(--editor-accent-warm); border-color: var(--editor-accent-warm);",
                        title: "Loads example posts and site info from a .json file. This only fills the live preview — it isn't part of your exported theme.",
                        onclick: move |_| {
                            let mut updated_config = match toml::from_str::<ThemeConfig>(&config_toml()) {
                                Ok(config) => config,
                                Err(err) => {
                                    status_msg.set(format!("Load Data failed: invalid TOML: {}", err));
                                    return;
                                }
                            };

                            let Some(path) = rfd::FileDialog::new()
                                .set_title("Load Site Data Profile")
                                .add_filter("JSON", &["json"])
                                .pick_file()
                            else {
                                status_msg.set("Load Data cancelled.".to_string());
                                return;
                            };

                            let json_string = match std::fs::read_to_string(&path) {
                                Ok(contents) => contents,
                                Err(err) => {
                                    status_msg.set(format!("Read failed {}: {}", path.display(), err));
                                    return;
                                }
                            };

                            let loaded_data = match serde_json::from_str::<ThemeConfig>(&json_string) {
                                Ok(config) => config,
                                Err(err) => {
                                    status_msg.set(format!("Invalid JSON: {}", err));
                                    return;
                                }
                            };

                            updated_config.apply_site_data(&loaded_data);
                            on_restore.call(updated_config);
                            status_msg.set(format!("Site data loaded: {}", path.display()));
                        },
                        "Load Sample Content"
                    }
                    button {
                        class: "editor-button", style: "color: var(--editor-accent-warm); border-color: var(--editor-accent-warm);",
                        title: "Saves the preview's example posts and site info to a .json file to reuse later.",
                        onclick: move |_| {
                            let current_config = match toml::from_str::<ThemeConfig>(&config_toml()) {
                                Ok(config) => config,
                                Err(err) => {
                                    status_msg.set(format!("Parse failed: {}", err));
                                    return;
                                }
                            };

                            let Some(path) = rfd::FileDialog::new()
                                .set_title("Save Site Data Profile")
                                .set_file_name("my_site_data.json")
                                .add_filter("JSON", &["json"])
                                .save_file()
                            else { return; };

                            match serde_json::to_string_pretty(&current_config) {
                                Ok(json_string) => match std::fs::write(&path, json_string) {
                                    Ok(()) => status_msg.set(format!("Site data saved: {}", path.display())),
                                    Err(err) => status_msg.set(format!("Save failed: {}", err)),
                                },
                                Err(err) => status_msg.set(format!("Serialize failed: {}", err))
                            }
                        },
                        "Save Sample Content"
                    }
                }

                div {
                    class: "export-action-group-final",
                    button {
                        class: if show_restore() { "editor-button editor-button-active" } else { "editor-button" },
                        onclick: move |_| { show_restore.set(!show_restore()); },
                        "Restore Workspace ▼"
                    }
                }
            }

            if !status_msg().is_empty() {
                div { class: "export-status", "{status_msg}" }
            }

            if !is_valid {
                div {
                    class: "export-error-banner",
                    span { style: "flex-shrink: 0;", "⚠" }
                    span { "Export disabled — {error_count} integrity error(s). Fix the template skeleton before copying." }
                }
            }

            if show_preview() {
                PreviewCanvas {
                    preview_viewport,
                    preview_width,
                    preview_html: preview_html(),
                    on_navigate: move |href: String| {
                        if let Some(handler) = on_navigate.as_ref() {
                            handler.call(href);
                        }
                    },
                }
            } else {
                div {
                    class: "export-viewport",
                    textarea { class: "export-xml-textarea", readonly: true, value: "{export_xml()}" }
                }
            }

            if show_restore() {
                ThemeRestoreDropZone {
                    on_restore: on_restore.clone(),
                    on_close: move |_| { show_restore.set(false); },
                }
            }

            div {
                class: "export-action-bar",
                style: "margin-top: 15px; border-top: 1px solid var(--editor-border-soft); padding-top: 15px;",

                div {
                    class: "export-action-group",

                    if is_valid {
                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                match build_fresh_export_xml(&config_toml(), active_preset()) {
                                    Ok(fresh_xml) => {
                                        copy_to_clipboard(fresh_xml);
                                        status_msg.set("XML copied to clipboard!".to_string());
                                    }
                                    Err(err) => status_msg.set(err),
                                }
                            },
                            "Copy XML"
                        }

                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                let fresh_xml = match build_fresh_export_xml(&config_toml(), active_preset()) {
                                    Ok(xml) => xml,
                                    Err(err) => { status_msg.set(err); return; }
                                };

                                match crate::render::save_xml_to_disk(&fresh_xml, "Moribund_Institute") {
                                    Ok(msg) => status_msg.set(msg),
                                    Err(err) => status_msg.set(format!("Export failed: {}", err)),
                                }
                            },
                            "Export XML to Disk"
                        }

                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                let fresh_xml = match build_fresh_export_xml(&config_toml(), active_preset()) {
                                    Ok(xml) => xml,
                                    Err(err) => { status_msg.set(err); return; }
                                };

                                let config = match toml::from_str::<ThemeConfig>(&config_toml()) {
                                    Ok(c) => c,
                                    Err(err) => { status_msg.set(format!("Config error: {}", err)); return; }
                                };

                                match crate::render::save_bundle_to_disk(&fresh_xml, "Moribund_Institute", &config.static_pages) {
                                    Ok(msg) => status_msg.set(msg),
                                    Err(err) => status_msg.set(format!("Bundle failed: {}", err)),
                                }
                            },
                            "Export Theme Bundle (.zip)"
                        }
                    } else {
                        button { class: "editor-button editor-button-disabled", title: "Fix errors", "Copy XML" }
                        button { class: "editor-button editor-button-disabled", title: "Fix errors", "Export XML to Disk" }
                        button { class: "editor-button editor-button-disabled", title: "Fix errors", "Export Theme Bundle (.zip)" }
                    }
                }
            }
        }
    }
}

fn build_fresh_export_xml(
    config_toml: &str,
    active_preset_name: Option<&'static str>,
) -> Result<String, String> {
    let config = toml::from_str::<ThemeConfig>(config_toml)
        .map_err(|err| format!("could not parse TOML: {}", err))?;

    let (light_palette, dark_palette) =
        crate::presets::resolve_palette_pair(active_preset_name, &config);

    let rendered_xml = crate::render::render_theme(&config, &light_palette, &dark_palette);
    crate::rehydration::inject_state(&rendered_xml, &config)
}