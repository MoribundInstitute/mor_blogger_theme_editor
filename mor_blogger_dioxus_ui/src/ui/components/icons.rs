use crate::app::state::{ContextMenuPayload, LayoutState};
use dioxus::prelude::*;

#[component]
pub fn IconPalette() -> Element {
    let layout = use_context::<LayoutState>();
    let mut active_context_menu = layout.active_context_menu;

    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            oncontextmenu: move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                let coords = evt.client_coordinates();
                active_context_menu.set(Some(ContextMenuPayload {
                    x: coords.x,
                    y: coords.y,
                    kind: "svg".to_string(),
                    target_id: "preview-icon".to_string(), // In production, pass the actual ID dynamically
                }));
            },
            circle { cx: "13.5", cy: "6.5", r: ".5", fill: "currentColor" }
            circle { cx: "17.5", cy: "10.5", r: ".5", fill: "currentColor" }
            circle { cx: "8.5", cy: "7.5", r: ".5", fill: "currentColor" }
            circle { cx: "6.5", cy: "12.5", r: ".5", fill: "currentColor" }
            path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
        }
    }
}

#[component]
pub fn IconCode() -> Element {
    let layout = use_context::<LayoutState>();
    let mut active_context_menu = layout.active_context_menu;

    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            oncontextmenu: move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                let coords = evt.client_coordinates();
                active_context_menu.set(Some(ContextMenuPayload {
                    x: coords.x,
                    y: coords.y,
                    kind: "svg".to_string(),
                    target_id: "preview-icon".to_string(), // In production, pass the actual ID dynamically
                }));
            },
            polyline { points: "16 18 22 12 16 6" }
            polyline { points: "8 6 2 12 8 18" }
        }
    }
}

#[component]
pub fn IconSiteData() -> Element {
    let layout = use_context::<LayoutState>();
    let mut active_context_menu = layout.active_context_menu;

    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            oncontextmenu: move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                let coords = evt.client_coordinates();
                active_context_menu.set(Some(ContextMenuPayload {
                    x: coords.x,
                    y: coords.y,
                    kind: "svg".to_string(),
                    target_id: "preview-icon".to_string(), // In production, pass the actual ID dynamically
                }));
            },
            ellipse { cx: "12", cy: "5", rx: "9", ry: "3" }
            path { d: "M21 12c0 1.66-4 3-9 3s-9-1.34-9-3" }
            path { d: "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5" }
        }
    }
}

#[component]
pub fn IconClose(#[props(default = "16".to_string())] size: String) -> Element {
    rsx! {
        svg { width: "{size}", height: "{size}", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: "M4.5 4.5l7 7M11.5 4.5l-7 7" }
        }
    }
}

#[component]
pub fn IconFloat(#[props(default = "16".to_string())] size: String) -> Element {
    rsx! {
        svg { width: "{size}", height: "{size}", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "1.5", width: "10", height: "8", rx: "1.5" }
            rect { x: "4.5", y: "6.5", width: "10", height: "8", rx: "1.5" }
        }
    }
}

#[component]
pub fn IconDockLeft() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M5.5 2.5v11" }
        }
    }
}

#[component]
pub fn IconDockRight() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M10.5 2.5v11" }
        }
    }
}
