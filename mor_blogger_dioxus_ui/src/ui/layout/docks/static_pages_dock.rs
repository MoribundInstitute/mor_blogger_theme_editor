use dioxus::prelude::*;

use crate::app::state::{DockPosition, LayoutState};
use crate::app::theme_signals::ThemeSignals;
use crate::ui::components::dock_chrome::DockChrome;
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesPanel;

use super::shared::{PANE_CSS, PANE_DRAG_JS, PANE_RESIZE_JS};

#[derive(Props, Clone, PartialEq)]
pub struct StaticPagesDockProps {
    pub signals: ThemeSignals,
    pub show_undocked_pages: Signal<bool>,
    pub preview_html: Signal<String>,
    pub base_preview_html: ReadSignal<String>,
}

/// First-class home for the static-page builder. The Theme Palette gets hidden
/// when you enter the Static Page Editor workspace, so this dock rides along
/// with that view (see LayoutState::enter_workspace) to keep the page picker
/// and settings beside the editor. Wraps the existing StaticPagesPanel as-is.
#[component]
pub fn StaticPagesDock(props: StaticPagesDockProps) -> Element {
    let mut layout = use_context::<LayoutState>();
    let pos = (layout.static_pages_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            script { dangerous_inner_html: "{PANE_DRAG_JS}" }
            script { dangerous_inner_html: "{PANE_RESIZE_JS}" }
            style { "{PANE_CSS}" }

            DockChrome {
                title: "Static Pages".to_string(),
                dock_id: "static_pages".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.static_pages_pos.set(DockPosition::Hidden);
                },
                div {
                    style: "height: calc(100% - 45px); overflow-y: auto; padding: 12px; background: var(--bg-panel);",
                    StaticPagesPanel {
                        signals: props.signals,
                        show_undocked_pages: props.show_undocked_pages,
                        preview_html: props.preview_html,
                        base_preview_html: props.base_preview_html,
                    }
                }
            }
        }
    }
}
