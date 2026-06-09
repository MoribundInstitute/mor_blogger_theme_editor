use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::diagnostics::DiagnosticResult;
use crate::ui::panels::presets::ThemeRestoreDropZone;
use crate::ui::workspace::layout::{
    apply_preview_viewport, clamp_preview_width, rotate_preview_width, PreviewViewport,
};
use mor_blogger_core::render::PreviewTemplateMode;
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use crate::utils::clipboard::copy_to_clipboard;
use dioxus::prelude::*;

const PICKER_ICONS: [(&str, &str); 10] = [
    ("Close", "M18 6 6 18M6 6l12 12"),
    ("Search", "M11 18a7 7 0 100-14 7 7 0 000 14zM20 20l-3.5-3.5"),
    ("Menu", "M4 7h16M4 12h16M4 17h16"),
    ("Left Sidebar", "M9 4v16M6 8h.01M6 12h.01 M3 4h18v16H3z"),
    ("Right Sidebar", "M15 4v16M18 8h.01M18 12h.01 M3 4h18v16H3z"),
    ("Chevron Left", "m15 18-6-6 6-6"),
    ("Chevron Right", "m9 18 6-6-6-6"),
    ("Home", "m3 11 9-8 9 8 M5 10v10h14V10 M9 20v-6h6v6"),
    ("Archive", "M5 8v12h14V8 M10 12h4 M3 4h18v4H3z"),
    ("Tag", "M20 13 12 21 3 12V3h9l8 8z M7.5 7.5A1.5 1.5 0 107.5 4a1.5 1.5 0 000 3.5z"),
];

fn encode_path_to_mask(path_d: &str) -> String {
    let raw = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"black\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"{}\"/></svg>", path_d);
    encode_full_svg_to_mask(&raw)
}

// Encodes raw files straight from disk without wrapping them in our own tags.
// Real GTK/Latte/Inkscape icons begin with an <?xml?> prologue, comments, or a
// DOCTYPE, so we slice from the first <svg before encoding. Percent-encoding of
// '%' MUST come first, otherwise it would re-escape the %XX sequences we emit.
fn encode_full_svg_to_mask(raw_svg: &str) -> String {
    let body = match raw_svg.find("<svg") {
        Some(start) => &raw_svg[start..],
        None => raw_svg,
    };
    let encoded = body
        .replace('%', "%25")
        .replace('"', "%22")
        .replace('\'', "%27")
        .replace('#', "%23")
        .replace('&', "%26")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('\r', "")
        .replace('\n', "%0A")
        .replace('\t', "%20")
        .replace(' ', "%20");
    // Wrap in SINGLE quotes. The stored value is later serialized into the
    // preview's HTML (doc.write in preview_canvas.rs) and lands inside a
    // double-quoted style="..." attribute. A double-quoted url("...") would
    // close that attribute early and silently drop the mask, leaving the icon
    // element showing its bare filled shape. Single quotes nest safely inside a
    // double-quoted attribute, and any quotes inside the SVG are encoded above.
    format!("url('data:image/svg+xml,{}')", encoded)
}

