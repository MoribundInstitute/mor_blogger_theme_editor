use dioxus::prelude::*;

// 1. IMPORT THE LOCAL UI KIT PIECES

use super::hotswap::apply_hotswap_json;
use super::state::{CenterView, DockPosition, LayoutState, RenderState, ThemeState};
use crate::app::config_bridge::{CompendiumManifest, EditorPrefs, PluginState};
use crate::ui::components::icon_context_menu::IconContextMenu;
use crate::ui::components::icons::{IconCode, IconPalette, IconSiteData};
use crate::ui::dialogs::about_dialog::AboutDialog;
use crate::ui::dialogs::advanced_colors_dialog::AdvancedColorsDialog;
use crate::ui::dialogs::advanced_presets_dialog::AdvancedPresetsDialog;
use crate::ui::dialogs::advanced_typography_dialog::AdvancedTypographyDialog;
use crate::ui::dialogs::css_token_builder_dialog::CssTokenBuilderDialog;
use crate::ui::dialogs::diagnostics_dialog::DiagnosticsDialog;
use crate::ui::dialogs::documentation_dialog::DocumentationDialog;
use crate::ui::dialogs::js_behavior_builder_dialog::JsBehaviorBuilderDialog;
use crate::ui::dialogs::plugin_manager_dialog::PluginManagerDialog;
use crate::ui::dialogs::shortcuts_dialog::ShortcutsDialog;
use crate::ui::dialogs::template_grid_dialog::TemplateGridDialog;
use crate::ui::dialogs::user_preferences_dialog::UserPreferencesDialog;
use crate::ui::dialogs::workspace_settings_dialog::WorkspaceSettingsDialog;
use crate::ui::panels::theme_palette::effects_panel_2::AdvancedGlowWindow;
use crate::ui::panels::theme_palette::presets::{PresetFloatingWindow, PresetsPanel};
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesFloatingWindow;
use crate::ui::shell::docks::{CssEditorPanel, JsEditorPanel, SiteDataDock, ThemePaletteDock};
use crate::ui::shell::menu_bar::AppMenuBar;
use crate::ui::shell::theme::{get_native_os_theme, MorStyleProvider};
use crate::ui::shell::window_frame::{MorHeaderBar, MorShell, MorWindowTitle};
use crate::ui::workspace::blogger_workspace::BloggerWorkspace;
use mor_blogger_core::config::ThemeConfig;

const EDITOR_UI_CSS: &str = include_str!("../editor_ui.css");

#[derive(Clone, Copy)]
struct DockLocalSignals {
    show_preview: Signal<bool>,
    show_undocked_pages: Signal<bool>,
    tv_monitor: Signal<String>,
    launch_plugins: Signal<Vec<PluginState>>,
    current_plugins: Signal<Vec<PluginState>>,
    compendium_registry: Signal<Vec<CompendiumManifest>>,
}

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

#[component]
fn DockZone(position: DockPosition) -> Element {
    let layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();
    let render = use_context::<RenderState>();
    let local = use_context::<DockLocalSignals>();

    let active_tab = match position {
        DockPosition::Left => layout.active_left_tab,
        _ => layout.active_right_tab,
    };

    let show_theme_palette = (layout.theme_palette_pos)() == position;
    let show_site_data = (layout.site_data_pos)() == position;
    let show_presets = (layout.presets_pos)() == position;
    let show_css_editor = (layout.css_editor_pos)() == position;

    let show_js_editor = (layout.js_editor_pos)() == position;

    if show_css_editor {
        return rsx! {
            CssEditorPanel {}
        };
    }

    if show_js_editor {
        return rsx! {
            JsEditorPanel {}
        };
    }

    if show_theme_palette {
        return rsx! {
            ThemePaletteDock {
                active_tab,
                active_preset: theme.active_preset,
                signals: theme.signals,
                show_preview: local.show_preview,
                current_config: (render.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    let mut theme = theme;
                    theme.signals.apply_config(&new_config);
                    theme.active_preset.set(None);
                    theme.commit();
                },
                show_undocked_presets: theme.show_undocked_presets,
                show_undocked_pages: local.show_undocked_pages,
                show_advanced_glow: theme.show_advanced_glow,
                preview_html: local.tv_monitor,
                base_preview_html: render.preview_html,
            }
        };
    }

    if show_site_data {
        return rsx! {
            SiteDataDock {
                active_tab,
                signals: theme.signals,
                current_config: (render.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    theme.signals.apply_config(&new_config);
                },
            }
        };
    }

    if show_presets {
        return rsx! {
            PresetsPanel {
                active_preset: theme.active_preset,
                signals: theme.signals,
                current_config: (render.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    let mut theme = theme;
                    theme.signals.apply_config(&new_config);
                    theme.active_preset.set(None);
                    theme.commit();
                },
                show_undocked_presets: theme.show_undocked_presets,
            }
        };
    }

    rsx! {}
}

