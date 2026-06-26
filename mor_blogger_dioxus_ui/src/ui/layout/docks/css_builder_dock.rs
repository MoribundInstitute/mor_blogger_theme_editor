use crate::app::state::{DockPosition, LayoutState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::dock_chrome::DockChrome;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum BuilderTab {
    Light,
    Dark,
    Shared,
}

#[component]
pub fn CssBuilderDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let pos = (layout.css_builder_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let mut active_tab = use_signal(|| BuilderTab::Light);
    let mut light_primary = use_signal(|| "#3b82f6".to_string());
    let mut light_bg = use_signal(|| "#ffffff".to_string());
    let mut dark_primary = use_signal(|| "#60a5fa".to_string());
    let mut dark_bg = use_signal(|| "#1e293b".to_string());
    let mut border_radius = use_signal(|| "6px".to_string());

    let css = match active_tab() {
        BuilderTab::Light => format!(
            ":root.light {{\n  --primary: {};\n  --bg: {};\n  --radius: {};\n}}",
            light_primary, light_bg, border_radius
        ),
        BuilderTab::Dark => format!(
            ":root.dark {{\n  --primary: {};\n  --bg: {};\n  --radius: {};\n}}",
            dark_primary, dark_bg, border_radius
        ),
        BuilderTab::Shared => format!(":root {{\n  --radius: {};\n}}", border_radius),
    };

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "CSS Token Builder".to_string(),
                dock_id: "css_builder".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.css_builder_pos.set(DockPosition::Hidden);
                },
                div {
                    style: "display: flex; flex-direction: column; height: calc(100% - 45px); overflow: hidden; background: var(--bg-panel); color: var(--fg-base); font-size: 0.85rem;",

                    // Tab Buttons
                    div {
                        style: "display: flex; border-bottom: 1px solid var(--border); background: var(--bg-elevated); padding: 4px 8px; gap: 4px;",
                        button {
                            style: format!("padding: 4px 8px; border: none; background: {}; color: {}; font-size: 0.8rem; cursor: pointer; border-radius: 2px;", if active_tab() == BuilderTab::Light { "var(--accent)" } else { "transparent" }, if active_tab() == BuilderTab::Light { "#111" } else { "inherit" }),
                            onclick: move |_| active_tab.set(BuilderTab::Light),
                            "Light"
                        }
                        button {
                            style: format!("padding: 4px 8px; border: none; background: {}; color: {}; font-size: 0.8rem; cursor: pointer; border-radius: 2px;", if active_tab() == BuilderTab::Dark { "var(--accent)" } else { "transparent" }, if active_tab() == BuilderTab::Dark { "#111" } else { "inherit" }),
                            onclick: move |_| active_tab.set(BuilderTab::Dark),
                            "Dark"
                        }
                        button {
                            style: format!("padding: 4px 8px; border: none; background: {}; color: {}; font-size: 0.8rem; cursor: pointer; border-radius: 2px;", if active_tab() == BuilderTab::Shared { "var(--accent)" } else { "transparent" }, if active_tab() == BuilderTab::Shared { "#111" } else { "inherit" }),
                            onclick: move |_| active_tab.set(BuilderTab::Shared),
                            "Shared"
                        }
                    }

                    // Inputs Section
                    div {
                        style: "padding: 12px; display: flex; flex-direction: column; gap: 10px; border-bottom: 1px solid var(--border);",
                        match active_tab() {
                            BuilderTab::Light => rsx! {
                                div { style: "display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 0.75rem; color: var(--fg-muted);", "Primary Color" }
                                    input {
                                        style: "padding: 6px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg-elevated); color: var(--fg-base); font-size: 0.8rem;",
                                        value: "{light_primary}",
                                        oninput: move |e| light_primary.set(e.value()),
                                    }
                                }
                                div { style: "display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 0.75rem; color: var(--fg-muted);", "Background Color" }
                                    input {
                                        style: "padding: 6px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg-elevated); color: var(--fg-base); font-size: 0.8rem;",
                                        value: "{light_bg}",
                                        oninput: move |e| light_bg.set(e.value()),
                                    }
                                }
                            },
                            BuilderTab::Dark => rsx! {
                                div { style: "display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 0.75rem; color: var(--fg-muted);", "Primary Color" }
                                    input {
                                        style: "padding: 6px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg-elevated); color: var(--fg-base); font-size: 0.8rem;",
                                        value: "{dark_primary}",
                                        oninput: move |e| dark_primary.set(e.value()),
                                    }
                                }
                                div { style: "display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 0.75rem; color: var(--fg-muted);", "Background Color" }
                                    input {
                                        style: "padding: 6px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg-elevated); color: var(--fg-base); font-size: 0.8rem;",
                                        value: "{dark_bg}",
                                        oninput: move |e| dark_bg.set(e.value()),
                                    }
                                }
                            },
                            BuilderTab::Shared => rsx! {
                                div { style: "display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 0.75rem; color: var(--fg-muted);", "Border Radius" }
                                    input {
                                        style: "padding: 6px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg-elevated); color: var(--fg-base); font-size: 0.8rem;",
                                        value: "{border_radius}",
                                        oninput: move |e| border_radius.set(e.value()),
                                    }
                                }
                            }
                        }
                    }

                    // Code Preview Section
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; min-height: 120px; margin-top: 10px;",
                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column; border: 1px solid var(--border); border-radius: 4px; overflow: hidden;",
                            CodeEditor {
                                value: css,
                                mode: "css".to_string(),
                                minimap_key: Some("css_builder".to_string()),
                                read_only: true,
                                on_change: |_| {},
                            }
                        }
                    }
                }
            }
        }
    }
}
