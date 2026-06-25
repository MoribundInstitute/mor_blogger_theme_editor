use dioxus::prelude::*;

use crate::app::state::{DockPosition, LayoutState};
use crate::ui::components::icons::{IconClose, IconDockLeft, IconDockRight, IconFloat, IconGrip};

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
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Left",
                    onclick: {
                        let dock_id = dock_id.clone();
                        move |_| {
                            layout.request_exclusive_dock(&dock_id, DockPosition::mor_panel_left);
                        }
                    },
                    IconDockLeft {}
                }
            }
            if pos != DockPosition::mor_panel_right {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Right",
                    onclick: {
                        let dock_id = dock_id.clone();
                        move |_| {
                            layout.request_exclusive_dock(&dock_id, DockPosition::mor_panel_right);
                        }
                    },
                    IconDockRight {}
                }
            }
            if pos != DockPosition::Floating {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Float Window",
                    onclick: {
                        let dock_id = dock_id.clone();
                        move |_| {
                            layout.request_exclusive_dock(&dock_id, DockPosition::Floating);
                        }
                    },
                    IconFloat {}
                }
            }
            button {
                class: "editor-mini-button",
                style: "display: flex; align-items: center; padding: 4px;",
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