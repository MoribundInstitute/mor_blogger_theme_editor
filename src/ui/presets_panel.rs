use dioxus::prelude::*;

use crate::config::{BackgroundConfig, SurfaceFill};
use crate::presets::{all_presets, Preset, PresetPalette};

#[derive(Clone, Copy, PartialEq)]
pub struct ThemeSignals {
    pub is_dark_mode: Signal<bool>,

    // Site identity
    pub site_title: Signal<String>,
    pub site_subtitle: Signal<String>,
    pub header_logo_url: Signal<String>,
    pub home_url: Signal<String>,

    // Color palette — bg_panel and bg_elevated are now SurfaceFill
    pub bg_base: Signal<String>,
    pub bg_panel: Signal<SurfaceFill>,
    pub bg_elevated: Signal<SurfaceFill>,
    pub fg_base: Signal<String>,
    pub fg_muted: Signal<String>,
    pub accent: Signal<String>,
    pub border: Signal<String>,

    // Buttons
    pub btn_radius: Signal<String>,
    pub btn_border_width: Signal<String>,
    pub btn_text_transform: Signal<String>,

    // Typography
    pub body_font_stack: Signal<String>,
    pub heading_font_stack: Signal<String>,
    pub mono_font_stack: Signal<String>,
    pub base_size: Signal<String>,
    pub scale_ratio: Signal<String>,
    pub line_height: Signal<String>,
    pub heading_weight: Signal<String>,

    // Media/assets
    pub background: Signal<BackgroundConfig>,
    pub favicon_url: Signal<String>,
    pub social_card_image_url: Signal<String>,

    // SEO
    pub meta_description: Signal<String>,
    pub meta_keywords: Signal<String>,
    pub custom_robots: Signal<String>,
    pub license_url: Signal<String>,
    pub author_name: Signal<String>,

    // Menu (4 slots)
    pub menu_1_label: Signal<String>,
    pub menu_1_url: Signal<String>,
    pub menu_2_label: Signal<String>,
    pub menu_2_url: Signal<String>,
    pub menu_3_label: Signal<String>,
    pub menu_3_url: Signal<String>,
    pub menu_4_label: Signal<String>,
    pub menu_4_url: Signal<String>,

    // Footer
    pub footer_text: Signal<String>,
    pub footer_license_label: Signal<String>,
    pub footer_license_url: Signal<String>,

    // Plugins
    pub custom_js: Signal<String>,

    // Active preset CSS (empty for hand-built themes and re-imported exports)
    pub preset_css: Signal<String>,
}

impl ThemeSignals {
    pub fn apply_preset(&self, preset: &Preset) {
        let is_dark = *self.is_dark_mode.read();
        let palette = if is_dark { &preset.dark } else { &preset.light };

        self.swap_palette(palette);

        let base = &preset.base_config;

        self.site_title.clone().set(base.site.site_title.clone());
        self.site_subtitle.clone().set(base.site.site_subtitle.clone());
        self.header_logo_url.clone().set(base.site.header_logo_url.clone());
        self.home_url.clone().set(base.site.home_url.clone());

        self.btn_radius.clone().set(base.buttons.radius.clone());
        self.btn_border_width.clone().set(base.buttons.border_width.clone());
        self.btn_text_transform.clone().set(base.buttons.text_transform.clone());

        self.body_font_stack.clone().set(base.typography.body_font_stack.clone());
        self.heading_font_stack.clone().set(base.typography.heading_font_stack.clone());
        self.mono_font_stack.clone().set(base.typography.mono_font_stack.clone());
        self.base_size.clone().set(base.typography.base_size.clone());
        self.scale_ratio.clone().set(base.typography.scale_ratio.clone());
        self.line_height.clone().set(base.typography.line_height.clone());
        self.heading_weight.clone().set(base.typography.heading_weight.clone());

        self.favicon_url.clone().set(base.assets.favicon_url.clone());
        self.social_card_image_url.clone().set(base.assets.social_card_image_url.clone());

        self.meta_description.clone().set(base.seo.meta_description.clone());
        self.meta_keywords.clone().set(base.seo.meta_keywords.clone());
        self.custom_robots.clone().set(base.seo.custom_robots.clone());
        self.license_url.clone().set(base.seo.license_url.clone());
        self.author_name.clone().set(base.seo.author_name.clone());

        let menu_pairs = [
            (self.menu_1_label, self.menu_1_url),
            (self.menu_2_label, self.menu_2_url),
            (self.menu_3_label, self.menu_3_url),
            (self.menu_4_label, self.menu_4_url),
        ];

        for (i, (mut label_sig, mut url_sig)) in menu_pairs.into_iter().enumerate() {
            let (label, url) = base
                .menu_links
                .get(i)
                .map(|menu| (menu.label.clone(), menu.url.clone()))
                .unwrap_or_default();
            label_sig.set(label);
            url_sig.set(url);
        }

        self.footer_text.clone().set(base.footer.footer_text.clone());
        self.footer_license_label.clone().set(base.footer.footer_license_label.clone());
        self.footer_license_url.clone().set(base.footer.footer_license_url.clone());

        self.custom_js.clone().set(base.plugins.custom_js.clone());
        
        // Carry the preset's optional CSS bundle into the live signals.
        self.preset_css.clone().set(preset.preset_css.to_string());
    }

    pub fn swap_palette(&self, palette: &PresetPalette) {
        self.bg_base.clone().set(palette.colors.bg_base.clone());
        self.bg_panel.clone().set(palette.colors.bg_panel.clone());
        self.bg_elevated.clone().set(palette.colors.bg_elevated.clone());
        self.fg_base.clone().set(palette.colors.fg_base.clone());
        self.fg_muted.clone().set(palette.colors.fg_muted.clone());
        self.accent.clone().set(palette.colors.accent.clone());
        self.border.clone().set(palette.colors.border.clone());
        self.background.clone().set(palette.background.clone());
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PresetsPanelProps {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
}

#[component]
pub fn PresetsPanel(props: PresetsPanelProps) -> Element {
    let presets = all_presets();
    let active = props.active_preset;
    let mut is_dark_mode = props.signals.is_dark_mode;

    rsx! {
        section {
            class: "editor-card preset-panel",

            div {
                class: "preset-panel-header",

                h3 {
                    class: "editor-card-title",
                    style: "margin-bottom: 0; padding-bottom: 0; border-bottom: none;",
                    "Theme Presets"
                }

                button {
                    class: "editor-button editor-button-small",
                    onclick: move |_| {
                        let new_mode = !is_dark_mode();
                        is_dark_mode.set(new_mode);

                        if let Some(active_id) = active() {
                            if let Some(preset) = presets.iter().find(|preset| preset.id == active_id) {
                                if new_mode {
                                    props.signals.swap_palette(&preset.dark);
                                } else {
                                    props.signals.swap_palette(&preset.light);
                                }
                            }
                        }
                    },
                    if is_dark_mode() { "☾ Dark Mode" } else { "☀ Light Mode" }
                }
            }

            p {
                class: "preset-panel-copy",
                "One click to swap the entire theme. Edit any field afterward to customize."
            }

            div {
                class: "preset-rail",

                for preset in presets.iter() {
                    PresetCard {
                        key: "{preset.id}",
                        preset: preset.clone(),
                        is_active: active.read().map(|id| id == preset.id).unwrap_or(false),
                        signals: props.signals,
                        active_preset: active,
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PresetCardProps {
    preset: Preset,
    is_active: bool,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
}

#[component]
fn PresetCard(props: PresetCardProps) -> Element {
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
            style: "--preset-accent: {colors.accent};",
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