#[component]
pub fn CenterWorkspacePanel(
    preview_viewport: Signal<PreviewViewport>,
    preview_width: Signal<u32>,
    preview_template_mode: Signal<PreviewTemplateMode>,

    preview_html: ReadSignal<String>,
    show_preview: Signal<bool>,
    diag: Signal<DiagnosticResult>,

    config_toml: ReadSignal<String>,
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
    let mut active_selection = use_signal(|| None::<String>);
    let mut active_icon_picker = use_signal(|| None::<String>);

    let export_xml = use_memo(move || {
        match build_fresh_export_xml(&config_toml(), active_preset()) {
            Ok(xml) => xml,
            Err(err) => {
                log::error!("Render failed: {}", err);
                format!("<!-- Render failed: {} -->", err)
            }
        }
    });

    rsx! {
        div {
            class: "editor-center-workspace",
            style: "flex: 1 1 auto; min-width: 0; min-height: 0; display: flex; flex-direction: column; padding: 24px; overflow: hidden; position: relative;",

            if let Some(icon_target) = active_icon_picker() {
                div {
                    class: "editor-modal-overlay",
                    style: "position: absolute; inset: 0; background: rgba(0,0,0,0.6); z-index: 100; display: flex; align-items: center; justify-content: center; backdrop-filter: blur(2px);",
                    onclick: move |_| active_icon_picker.set(None),

                    div {
                        class: "editor-panel",
                        style: "width: 460px; background: var(--bg-panel); border: 1px solid var(--border-color); box-shadow: 0 20px 50px rgba(0,0,0,0.5); padding: 20px; border-radius: 12px;",
                        onclick: move |e| e.stop_propagation(),

                        div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; border-bottom: 1px solid var(--border-color); padding-bottom: 10px;",
                            h3 { style: "margin: 0; color: var(--fg-base);", "Select Visual Icon" }
                            button {
                                class: "editor-mini-button",
                                onclick: move |_| active_icon_picker.set(None),
                                "×"
                            }
                        }

                        div { style: "font-size: 0.85em; color: var(--fg-muted); margin-bottom: 16px;",
                            "Target slot: ", code { style: "color: var(--accent);", "{icon_target}" }
                        }

                        // ROW 1: CUSTOM OS FILE PICKER
                        h4 { style: "margin: 0 0 10px 0; font-size: 0.85em; color: var(--fg-base); text-transform: uppercase; letter-spacing: 0.05em;", "Custom File" }
                        div {
                            style: "margin-bottom: 24px;",
                            button {
                                class: "editor-button",
                                style: "width: 100%; justify-content: center;",
                                onclick: {
                                    let target_slot = icon_target.clone();
                                    let restore = on_restore.clone();
                                    move |_| {
                                        let target_slot = target_slot.clone();
                                        spawn(async move {
                                            if let Some(file) = rfd::AsyncFileDialog::new().add_filter("SVG", &["svg"]).pick_file().await {
                                                let bytes = file.read().await;
                                                let raw_svg = String::from_utf8_lossy(&bytes).into_owned();
                                                if raw_svg.contains("<svg") {
                                                    let mask = encode_full_svg_to_mask(&raw_svg);
                                                    if let Ok(mut config) = toml::from_str::<ThemeConfig>(&config_toml()) {
                                                        match target_slot.as_str() {
                                                            "icons.panel_close" => config.icons.panel_close = mask,
                                                            "icons.search" => config.icons.search = mask,
                                                            "icons.menu" => config.icons.menu = mask,
                                                            "icons.sidebar_left" => config.icons.sidebar_left = mask,
                                                            "icons.sidebar_right" => config.icons.sidebar_right = mask,
                                                            _ => {}
                                                        }
                                                        restore.call(config);
                                                        active_icon_picker.set(None);
                                                        status_msg.set(format!("Custom SVG applied from {}", file.file_name()));
                                                    }
                                                } else {
                                                    status_msg.set("Error: File is not a valid SVG.".to_string());
                                                }
                                            }
                                        });
                                    }
                                },
                                "Browse System for .svg..."
                            }
                        }

                        // ROW 2: LOADED IN THEME
                        h4 { style: "margin: 0 0 10px 0; font-size: 0.85em; color: var(--fg-base); text-transform: uppercase; letter-spacing: 0.05em;", "Loaded in Current Theme" }
                        div {
                            style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px; margin-bottom: 24px;",
                            {
                                let parsed = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
                                let current_icons = [
                                    ("Panel Close", parsed.icons.panel_close),
                                    ("Search", parsed.icons.search),
                                    ("Menu", parsed.icons.menu),
                                    ("Left Sidebar", parsed.icons.sidebar_left),
                                    ("Right Sidebar", parsed.icons.sidebar_right),
                                ];

                                rsx! {
                                    for (label, mask_uri) in current_icons.into_iter() {
                                        button {
                                            class: "editor-button",
                                            style: "aspect-ratio: 1; padding: 0; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border-color: var(--border-color);",
                                            title: "{label}",
                                            onclick: {
                                                let icon_target = icon_target.clone();
                                                let mask_uri = mask_uri.clone();
                                                move |_| {
                                                    if let Ok(mut config) = toml::from_str::<ThemeConfig>(&config_toml()) {
                                                        match icon_target.as_str() {
                                                            "icons.panel_close" => config.icons.panel_close = mask_uri.clone(),
                                                            "icons.search" => config.icons.search = mask_uri.clone(),
                                                            "icons.menu" => config.icons.menu = mask_uri.clone(),
                                                            "icons.sidebar_left" => config.icons.sidebar_left = mask_uri.clone(),
                                                            "icons.sidebar_right" => config.icons.sidebar_right = mask_uri.clone(),
                                                            _ => {}
                                                        }
                                                        on_restore.call(config);
                                                        active_icon_picker.set(None);
                                                        status_msg.set(format!("Applied {} icon.", label));
                                                    }
                                                }
                                            },
                                            span {
                                                style: "display: block; width: 24px; height: 24px; background-color: var(--fg-base); -webkit-mask-image: {mask_uri}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // ROW 3: DEFAULTS
                        h4 { style: "margin: 0 0 10px 0; font-size: 0.85em; color: var(--fg-base); text-transform: uppercase; letter-spacing: 0.05em;", "Default Library" }
                        div {
                            style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px;",
                            for (label, path_d) in PICKER_ICONS.iter() {
                                button {
                                    class: "editor-button",
                                    style: "aspect-ratio: 1; padding: 0; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border-color: var(--border-color);",
                                    title: "{label}",
                                    onclick: {
                                        let icon_target = icon_target.clone();
                                        let mask = encode_path_to_mask(path_d);
                                        move |_| {
                                            if let Ok(mut config) = toml::from_str::<ThemeConfig>(&config_toml()) {
                                                match icon_target.as_str() {
                                                    "icons.panel_close" => config.icons.panel_close = mask.clone(),
                                                    "icons.search" => config.icons.search = mask.clone(),
                                                    "icons.menu" => config.icons.menu = mask.clone(),
                                                    "icons.sidebar_left" => config.icons.sidebar_left = mask.clone(),
                                                    "icons.sidebar_right" => config.icons.sidebar_right = mask.clone(),
                                                    _ => {}
                                                }
                                                on_restore.call(config);
                                                active_icon_picker.set(None);
                                                status_msg.set(format!("Applied {} icon.", label));
                                            }
                                        }
                                    },
                                    div {
                                        style: "width: 24px; height: 24px; color: var(--fg-base);",
                                        dangerous_inner_html: format!("<svg viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='{}'/></svg>", path_d)
                                    }
                                }
                            }
                        }
                    }
                }
            }

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

                    label {
                        class: "editor-button",
                        title: "Reopens a saved editor project \u{2014} a .toml file holding all your settings \u{2014} so you can keep editing.",
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
                        title: "Loads example posts and site info from a .json file. This only fills the live preview \u{2014} it isn't part of your exported theme.",
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
                    span { "Export disabled \u{2014} {error_count} integrity error(s). Fix the template skeleton before copying." }
                }
            }

            if show_preview() {
                div {
                    style: "display: flex; gap: 16px; flex: 1; min-height: 0;",
                    
                    div {
                        style: "flex: 1; min-width: 0; display: flex; flex-direction: column;",
                        PreviewCanvas {
                            preview_viewport,
                            preview_width,
                            preview_html: preview_html(),
                            on_navigate: move |href: String| {
                                if let Some(handler) = on_navigate.as_ref() {
                                    handler.call(href);
                                }
                            },
                            on_select: move |target: String| {
                                active_selection.set(Some(target));
                            },
                            on_context_menu: move |target: String| {
                                if target.starts_with("icons.") {
                                    active_icon_picker.set(Some(target));
                                }
                            }
                        }
                    }

                    if let Some(target) = active_selection() {
                        div {
                            class: "editor-docked-palette editor-panel",
                            style: "width: 320px; flex-shrink: 0; display: flex; flex-direction: column; border-left: 1px solid var(--editor-border-soft); background: var(--bg-panel); overflow: hidden;",
                            
                            div {
                                class: "palette-header",
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; border-bottom: 1px solid var(--editor-border-soft); background: color-mix(in srgb, var(--bg-panel) 90%, var(--editor-accent-muted));",
                                span { style: "font-weight: 600; font-size: 0.9em; text-transform: uppercase; letter-spacing: 0.05em;", "Edit Selection" }
                                button {
                                    class: "editor-mini-button",
                                    style: "padding: 2px 8px;",
                                    onclick: move |_| active_selection.set(None),
                                    "×"
                                }
                            }

                            div {
                                class: "palette-content",
                                style: "padding: 16px; display: flex; flex-direction: column; gap: 12px;",
                                
                                label {
                                    style: "font-size: 0.85em; color: var(--fg-muted); font-family: var(--font-mono, monospace);",
                                    "{target}"
                                }

                                {
                                    let current_val = match toml::from_str::<ThemeConfig>(&config_toml()) {
                                        Ok(c) => match target.as_str() {
                                            "site.site_title" => c.site.site_title,
                                            "site.site_subtitle" => c.site.site_subtitle,
                                            "footer.footer_text" => c.footer.footer_text,
                                            "colors.accent" => c.colors.accent,
                                            "colors.bg_panel" => c.colors.bg_panel.to_css(),
                                            "colors.bg_elevated" => c.colors.bg_elevated.to_css(),
                                            "typography.body_font_stack" => c.typography.body_font_stack,
                                            "typography.heading_font_stack" => c.typography.heading_font_stack,
                                            "typography.mono_font_stack" => c.typography.mono_font_stack,
                                            _ => String::new(),
                                        },
                                        Err(_) => String::new(),
                                    };

                                    match target.as_str() {
                                        "site.site_title" | "site.site_subtitle" | "typography.body_font_stack" | "typography.heading_font_stack" | "typography.mono_font_stack" => rsx! {
                                            input {
                                                class: "editor-input",
                                                style: "width: 100%; box-sizing: border-box;",
                                                value: "{current_val}",
                                                oninput: move |evt| {
                                                    if let Ok(mut config) = toml::from_str::<ThemeConfig>(&config_toml()) {
                                                        match target.as_str() {
                                                            "site.site_title" => config.site.site_title = evt.value(),
                                                            "site.site_subtitle" => config.site.site_subtitle = evt.value(),
                                                            "typography.body_font_stack" => config.typography.body_font_stack = evt.value(),
                                                            "typography.heading_font_stack" => config.typography.heading_font_stack = evt.value(),
                                                            "typography.mono_font_stack" => config.typography.mono_font_stack = evt.value(),
                                                            _ => {}
                                                        }
                                                        on_restore.call(config);
                                                    }
                                                }
                                            }
                                        },
                                        "colors.accent" | "colors.bg_panel" | "colors.bg_elevated" => rsx! {
                                            input {
                                                class: "editor-input",
                                                r#type: "text",
                                                placeholder: "#hex or rgb",
                                                style: "width: 100%; box-sizing: border-box;",
                                                value: "{current_val}",
                                                oninput: move |evt| {
                                                    if let Ok(mut config) = toml::from_str::<ThemeConfig>(&config_toml()) {
                                                        match target.as_str() {
                                                            "colors.accent" => config.colors.accent = evt.value(),
                                                            _ => {}
                                                        }
                                                        on_restore.call(config);
                                                    }
                                                }
                                            }
                                        },
                                        "footer.footer_text" => rsx! {
                                            textarea {
                                                class: "editor-textarea",
                                                style: "width: 100%; min-height: 100px; box-sizing: border-box; resize: vertical;",
                                                value: "{current_val}",
                                                oninput: move |evt| {
                                                    if let Ok(mut config) = toml::from_str::<ThemeConfig>(&config_toml()) {
                                                        config.footer.footer_text = evt.value();
                                                        on_restore.call(config);
                                                    }
                                                }
                                            }
                                        },
                                        _ if target.starts_with("icons.") => rsx! {
                                            div {
                                                style: "color: var(--fg-muted); font-size: 0.85em; padding: 8px; background: rgba(0,0,0,0.2); border-radius: 4px;",
                                                "Right-click this icon in the preview canvas to open the visual SVG picker."
                                            }
                                        },
                                        _ => rsx! {
                                            div {
                                                style: "color: var(--editor-accent-warm); font-size: 0.9em;",
                                                "Unmapped target field. Check tags in preview.rs."
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
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
                                copy_to_clipboard(export_xml());
                                status_msg.set("Theme XML copied to clipboard!".to_string());
                            },
                            "Copy XML"
                        }

                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                let xml_data = export_xml();
                                let mut local_status = status_msg;

                                spawn(async move {
                                    let file = rfd::AsyncFileDialog::new()
                                        .set_file_name("theme.xml")
                                        .add_filter("XML", &["xml"])
                                        .save_file()
                                        .await;

                                    if let Some(handle) = file {
                                        match mor_blogger_core::render::save_xml_to_disk(&xml_data, handle.path()) {
                                            Ok(msg) => local_status.set(msg),
                                            Err(err) => local_status.set(format!("Export failed: {}", err)),
                                        }
                                    }
                                });
                            },
                            "Export XML to Disk"
                        }

                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                let xml_data = export_xml();
                                let config = match toml::from_str::<ThemeConfig>(&config_toml()) {
                                    Ok(c) => c,
                                    Err(err) => { status_msg.set(format!("Config error: {}", err)); return; }
                                };
                                let mut local_status = status_msg;

                                spawn(async move {
                                    let file = rfd::AsyncFileDialog::new()
                                        .set_file_name("theme_bundle.zip")
                                        .add_filter("ZIP", &["zip"])
                                        .save_file()
                                        .await;

                                    if let Some(handle) = file {
                                        match mor_blogger_core::render::save_bundle_to_disk(&xml_data, "Moribund_Institute", &config.static_pages, handle.path()) {
                                            Ok(msg) => local_status.set(msg),
                                            Err(err) => local_status.set(format!("Bundle failed: {}", err)),
                                        }
                                    }
                                });
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
        mor_blogger_core::presets::resolve_palette_pair(active_preset_name, &config);

    let rendered_xml = mor_blogger_core::render::render_theme(&config, &light_palette, &dark_palette);
    mor_blogger_core::utils::rehydration::inject_state(&rendered_xml, &config)
}