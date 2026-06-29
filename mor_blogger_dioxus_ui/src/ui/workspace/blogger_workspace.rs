use crate::app::state::{
    CenterView, ContextMenuPayload, LayoutState, RenderState, ThemeState,
};
use mor_blogger_core::render::pages::{
    generate_about_html, generate_archive_html, generate_categories_html,
    generate_course_catalog_html, generate_my_courses_html, generate_portfolio_html,
};
use crate::app::vfs::VfsDictionary;
use crate::ui::workspace::layout::{
    apply_preview_viewport, clamp_preview_width, is_landscape, rotate_preview_width,
    PreviewViewport,
};
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::diagnostics::DiagnosticResult;
use mor_blogger_core::utils::svg_icons::{is_svg, svg_to_data_uri};

use super::js_workbench::JsWorkbench;
use super::module_workbench::ModuleWorkbench;
use super::static_page_editor::StaticPageEditor;
use super::widget_workbench::WidgetWorkbench;
use crate::ui::layout::docks::smart_code_dock::SmartCodeDock;
use crate::ui::layout::main_pane::MainPane;

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

#[component]
pub fn BloggerWorkspace(
    preview_viewport: Signal<PreviewViewport>,
    preview_width: Signal<u32>,

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
    let _ = show_preview;
    let _ = active_preset;
    let _ = on_load_hotswap;
    let mut layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let is_valid = render.diag.read().is_valid;
    let error_count = render.diag.read().errors.len();
    let mut active_icon_picker = layout.active_icon_picker;
    let is_xray_active = use_signal(|| false);
    let mut active_xray_target = use_signal(|| None::<String>);

    let mut is_fullscreen = use_signal(|| false);
    let vfs = use_context::<VfsDictionary>().0;

    let export_xml = use_memo(move || {
        match crate::app::services::workspace_service::build_fresh_export_xml(
            &config_toml(),
            &*vfs.read(),
        ) {
            Ok(xml) => xml,
            Err(err) => {
                log::error!("Render failed: {}", err);
                format!("Render failed: {}", err)
            }
        }
    });

    let apply_text_edit = {
        let restore = on_restore.clone();
        move |target: String, val: String, cfg: String| {
            if let Some(config) =
                crate::app::services::workspace_service::handle_text_edit(&target, val, &cfg)
            {
                restore.call(config);
            }
        }
    };

    let apply_widget_move = {
        let restore = on_restore.clone();
        move |id: String, dest: String, cfg: String| {
            if let Some(config) =
                crate::app::services::workspace_service::handle_widget_move(&id, &dest, &cfg)
            {
                restore.call(config);
            }
        }
    };

    let apply_drop_svg = {
        let restore = on_restore.clone();
        move |(target, content): (String, String), cfg: String| {
            if let Some(config) =
                crate::app::services::workspace_service::handle_drop_svg(&target, &content, &cfg)
            {
                restore.call(config);
            }
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
                    on_select_mask: {
                        let toml_str = config_toml.clone();
                        let restore = on_restore.clone();
                        let target = icon_target.clone();
                        move |mask: String| {
                            let mut config = toml::from_str::<ThemeConfig>(&toml_str()).unwrap_or_default();
                            match target.as_str() {
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
                            restore.call(config);
                        }
                    },
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
                        on_icon_context_menu: move |payload: ContextMenuPayload| { layout.active_context_menu.set(Some(payload)); },
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
                                on_icon_context_menu: move |payload: ContextMenuPayload| { layout.active_context_menu.set(Some(payload)); },
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
                CenterView::WidgetWorkbench => rsx! {
                    WidgetWorkbench {
                        config_toml,
                    }
                },
                CenterView::JsWorkbench => rsx! {
                    JsWorkbench {
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
fn WorkspaceTabs(center_view: Signal<CenterView>) -> Element {
    // Route every workspace switch through enter_workspace so the per-workspace
    // default dock layout (e.g. Template Modules on the left for ModuleWorkbench)
    // is applied on click, not via a use_effect.
    let mut layout = use_context::<LayoutState>();
    rsx! {
        button {
            class: if center_view() == CenterView::Preview { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            title: "Preview",
            onclick: move |_| layout.enter_workspace(CenterView::Preview),
            "👁️"
        }
        button {
            class: if center_view() == CenterView::CodeEditor { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            title: "Code Editor",
            onclick: move |_| layout.enter_workspace(CenterView::CodeEditor),
            "</>"
        }
        button {
            class: if center_view() == CenterView::ModuleWorkbench {
                "editor-mini-button editor-mini-button-active"
            } else {
                "editor-mini-button"
            },
            title: "Module Workbench",
            onclick: move |e| {
                e.stop_propagation();
                layout.enter_workspace(CenterView::ModuleWorkbench);
            },
            "🛠️ ┳━┳"
        }
        button {
            class: if center_view() == CenterView::WidgetWorkbench { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            title: "Widget Workbench",
            onclick: move |e| {
                e.stop_propagation();
                layout.enter_workspace(CenterView::WidgetWorkbench);
            },
            "🧩"
        }
        button {
            class: if center_view() == CenterView::JsWorkbench { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            title: "JavaScript Workspace",
            onclick: move |e| {
                e.stop_propagation();
                layout.enter_workspace(CenterView::JsWorkbench);
            },
            "JS"
        }
        button {
            class: if center_view() == CenterView::StaticPageEditor { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            title: "Static Pages",
            onclick: move |_| layout.enter_workspace(CenterView::StaticPageEditor),
            "🧊 📄"
        }
        // Export is the terminal pipeline step, so it sits at the far right.
        button {
            class: if center_view() == CenterView::Export { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
            title: "Export",
            onclick: move |_| layout.enter_workspace(CenterView::Export),
            "🚀"
        }
    }
}

#[component]
fn ViewportToolbar(
    mut preview_viewport: Signal<PreviewViewport>,
    mut preview_width: Signal<u32>,
    mut is_xray_active: Signal<bool>,
) -> Element {
    // The "rotate" control is really a portrait <-> landscape toggle for the
    // device frame (you can't rotate a website). Make the icon show the
    // orientation it switches TO, and say so in the tooltip.
    let rotatable = preview_viewport().is_rotatable();
    let landscape = is_landscape(preview_viewport(), preview_width());
    let rotate_icon = if landscape { "▯" } else { "▭" };
    let rotate_title = if !rotatable {
        "Orientation — pick Tablet, Phone, or Custom first"
    } else if landscape {
        "Switch to portrait"
    } else {
        "Switch to landscape"
    };

    rsx! {
        // One pill: X-Ray toggle, then the device controls. Keeping X-Ray in its
        // own bordered+shadowed pill was wasted chrome for a single button.
        div {
            class: "preview-toolbar-group",
            style: "margin: 0;",
            button {
                class: if is_xray_active() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                title: "X-Ray — widget map and editable overlay",
                onclick: move |_| is_xray_active.set(!is_xray_active()),
                "🩻"
            }
            div { class: "preview-toolbar-divider" }
            // Desktop/Laptop/Tablet/Phone/Fit are one mutually-exclusive choice,
            // so render them as a single segmented control rather than 5 pills.
            div {
                class: "editor-segmented",
                button {
                    class: if preview_viewport() == PreviewViewport::Desktop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Desktop",
                    onclick: move |_| { apply_preview_viewport(PreviewViewport::Desktop, preview_width); preview_viewport.set(PreviewViewport::Desktop); },
                    "🖥️"
                }
                button {
                    class: if preview_viewport() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Laptop",
                    onclick: move |_| { apply_preview_viewport(PreviewViewport::Laptop, preview_width); preview_viewport.set(PreviewViewport::Laptop); },
                    "💻"
                }
                button {
                    class: if preview_viewport() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Tablet",
                    onclick: move |_| { apply_preview_viewport(PreviewViewport::Tablet, preview_width); preview_viewport.set(PreviewViewport::Tablet); },
                    "📋"
                }
                button {
                    class: if preview_viewport() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Phone",
                    onclick: move |_| { apply_preview_viewport(PreviewViewport::Phone, preview_width); preview_viewport.set(PreviewViewport::Phone); },
                    "📱"
                }
                button {
                    class: if preview_viewport() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Fit to viewport",
                    onclick: move |_| { apply_preview_viewport(PreviewViewport::Fit, preview_width); preview_viewport.set(PreviewViewport::Fit); },
                    "↔️"
                }
            }
            button {
                class: if rotatable { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                title: rotate_title,
                onclick: move |_| { if preview_viewport().is_rotatable() { preview_width.set(rotate_preview_width(preview_viewport(), preview_width())); } },
                "{rotate_icon}"
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
    }
}

#[component]
fn IconPickerModal(
    target: String,
    config_toml: String,
    on_close: EventHandler<()>,
    on_select_mask: EventHandler<String>,
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
                        onclick: move |_| {
                            let raw_svg = raw_svg_input().trim().to_string();
                            if raw_svg.is_empty() || !is_svg(&raw_svg) {
                                status_msg.set("Error: Invalid or empty SVG.".to_string());
                                return;
                            }
                            let mask = svg_to_data_uri(&raw_svg);
                            on_select_mask.call(mask);
                            status_msg.set("SVG applied!".to_string());
                            raw_svg_input.set(String::new());
                        },
                        "Apply Pasted SVG"
                    }
                    button {
                        class: "editor-button",
                        style: "justify-content: center;",
                        onclick: move |_| {
                            let apply = on_select_mask.clone();
                            spawn(async move {
                                if let Some(file) = rfd::AsyncFileDialog::new().add_filter("SVG", &["svg"]).pick_file().await {
                                    let bytes = file.read().await;
                                    let raw_svg = String::from_utf8_lossy(&bytes).into_owned();
                                    if !is_svg(&raw_svg) {
                                        status_msg.set("Error: File is not a valid SVG.".to_string());
                                        return;
                                    }
                                    let mask = svg_to_data_uri(&raw_svg);
                                    apply.call(mask);
                                    status_msg.set(format!("SVG applied from {}", file.file_name()));
                                }
                            });
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
                                        let mask_uri = mask_uri.clone();
                                        let apply = on_select_mask.clone();
                                        move |_| {
                                            apply.call(mask_uri.clone());
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
                                let mask = encode_path_to_mask(path_d);
                                let apply = on_select_mask.clone();
                                move |_| {
                                    apply.call(mask.clone());
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

// (page id, dropdown label) — mirrors the Static Pages panel.
const PAGE_OPTIONS: &[(&str, &str)] = &[
    ("Archive", "Archive"),
    ("Directory", "Directory"),
    ("About", "About Me"),
    ("Portfolio", "Portfolio"),
    ("LMS", "Courses"),
];

/// Minimal escaping for a value placed inside a double-quoted JSON string.
/// ponytail: covers backslash/quote/newline — enough for widget names/types, which
/// don't carry control chars; widen if richer values ever flow through.
fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " ")
}

#[component]
fn ExportResultView(
    export_xml: Memo<String>,
    is_valid: bool,
    error_count: usize,
    config_toml: ReadSignal<String>,
) -> Element {
    use mor_blogger_core::utils::fs_bridge;

    let theme = use_context::<ThemeState>();
    let mut status_msg = use_signal(String::new);
    let mut selected_page = use_signal(|| "Archive".to_string());

    // Custom creations available to package & share (loaded once on entry).
    let widgets = use_signal(fs_bridge::load_widget_blueprints);
    let modules = use_signal(fs_bridge::load_modules);
    let mut sel_widget = use_signal(|| 0usize);
    let mut sel_module = use_signal(|| 0usize);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; flex: 1; min-height: 0; gap: 20px; padding: 4px;",

            if !status_msg().is_empty() {
                div { class: "export-status", "{status_msg}" }
            }

            if !is_valid {
                div { class: "export-error-banner",
                    span { style: "flex-shrink: 0;", "⚠" }
                    span { "Theme export disabled \u{2014} {error_count} integrity error(s). Fix the template skeleton before exporting." }
                }
            }

            // ── Theme export ────────────────────────────────────────────
            div {
                class: "editor-panel",
                style: "border: 1px solid var(--editor-border); border-radius: var(--radius-md); padding: 16px; background: var(--bg-panel);",
                h3 { style: "margin: 0 0 4px 0; font-size: 1rem; color: var(--fg-base);", "Theme Export" }
                p { style: "margin: 0 0 14px 0; font-size: 0.8rem; color: var(--fg-muted); line-height: 1.5;",
                    "Compile the live config into a Blogger XML theme. The bundle (.zip) also embeds a workspace backup and any pages flagged for inclusion."
                }
                div { class: "export-action-group", style: "display: flex; flex-wrap: wrap; gap: 10px;",
                    if is_valid {
                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                crate::app::services::workspace_service::handle_copy_xml(export_xml(), status_msg);
                            },
                            "Copy XML"
                        }
                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                crate::app::services::workspace_service::handle_xml_export(export_xml(), status_msg);
                            },
                            "Export XML to Disk"
                        }
                        button {
                            class: "editor-button editor-button-good",
                            onclick: move |_| {
                                crate::app::services::workspace_service::handle_zip_export(export_xml(), config_toml(), status_msg);
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

            // ── Static page export ──────────────────────────────────────
            div {
                class: "editor-panel",
                style: "border: 1px solid var(--editor-border); border-radius: var(--radius-md); padding: 16px; background: var(--bg-panel);",
                h3 { style: "margin: 0 0 4px 0; font-size: 1rem; color: var(--fg-base);", "Static Page Export" }
                p { style: "margin: 0 0 14px 0; font-size: 0.8rem; color: var(--fg-muted); line-height: 1.5;",
                    "Export a single page's generated HTML to a file. Paste it into Blogger's Pages editor (HTML view) to match your active theme."
                }
                div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 10px;",
                    select {
                        class: "editor-input",
                        style: "max-width: 200px;",
                        onchange: move |evt| selected_page.set(evt.value()),
                        for (id, label) in PAGE_OPTIONS.iter().copied() {
                            option { value: "{id}", selected: selected_page() == id, "{label}" }
                        }
                    }
                    button {
                        class: "editor-button editor-button-good",
                        onclick: move |_| {
                            let pages = (theme.signals.static_pages)();
                            let sel = selected_page();
                            let html = match sel.as_str() {
                                "Archive" => generate_archive_html(&pages.archive),
                                "Directory" => generate_categories_html(&pages.categories),
                                "Portfolio" => generate_portfolio_html(&pages.portfolio),
                                "About" => generate_about_html(&pages.about),
                                "LMS" => generate_course_catalog_html(&pages.lms),
                                "MyCourses" => generate_my_courses_html(&pages.lms),
                                _ => String::new(),
                            };
                            if html.is_empty() {
                                status_msg.set(format!("No HTML available for {}", sel));
                                return;
                            }
                            let default_name = format!("{}.html", sel.to_lowercase());
                            spawn(async move {
                                if let Some(handle) = rfd::AsyncFileDialog::new()
                                    .add_filter("HTML", &["html"])
                                    .set_file_name(default_name)
                                    .save_file()
                                    .await
                                {
                                    match std::fs::write(handle.path(), &html) {
                                        Ok(_) => status_msg.set(format!("Exported {} page \u{2192} {}", sel, handle.path().display())),
                                        Err(e) => status_msg.set(format!("Export failed: {}", e)),
                                    }
                                }
                            });
                        },
                        "Export Page HTML"
                    }
                }
            }

            // ── Share creations ─────────────────────────────────────────
            div {
                class: "editor-panel",
                style: "border: 1px solid var(--editor-border); border-radius: var(--radius-md); padding: 16px; background: var(--bg-panel);",
                h3 { style: "margin: 0 0 4px 0; font-size: 1rem; color: var(--fg-base);", "Share Creations" }
                p { style: "margin: 0 0 14px 0; font-size: 0.8rem; color: var(--fg-muted); line-height: 1.5;",
                    "Package a custom widget or template module as a .zip (XML + manifest.json + README) — ready to upload to itch.io or open as a compendium PR."
                }

                // Widget row
                div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 10px; margin-bottom: 10px;",
                    span { style: "font-size: 0.8rem; color: var(--fg-muted); min-width: 56px;", "Widget" }
                    select {
                        class: "editor-input",
                        style: "max-width: 260px;",
                        onchange: move |evt| sel_widget.set(evt.value().parse().unwrap_or(0)),
                        for (i, bp) in widgets().iter().enumerate() {
                            option { value: "{i}", selected: sel_widget() == i, "{bp.group}/{bp.name}" }
                        }
                    }
                    button {
                        class: "editor-button editor-button-good",
                        onclick: move |_| {
                            let list = widgets();
                            let Some(bp) = list.get(sel_widget()).cloned() else {
                                status_msg.set("No widget to export.".to_string());
                                return;
                            };
                            let slots = crate::ui::workspace::widget_layout::parse_slots(&bp.xml);
                            let (w_type, title) = slots.first().map(|s| (s.w_type.clone(), s.title.clone())).unwrap_or_default();
                            let name = if title.trim().is_empty() { bp.name.clone() } else { title };
                            let manifest = format!(
                                "{{\n  \"name\": \"{}\",\n  \"type\": \"{}\",\n  \"group\": \"{}\",\n  \"description\": \"\",\n  \"author\": \"\",\n  \"version\": \"1.0.0\",\n  \"tags\": []\n}}\n",
                                json_escape(&name), json_escape(&w_type), json_escape(&bp.group)
                            );
                            let readme = format!(
                                "# {name}\n\nA custom Blogger widget for the MorBlogger Theme Editor.\n\n## Install\n- Editor: Widgets dock → + New Widget → paste `widget.xml` into the Code tab.\n- Blogger: Layout → Add a Gadget → HTML/JavaScript, or include in your theme XML.\n\n## Customize\nValues marked `data-mor-field` are editable in the Widget Workbench Settings form.\n"
                            );
                            let files = vec![
                                ("widget.xml".to_string(), bp.xml.clone()),
                                ("manifest.json".to_string(), manifest),
                                ("README.md".to_string(), readme),
                            ];
                            let default_name = format!("{}.zip", bp.name);
                            let mut status = status_msg;
                            spawn(async move {
                                if let Some(handle) = rfd::AsyncFileDialog::new()
                                    .add_filter("Zip", &["zip"])
                                    .set_file_name(default_name)
                                    .save_file()
                                    .await
                                {
                                    match mor_blogger_core::render::theme::write_files_zip(handle.path(), &files) {
                                        Ok(_) => status.set(format!("Exported widget \u{2192} {}", handle.path().display())),
                                        Err(e) => status.set(format!("Export failed: {e}")),
                                    }
                                }
                            });
                        },
                        "Export Widget (.zip)"
                    }
                }

                // Module row
                div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 10px;",
                    span { style: "font-size: 0.8rem; color: var(--fg-muted); min-width: 56px;", "Module" }
                    select {
                        class: "editor-input",
                        style: "max-width: 260px;",
                        onchange: move |evt| sel_module.set(evt.value().parse().unwrap_or(0)),
                        for (i, m) in modules().iter().enumerate() {
                            option { value: "{i}", selected: sel_module() == i, "{m.category}/{m.name}" }
                        }
                    }
                    button {
                        class: "editor-button editor-button-good",
                        onclick: move |_| {
                            let list = modules();
                            let Some(m) = list.get(sel_module()).cloned() else {
                                status_msg.set("No module to export.".to_string());
                                return;
                            };
                            let manifest = format!(
                                "{{\n  \"name\": \"{}\",\n  \"kind\": \"module\",\n  \"category\": \"{}\",\n  \"description\": \"\",\n  \"author\": \"\",\n  \"version\": \"1.0.0\",\n  \"tags\": []\n}}\n",
                                json_escape(&m.name), json_escape(&m.category)
                            );
                            let readme = format!(
                                "# {}\n\nA custom Blogger template module ({}) for the MorBlogger Theme Editor.\n\n## Install\nOpen the editor's Module Workbench and import `module.xml`, or drop it into your `workspace/{}/` folder.\n",
                                m.name, m.category, m.category
                            );
                            let files = vec![
                                ("module.xml".to_string(), m.xml.clone()),
                                ("manifest.json".to_string(), manifest),
                                ("README.md".to_string(), readme),
                            ];
                            let default_name = format!("{}.zip", m.name);
                            let mut status = status_msg;
                            spawn(async move {
                                if let Some(handle) = rfd::AsyncFileDialog::new()
                                    .add_filter("Zip", &["zip"])
                                    .set_file_name(default_name)
                                    .save_file()
                                    .await
                                {
                                    match mor_blogger_core::render::theme::write_files_zip(handle.path(), &files) {
                                        Ok(_) => status.set(format!("Exported module \u{2192} {}", handle.path().display())),
                                        Err(e) => status.set(format!("Export failed: {e}")),
                                    }
                                }
                            });
                        },
                        "Export Module (.zip)"
                    }
                }
            }
        }
    }
}
