use dioxus::prelude::*;

use crate::clipboard::copy_to_clipboard;
use crate::config::ThemeConfig;
use crate::diagnostics::DiagnosticResult;
use crate::ui::layout::{
    apply_preview_viewport, clamp_preview_width, rotate_preview_width, PreviewTemplateMode,
    PreviewViewport,
};
use crate::ui::preview_canvas::PreviewCanvas;

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
    on_load_theme: EventHandler<String>,
    on_restore: EventHandler<ThemeConfig>,
    on_load_hotswap: EventHandler<String>,
) -> Element {
    let export_xml = crate::rehydration::inject_state(&generated_xml, &config_toml);

    let xml_for_copy = generated_xml.clone();
    let xml_for_download = generated_xml.clone();
    let toml_for_backup = config_toml.clone();
    let toml_for_hotswap = config_toml.clone();
    let toml_for_bottom_copy = config_toml.clone();
    let toml_for_disk = config_toml.clone();

    let is_valid = diag.read().is_valid;
    let error_count = diag.read().errors.len();

    let mut status_msg = use_signal(|| "".to_string());
    let mut import_text = use_signal(|| "".to_string());
    let mut import_status = use_signal(|| "".to_string());
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
                    label {
                        class: "editor-button", style: "color: var(--editor-accent-warm); border-color: var(--editor-accent-warm); cursor: pointer;",
                        "Load Data (.json)"
                        input {
                            r#type: "file", accept: ".json", style: "display: none;",
                            onchange: move |evt| {
                                let on_load_hotswap = on_load_hotswap.clone();
                                async move {
                                    if let Some(file_engine) = evt.files() {
                                        if let Some(file_name) = file_engine.files().first() {
                                            if let Some(contents) = file_engine.read_file_to_string(&file_name).await {
                                                on_load_hotswap.call(contents);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    button {
                        class: "editor-button", style: "color: var(--editor-accent-warm); border-color: var(--editor-accent-warm);",
                        onclick: move |_| {
                            let toml_copy = toml_for_hotswap.clone();
                            async move {
                                if let Ok(config_val) = toml::from_str::<serde_json::Value>(&toml_copy) {
                                    let hotswap = serde_json::json!({
                                        "site": config_val.get("site"), "assets": config_val.get("assets"), "menu_links": config_val.get("menu_links"),
                                        "seo": config_val.get("seo"), "footer": config_val.get("footer"), "ads": config_val.get("ads"),
                                        "plugins": config_val.get("plugins"), "static_pages": config_val.get("static_pages"),
                                    });
                                    if let Ok(json_str) = serde_json::to_string_pretty(&hotswap) {
                                        let mut eval = eval(r#"
                                            let text = await dioxus.recv();
                                            let blob = new Blob([text], { type: 'application/json' });
                                            let url = URL.createObjectURL(blob);
                                            let a = document.createElement('a'); a.href = url; a.download = 'hotswap_data.json';
                                            document.body.appendChild(a); a.click(); URL.revokeObjectURL(url); document.body.removeChild(a);
                                            dioxus.send("done");
                                        "#);
                                        let _ = eval.send(json_str.into());
                                        let _ = eval.recv().await;
                                    }
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
                div {
                    class: "restore-workspace-drawer",
                    div {
                        class: "restore-header",
                        h4 { class: "restore-title", "Restore Workspace from Blogger XML" }
                        button { class: "editor-mini-button", onclick: move |_| { show_restore.set(false); }, "Close" }
                    }
                    p { class: "restore-copy", "Paste a previously exported Blogger XML template here to restore your GUI state." }
                    div {
                        class: "editor-row-stretch",
                        textarea {
                            class: "editor-textarea restore-textarea", placeholder: "Paste Blogger XML here...", value: "{import_text}",
                            oninput: move |evt| { import_text.set(evt.value().clone()); import_status.set("".to_string()); },
                        }
                        button {
                            class: "editor-button restore-button",
                            onclick: {
                                let on_restore = on_restore.clone();
                                move |_| {
                                    let pasted_xml = import_text();
                                    if pasted_xml.trim().is_empty() {
                                        import_status.set("Paste exported Blogger XML before rehydrating.".to_string()); return;
                                    }
                                    match crate::rehydration::extract_and_decode(&pasted_xml) {
                                        Ok(config) => {
                                            on_restore.call(config);
                                            import_status.set("Workspace restored successfully.".to_string());
                                            import_text.set("".to_string());
                                        }
                                        Err(err) => { import_status.set(format!("Error: {}", err)); }
                                    }
                                }
                            },
                            "Rehydrate"
                        }
                    }
                    if !import_status().is_empty() {
                        div { class: "restore-status", "{import_status}" }
                    }
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
                                match build_fresh_export_xml(&toml_for_bottom_copy) {
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
                                let fresh_xml = match build_fresh_export_xml(&toml_for_disk) {
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

fn build_fresh_export_xml(config_toml: &str) -> Result<String, String> {
    let config = toml::from_str::<ThemeConfig>(config_toml)
        .map_err(|err| format!("could not parse current ThemeConfig TOML: {}", err))?;

    let rendered_xml = crate::render::render_theme(&config);
    Ok(crate::rehydration::inject_state(&rendered_xml, config_toml))
}