use dioxus::prelude::*;

use super::state::{CenterView, LayoutState, PluginManagerContext, RenderState, ThemeState};
use crate::app::config_bridge::{CompendiumManifest, EditorPrefs};
use crate::ui::layout::menu_bar::AppMenuBar;
use crate::ui::layout::theme::{get_native_os_theme, MorStyleProvider};
use crate::ui::layout::window_frame::{MorHeaderBar, MorShell, MorWindowTitle};

use super::shell_dialogs::MorDialogs;
use super::shell_file_actions;
use super::shell_layout::{DockLocalSignals, FloatingWindowManager, MorLayoutChrome};

const EDITOR_UI_CSS: &str = include_str!("../../assets/css/editor_ui.css");

fn fallback_compendium() -> Vec<CompendiumManifest> {
    vec![CompendiumManifest {
        id: "os_chameleon".to_string(),
        display_name: "OS Chameleon (Dark Mode)".to_string(),
        version: "1.0.0".to_string(),
        description:
            "Automatically toggles dark mode based on the user's OS preference. (Offline Fallback)"
                .to_string(),
        payload_url: "".to_string(),
    }]
}

#[derive(Clone, Copy)]
pub struct WorkbenchEditState {
    pub edited_xml: Signal<String>,
    pub module_xml_signal: Signal<String>,
    pub workbench_status: Signal<String>,
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
            .unwrap_or_else(|| get_native_os_theme().to_string())
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

    let mut active_view = use_signal(|| "preview");
    use_effect(move || {
        if center_view() == CenterView::ModuleWorkbench {
            active_view.set("workbench");
        } else {
            active_view.set("preview");
        }
    });

    let edited_xml = use_signal(String::new);
    let module_xml_signal = use_signal(String::new);
    let workbench_status = use_signal(String::new);

    provide_context(WorkbenchEditState {
        edited_xml,
        module_xml_signal,
        workbench_status,
    });

    provide_context(DockLocalSignals {
        show_preview,
        show_undocked_pages,
        tv_monitor,
    });

    rsx! {
        script { dangerous_inner_html: "document.addEventListener('contextmenu', event => event.preventDefault());" }
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
                        shell_file_actions::export_xml(&(render.generated_xml)());
                    },
                    on_export_zip: move |_| {
                        shell_file_actions::export_zip(&(render.generated_xml)(), &config_toml_signal());
                    },
                    on_copy_xml: move |_| {
                        shell_file_actions::copy_xml();
                    },
                    on_toggle_preview: move |_| {
                        if center_view() == CenterView::Preview {
                            center_view.set(CenterView::CodeEditor);
                        } else {
                            center_view.set(CenterView::Preview);
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
            }

            FloatingWindowManager {
                show_preview,
                show_undocked_pages,
                tv_monitor,
                active_view,
            }
        }
    }
}
