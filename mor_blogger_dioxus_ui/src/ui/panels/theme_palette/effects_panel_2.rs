use crate::app::theme_signals::ThemeSignals;
use crate::ui::dialogs::modal::Modal;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AdvancedGlowWindowProps {
    pub show_advanced_glow: Signal<bool>,
    pub signals: ThemeSignals,
}

#[component]
pub fn AdvancedGlowWindow(props: AdvancedGlowWindowProps) -> Element {
    let show_advanced_glow = props.show_advanced_glow;
    let mut signals = props.signals;

    rsx! {
        Modal {
            open: show_advanced_glow,
            title: "Advanced Glow Targets".to_string(),
            style: "width: 440px;".to_string(),

            div {
                style: "display: flex; flex-direction: column; gap: 10px;",

                div {
                    class: "editor-help-text",
                    style: "margin-bottom: 4px;",
                    "Glow is off by default. Enable it per target; leave the color blank to use the global glow color (or the accent)."
                }

                // Global override
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding-bottom: 8px; border-bottom: 1px solid var(--border-color); margin-bottom: 4px;",
                    label { style: "font-size: 12px;", "Global Glow Override" }
                    input {
                        r#type: "text",
                        placeholder: "#HEX or empty",
                        style: "width: 110px; background: var(--bg-soft, #2C2C2E); border: 1px solid var(--border-color); color: var(--fg-base); padding: 4px 8px; border-radius: 4px; font-size: 12px;",
                        value: (signals.glow_color)(),
                        oninput: move |evt| signals.glow_color.set(evt.value()),
                    }
                }

                // One row per glow target: color override + enable toggle.
                for (label_text, mut color_sig, mut bool_sig) in [
                    ("Header", signals.glow_header_color, signals.glow_header),
                    ("Main Content Area", signals.glow_main_color, signals.glow_main),
                    ("Footer", signals.glow_footer_color, signals.glow_footer),
                    ("Site Logo", signals.glow_logo_color, signals.glow_logo),
                    ("Post Titles", signals.glow_title_color, signals.glow_title),
                    ("Table of Contents", signals.glow_toc_color, signals.glow_toc),
                    ("Sidebar Widgets", signals.glow_sidebar_color, signals.glow_sidebar),
                    ("Typography (Headings & Text)", signals.glow_text_color, signals.glow_text),
                    ("UI Containers", signals.glow_containers_color, signals.glow_containers),
                    ("Icons & Buttons", signals.glow_icons_color, signals.glow_icons),
                ] {
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        label { style: "font-size: 12px;", "{label_text}" }
                        div {
                            style: "display: flex; gap: 8px; align-items: center;",
                            input {
                                r#type: "color",
                                style: "width: 24px; height: 24px; padding: 0; border: none; background: transparent; cursor: pointer;",
                                value: color_sig(),
                                oninput: move |evt| color_sig.set(evt.value()),
                            }
                            input {
                                r#type: "checkbox",
                                style: "cursor: pointer; accent-color: var(--accent);",
                                checked: bool_sig(),
                                onchange: move |evt| bool_sig.set(evt.checked()),
                                oninput: move |evt| bool_sig.set(evt.checked()),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EffectsPanel(
    glow_spread: Signal<String>,
    hover_scale: Signal<String>,
    mut show_advanced_glow: Signal<bool>,
) -> Element {
    rsx! {
        div { class: "editor-panel",

            div {
                class: "editor-help-text",
                style: "margin-bottom: 16px;",
                "Control structural hover effects and neon glows."
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Neon Glow Spread" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 10px, 20px, 0",
                    value: "{glow_spread}",
                    oninput: move |evt| glow_spread.set(evt.value()),
                }
            }

            button {
                class: "editor-btn secondary",
                onclick: move |_| show_advanced_glow.set(!show_advanced_glow()),
                "Advanced Glow Options"
            }

            div { class: "editor-field-group",
                style: "margin-top: 16px;",
                label { class: "editor-field-label", "Hover Scale (Zoom)" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 1.02, 1.05, 1",
                    value: "{hover_scale}",
                    oninput: move |evt| hover_scale.set(evt.value()),
                }
            }
        }
    }
}
