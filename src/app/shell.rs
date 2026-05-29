use dioxus::prelude::*;

use crate::config::ThemeConfig;
use crate::ui::panels::diagnostics_panel::DiagnosticsPanel;
use crate::ui::panels::presets_panel::PresetFloatingWindow;
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

    rsx! {
        style { "{EDITOR_UI_CSS}" }

        div { class: "editor-shell",
            // Standard internal branding header (OS handles the window above this)
            header { class: "editor-main-header",
                h1 { class: "editor-brand", "Moribund Institute Theme Architect" }
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
                }

                CenterWorkspacePanel {
                    preview_viewport: layout.preview_viewport,
                    preview_width: layout.preview_width,
                    preview_template_mode: layout.preview_template_mode,
                    generated_xml: (render.generated_xml)(),
                    preview_html: (render.preview_html)(),
                    show_preview,
                    diag: render.diag,
                    config_toml: toml::to_string_pretty(&current_config()).unwrap_or_default(),
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

            footer { class: "editor-footer",
                DiagnosticsPanel { result: render.diag }
            }
        }
    }
}
