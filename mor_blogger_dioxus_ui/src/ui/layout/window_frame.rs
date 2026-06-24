// src/shell.rs
// Modular CSD HeaderBar and Shell container. Zero layout lock-in.
use crate::ui::layout::shortcut::MorShortcutRoot;
use dioxus::desktop::tao::window::ResizeDirection;
use dioxus::desktop::window;
use dioxus::prelude::*;

#[component]
pub fn MorWindowTitle(title: String, #[props(default = None)] subtitle: Option<String>) -> Element {
    rsx! {
        div {
            class: "mor-window-title-block",
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; pointer-events: none;",
            span {
                class: "mor-window-title",
                style: "font-weight: 600; font-size: 13px; color: var(--mor-text);",
                "{title}"
            }
            if let Some(sub) = subtitle {
                span {
                    class: "mor-window-subtitle",
                    style: "font-size: 11px; color: var(--mor-text-muted); margin-top: -2px;",
                    "{sub}"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MorHeaderBarProps {
    #[props(default = None)]
    pub start: Option<Element>,
    #[props(default = None)]
    pub center: Option<Element>,
    #[props(default = None)]
    pub end: Option<Element>,
    #[props(default = true)]
    pub show_controls: bool,
}

#[component]
pub fn MorHeaderBar(props: MorHeaderBarProps) -> Element {
    let mut last_click =
        use_signal(|| std::time::Instant::now() - std::time::Duration::from_secs(10));

    let handle_drag = move |_| {
        let now = std::time::Instant::now();
        if now.duration_since(last_click()) < std::time::Duration::from_millis(400) {
            window().toggle_maximized();
            last_click.set(now - std::time::Duration::from_secs(10));
        } else {
            last_click.set(now);
            window().drag();
        }
    };

    rsx! {
        div {
            class:  "mor-headerbar ",
            style:  "display: grid; grid-template-columns: 1fr auto 1fr; align-items: center; min-height: 46px; background: var(--mor-header); border-bottom: 1px solid var(--mor-border); ",

            onmousedown: handle_drag,

            div {
                class:  "mor-headerbar-start ",
                style:  "display: flex; align-items: center; justify-content: flex-start; height: 100%; padding-left: 8px; gap: 6px; ",
                if let Some(s) = props.start { {s} }
            }

            div {
                class:  "mor-headerbar-center ",
                style:  "display: flex; align-items: center; justify-content: center; height: 100%; ",
                if let Some(c) = props.center { {c} }
            }

            div {
                class:  "mor-headerbar-end ",
                style:  "display: flex; align-items: center; justify-content: flex-end; height: 100%; padding-right: 6px; gap: 6px; ",
                if let Some(e) = props.end { {e} }

                if props.show_controls {
                    div { class:  "mor-window-controls ", style:  "display: flex; gap: 4px; margin-left: 8px; ",
                        onmousedown: |e| e.stop_propagation(),
                        button { class:  "window-btn ", onclick: move |_| window().set_minimized(true),  "— " }
                        button { class:  "window-btn ", onclick: move |_| window().toggle_maximized(),  "□ " }
                        button { class:  "window-btn close ", onclick: move |_| { window().close(); },  "× " }
                    }
                }
            }
        }
    }
}

#[component]
pub fn MorShell(children: Element) -> Element {
    let mode = std::env::var("MOR_ACTIVE_UI_MODE").unwrap_or_else(|_| "frameless".to_string());
    let is_frameless = mode == "frameless";

    rsx! {
        style {
             ".window-btn {{ width: 32px; height: 32px; border: none; border-radius: 6px; background: transparent; color: var(--mor-text-muted); cursor: default; font-family: sans-serif; font-s ize: 14px; transition: background 0.1s; }} "
             ".window-btn:hover {{ background: var(--mor-btn-hover); color: var(--mor-text); }} "
             ".window-btn.close:hover {{ background: var(--mor-destructive); color: white; }} "
             "body {{ margin: 0; padding: 0; overflow: hidden; }} "

             ".mor-resize-edge {{ position: absolute; z-index: 9999; }} "
             ".mor-resize-edge.top {{ top: 0; left: 6px; right: 6px; height: 6px; cursor: n-resize; }} "
             ".mor-resize-edge.bottom {{ bottom: 0; left: 6px; right: 6px; height: 6px; cursor: s-resize; }} "
             ".mor-resize-edge.left {{ top: 6px; bottom: 6px; left: 0; width: 6px; cursor: w-resize; }} "
             ".mor-resize-edge.right {{ top: 6px; bottom: 6px; right: 0; width: 6px; cursor: e-resize; }} "
             ".mor-resize-edge.top-left {{ top: 0; left: 0; width: 10px; height: 10px; cursor: nw-resize; }} "
             ".mor-resize-edge.top-right {{ top: 0; right: 0; width: 10px; height: 10px; cursor: ne-resize; }} "
             ".mor-resize-edge.bottom-left {{ bottom: 0; left: 0; width: 10px; height: 10px; cursor: sw-resize; }} "
             ".mor-resize-edge.bottom-right {{ bottom: 0; right: 0; width: 10px; height: 10px; cursor: se-resize; }} "
        }
        MorShortcutRoot {
            div {
                class:  "mor-root ",
                style:  "height: 100vh; width: 100vw; display: flex; flex-direction: column; background-color: var(--mor-bg); overflow: hidden; position: relative; ",

                if is_frameless {
                    div { class:  "mor-resize-edge top ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::North); } }
                    div { class:  "mor-resize-edge bottom ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::South); } }
                    div { class:  "mor-resize-edge left ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::West); } }
                    div { class:  "mor-resize-edge right ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::East); } }
                    div { class:  "mor-resize-edge top-left ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::NorthWest); } }
                    div { class:  "mor-resize-edge top-right ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::NorthEast); } }
                    div { class:  "mor-resize-edge bottom-left ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::SouthWest); } }
                    div { class:  "mor-resize-edge bottom-right ", onmousedown: move |_| { let _ = window().drag_resize_window(ResizeDirection::SouthEast); } }
                }

                {children}
            }
        }
    }
}
