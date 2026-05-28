use dioxus::prelude::*;

use crate::presets::Preset;

use super::signals::ThemeSignals;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct PresetCardProps {
    preset: Preset,
    is_active: bool,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
}

#[component]
pub(crate) fn PresetCard(props: PresetCardProps) -> Element {
    let preset = &props.preset;
    let mut active = props.active_preset;
    let preset_for_click = preset.clone();

    let is_dark = *props.signals.is_dark_mode.read();
    let palette = if is_dark { &preset.dark } else { &preset.light };
    let colors = &palette.colors;

    let bg_panel_css = colors.bg_panel.to_css();
    let bg_elevated_css = colors.bg_elevated.to_css();

    let active_class = if props.is_active {
        "preset-card preset-card-active"
    } else {
        "preset-card"
    };

    let sample_label = preset
        .base_config
        .menu_links
        .first()
        .map(|menu| menu.label.as_str())
        .filter(|label| !label.is_empty())
        .unwrap_or("Home");

    let sample_radius = &preset.base_config.buttons.radius;
    let sample_border_width = &preset.base_config.buttons.border_width;
    let sample_text_transform = &preset.base_config.buttons.text_transform;

    let body_font = &preset.base_config.typography.body_font_stack;
    let heading_font_raw = &preset.base_config.typography.heading_font_stack;
    let heading_font: &str = if heading_font_raw.trim().is_empty() {
        body_font
    } else {
        heading_font_raw
    };

    rsx! {
        button {
            class: "{active_class}",
            style: "--preset-accent: {colors.accent}; flex: 0 0 240px; min-width: 240px; scroll-snap-align: start;",
            onclick: move |_| {
                props.signals.apply_preset(&preset_for_click);
                active.set(Some(preset_for_click.id));
            },

            div {
                style: "padding: 10px 12px; background: {bg_panel_css}; border-bottom: 1px solid {colors.border};",

                div {
                    style: "color: {colors.accent}; font-family: {heading_font}; font-size: 0.9rem; font-weight: bold; line-height: 1.2;",
                    "{preset.base_config.site.site_title}"
                }

                if !preset.base_config.site.site_subtitle.is_empty() {
                    div {
                        style: "color: {colors.fg_muted}; font-family: {body_font}; font-size: 0.65rem; margin-top: 2px; line-height: 1.2; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        "{preset.base_config.site.site_subtitle}"
                    }
                }

                div {
                    style: "display: inline-block; margin-top: 6px; padding: 2px 6px; border: {sample_border_width} solid {colors.border}; border-radius: {sample_radius}; color: {colors.fg_base}; font-family: {body_font}; font-size: 0.65rem; text-transform: {sample_text_transform};",
                    "{sample_label}"
                }
            }

            div {
                style: "padding: 8px 12px; background: {bg_elevated_css}; min-height: 24px;",

                div {
                    style: "color: {colors.fg_muted}; font-family: {body_font}; font-size: 0.6rem; line-height: 1.3;",
                    "Sample content"
                }
            }

            div {
                class: "preset-footer",

                div {
                    class: "preset-name",
                    "{preset.name}"
                }

                div {
                    class: "preset-description",
                    "{preset.description}"
                }
            }
        }
    }
}
