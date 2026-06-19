use dioxus::prelude::*;

use mor_blogger_core::config::{
    AdsConfig, BackgroundConfig, IconConfig, SurfaceFill, TemplatePackConfig, ThemeConfig,
};
use mor_blogger_core::presets::{Preset, PresetPalette};

#[derive(Clone, Copy, PartialEq)]
pub struct ThemeSignals {
    pub is_dark_mode: Signal<bool>,

    pub site_title: Signal<String>,
    pub site_subtitle: Signal<String>,
    pub header_logo_url: Signal<String>,
    pub home_url: Signal<String>,

    pub bg_base: Signal<String>,
    pub bg_panel: Signal<SurfaceFill>,
    pub bg_elevated: Signal<SurfaceFill>,
    pub fg_base: Signal<String>,
    pub fg_muted: Signal<String>,
    pub accent: Signal<String>,
    pub border: Signal<String>,

    pub panel_border_width: Signal<String>,
    pub glow_spread: Signal<String>,
    pub hover_scale: Signal<String>,
    pub panel_border_image_url: Signal<String>,
    pub panel_border_image_slice: Signal<String>,
    pub panel_border_image_repeat: Signal<String>,

    pub btn_radius: Signal<String>,
    pub btn_border_width: Signal<String>,
    pub btn_text_transform: Signal<String>,

    pub body_font_stack: Signal<String>,
    pub heading_font_stack: Signal<String>,
    pub mono_font_stack: Signal<String>,
    pub base_size: Signal<String>,
    pub scale_ratio: Signal<String>,
    pub line_height: Signal<String>,
    pub heading_weight: Signal<String>,

    pub background: Signal<BackgroundConfig>,
    pub favicon_url: Signal<String>,
    pub social_card_image_url: Signal<String>,

    pub meta_description: Signal<String>,
    pub meta_keywords: Signal<String>,
    pub custom_robots: Signal<String>,
    pub license_url: Signal<String>,
    pub author_name: Signal<String>,

    pub menu_1_label: Signal<String>,
    pub menu_1_url: Signal<String>,
    pub menu_2_label: Signal<String>,
    pub menu_2_url: Signal<String>,
    pub menu_3_label: Signal<String>,
    pub menu_3_url: Signal<String>,
    pub menu_4_label: Signal<String>,
    pub menu_4_url: Signal<String>,

    pub footer_text: Signal<String>,
    pub footer_license_label: Signal<String>,
    pub footer_license_url: Signal<String>,

    pub custom_js: Signal<String>,
    pub preset_css: Signal<String>,
    pub static_pages: Signal<mor_blogger_core::config::StaticPagesConfig>,
    pub ads: Signal<AdsConfig>,
    pub icons: Signal<IconConfig>,
    pub template_pack: Signal<TemplatePackConfig>,
    pub enable_image_borders: Signal<bool>,
    pub custom_border_url: Signal<Option<String>>,
    pub svg_border_slice: Signal<String>,
    pub image_border_width: Signal<String>,
    pub target_sidebars: Signal<bool>,
    pub target_canvas: Signal<bool>,
    pub glow_color: Signal<String>,
    pub glow_logo: Signal<bool>,
    pub glow_title: Signal<bool>,
    pub glow_toc: Signal<bool>,
    pub glow_sidebar: Signal<bool>,
    pub glow_logo_color: Signal<String>,
    pub glow_title_color: Signal<String>,
    pub glow_toc_color: Signal<String>,
    pub glow_sidebar_color: Signal<String>,
    pub glow_text: Signal<bool>,
    pub glow_containers: Signal<bool>,
    pub glow_icons: Signal<bool>,
    pub glow_text_color: Signal<String>,
    pub glow_containers_color: Signal<String>,
    pub glow_icons_color: Signal<String>,
    pub cursor_style: Signal<String>,
    pub scrollbar_width: Signal<String>,
    pub scrollbar_track_color: Signal<String>,
    pub scrollbar_thumb_color: Signal<String>,
    pub scrollbar_thumb_hover_color: Signal<String>,
}

