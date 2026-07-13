use crate::app::state::{DockPosition, LayoutState, RenderState, ThemeState};
use crate::ui::components::dock_chrome::DockChrome;
use crate::ui::workspace::js_workbench::JsWorkspaceBody;
use dioxus::prelude::*;
use mor_blogger_core::config::ScriptBehaviorConfig;

/// Dockable "JS Behavior Builder" — the same JS workspace as the center view, so you
/// can tune behavior while keeping the preview visible. Renders the shared
/// [`JsWorkspaceBody`]; reads the live config from `RenderState` and writes script
/// changes straight to the theme signals (the canonical edit path for docks).
#[component]
pub fn JsBuilderDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let theme_state = use_context::<ThemeState>();
    let render = use_context::<RenderState>();
    let pos = (layout.js_builder_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let mut script_config = theme_state.signals.scripts;
    let config = (render.current_config)();

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "JS Behaviors".to_string(),
                dock_id: "js_builder".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.js_builder_pos.set(DockPosition::Hidden);
                },
                div {
                    style: "height: calc(100% - 45px); overflow-y: auto; padding: 12px; background: var(--bg-panel); color: var(--fg-base); display: flex; flex-direction: column; gap: 16px;",
                    JsWorkspaceBody {
                        config,
                        on_scripts_change: move |scripts: ScriptBehaviorConfig| {
                            script_config.set(scripts);
                        }
                    }
                }
            }
        }
    }
}
