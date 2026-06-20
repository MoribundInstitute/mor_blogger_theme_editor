use std::collections::HashMap;

use crate::app::layout_state::CenterView;
use crate::ui::docks::css_dock::VfsDictionary;
use crate::ui::workspace::layout::{
    apply_preview_viewport, clamp_preview_width, rotate_preview_width, PreviewViewport,
};
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use crate::utils::clipboard::copy_to_clipboard;
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::diagnostics::DiagnosticResult;
use mor_blogger_core::render::PreviewTemplateMode;
use mor_blogger_core::utils::svg_icons::{is_svg, svg_to_data_uri};

use crate::ui::shell::main_pane::MainPane;
use super::module_workbench::ModuleWorkbench;
use crate::ui::docks::smart_code_dock::SmartCodeDock;
use super::static_page_editor::StaticPageEditor;

const PICKER_ICONS: [(&str, &str); 15] = [
    ("Close", "M18 6 6 18M6 6l12 12"),
    ("Search", "M11 18a7 7 0 100-14 7 7 0 000 14zM20 20l-3.5-3.5"),
    ("Menu", "M4 7h16M4 12h16M4 17h16"),
    ("Left Sidebar", "M9 4v16M6 8h.01M6 12h.01 M3 4h18v16H3z"),
    ("Right Sidebar", "M15 4v16M18 8h.01M18 12h.01 M3 4h18v16H3z"),
    ("Chevron Left", "m15 18-6-6 6-6"),
    ("Chevron Right", "m9 18 6-6-6-6"),
    ("Home", "m3 11 9-8 9 8 M5 10v10h14V10 M9 20v-6h6v6"),
    ("Archive", "M5 8v12h14V8 M10 12h4 M3 4h18v4H3z"),
    (
        "Label",
        "M20 13 12 21 3 12V3h9l8 8z M7.5 7.5A1.5 1.5 0 107.5 4a1.5 1.5 0 000 3.5z",
    ),
    (
        "Share",
        "M4 12v8a2 2 0 002 2h12a2 2 0 002-2v-8 M16 6l-4-4-4 4 M12 2v13",
    ),
    (
        "User",
        "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2 M12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8",
    ),
    (
        "Comment",
        "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z",
    ),
    ("Arrow Up", "M12 19V5 M5 12l7-7 7 7"),
    (
        "External Link",
        "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6 M15 3h6v6 M10 14L21 3",
    ),
];

fn encode_path_to_mask(path_d: &str) -> String {
    let raw = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"{}\"/></svg>", path_d);
    svg_to_data_uri(&raw)
}

fn set_icon_slot(config: &mut ThemeConfig, slot: &str, mask: String) {
    match slot {
        "icons.panel_close" => config.icons.panel_close = mask,
        "icons.search" => config.icons.search = mask,
        "icons.menu" => config.icons.menu = mask,
        "icons.sidebar_left" => config.icons.sidebar_left = mask,
        "icons.sidebar_right" => config.icons.sidebar_right = mask,
        "icons.archive" => config.icons.archive = mask,
        "icons.label" => config.icons.label = mask,
        "icons.share" => config.icons.share = mask,
        "icons.user" => config.icons.user = mask,
        "icons.comment" => config.icons.comment = mask,
        "icons.arrow_up" => config.icons.arrow_up = mask,
        "icons.external_link" => config.icons.external_link = mask,
        _ => {}
    }
}

fn build_fresh_export_xml(config_toml: &str, vfs: &HashMap<String, String>) -> Result<String, String> {
    let config = toml::from_str::<ThemeConfig>(config_toml)
        .map_err(|err| format!("could not parse TOML: {}", err))?;

    let rendered_xml = mor_blogger_core::render::render_theme(&config, vfs);
    mor_blogger_core::utils::rehydration::inject_state(&rendered_xml, &config)
}

