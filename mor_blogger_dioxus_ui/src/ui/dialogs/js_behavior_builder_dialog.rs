use crate::ui::components::form::MorCheckbox;
use crate::ui::components::modal::Modal;
use crate::ui::components::slider::Slider;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct JsBehaviorBuilderDialogProps {
    pub open: Signal<bool>,
}

#[component]
pub fn JsBehaviorBuilderDialog(props: JsBehaviorBuilderDialogProps) -> Element {
    let theme_state = use_context::<crate::app::state::ThemeState>();
    let mut open = props.open;
    let mut script_config = theme_state.signals.scripts;

    // We read the current state to drive the inputs
    let current_config = script_config.read().clone();

    // Generate the "Live Preview" of the JavaScript constants this dialog controls
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
        Modal {
            open: open,
            title: "JavaScript Behavior Builder".to_string(),
            style: "width: 700px; height: 500px;".to_string(), // From modal.rs style prop
            on_close: move |_| open.set(false),

            div {
                style: "display: flex; gap: 24px; height: 100%;",

                // LEFT COLUMN: The Controls
                div {
                    style: "flex: 1; display: flex; flex-direction: column; gap: 16px; overflow-y: auto; padding-right: 8px;",

                    h3 { style: "font-size: 0.9rem; color: var(--fg-muted); border-bottom: 1px solid var(--editor-border-soft); padding-bottom: 8px; margin-bottom: 8px;", "Responsive Layout" }

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

                    h3 { style: "font-size: 0.9rem; color: var(--fg-muted); border-bottom: 1px solid var(--editor-border-soft); padding-bottom: 8px; margin-top: 16px; margin-bottom: 8px;", "Feature Flags" }

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

                // RIGHT COLUMN: The Live Code Preview
                div {
                    style: "flex: 1; background: #16140f; border: 1px solid var(--editor-border); border-radius: 4px; display: flex; flex-direction: column; overflow: hidden;",

                    div {
                        style: "padding: 8px 12px; background: #0b0a09; border-bottom: 1px solid var(--editor-border); font-size: 0.75rem; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;",
                        "Live JS Output"
                    }

                    pre {
                        style: "padding: 16px; margin: 0; font-family: monospace; font-size: 0.85rem; color: #ece7da; overflow-x: auto; white-space: pre-wrap;",
                        "{js_preview}"
                    }
                }
            }
        }
    }
}
