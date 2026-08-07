use dioxus::prelude::*;

use super::state::{
    CenterView, DockPosition, LayoutState, PluginManagerContext, RenderState, ThemeState,
};
use mor_blogger_core::diagnostics::Severity;
use crate::app::config_bridge::{CompendiumManifest, EditorPrefs};
use crate::ui::layout::menu_bar::AppMenuBar;
use crate::ui::layout::theme::MorStyleProvider;
use crate::ui::layout::window_frame::{MorHeaderBar, MorShell, MorWindowTitle};

use super::shell_dialogs::MorDialogs;
use super::shell_file_actions;
use super::shell_layout::{DockLocalSignals, FloatingWindowManager, MorLayoutChrome};

const EDITOR_UI_CSS: &str = include_str!("../../assets/css/editor_ui.css");
use crate::ui::components::code_editor::CM6_BUNDLE_JS;

fn fallback_compendium() -> Vec<CompendiumManifest> {
    vec![
        CompendiumManifest {
            id: "os_chameleon".to_string(),
            display_name: "OS Chameleon (Dark Mode)".to_string(),
            version: "1.0.0".to_string(),
            description:
                "Automatically toggles dark mode based on the user's OS preference. (Offline Fallback)"
                    .to_string(),
            payload_url: "".to_string(),
        },
        CompendiumManifest {
            id: "notification_bell".to_string(),
            display_name: "Notification Bell (Newest Post)".to_string(),
            version: "1.0.0".to_string(),
            description:
                "A header bell that opens a dropdown previewing your newest post. Colors follow the active theme."
                    .to_string(),
            payload_url: "".to_string(),
        },
    ]
}

#[derive(Clone, Copy)]
pub struct WorkbenchEditState {
    pub edited_xml: Signal<String>,
    pub workbench_status: Signal<String>,
    /// Set by the Widgets dock to ask the workbench to open a blueprint for editing.
    /// The workbench consumes it (loads the buffer) and resets it to None.
    pub edit_widget_request:
        Signal<Option<mor_blogger_core::utils::fs_bridge::WidgetBlueprint>>,
    /// Set by the Widgets dock to insert a blueprint into the active module. The
    /// workbench consumes it and resets to None.
    pub add_widget_request:
        Signal<Option<mor_blogger_core::utils::fs_bridge::WidgetBlueprint>>,
    /// Where an added gadget lands: Some(socket_key) → that slot; None → appended
    /// to the active module buffer. Set by the workbench's "+ Add" buttons.
    pub add_target: Signal<Option<String>>,
    /// Set by the Template Modules dock's file picker or a file dropped onto the
    /// workspace: the XML to load into the active slot's editor buffer (the sender
    /// also sets `active_workbench_module`). Non-destructive — edits the buffer
    /// until the user hits Save. The workbench consumes it and resets to None.
    pub load_module_request: Signal<Option<String>>,
}

