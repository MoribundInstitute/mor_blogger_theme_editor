use dioxus::prelude::*;

// 1. IMPORT THE LOCAL UI KIT PIECES
use crate::ui::components::modal::Modal;
use crate::ui::shell::menu_bar::AppMenuBar;
use crate::ui::shell::theme::{get_native_os_theme, MorStyleProvider};
use crate::ui::shell::window_frame::{MorHeaderBar, MorShell, MorWindowTitle};

// 2. IMPORT THE EDITOR PANELS
use crate::config::ThemeConfig;
use crate::ui::panels::diagnostics_panel::DiagnosticsPanel;
use crate::ui::panels::presets_panel::PresetFloatingWindow;
use crate::ui::panels::static_pages_panel::StaticPagesFloatingWindow;
use crate::ui::workspace::left_dock::LeftVisualsPanel;
use crate::ui::workspace::master_canvas::CenterWorkspacePanel;
use crate::ui::workspace::right_dock::RightDataPanel;

use super::config_bridge::panel_layout_class;
use super::hotswap::apply_hotswap_json;
use super::layout_state::AppLayoutState;
use super::render_state::AppRenderState;
use super::state::ThemeAppState;

const EDITOR_UI_CSS: &str = include_str!("../editor_ui.css");

pub fn render_app_shell(
    theme: ThemeAppState,
    layout: AppLayoutState,
    render: AppRenderState,
) -> Element {
    let signals = theme.signals;
    let current_config = theme.current_config;
    let mut active_preset = theme.active_preset;
    let show_preview = theme.show_preview;
    let show_undocked_presets = theme.show_undocked_presets;
    
    let show_undocked_pages = use_signal(|| false);

    let mut show_about = use_signal(|| false);
    let mut show_prefs = use_signal(|| false);

    let active_ui_mode = std::env::var("MOR_ACTIVE_UI_MODE").unwrap_or_else(|_| "frameless".to_string());
    
    // Matrix of UI state based on mode
    let mut ui_mode_pref = use_signal(|| active_ui_mode.clone());
    let show_window_buttons = active_ui_mode == "frameless";
    let show_custom_title = active_ui_mode != "native";

    let config_toml_signal = use_memo(move || {
        toml::to_string_pretty(&current_config()).unwrap_or_default()
    });

    // Writable TV monitor for preview output.
    let mut tv_monitor = use_signal(|| String::new());

    use_effect(move || {
        tv_monitor.set((render.preview_html)());
    });

    rsx! {
        // 3. AUTO OS CHAMELEON
        MorStyleProvider { theme_toml: get_native_os_theme().to_string() }
        style { "{EDITOR_UI_CSS}" }

        MorShell {
            if active_ui_mode != "native" {
                MorHeaderBar {
                    show_controls: show_window_buttons,

                    start: rsx! {
                        div { style: "width: 16px;" }
                    },

                    center: rsx! {
                        if show_custom_title {
                            MorWindowTitle { 
                                title: "Moribund Theme Architect".to_string(),
                                subtitle: Some(format!("{} Mode", active_ui_mode))
                            }
                        }
                    },

                    end: rsx! {
                        div { style: "width: 16px;" }
                    }
                }
            }

            Modal {
                title: "About Moribund Architect".to_string(),
                open: show_about,
                on_close: move |_| show_about.set(false),
                div { class: "editor-note",
                    p { class: "editor-note-title", "Version 0.1.0" }
                    p { class: "editor-note-body", "Frugal desktop theme engine for Blogger." }
                }
            }

            Modal {
                title: "Preferences".to_string(),
                open: show_prefs,
                on_close: move |_| show_prefs.set(false),
                div { class: "editor-field-group",
                    label { class: "editor-field-label", "Window Mode" }
                    select {
                        class: "editor-select",
                        value: "{ui_mode_pref}",
                        onchange: move |evt| {
                            let new_mode = evt.value();
                            ui_mode_pref.set(new_mode.clone());
                            let json = format!(r#"{{"ui_mode":"{}"}}"#, new_mode);
                            let _ = std::fs::write("editor_prefs.json", json);
                        },
                        option { value: "frameless", "Frameless (Custom OS Buttons)" }
                        option { value: "native", "Native OS Window" }
                        option { value: "tiling", "Tiling WM (No Buttons)" }
                    }
                    
                    if ui_mode_pref() != active_ui_mode {
                        div { class: "editor-note", style: "margin-top: 12px; border-color: var(--editor-warning); background: rgba(210, 153, 34, 0.05);",
                            p { class: "editor-note-title", style: "color: var(--editor-warning);", "Restart Required" }
                            p { class: "editor-note-body", "You must restart the application for the new window borders to take effect." }
                        }
                    }
                }
            }

            div { class: "editor-shell", style: "height: 100%;",
                
                // 4. WIRED MENU BAR
                AppMenuBar {
                    show_prefs,
                    show_about,
                    on_load_theme: move |_| {
                        if let Some(content) = crate::io::load_toml() {
                            if let Ok(new_config) = toml::from_str::<ThemeConfig>(&content) {
                                signals.apply_config(&new_config);
                            }
                        }
                    },
                    on_save_theme: move |_| {
                        crate::io::save_toml(&config_toml_signal());
                    },
                    on_export_xml: move |_| {
                        crate::io::save_xml(&(render.generated_xml)());
                    },
                    on_export_zip: move |_| {
                        crate::io::export_bundle(&(render.generated_xml)(), &config_toml_signal());
                    },
                }
                
                div {
                    class: "editor-main",
                    "data-left-layout": panel_layout_class((layout.left_layout)()),
                    "data-right-layout": panel_layout_class((layout.right_layout)()),

                    LeftVisualsPanel {
                        active_tab: layout.active_left_tab,
                        layout: layout.left_layout,
                        active_preset,
                        signals,
                        show_preview,
                        current_config: current_config(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            signals.apply_config(&new_config);
                            active_preset.set(None);
                        },
                        show_undocked_presets,
                        show_undocked_pages,
                        preview_html: tv_monitor,
                        base_preview_html: render.preview_html,
                    }

                    CenterWorkspacePanel {
                        preview_viewport: layout.preview_viewport,
                        preview_width: layout.preview_width,
                        preview_template_mode: layout.preview_template_mode,
                        generated_xml: render.generated_xml,
                        preview_html: tv_monitor,
                        show_preview,
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
                                crate::render::pages::generate_archive_html(&pages.archive)
                            } else if href.contains("categories") || href.contains("directory") {
                                crate::render::pages::generate_categories_html(&pages.categories)
                            } else if href.contains("about") {
                                crate::render::pages::generate_about_html(&pages.about)
                            } else if href.contains("portfolio") {
                                crate::render::pages::generate_portfolio_html(&pages.portfolio)
                            } else if href.contains("catalog") || href.contains("lessons") || href.contains("courses") {
                                crate::render::pages::generate_course_catalog_html(&pages.lms)
                            } else {
                                tv_monitor.set(base);
                                return;
                            };

                            tv_monitor.set(
                                crate::ui::panels::static_pages_panel::inject_static_page(&base, &new_html),
                            );
                        },
                    }

                    RightDataPanel {
                        active_tab: layout.active_right_tab,
                        layout: layout.right_layout,
                        signals,
                        current_config: current_config(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            signals.apply_config(&new_config);
                        },
                    }
                }

                if show_undocked_presets() {
                    PresetFloatingWindow {
                        signals,
                        active_preset,
                        show_undocked_presets,
                    }
                }

                if show_undocked_pages() {
                    StaticPagesFloatingWindow {
                        signals,
                        show_undocked_pages,
                        preview_html: tv_monitor,
                        base_preview_html: render.preview_html,
                    }
                }

                if !(render.diag)().errors.is_empty() || !(render.diag)().warnings.is_empty() {
                    footer { class: "editor-footer",
                        DiagnosticsPanel { result: render.diag }
                    }
                }
            }
        }
    }
}