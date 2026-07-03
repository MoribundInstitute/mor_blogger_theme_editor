use crate::app::state::DockPosition;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MorPanelWrapperProps {
    pub position: DockPosition,
    pub children: Element,
    #[props(default = DockPosition::mor_panel_left)]
    pub default_position: DockPosition,
    #[props(optional)]
    pub title: Option<&'static str>,
    /// Extra class applied only while floating (e.g. "floating-landscape").
    #[props(optional)]
    pub floating_class: Option<&'static str>,
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
            let base = match props.default_position {
                DockPosition::mor_panel_right => "mor_panel_right is-floating",
                _ => "mor_panel_left is-floating",
            };
            let class_name = match props.floating_class {
                Some(extra) => format!("{base} {extra}"),
                None => base.to_string(),
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
