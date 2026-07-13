use crate::app::state::ThemeState;
use crate::ui::components::inputs::{EditorCheckbox, EditorSelect, EditorTextField};
use crate::ui::dialogs::modal::Modal;
use crate::ui::dialogs::toml_quick_edit::TomlQuickEdit;
use dioxus::prelude::*;
use mor_blogger_core::render::xml_parts::css_generator::render_button_styles;

const LBL: &str = "font-size: 11px; color: #8e8e93;";
const H3: &str = "color: var(--editor-accent, #a9aae2); margin: 10px 0 0 0; font-size: 16px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;";

/// Shape shim: static option literals → the owned pairs [`EditorSelect`] takes.
fn opts(o: &[(&str, &str)]) -> Vec<(String, String)> {
    o.iter().map(|(v, l)| (v.to_string(), l.to_string())).collect()
}

#[component]
pub fn AdvancedButtonsDialog(mut open_signal: Signal<bool>) -> Element {
    let theme = use_context::<ThemeState>();
    let mut buttons = theme.signals.buttons;
    let b = buttons.read().clone();

    // Scoped preview: feed the live theme vars into a container and inject the
    // same CSS the export generates, prefixed so it only styles this preview.
    let accent = theme.signals.accent.read().clone();
    let fg = theme.signals.fg_base.read().clone();
    let bg = theme.signals.bg_base.read().clone();
    let border = theme.signals.border.read().clone();
    let preview_css = render_button_styles(&b, ".mor-btn-preview");
    let buttons_toml = toml::to_string_pretty(&b).unwrap_or_default();

    rsx! {
        Modal {
            open: open_signal,
            title: "Advanced Buttons",
            style: "width: 560px; max-height: 82vh; overflow-y: auto;".to_string(),
            on_close: move |_| open_signal.set(false),

            div { style: "padding: 20px; display: flex; flex-direction: column; gap: 18px;",

                // Live, faithful preview (same CSS the export emits, scoped).
                div {
                    class: "mor-btn-preview",
                    style: "--accent: {accent}; --fg-base: {fg}; --bg-base: {bg}; --border-color: {border}; --fg-dim: {border}; display: flex; gap: 10px; align-items: center; padding: 16px; background: {bg}; border: 1px solid #333; border-radius: 6px;",
                    style { "{preview_css}" }
                    span { style: "{LBL}", "Preview" }
                    button { class: "pager-btn", style: "margin-left: auto;", "Button" }
                    button { class: "pager-btn", "Read More" }
                }

                h3 { style: "{H3}", "Style Presets" }
                p { style: "{LBL} margin: 0;", "One-click starting points — tweak any field afterward." }
                div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    // Neon Link: outline that fills to the accent on hover (this blog's
                    // home/feed/pager links).
                    button { class: "editor-button", onclick: move |_| {
                        let mut w = buttons.write();
                        w.fill = "outline".into(); w.hover_effect = "invert".into();
                        w.border_width = "2px".into(); w.border_style = "solid".into();
                        w.glow_strength = "14px".into(); w.transition_ms = "200ms".into();
                        w.radius = "8px".into(); w.pressed_feedback = true;
                    }, "Neon Link" }
                    // Glow Pill: solid rounded button that blooms on hover.
                    button { class: "editor-button", onclick: move |_| {
                        let mut w = buttons.write();
                        w.fill = "solid".into(); w.hover_effect = "glow".into();
                        w.radius = "999px".into(); w.border_width = "0".into();
                        w.glow_strength = "14px".into(); w.transition_ms = "200ms".into();
                        w.pressed_feedback = true;
                    }, "Glow Pill" }
                    // Ghost: borderless, brightens on hover.
                    button { class: "editor-button", onclick: move |_| {
                        let mut w = buttons.write();
                        w.fill = "ghost".into(); w.hover_effect = "brighten".into();
                        w.border_width = "0".into(); w.transition_ms = "150ms".into();
                        w.radius = "8px".into();
                    }, "Ghost" }
                    // Glass: frosted blur that lifts on hover.
                    button { class: "editor-button", onclick: move |_| {
                        let mut w = buttons.write();
                        w.fill = "glass".into(); w.hover_effect = "lift".into();
                        w.glass_blur = "12px".into(); w.glass_opacity = "0.4".into();
                        w.radius = "12px".into(); w.transition_ms = "200ms".into();
                    }, "Glass" }
                    // Flat: plain solid, no motion — a clean reset.
                    button { class: "editor-button", onclick: move |_| {
                        let mut w = buttons.write();
                        w.fill = "solid".into(); w.hover_effect = "none".into();
                        w.elevation = "flat".into(); w.radius = "6px".into();
                        w.border_width = "1px".into(); w.glow_strength = "".into();
                        w.box_shadow = "".into(); w.transition_ms = "150ms".into();
                    }, "Flat" }
                }

                h3 { style: "{H3}", "Geometry" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                    EditorTextField { label: "Radius", value: b.radius.clone(), placeholder: "8px",
                        oninput: move |v| { buttons.write().radius = v; } }
                    EditorTextField { label: "Padding X", value: b.padding_x.clone(), placeholder: "14px",
                        oninput: move |v| { buttons.write().padding_x = v; } }
                    EditorTextField { label: "Padding Y", value: b.padding_y.clone(), placeholder: "6px",
                        oninput: move |v| { buttons.write().padding_y = v; } }
                    EditorTextField { label: "Border Width", value: b.border_width.clone(), placeholder: "1px",
                        oninput: move |v| { buttons.write().border_width = v; } }
                    EditorSelect { label: "Border Style", value: b.border_style.clone(),
                        options: opts(&[("solid","Solid"),("dashed","Dashed"),("none","None"),("outset","Outset (3D)"),("inset","Inset (3D)"),("ridge","Ridge"),("groove","Groove")]),
                        onchange: move |e: Event<FormData>| { buttons.write().border_style = e.value(); } }
                    div { style: "margin-top: 18px;",
                        EditorCheckbox { label: "Full width", checked: b.full_width,
                            onchange: move |v| { buttons.write().full_width = v; } }
                    }
                }

                h3 { style: "{H3}", "Type" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    EditorSelect { label: "Font Weight", value: b.font_weight.clone(),
                        options: opts(&[("","Inherit"),("400","Normal (400)"),("500","Medium (500)"),("600","Semibold (600)"),("700","Bold (700)")]),
                        onchange: move |e: Event<FormData>| { buttons.write().font_weight = e.value(); } }
                    EditorTextField { label: "Letter Spacing", value: b.letter_spacing.clone(), placeholder: "0.02em",
                        oninput: move |v| { buttons.write().letter_spacing = v; } }
                }

                h3 { style: "{H3}", "Interaction & Motion" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    EditorSelect { label: "Hover Effect", value: b.hover_effect.clone(),
                        options: opts(&[("none","None"),("lift","Lift"),("grow","Grow"),("brighten","Brighten"),("glow","Glow"),("invert","Invert (neon fill)")]),
                        onchange: move |e: Event<FormData>| { buttons.write().hover_effect = e.value(); } }
                    EditorTextField { label: "Transition", value: b.transition_ms.clone(), placeholder: "150ms",
                        oninput: move |v| { buttons.write().transition_ms = v; } }
                    EditorSelect { label: "Easing", value: b.easing.clone(),
                        options: opts(&[
                            ("ease","Ease (default)"),("linear","Linear"),
                            ("ease-in","Ease In"),("ease-out","Ease Out"),("ease-in-out","Ease In-Out"),
                            ("cubic-bezier(0.22,1,0.36,1)","Snappy (expo out)"),
                            ("cubic-bezier(0.83,0,0.17,1)","Dramatic (expo in-out)"),
                            ("cubic-bezier(0.34,1.56,0.64,1)","Back (overshoot)"),
                            ("cubic-bezier(0.68,-0.55,0.27,1.55)","Anticipate"),
                        ]),
                        onchange: move |e: Event<FormData>| { buttons.write().easing = e.value(); } }
                    div { style: "margin-top: 18px;",
                        EditorCheckbox { label: "Pressed feedback", checked: b.pressed_feedback,
                            onchange: move |v| { buttons.write().pressed_feedback = v; } }
                    }
                }

                h3 { style: "{H3}", "Effects" }
                p { style: "{LBL} margin: 0;", "Glass = frosted blur · Neon = glowing edge · Glossy/Gradient = sheen. Glow params also drive the Glow hover." }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    EditorSelect { label: "Fill / Effect", value: b.fill.clone(),
                        options: opts(&[("outline","Outline"),("solid","Solid"),("soft","Soft"),("ghost","Ghost"),("glass","Glass"),("neon","Neon"),("glossy","Glossy"),("gradient","Gradient")]),
                        onchange: move |e: Event<FormData>| { buttons.write().fill = e.value(); } }
                    EditorTextField { label: "Glow Color", value: b.glow_color.clone(), placeholder: "auto (accent)",
                        oninput: move |v| { buttons.write().glow_color = v; } }
                    EditorTextField { label: "Glow Strength", value: b.glow_strength.clone(), placeholder: "12px",
                        oninput: move |v| { buttons.write().glow_strength = v; } }
                    EditorTextField { label: "Glass Blur", value: b.glass_blur.clone(), placeholder: "12px",
                        oninput: move |v| { buttons.write().glass_blur = v; } }
                    EditorTextField { label: "Glass Opacity (0-1)", value: b.glass_opacity.clone(), placeholder: "0.4",
                        oninput: move |v| { buttons.write().glass_opacity = v; } }
                }
                p { style: "{LBL} margin: 8px 0 0;", "Gradient fill (from → to). Accepts colors or CSS like color-mix()/var()." }
                div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                    EditorTextField { label: "Gradient From", value: b.gradient_from.clone(), placeholder: "var(--bg-panel)",
                        oninput: move |v| { buttons.write().gradient_from = v; } }
                    EditorTextField { label: "Gradient To", value: b.gradient_to.clone(), placeholder: "var(--bg-base)",
                        oninput: move |v| { buttons.write().gradient_to = v; } }
                    EditorTextField { label: "Angle", value: b.gradient_angle.clone(), placeholder: "180deg",
                        oninput: move |v| { buttons.write().gradient_angle = v; } }
                }

                h3 { style: "{H3}", "Elevation" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    EditorSelect { label: "Shadow Depth", value: b.elevation.clone(),
                        options: opts(&[("flat","Flat"),("subtle","Subtle"),("raised","Raised")]),
                        onchange: move |e: Event<FormData>| { buttons.write().elevation = e.value(); } }
                    EditorTextField { label: "Custom Shadow (overrides depth)", value: b.box_shadow.clone(), placeholder: "e.g. inset 0 1px 0 #fff",
                        oninput: move |v| { buttons.write().box_shadow = v; } }
                }

                h3 { style: "{H3}", "Color Overrides" }
                p { style: "{LBL} margin: 0;", "Leave blank to derive from the theme accent." }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    EditorTextField { label: "Fill", value: b.bg_color.clone(), placeholder: "#c2622a",
                        oninput: move |v| { buttons.write().bg_color = v; } }
                    EditorTextField { label: "Text", value: b.text_color.clone(), placeholder: "#ffffff",
                        oninput: move |v| { buttons.write().text_color = v; } }
                    EditorTextField { label: "Border", value: b.border_color.clone(), placeholder: "auto",
                        oninput: move |v| { buttons.write().border_color = v; } }
                    EditorTextField { label: "Hover Fill", value: b.hover_bg_color.clone(), placeholder: "auto",
                        oninput: move |v| { buttons.write().hover_bg_color = v; } }
                }

                h3 { style: "{H3}", "Focus Ring" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    EditorTextField { label: "Color", value: b.focus_ring_color.clone(), placeholder: "auto",
                        oninput: move |v| { buttons.write().focus_ring_color = v; } }
                    EditorTextField { label: "Width", value: b.focus_ring_width.clone(), placeholder: "2px",
                        oninput: move |v| { buttons.write().focus_ring_width = v; } }
                }

                h3 { style: "{H3}", "Edit as TOML" }
                TomlQuickEdit {
                    toml_text: buttons_toml,
                    subfolder: "buttons".to_string(),
                    on_apply: move |txt: String| {
                        if let Ok(cfg) = toml::from_str::<mor_blogger_core::config::ButtonConfig>(&txt) {
                            buttons.set(cfg);
                        }
                    }
                }
            }
        }
    }
}