/// VS Code-style bottom status strip, three zones:
/// left = health (diagnostics count + unsaved dot), center = transient workbench
/// messages (`WorkbenchEditState::workbench_status`, otherwise unrendered), right =
/// active-workspace context, exported-XML size vs Blogger's ~1 MB cap, active preset.
#[component]
fn StatusBar(config_toml_signal: Memo<String>, original_toml: Signal<String>) -> Element {
    let mut layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let theme = use_context::<ThemeState>();
    let edit_state = use_context::<WorkbenchEditState>();

    let result = render.diag.read();
    let error_count = result.errors.len();
    let warning_count = result
        .warnings
        .iter()
        .filter(|w| w.severity == Severity::Warning)
        .count();
    let clean = error_count == 0 && warning_count == 0;
    // Surface the first problem in the tooltip so the count is actionable on hover.
    let diag_title = result
        .errors
        .first()
        .cloned()
        .or_else(|| {
            result
                .warnings
                .iter()
                .find(|w| w.severity == Severity::Warning)
                .map(|w| w.message.clone())
        })
        .map(|m| format!("{m} — click for all diagnostics"))
        .unwrap_or_else(|| "Theme Diagnostics — click to open panel".to_string());

    let dirty = config_toml_signal() != original_toml();
    let message = (edit_state.workbench_status)();

    // What the active workspace is looking at.
    let context = match (layout.center_view)() {
        CenterView::Preview | CenterView::Split => format!(
            "{} \u{00b7} {}px \u{00b7} {}",
            (layout.preview_viewport)().label(),
            (layout.preview_width)(),
            (layout.preview_template_mode)().label(),
        ),
        CenterView::CodeEditor => if (layout.code_show_xml)() {
            "compiled XML".to_string()
        } else {
            "config TOML".to_string()
        },
        CenterView::ModuleWorkbench => match (layout.active_workbench_module)() {
            Some(key) => {
                let name =
                    crate::ui::layout::docks::template_editor_dock::slot_display_name(key);
                if mor_blogger_core::render::template_resolver::module_override(key).is_some() {
                    format!("{name} \u{00b7} customized")
                } else {
                    name.to_string()
                }
            }
            None => "no module selected".to_string(),
        },
        CenterView::StaticPageEditor => {
            (layout.active_static_page)().unwrap_or_else(|| "no page selected".to_string())
        }
        _ => String::new(),
    };

    // Blogger rejects theme XML over ~1 MB; warn as the export approaches it.
    let xml_bytes = render.generated_xml.read().len();
    let size_color = if xml_bytes > 1_000_000 {
        "#ea8285"
    } else if xml_bytes > 900_000 {
        "#d29922"
    } else {
        "inherit"
    };

    let preset_label = (theme.active_preset)()
        .map(|id| {
            mor_blogger_core::presets::all_presets()
                .iter()
                .find(|p| p.id == id)
                .map(|p| p.name)
                .unwrap_or(id)
        })
        .unwrap_or("no preset");

    rsx! {
        div {
            class: "editor-statusbar",
            style: "flex-shrink: 0; display: flex; align-items: center; gap: 8px; height: 24px; padding: 0 8px; font-size: 0.72rem; background: var(--mor-header, #10161f); border-top: 1px solid var(--mor-border, #2a2a2a); color: var(--editor-text, #ddd); user-select: none; white-space: nowrap;",
            button {
                class: "statusbar-diagnostics",
                title: "{diag_title}",
                style: "display: flex; align-items: center; gap: 12px; background: transparent; border: none; color: inherit; font: inherit; cursor: pointer; padding: 2px 6px; border-radius: 3px;",
                onclick: move |_| {
                    if (layout.diagnostics_pos)() == DockPosition::Hidden {
                        layout.request_dock("diagnostics", DockPosition::mor_panel_left);
                    } else {
                        layout.diagnostics_pos.set(DockPosition::Hidden);
                    }
                },
                if clean {
                    span { style: "color: #73c991;", "\u{2713} no problems" }
                } else {
                    span { style: "color: #ea8285; font-weight: bold;", "\u{2298} {error_count}" }
                    span { style: "color: #d29922; font-weight: bold;", "\u{25B3} {warning_count}" }
                }
            }
            if dirty {
                span {
                    title: "Theme config has unsaved changes (File \u{2192} Save Theme)",
                    style: "color: #d29922;",
                    "\u{25cf} unsaved"
                }
            }

            // Center: transient workbench/dock messages; click to dismiss.
            div {
                style: "flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; text-align: center;",
                if !message.is_empty() {
                    button {
                        title: "Click to dismiss",
                        style: "background: transparent; border: none; color: inherit; font: inherit; cursor: pointer; opacity: 0.85; max-width: 100%; overflow: hidden; text-overflow: ellipsis;",
                        onclick: move |_| {
                            let mut status = edit_state.workbench_status;
                            status.set(String::new());
                        },
                        "{message}"
                    }
                }
            }

            if !context.is_empty() {
                span { style: "opacity: 0.8;", "{context}" }
            }
            span {
                title: "Exported theme XML size \u{2014} Blogger caps themes at ~1 MB",
                style: "color: {size_color}; opacity: 0.9;",
                "{xml_bytes / 1024} KB"
            }
            button {
                title: "Active preset \u{2014} click to open Presets",
                style: "background: transparent; border: none; color: inherit; font: inherit; cursor: pointer; padding: 2px 6px; border-radius: 3px; opacity: 0.9;",
                onclick: move |_| {
                    if (layout.presets_pos)() == DockPosition::Hidden {
                        layout.request_dock("presets", DockPosition::mor_panel_left);
                    } else {
                        layout.presets_pos.set(DockPosition::Hidden);
                    }
                },
                "\u{25c6} {preset_label}"
            }
        }
    }
}