pub fn render_app_shell(
    theme: ThemeState,
    mut layout: LayoutState,
    render: RenderState,
) -> Element {
    let signals = theme.signals;
    let current_config = render.current_config;
    let mut active_preset = theme.active_preset;
    let mut center_view = layout.center_view;

    provide_context(render);

    // Active panels are determined reactively based on center_view()

    let show_preview = use_signal(|| true);

    let show_undocked_presets = theme.show_undocked_presets;
    let show_undocked_pages = use_signal(|| false);
    let show_advanced_glow = theme.show_advanced_glow;

    let show_about = use_signal(|| false);
    let show_prefs = use_signal(|| false);
    let show_editor_settings = use_signal(|| false);
    let show_shortcuts = use_signal(|| false);
    let show_css_builder = use_signal(|| false);
    let show_js_builder = use_signal(|| false);
    let show_docs = use_signal(|| false);
    let show_diagnostics = use_signal(|| false);
    let show_plugin_manager = use_signal(|| false);

    let prefs = use_signal(|| EditorPrefs::load());
    let launch_plugins = use_signal(|| prefs().plugins.clone());
    let current_plugins = use_signal(|| prefs().plugins.clone());
    let mut compendium_registry = use_signal(|| Vec::<CompendiumManifest>::new());

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

    let mut original_toml =
        use_signal(|| toml::to_string_pretty(&current_config()).unwrap_or_default());
    let config_toml_signal = use_memo(move || {
        let updated = current_config();
        mor_blogger_core::config::update_toml_preserve_comments(&original_toml(), &updated)
    });
    let mut tv_monitor = use_signal(|| String::new());

    use_effect(move || {
        tv_monitor.set((render.preview_html)());
    });

    provide_context(DockLocalSignals {
        show_preview,
        show_undocked_pages,
        tv_monitor,
        launch_plugins,
        current_plugins,
        compendium_registry,
    });

    let has_left_dock = (layout.theme_palette_pos)() == DockPosition::Left
        || (layout.site_data_pos)() == DockPosition::Left
        || (layout.css_editor_pos)() == DockPosition::Left;

    let has_right_dock = (layout.theme_palette_pos)() == DockPosition::Right
        || (layout.site_data_pos)() == DockPosition::Right
        || (layout.css_editor_pos)() == DockPosition::Right;

    let left_active = has_left_dock;
    let right_active = has_right_dock;

    let left_width = if left_active {
        "var(--left-pane-width, 360px)"
    } else {
        "0px"
    };
    let right_width = if right_active {
        "var(--right-pane-width, 360px)"
    } else {
        "0px"
    };

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

            AboutDialog { open: show_about }

            UserPreferencesDialog {
                show_prefs,
                ui_mode_pref,
                active_ui_mode: active_ui_mode.clone(),
            }

            WorkspaceSettingsDialog {
                open: show_editor_settings,
                active_theme_toml: ui_theme_pref,
            }

            ShortcutsDialog { open: show_shortcuts }

            CssTokenBuilderDialog { open: show_css_builder }
            JsBehaviorBuilderDialog { open: show_js_builder }

            DocumentationDialog { open: show_docs }

            DiagnosticsDialog {
                open: show_diagnostics,
                result: render.diag,
            }

            PluginManagerDialog {
                show_panel: show_plugin_manager,
                launch_state: launch_plugins,
                current_state: current_plugins,
                compendium_registry,
            }

            div { class: "editor-shell", style: "height: 100%;",

                AppMenuBar {
                    show_prefs,
                    show_editor_settings,
                    show_about,
                    show_shortcuts,
                    show_plugin_manager,
                    show_css_builder,
                    show_js_builder,
                    show_diagnostics,
                    show_docs,

                    on_new_workspace: move |_| {
                        let fresh_prefs = crate::app::config_bridge::EditorPrefs::load();
                        let mut config = mor_blogger_core::config::defaults::default_theme_config();
                        if let Some(pack) = fresh_prefs.default_template_pack {
                            config.template_pack = pack;
                        }
                        signals.apply_config(&config);
                        original_toml.set(toml::to_string_pretty(&config).unwrap_or_default());
                        active_preset.set(None);
                        theme.commit();
                    },
                    on_load_theme: move |_| {
                        if let Some(content) = crate::utils::io::load_toml() {
                            if let Ok(new_config) = toml::from_str::<ThemeConfig>(&content) {
                                signals.apply_config(&new_config);
                                original_toml.set(content);
                            }
                        }
                    },
                    on_save_theme: move |_| {
                        let toml_str = config_toml_signal();
                        crate::utils::io::save_toml(&toml_str);
                        original_toml.set(toml_str);
                    },

                    on_import_xml: {
                        let sigs = signals.clone();
                        move |_| {
                            let sigs = sigs.clone();
                            spawn(async move {
                                let Some(file) = rfd::AsyncFileDialog::new()
                                    .set_title("Import Blogger XML")
                                    .add_filter("XML", &["xml"])
                                    .pick_file()
                                    .await else { return; };
                                let Ok(xml_str) = std::fs::read_to_string(file.path()) else { return; };
                                match mor_blogger_core::utils::rehydration::extract_and_decode(&xml_str) {
                                    Ok(restored_config) => {
                                        sigs.apply_config(&restored_config);
                                        original_toml.set(toml::to_string_pretty(&restored_config).unwrap_or_default());
                                    }
                                    Err(e) => log::error!("Failed to import XML: {}", e),
                                }
                            });
                        }
                    },
                    on_load_data: {
                        let current = current_config();
                        let sigs = signals.clone();
                        move |_| {
                            let mut current_cfg = current.clone();
                            let sigs = sigs.clone();
                            spawn(async move {
                                let Some(file) = rfd::AsyncFileDialog::new().set_title("Load Site Data").add_filter("JSON", &["json"]).pick_file().await else { return; };
                                let Ok(json) = std::fs::read_to_string(file.path()) else { return; };
                                let Ok(loaded_data) = serde_json::from_str::<ThemeConfig>(&json) else { return; };
                                current_cfg.apply_site_data(&loaded_data);
                                sigs.apply_config(&current_cfg);
                            });
                        }
                    },
                    on_save_data: {
                        let current = current_config();
                        move |_| {
                            let current_cfg = current.clone();
                            spawn(async move {
                                let Some(file) = rfd::AsyncFileDialog::new().set_file_name("my_site_data.json").add_filter("JSON", &["json"]).save_file().await else { return; };
                                let Ok(json) = serde_json::to_string_pretty(&current_cfg) else { return; };
                                let _ = std::fs::write(file.path(), json);
                            });
                        }
                    },
                    on_export_xml: move |_| { crate::utils::io::save_xml(&(render.generated_xml)()); },
                    on_export_zip: move |_| { crate::utils::io::export_bundle(&(render.generated_xml)(), &config_toml_signal()); },
                    on_copy_xml: move |_| {
                        log::info!("Copy XML triggered (requires arboard binding)");
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

                div {
                    class: "editor-main",
                    style: "grid-template-columns: 48px {left_width} 1fr {right_width} 48px;",
                    "data-left-layout": if left_active { "split" } else { "hidden" },
                    "data-right-layout": if right_active { "split" } else { "hidden" },
                    "data-left-pinned": "{left_active}",
                    "data-right-pinned": "{right_active}",

                    // Left Dock Strip - Enforcing Flex Column natively
                    div {
                        class: "cinnamon-dock left",
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px; width: 48px; padding-top: 12px;",

                        button { class: "dock-btn", title: "Theme Palette",
                            onclick: move |_| {
                                if (layout.theme_palette_pos)() == DockPosition::Left {
                                    layout.theme_palette_pos.set(DockPosition::Hidden);
                                } else {
                                    layout.theme_palette_pos.set(DockPosition::Left);
                                }
                            },
                            IconPalette {}
                        }
                        button { class: "dock-btn", title: "CSS Editor",
                            onclick: move |_| {
                                if (layout.css_editor_pos)() == DockPosition::Hidden {
                                    layout.css_editor_pos.set(DockPosition::Floating);
                                } else {
                                    layout.css_editor_pos.set(DockPosition::Hidden);
                                }
                            },
                            IconCode {}
                        }
                    }

                    // Left Panel Data
                    div { class: "panel-container left",
                        DockZone { position: DockPosition::Left }
                    }

                    // Workspace Center
                    BloggerWorkspace {
                        preview_viewport: layout.preview_viewport,
                        preview_width: layout.preview_width,
                        preview_template_mode: layout.preview_template_mode,
                        preview_html: tv_monitor,
                        show_preview,
                        center_view,
                        diag: render.diag,
                        config_toml: config_toml_signal,
                        active_preset,
                        on_load_theme: move |toml_text: String| {
                            if let Ok(new_config) = toml::from_str::<ThemeConfig>(&toml_text) {
                                signals.apply_config(&new_config);
                            }
                        },
                        on_restore: move |new_config: ThemeConfig| {
                            signals.apply_config(&new_config);
                        },
                        on_load_hotswap: move |json_text: String| {
                            apply_hotswap_json(signals, json_text);
                        },
                        on_navigate: move |href: String| {
                            let base = (render.preview_html)();
                            let pages = (signals.static_pages)();

                            let new_html = if href.contains("archive") {
                                mor_blogger_core::render::pages::generate_archive_html(&pages.archive)
                            } else if href.contains("categories") || href.contains("directory") {
                                mor_blogger_core::render::pages::generate_categories_html(&pages.categories)
                            } else if href.contains("about") {
                                mor_blogger_core::render::pages::generate_about_html(&pages.about)
                            } else if href.contains("portfolio") {
                                mor_blogger_core::render::pages::generate_portfolio_html(&pages.portfolio)
                            } else if href.contains("catalog") || href.contains("lessons") || href.contains("courses") {
                                mor_blogger_core::render::pages::generate_course_catalog_html(&pages.lms)
                            } else {
                                tv_monitor.set(base);
                                return;
                            };

                            tv_monitor.set(
                                crate::ui::panels::theme_palette::static_pages_panel::inject_static_page(&base, &new_html),
                            );
                        },
                        on_toggle_dark_mode: {
                            let theme_state = theme;
                            move |_| {
                                theme_state.perform_dark_mode_toggle();
                            }
                        },
                    }

                    // Right Panel Data
                    div { class: "panel-container right",
                        DockZone { position: DockPosition::Right }
                    }

                    // Right Dock Strip - Enforcing Flex Column natively
                    div {
                        class: "cinnamon-dock right",
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px; width: 48px; padding-top: 12px;",

                        button { class: "dock-btn", title: "Site Data",
                            onclick: move |_| {
                                if (layout.site_data_pos)() == DockPosition::Right {
                                    layout.site_data_pos.set(DockPosition::Hidden);
                                } else {
                                    layout.site_data_pos.set(DockPosition::Right);
                                }
                            },
                            IconSiteData {}
                        }
                    }
                }
            }

            if (layout.theme_palette_pos)() == DockPosition::Floating {
                ThemePaletteDock {
                    active_tab: layout.active_left_tab,
                    active_preset: theme.active_preset,
                    signals: theme.signals,
                    show_preview: show_preview,
                    current_config: (render.current_config)(),
                    on_apply_theme: move |new_config: ThemeConfig| {
                        let mut theme = theme;
                        theme.signals.apply_config(&new_config);
                        theme.active_preset.set(None);
                        theme.commit();
                    },
                    show_undocked_presets: theme.show_undocked_presets,
                    show_undocked_pages: show_undocked_pages,
                    show_advanced_glow: theme.show_advanced_glow,
                    preview_html: tv_monitor,
                    base_preview_html: render.preview_html,
                }
            }

            if (layout.site_data_pos)() == DockPosition::Floating {
                SiteDataDock {
                    active_tab: layout.active_right_tab,
                    signals: theme.signals,
                    current_config: (render.current_config)(),
                    on_apply_theme: move |new_config: ThemeConfig| {
                        theme.signals.apply_config(&new_config);
                    },
                }
            }

            if (layout.css_editor_pos)() == DockPosition::Floating {
                CssEditorPanel {}
            }

            if (layout.js_editor_pos)() == DockPosition::Floating {
                JsEditorPanel {}
            }

            if show_undocked_presets() {
                PresetFloatingWindow { signals, active_preset, show_undocked_presets }
            }

            if show_advanced_glow() {
                AdvancedGlowWindow { show_advanced_glow, signals: signals.clone() }
            }

            if show_undocked_pages() {
                StaticPagesFloatingWindow { signals, show_undocked_pages, preview_html: tv_monitor, base_preview_html: render.preview_html }
            }

            if let Some(payload) = (layout.active_context_menu)() {
                IconContextMenu { payload }
            }

            AdvancedPresetsDialog {
                open: theme.show_advanced_presets,
            }

            TemplateGridDialog {
                open: layout.show_advanced_modules,
            }

            AdvancedColorsDialog {
                open_signal: theme.show_advanced_colors,
            }

            AdvancedTypographyDialog {
                open_signal: theme.show_advanced_typography,
            }
        }
    }
}
