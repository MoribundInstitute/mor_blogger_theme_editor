use dioxus::prelude::*;

// 1. IMPORT THE LOCAL UI KIT PIECES
use crate::ui::components::modal::Modal;
use crate::ui::shell::css_builder_modal::CssBuilderModal;
use crate::ui::shell::editor_settings_modal::EditorSettingsModal;
use crate::ui::shell::menu_bar::AppMenuBar;
use crate::ui::shell::prefs_modal::UserPreferencesModal;
use crate::ui::shell::shortcut_modal::KeyboardShortcutsModal;
use crate::ui::shell::theme::{get_native_os_theme, MorStyleProvider};
use crate::ui::shell::window_frame::{MorHeaderBar, MorShell, MorWindowTitle};
// 2. IMPORT THE EDITOR PANELS
use super::config_bridge::panel_layout_class;
use super::hotswap::apply_hotswap_json;
use super::layout_state::{AppLayoutState, CenterView, DockPosition, use_app_layout_state};
use super::render_state::AppRenderState;
use super::state::ThemeAppState;
use crate::app::config_bridge::{CompendiumManifest, EditorPrefs, PluginState};
use crate::ui::panels::diagnostics_panel::DiagnosticsPanel;
use crate::ui::panels::plugin_manager_panel::PluginManagerPanel;
use crate::ui::panels::theme_palette::presets::{PresetFloatingWindow, PresetsPanel};
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesFloatingWindow;
use crate::ui::panels::theme_palette::template_modules::TemplateModulesFloatingWindow;
use crate::ui::panels::theme_palette::effects_panel_2::AdvancedGlowWindow;
use crate::ui::workspace::blogger_workspace::BloggerWorkspace;
use crate::ui::workspace::layout::PanelLayout;
use crate::ui::shell::panes::left_pane::LeftVisualsPanel;
use crate::ui::shell::panes::right_pane::RightDataPanel;
use crate::ui::docks::CssEditorPanel;
use mor_blogger_core::config::ThemeConfig;

const EDITOR_UI_CSS: &str = include_str!("../editor_ui.css");

#[derive(Clone, Copy)]
struct DockLocalSignals {
    show_preview: Signal<bool>,
    show_undocked_pages: Signal<bool>,
    show_undocked_modules: Signal<bool>,
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
        description: "Automatically toggles dark mode based on the user's OS preference. (Offline Fallback)".to_string(),
        payload_url: "".to_string(),
    }]
}

