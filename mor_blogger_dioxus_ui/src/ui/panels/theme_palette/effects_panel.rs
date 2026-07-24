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

    // (label, color signal, enabled signal). Signals are Copy, so this array is
    // reused both by the combo presets and the per-target rows below.
    let targets: [(&str, Signal<String>, Signal<bool>); 10] = [
        ("Header", signals.glow_header_color, signals.glow_header),
        ("Main Content Area", signals.glow_main_color, signals.glow_main),
        ("Footer", signals.glow_footer_color, signals.glow_footer),
        ("Site Logo", signals.glow_logo_color, signals.glow_logo),
        ("Post Titles", signals.glow_title_color, signals.glow_title),
        ("Table of Contents", signals.glow_toc_color, signals.glow_toc),
        ("Sidebars", signals.glow_sidebar_color, signals.glow_sidebar),
        ("Typography (Headings & Text)", signals.glow_text_color, signals.glow_text),
        ("Post Cards", signals.glow_containers_color, signals.glow_containers),
        ("Icons & Buttons", signals.glow_icons_color, signals.glow_icons),
    ];

    // One-click starting points. Each lists the targets it turns ON; everything
    // else turns OFF, so the buttons are also a clean "reset".
    let combos: [(&str, &[&str]); 4] = [
        ("Off", &[]),
        ("Subtle", &["Post Titles", "Site Logo"]),
        (
            "Neon",
            &["Post Titles", "Site Logo", "Typography (Headings & Text)", "Sidebars", "Icons & Buttons"],
        ),
        (
            "Everything",
            &[
                "Header", "Main Content Area", "Footer", "Site Logo", "Post Titles",
                "Table of Contents", "Sidebars", "Typography (Headings & Text)",
                "Post Cards", "Icons & Buttons",
            ],
        ),
    ];

    rsx! {
        Modal {
            open: show_advanced_glow,
            title: "Advanced Glow Targets".to_string(),
            style: "width: 460px;".to_string(),

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                div { class: "editor-help-text",
                    "Pick a starting combo, then fine-tune each target. Blank colour = global glow colour (or the accent)."
                }

                // Quick combos
                div { class: "editor-segmented", style: "display: flex; gap: 6px;",
                    for (name, on_labels) in combos.into_iter() {
                        button {
                            class: "editor-mini-button",
                            style: "flex: 1; justify-content: center;",
                            onclick: move |_| {
                                for (label, _c, mut enabled) in targets.into_iter() {
                                    enabled.set(on_labels.contains(&label));
                                }
                            },
                            "{name}"
                        }
                    }
                }

                // Global override
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding-bottom: 10px; border-bottom: 1px solid var(--border-color);",
                    label { style: "font-size: 12px;", "Global Glow Override" }
                    input {
                        r#type: "text",
                        placeholder: "#HEX or empty",
                        style: "width: 120px; background: var(--bg-soft, #2C2C2E); border: 1px solid var(--border-color); color: var(--fg-base); padding: 4px 8px; border-radius: 4px; font-size: 12px;",
                        value: (signals.glow_color)(),
                        oninput: move |evt| signals.glow_color.set(evt.value()),
                    }
                }

                // Per-target rows: the whole row toggles; ON rows light up with the accent.
                for (label_text, mut color_sig, mut bool_sig) in targets.into_iter() {
                    div {
                        style: format!(
                            "display: flex; justify-content: space-between; align-items: center; gap: 10px; padding: 8px 10px; border-radius: 8px; cursor: pointer; transition: background 0.12s ease; border-left: 3px solid {}; background: {};",
                            if bool_sig() { "var(--accent)" } else { "transparent" },
                            if bool_sig() { "color-mix(in srgb, var(--accent) 12%, var(--bg-elevated))" } else { "var(--bg-elevated)" },
                        ),
                        onclick: move |_| { let v = bool_sig(); bool_sig.set(!v); },

                        span { style: "font-size: 12px; color: var(--fg-base);", "{label_text}" }
                        div { style: "display: flex; gap: 10px; align-items: center;",
                            crate::ui::components::inputs::ColorInput {
                                class: "",
                                style: "width: 22px; height: 22px; padding: 0; border: none; background: transparent; cursor: pointer;",
                                value: color_sig(),
                                swatches: false,
                                oninput: move |v: String| color_sig.set(v),
                            }
                            span {
                                style: format!(
                                    "font-size: 11px; font-weight: 600; letter-spacing: 0.04em; padding: 3px 10px; border-radius: 999px; {}",
                                    if bool_sig() { "background: var(--accent); color: #0a0a0a;" } else { "background: var(--bg-soft, #2C2C2E); color: var(--fg-muted);" },
                                ),
                                if bool_sig() { "ON" } else { "OFF" }
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
    mut glow_intensity: Signal<String>,
    mut glow_hover: Signal<bool>,
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

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Glow Intensity (layers, 1–4)" }
                input {
                    class: "editor-input",
                    r#type: "text",
                    placeholder: "e.g. 1 (soft), 2, 3 (deep neon)",
                    value: "{glow_intensity}",
                    oninput: move |evt| glow_intensity.set(evt.value()),
                }
            }

            label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; font-size: 13px; margin: 0 0 4px;",
                input {
                    r#type: "checkbox",
                    checked: glow_hover(),
                    onchange: move |evt| glow_hover.set(evt.checked()),
                }
                "Glow only on hover"
            }
            div { class: "editor-help-text", style: "margin: 0 0 14px;",
                "On = enabled glow targets light up on hover/focus. Off = glow is always on (and brighter on hover)."
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