pub fn render_app_shell(
    theme: ThemeState,
    mut layout: LayoutState,
    render: RenderState,
) -> Element {
    let _signals = theme.signals;
    let current_config = render.current_config;
    let active_preset = theme.active_preset;
    let mut center_view = layout.center_view;

    provide_context(render);

    let show_preview = use_signal(|| true);
    let show_undocked_pages = use_signal(|| false);

    let show_about = use_signal(|| false);
    let show_prefs = use_signal(|| false);
    let show_editor_settings = use_signal(|| false);
    let show_shortcuts = use_signal(|| false);
    let show_docs = use_signal(|| false);

    let prefs = use_signal(|| EditorPrefs::load());

    // Editor minimap: global default (Editor Settings → General) plus per-workspace
    // overrides, both seeded from prefs and provided as context so toggles apply live.
    let minimap_setting = use_signal(|| prefs().show_minimap.unwrap_or(true));
    provide_context(crate::ui::components::code_editor::MinimapSetting(minimap_setting));
    let minimap_overrides = use_signal(|| prefs().minimap_overrides.clone());
    provide_context(crate::ui::components::code_editor::MinimapOverrides(minimap_overrides));
    let launch_plugins = use_signal(|| prefs().plugins.clone());
    let current_plugins = use_signal(|| prefs().plugins.clone());
    let mut compendium_registry = use_signal(|| Vec::<CompendiumManifest>::new());

    provide_context(PluginManagerContext {
        launch_plugins,
        current_plugins,
        compendium_registry,
    });

    use_effect(|| {
        if let Err(e) = mor_blogger_core::utils::fs_bridge::init_template_dirs() {
            log::warn!("[startup] Template dir init failed: {}", e);
        }
    });

    use_effect(move || {
        let plugins = current_plugins();
        let mut p = EditorPrefs::load();
        if p.plugins != plugins {
            p.plugins = plugins;
            let _ = p.save();
        }
    });

    use_effect(move || {
        spawn(async move {
            let target_url = "https://raw.githubusercontent.com/MoribundInstitute/mor-blogger-theme-editor-plugin-compendium/main/registry.json";
            let Ok(res) = reqwest::get(target_url).await else {
                log::warn!("Network request completely failed. Triggering fallback.");
                compendium_registry.set(fallback_compendium());
                return;
            };

            if !res.status().is_success() {
                log::warn!(
                    "GitHub returned a {} status. Triggering fallback.",
                    res.status()
                );
                compendium_registry.set(fallback_compendium());
                return;
            }

            let Ok(remote_registry) = res.json::<Vec<CompendiumManifest>>().await else {
                compendium_registry.set(fallback_compendium());
                return;
            };

            compendium_registry.set(remote_registry);
        });
    });

    let active_ui_mode =
        std::env::var("MOR_ACTIVE_UI_MODE").unwrap_or_else(|_| "frameless".to_string());
    let ui_mode_pref = use_signal(|| {
        prefs()
            .ui_mode
            .clone()
            .unwrap_or_else(|| active_ui_mode.clone())
    });
    let ui_theme_pref = use_signal(|| {
        prefs()
            .workspace_theme
            .clone()
            .unwrap_or_else(|| crate::ui::layout::theme::MOR_STUDIO_TOML.to_string())
    });
    let show_window_buttons = active_ui_mode == "frameless";
    let show_custom_title = active_ui_mode != "native";

    let original_toml =
        use_signal(|| toml::to_string_pretty(&current_config()).unwrap_or_default());
    let config_toml_signal = use_memo(move || {
        let updated = current_config();
        mor_blogger_core::config::update_toml_preserve_comments(&original_toml(), &updated)
    });
    let mut tv_monitor = use_signal(|| String::new());

    use_effect(move || {
        tv_monitor.set((render.preview_html)());
    });

    let edited_xml = use_signal(String::new);
    let workbench_status = use_signal(String::new);

    provide_context(WorkbenchEditState {
        edited_xml,
        workbench_status,
        edit_widget_request: use_signal(|| None),
        add_widget_request: use_signal(|| None),
        add_target: use_signal(|| None),
        load_module_request: use_signal(|| None),
    });

    provide_context(DockLocalSignals {
        show_preview,
        show_undocked_pages,
        tv_monitor,
    });

    rsx! {
        script { dangerous_inner_html: "document.addEventListener('contextmenu', event => event.preventDefault());" }
        script { dangerous_inner_html: "{CM6_BUNDLE_JS}" }
        // Theme-aware JS completions (read lazily by the CM6 bundle's js mode).
        script { dangerous_inner_html: "window.MOR_JS_HINTS = {mor_blogger_core::render::js_behaviors::editor_hints_json()};" }
        MorStyleProvider { theme_toml: ui_theme_pref() }
        style { "{EDITOR_UI_CSS}" }

        MorShell {
            if active_ui_mode != "native" {
                MorHeaderBar {
                    show_controls: show_window_buttons,
                    start: rsx! { div { style: "width: 16px;" } },
                    center: rsx! {
                        if show_custom_title {
                            MorWindowTitle {
                                title: "MorBlogger Theme Editor".to_string(),
                                subtitle: Some(format!("{} Mode", active_ui_mode))
                            }
                        }
                    },
                    end: rsx! { div { style: "width: 16px;" } }
                }
            }

            MorDialogs {
                show_about,
                show_prefs,
                show_editor_settings,
                show_shortcuts,
                show_docs,
                ui_mode_pref,
                ui_theme_pref,
                active_ui_mode: active_ui_mode.clone(),
            }

            div { class: "editor-shell", style: "height: 100%;",
                AppMenuBar {
                    show_prefs,
                    show_editor_settings,
                    show_about,
                    show_shortcuts,
                    show_docs,

                    on_new_workspace: move |_| {
                        shell_file_actions::new_workspace(theme, original_toml);
                    },
                    on_load_theme: move |_| {
                        shell_file_actions::load_theme(theme, original_toml);
                    },
                    on_save_theme: move |_| {
                        shell_file_actions::save_theme(&config_toml_signal(), original_toml);
                    },
                    on_import_xml: move |_| {
                        shell_file_actions::import_xml(theme, original_toml);
                    },
                    on_load_data: move |_| {
                        shell_file_actions::load_data(current_config(), theme);
                    },
                    on_save_data: move |_| {
                        shell_file_actions::save_data(current_config());
                    },
                    on_export_xml: move |_| {
                        shell_file_actions::export_xml(&render.export_xml_now());
                    },
                    on_export_zip: move |_| {
                        shell_file_actions::export_zip(&render.export_xml_now(), &config_toml_signal());
                    },
                    on_copy_xml: move |_| {
                        shell_file_actions::copy_xml();
                    },
                    on_toggle_preview: move |_| {
                        // Route through enter_workspace so dock state (incl. Code Nav)
                        // stays consistent however you reach the Code Editor.
                        if center_view() == CenterView::Preview {
                            layout.enter_workspace(CenterView::CodeEditor);
                        } else {
                            layout.enter_workspace(CenterView::Preview);
                        }
                    },
                    on_toggle_split: move |_| { center_view.set(CenterView::Split); },
                    on_reset_viewport: move |_| { layout.preview_width.set(1200u32); },
                }

                MorLayoutChrome {
                    show_preview,
                    center_view,
                    tv_monitor,
                    config_toml_signal,
                    active_preset,
                    original_toml,
                }

                StatusBar { config_toml_signal, original_toml }
            }

            FloatingWindowManager {
                show_preview,
                show_undocked_pages,
                tv_monitor,
            }
        }
    }
}
