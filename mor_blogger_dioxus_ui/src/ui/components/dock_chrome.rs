use dioxus::prelude::*;

use crate::app::state::{DockPosition, LayoutState};
use crate::ui::components::icons::{IconClose, IconDockLeft, IconDockRight, IconFloat, IconGrip};

/// Shared workbench pane header: filename + badge on the left, the Layout/Code
/// view toggle plus any workbench-specific toolbar buttons (`children`) on the right.
#[component]
pub fn WorkbenchPaneHeader(
    filename: String,
    badge: String,
    layout_view: Signal<bool>,
    children: Element,
) -> Element {
    let mut layout_view = layout_view;
    rsx! {
        div {
            class: "editor-pane-header",
            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: rgba(0,0,0,0.2); border-bottom: 1px solid var(--border-color); flex-shrink: 0;",
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span {
                    style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);",
                    "{filename}"
                }
                span {
                    style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                    "{badge}"
                }
            }
            div {
                style: "display: flex; align-items: center; gap: 6px;",
                button {
                    class: if layout_view() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Visual widget layout (drag to reorder)",
                    onclick: move |_| layout_view.set(true),
                    "Layout"
                }
                button {
                    class: if !layout_view() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                    title: "Raw Blogger XML",
                    onclick: move |_| layout_view.set(false),
                    "Code"
                }
                {children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DockChromeProps {
    pub title: String,
    pub dock_id: String,
    pub position: DockPosition,
    pub on_close: EventHandler<()>,
    pub children: Element,
}

#[component]
pub fn DockChrome(props: DockChromeProps) -> Element {
    let mut layout = use_context::<LayoutState>();
    let pos = props.position;
    let dock_id = props.dock_id.clone();

    let header_actions = rsx! {
        div { class: "floating-editor-window-actions",
            if pos != DockPosition::mor_panel_left {
                button {
                    class: "editor-mini-button",
                    title: "Dock Left",
                    onclick: {
                        let dock_id = dock_id.clone();
                        move |_| {
                            layout.request_dock(&dock_id, DockPosition::mor_panel_left);
                        }
                    },
                    IconDockLeft {}
                }
            }
            if pos != DockPosition::mor_panel_right {
                button {
                    class: "editor-mini-button",
                    title: "Dock Right",
                    onclick: {
                        let dock_id = dock_id.clone();
                        move |_| {
                            layout.request_dock(&dock_id, DockPosition::mor_panel_right);
                        }
                    },
                    IconDockRight {}
                }
            }
            if pos != DockPosition::Floating {
                button {
                    class: "editor-mini-button",
                    title: "Float Window",
                    onclick: {
                        let dock_id = dock_id.clone();
                        move |_| {
                            layout.request_dock(&dock_id, DockPosition::Floating);
                        }
                    },
                    IconFloat {}
                }
            }
            button {
                class: "editor-mini-button",
                title: "Close",
                onclick: move |_| props.on_close.call(()),
                IconClose {}
            }
        }
    };

    rsx! {
        if pos == DockPosition::Floating {
            div {
                class: "floating-editor-window-bar",
                "data-dock-id": props.dock_id.clone(),
                div {
                    class: "floating-editor-grip-group",
                    span {
                        class: "floating-editor-grip",
                        style: "display: flex; align-items: center;",
                        IconGrip {}
                    }
                    span {
                        class: "floating-editor-title",
                        "{props.title}"
                    }
                }
                {header_actions}
            }
        } else {
            div { class: "editor-panel-header",
                "data-dock-id": props.dock_id.clone(),
                title: "Drag onto a side panel to tab this dock there",
                h2 {
                    class: "editor-panel-title",
                    "{props.title}"
                }
                {header_actions}
            }
        }

        {props.children}
    }
}