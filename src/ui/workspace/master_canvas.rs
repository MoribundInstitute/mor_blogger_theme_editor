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

    generated_xml: String,
    preview_html: String,
    show_preview: Signal<bool>,
    diag: Signal<DiagnosticResult>,

    config_toml: String,
    active_preset: Signal<Option<&'static str>>,
    on_load_theme: EventHandler<String>,
    on_restore: EventHandler<ThemeConfig>,
    on_load_hotswap: EventHandler<String>,
) -> Element {
    // Parse the TOML into our struct so we can pass it into the new binary injection pipeline
    let export_xml = match toml::from_str::<ThemeConfig>(&config_toml) {
        Ok(config) => crate::rehydration::inject_state(&generated_xml, &config)
            .unwrap_or_else(|err| {
                log::error!("Failed to inject state: {}", err);
                generated_xml.clone()
            }),
        Err(err) => {
            log::error!("Failed to parse config for state injection: {}", err);
            generated_xml.clone()
        }
    };

    let xml_for_copy = export_xml.clone();
    let xml_for_download = export_xml.clone();
    let toml_for_backup = config_toml.clone();
    let toml_for_hotswap = config_toml.clone();
    let toml_for_save_data = config_toml.clone();
    let toml_for_bottom_copy = config_toml.clone();
    let toml_for_disk = config_toml.clone();
    
    let preset_for_bottom_copy = active_preset();
    let preset_for_disk = active_preset();

    let is_valid = diag.read().is_valid;
    let error_count = diag.read().errors.len();

    let mut status_msg = use_signal(|| "".to_string());
    let mut show_restore = use_signal(|| false);

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
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Desktop);
                                apply_preview_viewport(PreviewViewport::Desktop, preview_width);
                            },
                            "Desktop"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Laptop);
                                apply_preview_viewport(PreviewViewport::Laptop, preview_width);
                            },
                            "Laptop"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Tablet);
                                apply_preview_viewport(PreviewViewport::Tablet, preview_width);
                            },
                            "Tablet"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Phone);
                                apply_preview_viewport(PreviewViewport::Phone, preview_width);
                            },
                            "Phone"
                        }

                        button {
                            class: if preview_viewport() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| {
                                let mut viewport = preview_viewport;
                                viewport.set(PreviewViewport::Fit);
                                apply_preview_viewport(PreviewViewport::Fit, preview_width);
                            },
                            "Fit"
                        }

                        button {
                            class: if preview_viewport().is_rotatable() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                            title: "Rotate tablet, phone, or custom preview width",
                            onclick: move |_| {
                                if preview_viewport().is_rotatable() {
                                    let next_width = rotate_preview_width(preview_viewport(), preview_width());
                                    let mut width = preview_width;
                                    width.set(next_width);
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
                        span { class: "preview-width-label", "Layout" }
                        button {
                            class: if preview_template_mode() == PreviewTemplateMode::Modern { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| { let mut mode = preview_template_mode; mode.set(PreviewTemplateMode::Modern); },
                            "Modern"
                        }
                        button {
                            class: if preview_template_mode() == PreviewTemplateMode::Sidebars { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| { let mut mode = preview_template_mode; mode.set(PreviewTemplateMode::Sidebars); },
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
                        onclick: move |_| {
                            copy_to_clipboard(xml_for_copy.clone());
                            status_msg.set("XML copied to clipboard!".to_string());
                        },
                        "Copy XML"
                    }
                    button {
                        class: "editor-button",
                        onclick: move |_| {
                            let xml = xml_for_download.clone();
                            async move {
                                let mut eval = eval(r#"
                                    let text = await dioxus.recv();
                                    let blob = new Blob([text], { type: 'text/xml' });
                                    let url = URL.createObjectURL(blob);
                                    let a = document.createElement('a'); a.href = url; a.download = 'theme.xml';
                                    document.body.appendChild(a); a.click(); URL.revokeObjectURL(url); document.body.removeChild(a);
                                    dioxus.send("done");
                                "#);
                                let _ = eval.send(xml.into());
                                let _ = eval.recv().await; 
                            }
                        },
                        "Download .xml"
                    }
                }

                div {
                    class: "export-action-group",

                    label {
                        class: "editor-button",
                        "Load Theme (.toml)"
                        input {
                            r#type: "file", accept: ".toml", style: "display: none;",
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
                        onclick: move |_| {
                            let toml_data = toml_for_backup.clone();
                            async move {
                                let mut eval = eval(r#"
                                    let text = await dioxus.recv();
                                    let blob = new Blob([text], { type: 'text/plain' });
                                    let url = URL.createObjectURL(blob);
                                    let a = document.createElement('a'); a.href = url; a.download = 'my_theme.toml';
                                    document.body.appendChild(a); a.click(); URL.revokeObjectURL(url); document.body.removeChild(a);
                                    dioxus.send("done");
                                "#);
                                let _ = eval.send(toml_data.into());
                                let _ = eval.recv().await;
                                status_msg.set("Theme saved!".to_string());
                            }
                        },
                        "Save .toml"
                    }
                }

                div {
                    class: "export-action-group",
                    button {
                        class: "editor-button", style: "color: var(--editor-accent-warm); border-color: var(--editor-accent-warm);",
                        onclick: {
                            let on_restore = on_restore.clone();
                            move |_| {
                                let toml_copy = toml_for_hotswap.clone();

                                let mut updated_config = match toml::from_str::<ThemeConfig>(&toml_copy) {
                                    Ok(config) => config,
                                    Err(err) => {
                                        let msg = format!("Load Data failed: could not parse current ThemeConfig TOML: {}", err);
                                        eprintln!("{}", msg);
                                        status_msg.set(msg);
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
                                        let msg = format!("Load Data failed: could not read {}: {}", path.display(), err);
                                        eprintln!("{}", msg);
                                        status_msg.set(msg);
                                        return;
                                    }
                                };

                                let loaded_data = match serde_json::from_str::<ThemeConfig>(&json_string) {
                                    Ok(config) => config,
                                    Err(err) => {
                                        let msg = format!("Load Data failed: invalid ThemeConfig JSON: {}", err);
                                        eprintln!("{}", msg);
                                        status_msg.set(msg);
                                        return;
                                    }
                                };

                                updated_config.apply_site_data(&loaded_data);
                                on_restore.call(updated_config);
                                status_msg.set(format!("Site data loaded: {}", path.display()));
                            }
                        },
                        "Load Data (.json)"
                    }
                    button {
                        class: "editor-button", style: "color: var(--editor-accent-warm); border-color: var(--editor-accent-warm);",
                        onclick: move |_| {
                            let toml_copy = toml_for_save_data.clone();

                            let current_config = match toml::from_str::<ThemeConfig>(&toml_copy) {
                                Ok(config) => config,
                                Err(err) => {
                                    let msg = format!("Save Data failed: could not parse current ThemeConfig TOML: {}", err);
                                    eprintln!("{}", msg);
                                    status_msg.set(msg);
                                    return;
                                }
                            };

                            let Some(path) = rfd::FileDialog::new()
                                .set_title("Save Site Data Profile")
                                .set_file_name("my_site_data.json")
                                .add_filter("JSON", &["json"])
                                .save_file()
                            else {
                                status_msg.set("Save Data cancelled.".to_string());
                                return;
                            };

                            match serde_json::to_string_pretty(&current_config) {
                                Ok(json_string) => match std::fs::write(&path, json_string) {
                                    Ok(()) => {
                                        status_msg.set(format!("Site data saved: {}", path.display()));
                                    }
                                    Err(err) => {
                                        let msg = format!("Save Data failed: {}", err);
                                        eprintln!("{}", msg);
                                        status_msg.set(msg);
                                    }
                                },
                                Err(err) => {
                                    let msg = format!("Save Data failed: could not serialize ThemeConfig: {}", err);
                                    eprintln!("{}", msg);
                                    status_msg.set(msg);
                                }
                            }
                        },
                        "Save Data (.json)"
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
                PreviewCanvas { preview_viewport, preview_width, preview_html }
            } else {
                div {
                    class: "export-viewport",
                    textarea { class: "export-xml-textarea", readonly: true, value: "{export_xml}" }
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
                                match build_fresh_export_xml(&toml_for_bottom_copy, preset_for_bottom_copy) {
                                    Ok(fresh_xml) => {
                                        copy_to_clipboard(fresh_xml);
                                        status_msg.set("XML copied to clipboard!".to_string());
                                    }
                                    Err(err) => {
                                        let msg = format!("Copy failed: {}", err);
                                        eprintln!("{}", msg);
                                        status_msg.set(msg);
                                    }
                                }
                            },
                            "Copy XML"
                        }

                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                let fresh_xml = match build_fresh_export_xml(&toml_for_disk, preset_for_disk) {
                                    Ok(xml) => xml,
                                    Err(err) => {
                                        let msg = format!("Export failed: {}", err);
                                        eprintln!("{}", msg);
                                        status_msg.set(msg);
                                        return;
                                    }
                                };

                                match crate::render::save_xml_to_disk(&fresh_xml, "Moribund_Institute") {
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
                            },
                            "Export Theme to Disk"
                        }
                    } else {
                        button { class: "editor-button editor-button-disabled", title: "Fix template integrity errors before exporting", "Copy XML" }
                        button { class: "editor-button editor-button-disabled", title: "Fix template integrity errors before exporting", "Export Theme to Disk" }
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
        .map_err(|err| format!("could not parse current ThemeConfig TOML: {}", err))?;

    let (light_palette, dark_palette) =
        crate::presets::resolve_palette_pair(active_preset_name, &config);

    let rendered_xml =
        crate::render::render_theme(&config, &light_palette, &dark_palette);
        
    crate::rehydration::inject_state(&rendered_xml, &config)
}