impl ThemeSignals {
    pub fn apply_preset(&self, preset: &Preset) {
        let is_dark = *self.is_dark_mode.read();
        let palette = if is_dark { &preset.dark } else { &preset.light };

        self.swap_palette(palette);
        self.panel_border_width
            .clone()
            .set(palette.colors.panel_border_width.clone());
        self.glow_spread
            .clone()
            .set(palette.colors.glow_spread.clone());
        self.hover_scale
            .clone()
            .set(palette.colors.hover_scale.clone());
        self.panel_border_image_url
            .clone()
            .set(palette.colors.panel_border_image_url.clone());
        self.panel_border_image_slice
            .clone()
            .set(palette.colors.panel_border_image_slice.clone());
        self.panel_border_image_repeat
            .clone()
            .set(palette.colors.panel_border_image_repeat.clone());
        self.glow_color
            .clone()
            .set(palette.colors.glow_color.clone());
        self.glow_logo_color
            .clone()
            .set(palette.colors.glow_logo_color.clone());
        self.glow_title_color
            .clone()
            .set(palette.colors.glow_title_color.clone());
        self.glow_toc_color
            .clone()
            .set(palette.colors.glow_toc_color.clone());
        self.glow_sidebar_color
            .clone()
            .set(palette.colors.glow_sidebar_color.clone());
        self.glow_logo
            .clone()
            .set(palette.colors.glow_logo);
        self.glow_title
            .clone()
            .set(palette.colors.glow_title);
        self.glow_toc
            .clone()
            .set(palette.colors.glow_toc);
        self.glow_sidebar
            .clone()
            .set(palette.colors.glow_sidebar);
        self.glow_text
            .clone()
            .set(palette.colors.glow_text);
        self.glow_containers
            .clone()
            .set(palette.colors.glow_containers);
        self.glow_icons
            .clone()
            .set(palette.colors.glow_icons);
        self.glow_text_color
            .clone()
            .set(palette.colors.glow_text_color.clone());
        self.glow_containers_color
            .clone()
            .set(palette.colors.glow_containers_color.clone());
        self.glow_icons_color
            .clone()
            .set(palette.colors.glow_icons_color.clone());

        let base = &preset.base_config;
        self.icons.clone().set(base.icons.clone());
        self.apply_preset_css(preset);

        self.btn_radius.clone().set(base.buttons.radius.clone());
        self.btn_border_width
            .clone()
            .set(base.buttons.border_width.clone());
        self.btn_text_transform
            .clone()
            .set(base.buttons.text_transform.clone());

        self.body_font_stack
            .clone()
            .set(base.typography.body_font_stack.clone());
        self.heading_font_stack
            .clone()
            .set(base.typography.heading_font_stack.clone());
        self.mono_font_stack
            .clone()
            .set(base.typography.mono_font_stack.clone());
        self.base_size
            .clone()
            .set(base.typography.base_size.clone());
        self.scale_ratio
            .clone()
            .set(base.typography.scale_ratio.clone());
        self.line_height
            .clone()
            .set(base.typography.line_height.clone());
        self.heading_weight
            .clone()
            .set(base.typography.heading_weight.clone());
        self.scrollbar_width
            .clone()
            .set(base.scrollbar_width.clone());
        self.scrollbar_track_color
            .clone()
            .set(base.scrollbar_track_color.clone());
        self.scrollbar_thumb_color
            .clone()
            .set(base.scrollbar_thumb_color.clone());
        self.scrollbar_thumb_hover_color
            .clone()
            .set(base.scrollbar_thumb_hover_color.clone());
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

        self.panel_border_width
            .clone()
            .set(config.colors.panel_border_width.clone());
        self.glow_spread
            .clone()
            .set(config.colors.glow_spread.clone());
        self.hover_scale
            .clone()
            .set(config.colors.hover_scale.clone());
        self.panel_border_image_url
            .clone()
            .set(config.colors.panel_border_image_url.clone());
        self.panel_border_image_slice
            .clone()
            .set(config.colors.panel_border_image_slice.clone());
        self.panel_border_image_repeat
            .clone()
            .set(config.colors.panel_border_image_repeat.clone());

        self.background.clone().set(config.background.clone());
        self.icons.clone().set(config.icons.clone());

        self.template_pack.clone().set(config.template_pack.clone());
        self.enable_image_borders
            .clone()
            .set(config.enable_image_borders);
        self.custom_border_url
            .clone()
            .set(config.custom_border_url.clone());
        self.svg_border_slice
            .clone()
            .set(config.svg_border_slice.clone());
        self.image_border_width
            .clone()
            .set(config.image_border_width.clone());
        self.target_sidebars.clone().set(config.target_sidebars);
        self.target_canvas.clone().set(config.target_canvas);
        self.glow_color
            .clone()
            .set(config.colors.glow_color.clone());
        self.glow_logo_color
            .clone()
            .set(config.colors.glow_logo_color.clone());
        self.glow_title_color
            .clone()
            .set(config.colors.glow_title_color.clone());
        self.glow_toc_color
            .clone()
            .set(config.colors.glow_toc_color.clone());
        self.glow_sidebar_color
            .clone()
            .set(config.colors.glow_sidebar_color.clone());
        self.glow_logo
            .clone()
            .set(config.colors.glow_logo);
        self.glow_title
            .clone()
            .set(config.colors.glow_title);
        self.glow_toc
            .clone()
            .set(config.colors.glow_toc);
        self.glow_sidebar
            .clone()
            .set(config.colors.glow_sidebar);
        self.glow_text
            .clone()
            .set(config.colors.glow_text);
        self.glow_containers
            .clone()
            .set(config.colors.glow_containers);
        self.glow_icons
            .clone()
            .set(config.colors.glow_icons);
        self.glow_text_color
            .clone()
            .set(config.colors.glow_text_color.clone());
        self.glow_containers_color
            .clone()
            .set(config.colors.glow_containers_color.clone());
        self.glow_icons_color
            .clone()
            .set(config.colors.glow_icons_color.clone());
        self.scrollbar_width
            .clone()
            .set(config.scrollbar_width.clone());
        self.scrollbar_track_color
            .clone()
            .set(config.scrollbar_track_color.clone());
        self.scrollbar_thumb_color
            .clone()
            .set(config.scrollbar_thumb_color.clone());
        self.scrollbar_thumb_hover_color
            .clone()
            .set(config.scrollbar_thumb_hover_color.clone());
        self.apply_config_except_palette(config);
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
        self.enable_image_borders
            .clone()
            .set(config.enable_image_borders);
        self.custom_border_url
            .clone()
            .set(config.custom_border_url.clone());
        self.svg_border_slice
            .clone()
            .set(config.svg_border_slice.clone());
        self.image_border_width
            .clone()
            .set(config.image_border_width.clone());
        self.target_sidebars.clone().set(config.target_sidebars);
        self.target_canvas.clone().set(config.target_canvas);
        self.cursor_style.clone().set(config.cursor_style.clone());
        self.scrollbar_width
            .clone()
            .set(config.scrollbar_width.clone());
        self.scrollbar_track_color
            .clone()
            .set(config.scrollbar_track_color.clone());
        self.scrollbar_thumb_color
            .clone()
            .set(config.scrollbar_thumb_color.clone());
        self.scrollbar_thumb_hover_color
            .clone()
            .set(config.scrollbar_thumb_hover_color.clone());
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
        self.glow_color
            .clone()
            .set(palette.colors.glow_color.clone());
        self.glow_logo_color
            .clone()
            .set(palette.colors.glow_logo_color.clone());
        self.glow_title_color
            .clone()
            .set(palette.colors.glow_title_color.clone());
        self.glow_toc_color
            .clone()
            .set(palette.colors.glow_toc_color.clone());
        self.glow_sidebar_color
            .clone()
            .set(palette.colors.glow_sidebar_color.clone());
        self.glow_text
            .clone()
            .set(palette.colors.glow_text);
        self.glow_containers
            .clone()
            .set(palette.colors.glow_containers);
        self.glow_icons
            .clone()
            .set(palette.colors.glow_icons);
        self.glow_text_color
            .clone()
            .set(palette.colors.glow_text_color.clone());
        self.glow_containers_color
            .clone()
            .set(palette.colors.glow_containers_color.clone());
        self.glow_icons_color
            .clone()
            .set(palette.colors.glow_icons_color.clone());

        self.background.clone().set(palette.background.clone());
    }
}
