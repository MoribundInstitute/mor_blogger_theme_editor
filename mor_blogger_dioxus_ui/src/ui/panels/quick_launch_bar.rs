use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum LaunchBarSide {
    Left,
    Right,
}

#[derive(Props, PartialEq, Clone)]
pub struct MorQuickLaunchBarProps {
    pub side: LaunchBarSide,
    pub children: Element,
}

#[component]
pub fn MorQuickLaunchBar(props: MorQuickLaunchBarProps) -> Element {
    let side_class = match props.side {
        LaunchBarSide::Left => "launch-bar-left",
        LaunchBarSide::Right => "launch-bar-right",
    };

    rsx! {
        aside {
            class: "mor-quick-launch-bar {side_class}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct LaunchButtonProps {
    pub is_active: bool,
    pub is_visible: bool,
    #[props(into)]
    pub tooltip: String,
    pub onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    pub oncontextmenu: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn LaunchButton(props: LaunchButtonProps) -> Element {
    let active_class = if props.is_active { "active" } else { "" };
    let display_style = if props.is_visible { "" } else { "display: none;" };

    rsx! {
        button {
            class: "launch-btn {active_class}",
            style: "{display_style}",
            title: "{props.tooltip}",
            onclick: move |e| props.onclick.call(e),
            oncontextmenu: move |e| {
                if let Some(ref handler) = props.oncontextmenu {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}