#[component]
fn DockZone(position: DockPosition) -> Element {
    let layout = use_context::<AppLayoutState>();
    let theme = use_context::<ThemeAppState>();
    let render = use_context::<AppRenderState>();
    let local = use_context::<DockLocalSignals>();
    let show_plugin_panel = use_signal(|| true);

    let active_tab = match position {
        DockPosition::Left => layout.active_left_tab,
        _ => layout.active_right_tab,
    };
    let current_layout = match position {
        DockPosition::Left => layout.left_layout,
        _ => layout.right_layout,
    };

    let show_theme_palette = (layout.theme_palette_pos)() == position;
    let show_site_data = (layout.site_data_pos)() == position;
    let show_diagnostics = (layout.diagnostics_pos)() == position;
    let show_plugin_manager = (layout.plugin_manager_pos)() == position;
    let show_presets = (layout.presets_pos)() == position;
    let show_css_editor = (layout.css_editor_pos)() == position;

    let _ = use_app_layout_state;

    if show_css_editor {
        return rsx! {
            CssEditorPanel {}
        };
    }

    if show_theme_palette {
        return rsx! {
            LeftVisualsPanel {
                active_tab,
                layout: current_layout,
                active_preset: theme.active_preset,
                signals: theme.signals,
                show_preview: local.show_preview,
                current_config: (theme.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    let mut theme = theme;
                    theme.signals.apply_config(&new_config);
                    theme.active_preset.set(None);
                    theme.commit();
                },
                show_undocked_presets: theme.show_undocked_presets,
                show_undocked_pages: local.show_undocked_pages,
                show_undocked_modules: local.show_undocked_modules,
                show_advanced_glow: theme.show_advanced_glow,
                preview_html: local.tv_monitor,
                base_preview_html: render.preview_html,
            }
        };
    }

    if show_site_data {
        return rsx! {
            RightDataPanel {
                active_tab,
                layout: current_layout,
                signals: theme.signals,
                current_config: (theme.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    theme.signals.apply_config(&new_config);
                },
            }
        };
    }

    if show_diagnostics {
        return rsx! {
            DiagnosticsPanel { result: render.diag }
        };
    }

    if show_plugin_manager {
        return rsx! {
            PluginManagerPanel {
                show_panel: show_plugin_panel,
                launch_state: local.launch_plugins,
                current_state: local.current_plugins,
                compendium_registry: local.compendium_registry,
            }
        };
    }

    if show_presets {
        return rsx! {
            PresetsPanel {
                active_preset: theme.active_preset,
                signals: theme.signals,
                current_config: (theme.current_config)(),
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
    theme: ThemeAppState,
    mut layout: AppLayoutState,
    render: AppRenderState,
) -> Element {
    let signals = theme.signals;
    let current_config = theme.current_config;
    let mut active_preset = theme.active_preset;
    let mut center_view = theme.center_view;

    provide_context(render);

    use_effect(move || {
        let view = center_view();

        if let CenterView::CodeEditor | CenterView::Export | CenterView::ModuleWorkbench = view {
            layout.left_layout.set(PanelLayout::Hidden);
            layout.right_layout.set(PanelLayout::Hidden);
        }
    });

    let show_preview = use_signal(|| true);

    let show_undocked_presets = theme.show_undocked_presets;
    let show_undocked_pages = use_signal(|| false);
    let show_undocked_modules = use_signal(|| false);
    let show_advanced_glow = theme.show_advanced_glow;

    let mut show_about = use_signal(|| false);
    let show_prefs = use_signal(|| false);
    let show_editor_settings = use_signal(|| false);
    let show_shortcuts = use_signal(|| false);
    let show_css_builder = use_signal(|| false);
    let mut show_docs = use_signal(|| false);

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
                log::warn!("GitHub returned a {} status. Triggering fallback.", res.status());
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

    let config_toml_signal =
        use_memo(move || toml::to_string_pretty(&current_config()).unwrap_or_default());
    let mut tv_monitor = use_signal(|| String::new());

    use_effect(move || {
        tv_monitor.set((render.preview_html)());
    });

    provide_context(DockLocalSignals {
        show_preview,
        show_undocked_pages,
        show_undocked_modules,
        tv_monitor,
        launch_plugins,
        current_plugins,
        compendium_registry,
    });

    rsx! {
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

            Modal {
                title: "About MorBlogger".to_string(), open: show_about, on_close: move |_| show_about.set(false),
                div { class: "editor-note",
                    p { class: "editor-note-title", "Version 0.1.0" }
                    p { class: "editor-note-body", "Frugal desktop theme engine for Blogger." }
                }
            }

            UserPreferencesModal {
                show_prefs,
                ui_mode_pref,
                active_ui_mode: active_ui_mode.clone(),
            }

            EditorSettingsModal {
                open: show_editor_settings,
                active_theme_toml: ui_theme_pref,
            }

            KeyboardShortcutsModal { open: show_shortcuts }

            CssBuilderModal { open: show_css_builder }

            Modal {
                title: "Documentation".to_string(), open: show_docs, on_close: move |_| show_docs.set(false),
                div { class: "editor-note",
                    p { class: "editor-note-title", "Online Resources" }
                    p { class: "editor-note-body", "Read the architecture and integration guides in the MOR_PLAN.md at the repository root." }
                }
            }

            div { class: "editor-shell", style: "height: 100%;",

                AppMenuBar {
                    show_prefs,
                    show_editor_settings,
                    show_about,
                    show_shortcuts,
                    plugin_manager_pos: layout.plugin_manager_pos,
                    show_css_builder,
                    diagnostics_pos: layout.diagnostics_pos,
                    show_docs,
                    left_layout: layout.left_layout,
                    right_layout: layout.right_layout,

                    on_new_workspace: move |_| {
                        let fresh_prefs = crate::app::config_bridge::EditorPrefs::load();
                        let mut config = mor_blogger_core::config::defaults::default_theme_config();
                        if let Some(pack) = fresh_prefs.default_template_pack {
                            config.template_pack = pack;
                        }
                        signals.apply_config(&config);
                        active_preset.set(None);
                        theme.commit();
                    },
                    on_load_theme: move |_| {
                        if let Some(content) = crate::utils::io::load_toml() {
                            if let Ok(new_config) = toml::from_str::<ThemeConfig>(&content) {
                                signals.apply_config(&new_config);
                            }
                        }
                    },
                    on_save_theme: move |_| { crate::utils::io::save_toml(&config_toml_signal()); },

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
                                    Ok(restored_config) => sigs.apply_config(&restored_config),
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
                    "data-left-layout": panel_layout_class((layout.left_layout)()),
                    "data-right-layout": panel_layout_class((layout.right_layout)()),
                    // Read Grid Pinned state directly from the Panel Layout to kill split-brain logic
                    "data-left-pinned": "{(layout.left_layout)() != PanelLayout::Floating}",
                    "data-right-pinned": "{(layout.right_layout)() != PanelLayout::Floating}",

                    // Left Dock Strip - Enforcing Flex Column natively
                    div { 
                        class: "cinnamon-dock left",
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px; width: 48px; padding-top: 12px;",
                        
                        button { class: "dock-btn", title: "Theme Palette",
                            onclick: move |_| {
                                if (layout.theme_palette_pos)() == DockPosition::Left {
                                    let mut current = layout.left_layout;
                                    current.set(if current() == PanelLayout::Hidden { PanelLayout::Split } else { PanelLayout::Hidden });
                                } else {
                                    layout.theme_palette_pos.set(DockPosition::Left);
                                    layout.css_editor_pos.set(DockPosition::Hidden);
                                    layout.left_layout.set(PanelLayout::Split);
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
                                let mut current = layout.right_layout;
                                if current() == PanelLayout::Hidden {
                                    current.set(PanelLayout::Split);
                                } else {
                                    current.set(PanelLayout::Hidden);
                                }
                            },
                            IconSiteData {}
                        }
                    }
                }

                if (layout.css_editor_pos)() == DockPosition::Floating {
                    CssEditorPanel {}
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

                if show_undocked_modules() {
                    TemplateModulesFloatingWindow { current_config: current_config(), show_undocked_modules, on_apply_theme: move |new_config: ThemeConfig| { signals.apply_config(&new_config); } }
                }
            }
        }
    }
}

// Icons
#[component]
fn IconPalette() -> Element {
    rsx! {
        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            circle { cx: "13.5", cy: "6.5", r: ".5", fill: "currentColor" }
            circle { cx: "17.5", cy: "10.5", r: ".5", fill: "currentColor" }
            circle { cx: "8.5", cy: "7.5", r: ".5", fill: "currentColor" }
            circle { cx: "6.5", cy: "12.5", r: ".5", fill: "currentColor" }
            path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
        }
    }
}

#[component]
fn IconCode() -> Element {
    rsx! {
        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            polyline { points: "16 18 22 12 16 6" }
            polyline { points: "8 6 2 12 8 18" }
        }
    }
}

#[component]
fn IconSiteData() -> Element {
    rsx! {
        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            ellipse { cx: "12", cy: "5", rx: "9", ry: "3" }
            path { d: "M21 12c0 1.66-4 3-9 3s-9-1.34-9-3" }
            path { d: "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5" }
        }
    }
}