use dioxus::prelude::*;
use crate::ui::panels::theme_palette::presets::ThemeSignals;

#[derive(Props, Clone, PartialEq)]
pub struct AdvancedGlowWindowProps {
    pub show_advanced_glow: Signal<bool>,
    pub signals: ThemeSignals,
}

#[component]
pub fn AdvancedGlowWindow(props: AdvancedGlowWindowProps) -> Element {
    let mut show_advanced_glow = props.show_advanced_glow;
    let mut signals = props.signals;

    let mut is_dragging = use_signal(|| false);
    let mut window_pos = use_signal(|| (250.0, 150.0));
    let mut drag_offset = use_signal(|| (0.0, 0.0));

    rsx! {
        div {
            class: "floating-window advanced-glow-window",
            style: "position: fixed; left: {window_pos().0}px; top: {window_pos().1}px; z-index: 1000; background: #1C1C1E; border: 1px solid #3A3A3C; border-radius: 10px; box-shadow: 0 20px 40px rgba(0,0,0,0.7); width: 320px; font-family: system-ui, -apple-system, sans-serif;",

            // Frameless Header
            div {
                class: "window-header",
                style: "padding: 12px 16px; background: transparent; cursor: grab; display: flex; justify-content: space-between; align-items: center;",
                onmousedown: move |e| {
                    is_dragging.set(true);
                    let coords = e.client_coordinates();
                    drag_offset.set((coords.x - window_pos().0, coords.y - window_pos().1));
                },
                onmousemove: move |e| {
                    if is_dragging() {
                        let coords = e.client_coordinates();
                        window_pos.set((coords.x - drag_offset().0, coords.y - drag_offset().1));
                    }
                },
                onmouseup: move |_| is_dragging.set(false),
                onmouseleave: move |_| is_dragging.set(false),

                h3 { style: "margin: 0; font-size: 13px; font-weight: 600; color: #E5E5EA; letter-spacing: 0.3px;", "Advanced Glow Targets" }
                button {
                    class: "close-btn",
                    style: "background: #ff5f56; border: 1px solid #e0443e; color: transparent; width: 12px; height: 12px; border-radius: 50%; cursor: pointer; padding: 0; display: flex; justify-content: center; align-items: center;",
                    onclick: move |_| show_advanced_glow.set(false),
                    ""
                }
            }

            // Body
            div {
                class: "window-body",
                style: "padding: 0 16px 16px 16px; display: flex; flex-direction: column; gap: 10px;",

                div { 
                    style: "font-size: 11px; color: #8E8E93; margin-bottom: 8px; line-height: 1.4;",
                    "Target specific UI elements. Override global glow color or disable entirely."
                }

                // Global Override
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding-bottom: 8px; border-bottom: 1px solid #2C2C2E; margin-bottom: 4px;",
                    label { style: "font-size: 12px; color: #E5E5EA;", "Global Glow Override" }
                    input {
                        r#type: "text",
                        placeholder: "#HEX or empty",
                        style: "width: 100px; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; padding: 4px 8px; border-radius: 4px; font-size: 12px;",
                        value: (signals.glow_color)(),
                        oninput: move |evt| signals.glow_color.set(evt.value()),
                    }
                }

                // Dynamic loop for all targets
                for (label_text, mut color_sig, mut bool_sig) in [
                    ("Site Logo", signals.glow_logo_color, signals.glow_logo),
                    ("Post Titles", signals.glow_title_color, signals.glow_title),
                    ("Table of Contents", signals.glow_toc_color, signals.glow_toc),
                    ("Sidebar Widgets", signals.glow_sidebar_color, signals.glow_sidebar),
                    ("Headings & Text", signals.glow_text_color, signals.glow_text),
                    ("UI Containers", signals.glow_containers_color, signals.glow_containers),
                    ("Icons & Buttons", signals.glow_icons_color, signals.glow_icons),
                ] {
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        label { style: "font-size: 12px; color: #D1D1D6;", "{label_text}" }
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
pub fn EffectsPanel(glow_spread: Signal<String>, hover_scale: Signal<String>, mut show_advanced_glow: Signal<bool>) -> Element {
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