use crate::ui::components::modal::Modal;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tab {
    Light,
    Dark,
    Shared,
}

#[derive(Props, Clone, PartialEq)]
pub struct CssBuilderModalProps {
    pub open: Signal<bool>,
}

#[component]
pub fn CssBuilderModal(props: CssBuilderModalProps) -> Element {
    let mut open = props.open;
    let active_tab = use_signal(|| Tab::Light);
    let light_primary = use_signal(|| "#3b82f6".to_string());
    let light_bg = use_signal(|| "#ffffff".to_string());
    let dark_primary = use_signal(|| "#60a5fa".to_string());
    let dark_bg = use_signal(|| "#1e293b".to_string());
    let border_radius = use_signal(|| "6px".to_string());

    let lp = LeftPaneProps {
        active_tab: active_tab(),
        light_primary,
        light_bg,
        dark_primary,
        dark_bg,
        border_radius,
    };

    let rp = RightPaneProps {
        active_tab: active_tab(),
        light_primary: light_primary(),
        light_bg: light_bg(),
        dark_primary: dark_primary(),
        dark_bg: dark_bg(),
        border_radius: border_radius(),
    };

    rsx! {
      Modal {
        open,
        title: "CSS Token Builder".to_string(),
        style: "min-width: 700px; max-width: 900px; height: 500px;".to_string(),
        on_close: move |_| open.set(false),
        ModalContent { active_tab, lp, rp }
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ModalContentProps {
    active_tab: Signal<Tab>,
    lp: LeftPaneProps,
    rp: RightPaneProps,
}

#[component]
fn ModalContent(props: ModalContentProps) -> Element {
    rsx! {
      div {
        style: "display: flex; flex-direction: column; gap: 16px; height: 100%;",
        TabHeader { active_tab: props.active_tab }
        SplitView { lp: props.lp, rp: props.rp }
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TabHeaderProps {
    active_tab: Signal<Tab>,
}

#[component]
fn TabHeader(mut props: TabHeaderProps) -> Element {
    let current = (props.active_tab)();
    let l_act = current == Tab::Light;
    let d_act = current == Tab::Dark;
    let s_act = current == Tab::Shared;

    rsx! {
      div {
        style: "display: flex; gap: 8px; border-bottom: 1px solid var(--editor-border, #ddd); padding-bottom: 8px;",
        TabButton { active: l_act, label: "Light Theme".to_string(), onclick: move |_| props.active_tab.set(Tab::Light) }
        TabButton { active: d_act, label: "Dark Theme".to_string(), onclick: move |_| props.active_tab.set(Tab::Dark) }
        TabButton { active: s_act, label: "Shared Tokens".to_string(), onclick: move |_| props.active_tab.set(Tab::Shared) }
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TabButtonProps {
    active: bool,
    label: String,
    onclick: EventHandler<MouseEvent>,
}

#[component]
fn TabButton(props: TabButtonProps) -> Element {
    let style_str = if props.active {
        "padding: 6px 12px; font-weight: bold; border: none; border-bottom: 2px solid var(--editor-accent, #007acc); background: transparent; cursor: pointer; color: var(--editor-text, #333);"
    } else {
        "padding: 6px 12px; border: none; background: transparent; cursor: pointer; color: var(--editor-muted, #666);"
    };
    rsx! {
      button {
        style: "{style_str}",
        onclick: move |e| props.onclick.call(e),
        "{props.label}"
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SplitViewProps {
    lp: LeftPaneProps,
    rp: RightPaneProps,
}

#[component]
fn SplitView(props: SplitViewProps) -> Element {
    rsx! {
      div {
        style: "display: flex; gap: 20px; flex: 1; min-height: 0;",
        LeftPane {
          active_tab: props.lp.active_tab,
          light_primary: props.lp.light_primary,
          light_bg: props.lp.light_bg,
          dark_primary: props.lp.dark_primary,
          dark_bg: props.lp.dark_bg,
          border_radius: props.lp.border_radius,
        }
        RightPane {
          active_tab: props.rp.active_tab,
          light_primary: props.rp.light_primary,
          light_bg: props.rp.light_bg,
          dark_primary: props.rp.dark_primary,
          dark_bg: props.rp.dark_bg,
          border_radius: props.rp.border_radius,
        }
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct LeftPaneProps {
    active_tab: Tab,
    light_primary: Signal<String>,
    light_bg: Signal<String>,
    dark_primary: Signal<String>,
    dark_bg: Signal<String>,
    border_radius: Signal<String>,
}

#[component]
fn LeftPane(props: LeftPaneProps) -> Element {
    let content = match props.active_tab {
        Tab::Light => rsx! {
          TokenInput { label: "Primary Color".to_string(), val: props.light_primary }
          TokenInput { label: "Background Color".to_string(), val: props.light_bg }
        },
        Tab::Dark => rsx! {
          TokenInput { label: "Primary Color".to_string(), val: props.dark_primary }
          TokenInput { label: "Background Color".to_string(), val: props.dark_bg }
        },
        Tab::Shared => rsx! {
          TokenInput { label: "Border Radius".to_string(), val: props.border_radius }
        },
    };

    rsx! {
      div {
        style: "flex: 1; display: flex; flex-direction: column; gap: 12px; overflow-y: auto;",
        {content}
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TokenInputProps {
    label: String,
    val: Signal<String>,
}

#[component]
fn TokenInput(mut props: TokenInputProps) -> Element {
    rsx! {
      div {
        style: "display: flex; flex-direction: column; gap: 4px;",
        label {
          style: "font-size: 12px; font-weight: bold; color: var(--editor-text-muted, #777);",
          "{props.label}"
        }
        input {
          style: "padding: 6px 8px; border: 1px solid var(--editor-border, #ccc); border-radius: 4px; background: var(--editor-bg, #fff); color: var(--editor-text, #000);",
          value: "{props.val}",
          oninput: move |e| props.val.set(e.value()),
        }
      }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RightPaneProps {
    active_tab: Tab,
    light_primary: String,
    light_bg: String,
    dark_primary: String,
    dark_bg: String,
    border_radius: String,
}

#[component]
fn RightPane(props: RightPaneProps) -> Element {
    let css = match props.active_tab {
        Tab::Light => format!(
            ":root.light {{\n  --primary: {};\n  --bg: {};\n  --radius: {};\n}}",
            props.light_primary, props.light_bg, props.border_radius
        ),
        Tab::Dark => format!(
            ":root.dark {{\n  --primary: {};\n  --bg: {};\n  --radius: {};\n}}",
            props.dark_primary, props.dark_bg, props.border_radius
        ),
        Tab::Shared => format!(":root {{\n  --radius: {};\n}}", props.border_radius),
    };

    rsx! {
      div {
        style: "flex: 1; display: flex; flex-direction: column;",
        div {
          style: "font-size: 12px; font-weight: bold; color: var(--editor-text-muted, #777); margin-bottom: 4px;",
          "Live CSS Preview"
        }
        pre {
          style: "flex: 1; margin: 0; padding: 12px; background: var(--editor-preview-bg, #1e1e1e); color: var(--editor-preview-text, #d4d4d4); font-family: monospace; border-radius: 4px; overflow: auto; white-space: pre-wrap;",
          "{css}"
        }
      }
    }
}
