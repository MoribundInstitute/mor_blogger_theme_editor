use crate::app::state::{DockPosition, LayoutState, ThemeState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::form::MorCheckbox;
use crate::ui::components::dock_chrome::DockChrome;
use crate::ui::components::slider::Slider;
use dioxus::prelude::*;

#[component]
pub fn JsBuilderDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let theme_state = use_context::<ThemeState>();
    let pos = (layout.js_builder_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let mut script_config = theme_state.signals.scripts;

    // Read scripts config
    let current_config = script_config.read().clone();

    // Generate output code
    let js_preview = format!(
        "// Generated Core Constants\n\
        const MOBILE_BREAKPOINT = {}px;\n\
        const PANELS_COLLAPSED_DEFAULT = {};\n\
        const FEATURE_THEME_TOGGLE = {};\n\
        const FEATURE_SHARE_ACTIONS = {};",
        current_config.mobile_breakpoint,
        current_config.panels_collapsed_mobile,
        current_config.enable_theme_toggle,
        current_config.enable_share_actions
    );

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "JS Behavior Builder".to_string(),
                dock_id: "js_builder".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.js_builder_pos.set(DockPosition::Hidden);
                },
                div {
                    style: "display: flex; flex-direction: column; height: calc(100% - 45px); padding: 12px; overflow-y: auto; background: var(--bg-panel); color: var(--fg-base); font-size: 0.85rem; gap: 16px;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        h3 { style: "font-size: 0.85rem; color: var(--fg-muted); border-bottom: 1px solid var(--border); padding-bottom: 4px; margin: 0;", "Responsive Layout" }

                        Slider {
                            label: Some("Mobile Breakpoint (px)"),
                            value: current_config.mobile_breakpoint,
                            min: 320.0,
                            max: 1200.0,
                            step: 10.0,
                            show_value: true,
                            oninput: move |val| {
                                script_config.with_mut(|c| c.mobile_breakpoint = val);
                            }
                        }

                        MorCheckbox {
                            label: "Auto-collapse side panels on mobile".to_string(),
                            checked: current_config.panels_collapsed_mobile,
                            onchange: move |val| {
                                script_config.with_mut(|c| c.panels_collapsed_mobile = val);
                            }
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        h3 { style: "font-size: 0.85rem; color: var(--fg-muted); border-bottom: 1px solid var(--border); padding-bottom: 4px; margin: 0;", "Feature Flags" }

                        MorCheckbox {
                            label: "Enable Dark/Light Theme Toggler".to_string(),
                            checked: current_config.enable_theme_toggle,
                            onchange: move |val| {
                                script_config.with_mut(|c| c.enable_theme_toggle = val);
                            }
                        }

                        MorCheckbox {
                            label: "Enable Article Share Actions".to_string(),
                            checked: current_config.enable_share_actions,
                            onchange: move |val| {
                                script_config.with_mut(|c| c.enable_share_actions = val);
                            }
                        }
                    }

                    // Live Code Preview
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; min-height: 120px; border: 1px solid var(--border); border-radius: 4px; overflow: hidden; margin-top: 10px;",
                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column;",
                            CodeEditor {
                                value: js_preview,
                                mode: "javascript".to_string(),
                                read_only: true,
                                on_change: |_| {},
                            }
                        }
                    }
                }
            }
        }
    }
}
