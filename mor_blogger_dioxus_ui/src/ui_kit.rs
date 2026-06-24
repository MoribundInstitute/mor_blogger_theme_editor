use crate::app::state::{ContextMenuPayload, LayoutState, DockPosition};
use crate::ui::components::icons::{IconClose, IconGrip};
use dioxus::prelude::*;

#[component]
pub fn IconMaximize() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "2.5", y: "2.5", width: "11", height: "11", rx: "1.5" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MorDockProps {
    pub title: String,
    pub children: Element,
    #[props(optional)]
    pub on_close: Option<EventHandler<MouseEvent>>,
    #[props(default = "320px".to_string())]
    pub width: String,
    #[props(default = "120px".to_string())]
    pub left: String,
    #[props(default = "100px".to_string())]
    pub top: String,
    #[props(default = "80vh".to_string())]
    pub max_height: String,
    #[props(default = "var(--bg-surface, #1e1e1e)".to_string())]
    pub background: String,
    #[props(default = false)]
    pub is_native_window: bool,
}

#[component]
pub fn MorDock(props: MorDockProps) -> Element {
    let mut is_maximized = use_signal(|| false);

    // Coordinate state for drag gesture
    let mut x = use_signal(|| props.left.clone());
    let mut y = use_signal(|| props.top.clone());
    let mut dragging = use_signal(|| false);
    let mut start_offset = use_signal(|| (0.0, 0.0));

    #[cfg(not(target_arch = "wasm32"))]
    let window = dioxus::desktop::use_window();

    let container_style = if props.is_native_window {
        format!("position: absolute; left: 0; top: 0; width: 100vw; height: 100vh; display: flex; flex-direction: column; overflow: hidden; background: {};", props.background)
    } else if is_maximized() {
        format!("position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 9999; border-radius: 0; display: flex; flex-direction: column; overflow: hidden; background: {}; border: 1px solid var(--border-color); box-shadow: 0 10px 40px rgba(0,0,0,0.5);", props.background)
    } else {
        format!(
            "position: fixed; left: {}; top: {}; width: {}; max-height: {}; z-index: 150; display: flex; flex-direction: column; overflow: hidden; background: {}; border: 1px solid var(--border-color); box-shadow: 0 10px 40px rgba(0,0,0,0.5); border-radius: 8px;",
            x(), y(), props.width, props.max_height, props.background
        )
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window_mousedown = window.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let window_close = window.clone();

    let onmousedown = move |e: MouseEvent| {
        if props.is_native_window {
            #[cfg(not(target_arch = "wasm32"))]
            window_mousedown.drag();
        } else if e.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
            let coords = e.client_coordinates();
            let parse_px = |s: String| s.trim_end_matches("px").parse::<f64>().unwrap_or(0.0);
            let current_x = parse_px(x());
            let current_y = parse_px(y());
            start_offset.set((coords.x - current_x, coords.y - current_y));
            dragging.set(true);
        }
    };

    let onmousemove = move |e: MouseEvent| {
        if dragging() && !props.is_native_window && !is_maximized() {
            let coords = e.client_coordinates();
            let offset = start_offset();
            let new_x = coords.x - offset.0;
            let new_y = coords.y - offset.1;
            let clamped_x = new_x.max(0.0);
            let clamped_y = new_y.max(0.0);
            x.set(format!("{}px", clamped_x));
            y.set(format!("{}px", clamped_y));
        }
    };

    let onmouseup = move |_| {
        dragging.set(false);
    };

    let mut layout = use_context::<LayoutState>();

    rsx! {
        aside {
            class: "mor_panel_left is-floating",
            style: "{container_style}",
            onmousemove: onmousemove,
            onmouseup: onmouseup,

            // Header bar
            div {
                class: "floating-editor-window-bar",
                onmousedown: onmousedown,
                ondoubleclick: move |_| {
                    if !props.is_native_window {
                        is_maximized.set(!is_maximized());
                    }
                },
                oncontextmenu: {
                    let title = props.title.clone();
                    move |e| {
                        e.prevent_default();
                        e.stop_propagation();
                        let coords = e.client_coordinates();
                        layout.active_context_menu.set(Some(ContextMenuPayload {
                            x: coords.x,
                            y: coords.y,
                            kind: "dock".to_string(),
                            target_id: title.clone(),
                        }));
                    }
                },

                div {
                    class: "floating-editor-grip-group",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center; cursor: grab;", IconGrip {} }
                    span {
                        class: "floating-editor-title",
                        "{props.title}"
                    }
                }

                // Actions (Maximize, Close)
                div { class: "floating-editor-window-actions",
                    if !props.is_native_window {
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: if is_maximized() { "Restore Window" } else { "Maximize Window" },
                            onclick: move |_| {
                                is_maximized.set(!is_maximized());
                            },
                            IconMaximize {}
                        }
                    }
                    button {
                        class: "editor-mini-button",
                        style: "display: flex; align-items: center; padding: 4px;",
                        title: "Close",
                        onclick: move |evt| {
                            if let Some(ref handler) = props.on_close {
                                handler.call(evt);
                            } else {
                                #[cfg(not(target_arch = "wasm32"))]
                                window_close.close();
                            }
                        },
                        IconClose {}
                    }
                }
            }

            // Content
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StandardNativeWindowProps {
    pub title: String,
    pub children: Element,
    pub on_close: EventHandler<MouseEvent>,
    #[props(default = "800px".to_string())]
    pub width: String,
    #[props(default = "150px".to_string())]
    pub left: String,
    #[props(default = "100px".to_string())]
    pub top: String,
    #[props(default = "600px".to_string())]
    pub max_height: String,
    #[props(default = false)]
    pub is_native_window: bool,
}

const NATIVE_WINDOW_CSS: &str = r#"
            .native-window-header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                background: linear-gradient(180deg, #2d2a22 0%, #201e18 100%);
                border-bottom: 1px solid #3e3a32;
                padding: 6px 12px;
                user-select: none;
                cursor: move;
                height: 32px;
            }
            .native-window-title {
                font-family: var(--font-mono, monospace);
                font-size: 0.8rem;
                font-weight: bold;
                color: #ece7da;
                letter-spacing: 0.05em;
            }
            .native-window-controls {
                display: flex;
                gap: 8px;
            }
            .native-window-btn {
                background: #2d2a22;
                border: 1px solid #4a4538;
                color: #ece7da;
                width: 18px;
                height: 18px;
                border-radius: 3px;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 0.7rem;
                cursor: pointer;
                padding: 0;
            }
            .native-window-btn:hover {
                background: #3e3a32;
                border-color: #5c5546;
            }
            .native-window-btn.close-btn:hover {
                background: #992525;
                border-color: #b83232;
            }
            .window-menubar {
                display: flex;
                gap: 16px;
                background: #1c1a14;
                border-bottom: 1px solid #2d2a22;
                padding: 6px 16px;
                font-size: 0.75rem;
                color: #b0aa9c;
                user-select: none;
            }
            .window-menubar span {
                cursor: pointer;
                transition: color 0.15s;
            }
            .window-menubar span:hover {
                color: #ece7da;
            }
"#;

#[component]
pub fn StandardNativeWindow(props: StandardNativeWindowProps) -> Element {
    let mut is_maximized = use_signal(|| false);

    // Coordinate state for drag gesture
    let mut x = use_signal(|| props.left.clone());
    let mut y = use_signal(|| props.top.clone());
    let mut dragging = use_signal(|| false);
    let mut start_offset = use_signal(|| (0.0, 0.0));

    #[cfg(not(target_arch = "wasm32"))]
    let window = dioxus::desktop::use_window();

    let container_style = if props.is_native_window {
        "position: absolute; left: 0; top: 0; width: 100vw; height: 100vh; display: flex; flex-direction: column; overflow: hidden; background: #16140f;".to_string()
    } else if is_maximized() {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 9999; border-radius: 0; display: flex; flex-direction: column; overflow: hidden; background: #16140f; border: 2px solid #3e3a32; box-shadow: 0 20px 60px rgba(0,0,0,0.7);".to_string()
    } else {
        format!(
            "position: fixed; left: {}; top: {}; width: {}; height: {}; z-index: 150; display: flex; flex-direction: column; overflow: hidden; background: #16140f; border: 2px solid #3e3a32; box-shadow: 0 20px 60px rgba(0,0,0,0.7); border-radius: 6px;",
            x(), y(), props.width, props.max_height
        )
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window_mousedown = window.clone();

    let onmousedown = move |e: MouseEvent| {
        if props.is_native_window {
            #[cfg(not(target_arch = "wasm32"))]
            window_mousedown.drag();
        } else if e.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
            let coords = e.client_coordinates();
            let parse_px = |s: String| s.trim_end_matches("px").parse::<f64>().unwrap_or(0.0);
            let current_x = parse_px(x());
            let current_y = parse_px(y());
            start_offset.set((coords.x - current_x, coords.y - current_y));
            dragging.set(true);
        }
    };

    let onmousemove = move |e: MouseEvent| {
        if dragging() && !props.is_native_window && !is_maximized() {
            let coords = e.client_coordinates();
            let offset = start_offset();
            let new_x = coords.x - offset.0;
            let new_y = coords.y - offset.1;
            let clamped_x = new_x.max(0.0);
            let clamped_y = new_y.max(0.0);
            x.set(format!("{}px", clamped_x));
            y.set(format!("{}px", clamped_y));
        }
    };

    let onmouseup = move |_| {
        dragging.set(false);
    };

    rsx! {
        style { dangerous_inner_html: "{NATIVE_WINDOW_CSS}" }
        div {
            class: "standard-native-window",
            style: "{container_style}",
            onmousemove: onmousemove,
            onmouseup: onmouseup,

            // Header Bar
            div {
                class: "native-window-header",
                onmousedown: onmousedown,
                ondoubleclick: move |_| {
                    if !props.is_native_window {
                        is_maximized.set(!is_maximized());
                    }
                },
                span { class: "native-window-title", "{props.title}" }
                div { class: "native-window-controls",
                    if !props.is_native_window {
                        button {
                            class: "native-window-btn",
                            title: if is_maximized() { "Restore" } else { "Maximize" },
                            onclick: move |_| is_maximized.set(!is_maximized()),
                            if is_maximized() { "❐" } else { "⛶" }
                        }
                    }
                    button {
                        class: "native-window-btn close-btn",
                        title: "Close",
                        onclick: move |evt| {
                            props.on_close.call(evt);
                        },
                        "×"
                    }
                }
            }

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MorPanelWrapperProps {
    pub position: DockPosition,
    pub children: Element,
    #[props(default = DockPosition::mor_panel_left)]
    pub default_position: DockPosition,
    #[props(optional)]
    pub title: Option<&'static str>,
}

#[component]
pub fn MorPanelWrapper(props: MorPanelWrapperProps) -> Element {
    match props.position {
        DockPosition::mor_panel_left => rsx! {
            aside { class: "mor_panel_left",
                {props.children}
                div { class: "pane-resizer pane-resizer-right" }
            }
        },
        DockPosition::mor_panel_right => rsx! {
            aside { class: "mor_panel_right",
                {props.children}
                div { class: "pane-resizer pane-resizer-left" }
            }
        },
        DockPosition::Floating => {
            let class_name = match props.default_position {
                DockPosition::mor_panel_right => "mor_panel_right is-floating",
                _ => "mor_panel_left is-floating",
            };
            rsx! {
                aside { class: "{class_name}",
                    {props.children}
                }
            }
        },
        DockPosition::Hidden => rsx! {},
    }
}
