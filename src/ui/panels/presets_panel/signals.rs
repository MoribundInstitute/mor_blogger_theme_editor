use dioxus::prelude::*;

use crate::config::{AdsConfig, BackgroundConfig, IconConfig, SurfaceFill, ThemeConfig};
use crate::presets::{Preset, PresetPalette};

#[derive(Clone, Copy, PartialEq)]
pub struct ThemeSignals {
    pub is_dark_mode: Signal<bool>,

    // Site identity
    pub site_title: Signal<String>,
    pub site_subtitle: Signal<String>,
    pub header_logo_url: Signal<String>,
    pub home_url: Signal<String>,

    // Color palette — bg_panel and bg_elevated are SurfaceFill.
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

    // Menu: four slots for the GUI.
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

    // Active preset CSS.
    //
    // This is the important export/preview bridge:
    // - presets/css/*.css lives in the preset module through include_str!
    // - selecting a preset must copy that string here
    // - ThemeConfig must then carry this into render/xml_generator.rs
    // - xml_generator.rs must replace {{PRESET_CSS}} with this value
    pub preset_css: Signal<String>,

    // Static pages
    pub static_pages: Signal<crate::config::StaticPagesConfig>,

    // Ads
    pub ads: Signal<AdsConfig>,

    // GTK4-style icon masks
    pub icons: Signal<IconConfig>,
}

impl ThemeSignals {
    pub fn apply_preset(&self, preset: &Preset) {
        let is_dark = *self.is_dark_mode.read();
        let palette = if is_dark { &preset.dark } else { &preset.light };

        self.swap_palette(palette);

        let base = &preset.base_config;
        self.icons.clone().set(base.icons.clone());
        self.apply_config_except_palette(base);

        self.apply_preset_css(preset);
    }

    pub fn apply_config(&self, config: &ThemeConfig) {
        self.bg_base.clone().set(config.colors.bg_base.clone());
        self.bg_panel.clone().set(config.colors.bg_panel.clone());
        self.bg_elevated
            .clone()
            .set(config.colors.bg_elevated.clone());
        self.fg_base.clone().set(config.colors.fg_base.clone());
        self.fg_muted.clone().set(config.colors.fg_muted.clone());
        self.accent.clone().set(config.colors.accent.clone());
        self.border.clone().set(config.colors.border.clone());
        self.background.clone().set(config.background.clone());
        self.icons.clone().set(config.icons.clone());

        self.apply_config_except_palette(config);

        // Imported/restored themes may include preset CSS. Preserve it.
        self.preset_css.clone().set(config.preset_css.clone());
    }

    pub fn apply_preset_css(&self, preset: &Preset) {
        self.preset_css.clone().set(preset.preset_css.to_string());
    }

    fn apply_config_except_palette(&self, config: &ThemeConfig) {
        self.site_title.clone().set(config.site.site_title.clone());
        self.site_subtitle
            .clone()
            .set(config.site.site_subtitle.clone());
        self.header_logo_url
            .clone()
            .set(config.site.header_logo_url.clone());
        self.home_url.clone().set(config.site.home_url.clone());

        self.btn_radius.clone().set(config.buttons.radius.clone());
        self.btn_border_width
            .clone()
            .set(config.buttons.border_width.clone());
        self.btn_text_transform
            .clone()
            .set(config.buttons.text_transform.clone());

        self.body_font_stack
            .clone()
            .set(config.typography.body_font_stack.clone());
        self.heading_font_stack
            .clone()
            .set(config.typography.heading_font_stack.clone());
        self.mono_font_stack
            .clone()
            .set(config.typography.mono_font_stack.clone());
        self.base_size
            .clone()
            .set(config.typography.base_size.clone());
        self.scale_ratio
            .clone()
            .set(config.typography.scale_ratio.clone());
        self.line_height
            .clone()
            .set(config.typography.line_height.clone());
        self.heading_weight
            .clone()
            .set(config.typography.heading_weight.clone());

        self.favicon_url
            .clone()
            .set(config.assets.favicon_url.clone());
        self.social_card_image_url
            .clone()
            .set(config.assets.social_card_image_url.clone());

        self.meta_description
            .clone()
            .set(config.seo.meta_description.clone());
        self.meta_keywords
            .clone()
            .set(config.seo.meta_keywords.clone());
        self.custom_robots
            .clone()
            .set(config.seo.custom_robots.clone());
        self.license_url.clone().set(config.seo.license_url.clone());
        self.author_name.clone().set(config.seo.author_name.clone());

        let menu_pairs = [
            (self.menu_1_label, self.menu_1_url),
            (self.menu_2_label, self.menu_2_url),
            (self.menu_3_label, self.menu_3_url),
            (self.menu_4_label, self.menu_4_url),
        ];

        for (i, (mut label_sig, mut url_sig)) in menu_pairs.into_iter().enumerate() {
            let (label, url) = config
                .menu_links
                .get(i)
                .map(|menu| (menu.label.clone(), menu.url.clone()))
                .unwrap_or_default();

            label_sig.set(label);
            url_sig.set(url);
        }

        self.footer_text
            .clone()
            .set(config.footer.footer_text.clone());
        self.footer_license_label
            .clone()
            .set(config.footer.footer_license_label.clone());
        self.footer_license_url
            .clone()
            .set(config.footer.footer_license_url.clone());

        self.custom_js.clone().set(config.plugins.custom_js.clone());
        self.static_pages.clone().set(config.static_pages.clone());
        self.ads.clone().set(config.ads.clone());
    }

    pub fn swap_palette(&self, palette: &PresetPalette) {
        self.bg_base.clone().set(palette.colors.bg_base.clone());
        self.bg_panel.clone().set(palette.colors.bg_panel.clone());
        self.bg_elevated
            .clone()
            .set(palette.colors.bg_elevated.clone());
        self.fg_base.clone().set(palette.colors.fg_base.clone());
        self.fg_muted.clone().set(palette.colors.fg_muted.clone());
        self.accent.clone().set(palette.colors.accent.clone());
        self.border.clone().set(palette.colors.border.clone());
        self.background.clone().set(palette.background.clone());
    }
}