#[component]
pub fn BloggerWorkspace(
    preview_viewport: Signal<PreviewViewport>,
    preview_width: Signal<u32>,
    preview_template_mode: Signal<PreviewTemplateMode>,

    preview_html: Signal<String>,
    show_preview: Signal<bool>,
    mut center_view: Signal<CenterView>,
    diag: Signal<DiagnosticResult>,

    config_toml: ReadSignal<String>,
    active_preset: Signal<Option<&'static str>>,
    on_load_theme: EventHandler<String>,
    on_restore: EventHandler<ThemeConfig>,
    on_load_hotswap: EventHandler<String>,
    #[props(default)] on_navigate: Option<EventHandler<String>>,
    #[props(default)] on_toggle_dark_mode: Option<EventHandler<()>>,
) -> Element {
    let is_valid = diag.read().is_valid;
    let error_count = diag.read().errors.len();
    let mut active_icon_picker = use_signal(|| None::<String>);
    let is_xray_active = use_signal(|| false);
    let mut active_xray_target = use_signal(|| None::<String>);

    let mut is_fullscreen = use_signal(|| false);
    let vfs = use_context::<VfsDictionary>().0;

    let export_xml = use_memo(move || match build_fresh_export_xml(&config_toml(), &*vfs.read()) {
        Ok(xml) => xml,
        Err(err) => {
            log::error!("Render failed: {}", err);
            format!("Render failed: {}", err)
        }
    });

    let apply_text_edit = {
        let restore = on_restore.clone();
        move |target: String, val: String, cfg: String| {
            if target.is_empty() {
                return;
            }
            let mut config = toml::from_str::<ThemeConfig>(&cfg).unwrap_or_default();
            if let Some(widget_id) = target
                .strip_prefix("widget.")
                .and_then(|s| s.strip_suffix(".title"))
            {
                config
                    .template_pack
                    .widget_titles
                    .insert(widget_id.to_string(), val);
            } else {
                match target.as_str() {
                    "site.site_title" => config.site.site_title = val,
                    "site.site_subtitle" => config.site.site_subtitle = val,
                    "footer.footer_text" => config.footer.footer_text = val,
                    "typography.body_font_stack" => config.typography.body_font_stack = val,
                    "typography.heading_font_stack" => config.typography.heading_font_stack = val,
                    "typography.mono_font_stack" => config.typography.mono_font_stack = val,
                    _ => return,
                }
            }
            restore.call(config);
        }
    };

    let apply_widget_move = {
        let restore = on_restore.clone();
        move |id: String, dest: String, cfg: String| {
            if id.is_empty() || dest.is_empty() {
                return;
            }
            let mut config = toml::from_str::<ThemeConfig>(&cfg).unwrap_or_default();
            config.template_pack.move_widget(&id, &dest);
            restore.call(config);
        }
    };

    let apply_drop_svg = {
        let restore = on_restore.clone();
        move |(target, content): (String, String), cfg: String| {
            if target.is_empty() || !is_svg(&content) {
                return;
            }
            let mask = svg_to_data_uri(&content);
            let mut config = toml::from_str::<ThemeConfig>(&cfg).unwrap_or_default();
            set_icon_slot(&mut config, &target, mask);
            restore.call(config);
        }
    };

    rsx! {
        script { dangerous_inner_html: "window.addEventListener('dragover', function(e) {{ e.preventDefault(); }}, false); window.addEventListener('drop', function(e) {{ e.preventDefault(); }}, false);" }
        MainPane {
            hide_header: is_fullscreen(),

            tabs: rsx! {
                if !is_fullscreen() {
                    button {
                        class: "editor-mini-button",
                        title: "Collapse Workspace Header",
                        onclick: move |_| is_fullscreen.set(true),
                        "▲"
                    }
                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 4px;" }
                    WorkspaceTabs { center_view }
                }
            },

            toolbar: if center_view() == CenterView::Preview && !is_fullscreen() {
                Some(rsx! {
                    ViewportToolbar {
                        preview_viewport,
                        preview_width,
                        preview_template_mode,
                        is_xray_active,
                    }
                })
            } else {
                None
            },

            if is_fullscreen() {
                div {
                    class: "editor-panel",
                    style: "position: absolute; bottom: 24px; left: 50%; transform: translateX(-50%); z-index: 9000; display: flex; align-items: center; gap: 12px; padding: 8px 16px; border-radius: 30px; box-shadow: 0 15px 40px rgba(0,0,0,0.6);",

                    if center_view() == CenterView::Preview {
                        ViewportToolbar {
                            preview_viewport,
                            preview_width,
                            preview_template_mode,
                            is_xray_active,
                        }
                        div { style: "width: 1px; height: 16px; background: var(--editor-border-soft);" }
                    }

                    WorkspaceTabs { center_view }

                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft);" }

                    button {
                        class: "editor-mini-button",
                        onclick: move |_| is_fullscreen.set(false),
                        "Exit Fullscreen ×"
                    }
                }
            }

            if let Some(icon_target) = active_icon_picker() {
                IconPickerModal {
                    target: icon_target.clone(),
                    config_toml: config_toml(),
                    on_close: move |_| active_icon_picker.set(None),
                    on_restore: on_restore.clone(),
                }
            }

            match center_view() {
                CenterView::Preview => rsx! {
                    PreviewCanvas {
                        preview_viewport,
                        preview_width,
                        xray_active: Some(is_xray_active),
                        preview_html: preview_html(),
                        on_navigate: move |href: String| { if let Some(handler) = on_navigate.as_ref() { handler.call(href); } },
                        on_select: move |target: String| { active_xray_target.set(Some(target)); },
                        on_icon_edit: move |target: String| { active_icon_picker.set(Some(target)); },
                        on_toggle_dark_mode: move |_| { if let Some(handler) = on_toggle_dark_mode.as_ref() { handler.call(()); } },
                        on_update_value: {
                            let mutator = apply_text_edit.clone();
                            move |(target, val): (String, String)| { mutator(target, val, config_toml()); }
                        },
                        on_move_widget: {
                            let mutator = apply_widget_move.clone();
                            move |(id, dest): (String, String)| { mutator(id, dest, config_toml()); }
                        },
                        on_drop_svg: {
                            let mutator = apply_drop_svg.clone();
                            move |(target, content): (String, String)| { mutator((target, content), config_toml()); }
                        }
                    }
                },
                CenterView::CodeEditor => rsx! {
                    SmartCodeDock {
                        config_toml,
                        on_load_theme: on_load_theme.clone(),
                        active_xray_target,
                    }
                },
                CenterView::Split => rsx! {
                    div { style: "display:flex; flex-direction: row; height: 100%; width: 100%;",
                        div { style: "flex: 1; border-right: 1px solid var(--editor-border-soft);",
                            PreviewCanvas {
                                preview_viewport,
                                preview_width,
                                xray_active: Some(is_xray_active),
                                preview_html: preview_html(),
                                on_navigate: move |href: String| { if let Some(handler) = on_navigate.as_ref() { handler.call(href); } },
                                on_select: move |target: String| { active_xray_target.set(Some(target)); },
                                on_icon_edit: move |target: String| { active_icon_picker.set(Some(target)); },
                                on_toggle_dark_mode: move |_| { if let Some(handler) = on_toggle_dark_mode.as_ref() { handler.call(()); } },
                                on_update_value: {
                                    let mutator = apply_text_edit.clone();
                                    move |(target, val): (String, String)| { mutator(target, val, config_toml()); }
                                },
                                on_move_widget: {
                                    let mutator = apply_widget_move.clone();
                                    move |(id, dest): (String, String)| { mutator(id, dest, config_toml()); }
                                },
                                on_drop_svg: {
                                    let mutator = apply_drop_svg.clone();
                                    move |(target, content): (String, String)| { mutator((target, content), config_toml()); }
                                }
                            }
                        }
                        div { style: "flex: 1;",
                            SmartCodeDock {
                                config_toml,
                                on_load_theme: on_load_theme.clone(),
                                active_xray_target,
                            }
                        }
                    }
                },
                CenterView::Export => rsx! {
                    ExportResultView {
                        export_xml,
                        is_valid,
                        error_count,
                        config_toml,
                    }
                },
                CenterView::ModuleWorkbench => rsx! {
                    ModuleWorkbench {
                        config_toml,
                        on_load_theme: on_load_theme.clone(),
                    }
                },
                CenterView::StaticPageEditor => rsx! {
                    StaticPageEditor {
                        preview_html,
                    }
                }
            }
        }
    }
}

#[component]
fn WorkspaceTabs(mut center_view: Signal<CenterView>) -> Element {
    rsx! {
        button {
            class: if center_view() == CenterView::Preview { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            onclick: move |_| center_view.set(CenterView::Preview),
            "Preview"
        }
        button {
            class: if center_view() == CenterView::CodeEditor { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            onclick: move |_| center_view.set(CenterView::CodeEditor),
            "Code Editor"
        }
        button {
            class: if center_view() == CenterView::Export { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            onclick: move |_| center_view.set(CenterView::Export),
            "Export XML"
        }
        button {
            class: if center_view() == CenterView::ModuleWorkbench { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            onclick: move |_| center_view.set(CenterView::ModuleWorkbench),
            "Module Workbench"
        }
        button {
            class: if center_view() == CenterView::StaticPageEditor { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            onclick: move |_| center_view.set(CenterView::StaticPageEditor),
            "Static Pages"
        }
    }
}

#[component]
fn ViewportToolbar(
    mut preview_viewport: Signal<PreviewViewport>,
    mut preview_width: Signal<u32>,
    mut preview_template_mode: Signal<PreviewTemplateMode>,
    mut is_xray_active: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "preview-toolbar-group",
            style: "margin: 0;",
            button {
                class: if is_xray_active() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                title: "Widget map and editable overlay",
                onclick: move |_| is_xray_active.set(!is_xray_active()),
                "X-Ray"
            }
        }
        div {
            class: "preview-toolbar-group",
            style: "margin: 0;",
            button {
                class: if preview_viewport() == PreviewViewport::Desktop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                onclick: move |_| { apply_preview_viewport(PreviewViewport::Desktop, preview_width); preview_viewport.set(PreviewViewport::Desktop); },
                "Desktop"
            }
            button {
                class: if preview_viewport() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                onclick: move |_| { apply_preview_viewport(PreviewViewport::Laptop, preview_width); preview_viewport.set(PreviewViewport::Laptop); },
                "Laptop"
            }
            button {
                class: if preview_viewport() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                onclick: move |_| { apply_preview_viewport(PreviewViewport::Tablet, preview_width); preview_viewport.set(PreviewViewport::Tablet); },
                "Tablet"
            }
            button {
                class: if preview_viewport() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                onclick: move |_| { apply_preview_viewport(PreviewViewport::Phone, preview_width); preview_viewport.set(PreviewViewport::Phone); },
                "Phone"
            }
            button {
                class: if preview_viewport() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                onclick: move |_| { apply_preview_viewport(PreviewViewport::Fit, preview_width); preview_viewport.set(PreviewViewport::Fit); },
                "Fit"
            }
            button {
                class: if preview_viewport().is_rotatable() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                title: "Rotate tablet, phone, or custom preview width",
                onclick: move |_| { if preview_viewport().is_rotatable() { preview_width.set(rotate_preview_width(preview_viewport(), preview_width())); } },
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
            style: "margin: 0;",
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

#[component]
fn IconPickerModal(
    target: String,
    config_toml: String,
    on_close: EventHandler<()>,
    on_restore: EventHandler<ThemeConfig>,
) -> Element {
    let mut status_msg = use_signal(String::new);
    let mut raw_svg_input = use_signal(String::new);

    rsx! {
        div {
            class: "editor-modal-overlay",
            style: "position: absolute; inset: 0; background: rgba(0,0,0,0.6); z-index: 100; display: flex; align-items: center; justify-content: center; backdrop-filter: blur(2px);",
            onclick: move |_| on_close.call(()),

            div {
                class: "editor-panel",
                style: "width: 460px; background: var(--bg-panel); border: 1px solid var(--border-color); box-shadow: 0 20px 50px rgba(0,0,0,0.5); padding: 20px; border-radius: 12px;",
                onclick: move |e| e.stop_propagation(),

                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; border-bottom: 1px solid var(--border-color); padding-bottom: 10px;",
                    h3 { style: "margin: 0; color: var(--fg-base);", "Select Visual Icon" }
                    button { class: "editor-mini-button", onclick: move |_| on_close.call(()), "×" }
                }

                div { style: "font-size: 0.85em; color: var(--fg-muted); margin-bottom: 16px;", "Target slot: ", code { style: "color: var(--accent);", "{target}" } }
                if !status_msg().is_empty() { div { class: "export-status", "{status_msg}" } }

                h4 { style: "margin: 0 0 10px 0; font-size: 0.85em; color: var(--fg-base); text-transform: uppercase; letter-spacing: 0.05em;", "Paste Raw SVG" }
                div { style: "display: flex; flex-direction: column; gap: 8px; margin-bottom: 24px;",
                    textarea {
                        class: "editor-textarea",
                        style: "width: 100%; box-sizing: border-box; min-height: 80px; resize: vertical; font-family: monospace; font-size: 11px;",
                        placeholder: "Paste raw <svg> code here...",
                        value: "{raw_svg_input}",
                        oninput: move |evt| raw_svg_input.set(evt.value()),
                    }
                    button {
                        class: "editor-button",
                        style: "justify-content: center;",
                        onclick: {
                            let icon_target = target.clone();
                            let toml_str = config_toml.clone();
                            let restore = on_restore.clone();
                            move |_| {
                                let raw_svg = raw_svg_input().trim().to_string();
                                if raw_svg.is_empty() || !is_svg(&raw_svg) {
                                    status_msg.set("Error: Invalid or empty SVG.".to_string());
                                    return;
                                }
                                let mask = svg_to_data_uri(&raw_svg);
                                let mut config = toml::from_str::<ThemeConfig>(&toml_str).unwrap_or_default();
                                set_icon_slot(&mut config, &icon_target, mask);
                                restore.call(config);
                                status_msg.set("SVG applied!".to_string());
                                raw_svg_input.set(String::new());
                            }
                        },
                        "Apply Pasted SVG"
                    }
                    button {
                        class: "editor-button",
                        style: "justify-content: center;",
                        onclick: {
                            let icon_target = target.clone();
                            let toml_str = config_toml.clone();
                            let restore = on_restore.clone();
                            move |_| {
                                let slot = icon_target.clone();
                                let cfg_str = toml_str.clone();
                                let apply = restore.clone();
                                spawn(async move {
                                    if let Some(file) = rfd::AsyncFileDialog::new().add_filter("SVG", &["svg"]).pick_file().await {
                                        let bytes = file.read().await;
                                        let raw_svg = String::from_utf8_lossy(&bytes).into_owned();
                                        if !is_svg(&raw_svg) {
                                            status_msg.set("Error: File is not a valid SVG.".to_string());
                                            return;
                                        }
                                        let mask = svg_to_data_uri(&raw_svg);
                                        let mut config = toml::from_str::<ThemeConfig>(&cfg_str).unwrap_or_default();
                                        set_icon_slot(&mut config, &slot, mask);
                                        apply.call(config);
                                        status_msg.set(format!("SVG applied from {}", file.file_name()));
                                    }
                                });
                            }
                        },
                        "Browse OS for .svg..."
                    }
                }

                h4 { style: "margin: 0 0 10px 0; font-size: 0.85em; color: var(--fg-base); text-transform: uppercase; letter-spacing: 0.05em;", "Loaded in Current Theme" }
                div {
                    style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px; margin-bottom: 24px;",
                    {
                        let parsed = toml::from_str::<ThemeConfig>(&config_toml).unwrap_or_default();
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
                                    class: "editor-button", style: "aspect-ratio: 1; padding: 0; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border-color: var(--border-color);", title: "{label}",
                                    onclick: {
                                        let icon_target = target.clone();
                                        let mask_uri = mask_uri.clone();
                                        let toml_str = config_toml.clone();
                                        move |_| {
                                            let mut config = toml::from_str::<ThemeConfig>(&toml_str).unwrap_or_default();
                                            set_icon_slot(&mut config, &icon_target, mask_uri.clone());
                                            on_restore.call(config);
                                            status_msg.set(format!("Applied {} icon.", label));
                                        }
                                    },
                                    span { style: "display: block; width: 24px; height: 24px; background-color: var(--editor-text); -webkit-mask-image: {mask_uri}; -webkit-mask-size: contain; -webkit-mask-repeat: no-repeat; -webkit-mask-position: center;" }
                                }
                            }
                        }
                    }
                }

                h4 { style: "margin: 0 0 10px 0; font-size: 0.85em; color: var(--fg-base); text-transform: uppercase; letter-spacing: 0.05em;", "Default Library" }
                div {
                    style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px;",
                    for (label, path_d) in PICKER_ICONS.iter() {
                        button {
                            class: "editor-button", style: "aspect-ratio: 1; padding: 0; display: flex; align-items: center; justify-content: center; background: var(--bg-elevated); border-color: var(--border-color);", title: "{label}",
                            onclick: {
                                let icon_target = target.clone();
                                let mask = encode_path_to_mask(path_d);
                                let toml_str = config_toml.clone();
                                move |_| {
                                    let mut config = toml::from_str::<ThemeConfig>(&toml_str).unwrap_or_default();
                                    set_icon_slot(&mut config, &icon_target, mask.clone());
                                    on_restore.call(config);
                                    status_msg.set(format!("Applied {} icon.", label));
                                }
                            },
                            div { style: "width: 24px; height: 24px; color: var(--fg-base);", dangerous_inner_html: format!("<svg viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='{}'/></svg>", path_d) }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExportResultView(
    export_xml: Memo<String>,
    is_valid: bool,
    error_count: usize,
    config_toml: ReadSignal<String>,
) -> Element {
    let mut status_msg = use_signal(String::new);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",

            if !status_msg().is_empty() {
                div { class: "export-status", "{status_msg}" }
            }

            if !is_valid {
                div { class: "export-error-banner", style: "margin-bottom: 12px;",
                    span { style: "flex-shrink: 0;", "⚠" }
                    span { "Export disabled \u{2014} {error_count} integrity error(s). Fix the template skeleton before copying." }
                }
            }

            div { class: "export-viewport", style: "flex: 1; min-height: 0; display: flex; flex-direction: column; margin-top: 0;",
                textarea { class: "export-xml-textarea", style: "flex: 1;", readonly: true, value: "{export_xml()}" }
            }

            div { class: "export-action-bar", style: "margin-top: 15px; border-top: 1px solid var(--editor-border-soft); padding-top: 15px;",
                div { class: "export-action-group",
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
                                let xml = export_xml();
                                spawn(async move {
                                    if let Some(handle) = rfd::AsyncFileDialog::new().set_file_name("theme.xml").add_filter("XML", &["xml"]).save_file().await {
                                        match mor_blogger_core::render::save_xml_to_disk(&xml, handle.path()) {
                                            Ok(_) => status_msg.set("Exported to disk.".to_string()),
                                            Err(err) => status_msg.set(format!("Export failed: {}", err)),
                                        }
                                    }
                                });
                            },
                            "Export XML to Disk"
                        }
                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                let xml = export_xml();
                                let cfg_str = config_toml();
                                spawn(async move {
                                    if let Err(err) = toml::from_str::<ThemeConfig>(&cfg_str) {
                                        status_msg.set(format!("Config error: {}", err));
                                        return;
                                    }
                                    if let Some(handle) = rfd::AsyncFileDialog::new().set_file_name("theme_bundle.zip").add_filter("ZIP", &["zip"]).save_file().await {
                                        match mor_blogger_core::render::save_bundle_to_disk(&xml, &cfg_str, handle.path()) {
                                            Ok(_) => status_msg.set("Bundle exported.".to_string()),
                                            Err(err) => status_msg.set(format!("Bundle failed: {}", err)),
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
