use dioxus::prelude::*;

use mor_blogger_core::config::{
    AdsConfig, AssetConfig, BackgroundConfig, ButtonConfig, ColorConfig, ElementStyle, FooterConfig,
    IconConfig, MenuLink, PluginConfig, SeoConfig, SiteConfig, SurfaceFill, TemplatePackConfig,
    ThemeConfig, TypographyConfig,
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
    pub header_fill: Signal<SurfaceFill>,
    pub fg_base: Signal<String>,
    pub fg_muted: Signal<String>,
    pub accent: Signal<String>,
    pub border: Signal<String>,

    pub panel_border_width: Signal<String>,
    pub glow_spread: Signal<String>,
    pub hover_scale: Signal<String>,
    pub glow_intensity: Signal<String>,
    pub glow_hover: Signal<bool>,
    pub panel_border_image_url: Signal<String>,
    pub panel_border_image_slice: Signal<String>,
    pub panel_border_image_repeat: Signal<String>,

    pub buttons: Signal<ButtonConfig>,

    pub body_font_stack: Signal<String>,
    pub heading_font_stack: Signal<String>,
    pub mono_font_stack: Signal<String>,
    pub base_size: Signal<String>,
    pub scale_ratio: Signal<String>,
    pub line_height: Signal<String>,
    pub heading_weight: Signal<String>,
    pub type_elements: Signal<Vec<ElementStyle>>,

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
    pub scripts: Signal<mor_blogger_core::config::ScriptBehaviorConfig>,
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
    pub glow_footer: Signal<bool>,
    pub glow_header: Signal<bool>,
    pub glow_main: Signal<bool>,
    pub glow_footer_color: Signal<String>,
    pub glow_header_color: Signal<String>,
    pub glow_main_color: Signal<String>,
    pub cursor_style: Signal<String>,
    pub cursor_set: Signal<mor_blogger_core::config::CursorSetConfig>,
    pub scrollbar_width: Signal<String>,
    pub scrollbar_track_color: Signal<String>,
    pub scrollbar_thumb_color: Signal<String>,
    pub scrollbar_thumb_hover_color: Signal<String>,
}

impl ThemeSignals {
    pub fn from_config(config: &ThemeConfig) -> Self {
        Self {
            is_dark_mode: Signal::new(true),

            site_title: Signal::new(config.site.site_title.clone()),
            site_subtitle: Signal::new(config.site.site_subtitle.clone()),
            header_logo_url: Signal::new(config.site.header_logo_url.clone()),
            home_url: Signal::new(config.site.home_url.clone()),

            bg_base: Signal::new(config.colors.bg_base.clone()),
            bg_panel: Signal::new(config.colors.bg_panel.clone()),
            bg_elevated: Signal::new(config.colors.bg_elevated.clone()),
            header_fill: Signal::new(config.colors.header_fill.clone()),
            fg_base: Signal::new(config.colors.fg_base.clone()),
            fg_muted: Signal::new(config.colors.fg_muted.clone()),
            accent: Signal::new(config.colors.accent.clone()),
            border: Signal::new(config.colors.border.clone()),

            panel_border_width: Signal::new(config.colors.panel_border_width.clone()),
            glow_spread: Signal::new(config.colors.glow_spread.clone()),
            hover_scale: Signal::new(config.colors.hover_scale.clone()),
            glow_intensity: Signal::new(config.colors.glow_intensity.clone()),
            glow_hover: Signal::new(config.colors.glow_hover),
            panel_border_image_url: Signal::new(config.colors.panel_border_image_url.clone()),
            panel_border_image_slice: Signal::new(config.colors.panel_border_image_slice.clone()),
            panel_border_image_repeat: Signal::new(config.colors.panel_border_image_repeat.clone()),

            buttons: Signal::new(config.buttons.clone()),

            body_font_stack: Signal::new(config.typography.body_font_stack.clone()),
            heading_font_stack: Signal::new(config.typography.heading_font_stack.clone()),
            mono_font_stack: Signal::new(config.typography.mono_font_stack.clone()),
            base_size: Signal::new(config.typography.base_size.clone()),
            scale_ratio: Signal::new(config.typography.scale_ratio.clone()),
            line_height: Signal::new(config.typography.line_height.clone()),
            heading_weight: Signal::new(config.typography.heading_weight.clone()),
            type_elements: Signal::new(config.typography.elements.clone()),

            background: Signal::new(config.background.clone()),
            favicon_url: Signal::new(config.assets.favicon_url.clone()),
            social_card_image_url: Signal::new(config.assets.social_card_image_url.clone()),

            meta_description: Signal::new(config.seo.meta_description.clone()),
            meta_keywords: Signal::new(config.seo.meta_keywords.clone()),
            custom_robots: Signal::new(config.seo.custom_robots.clone()),
            license_url: Signal::new(config.seo.license_url.clone()),
            author_name: Signal::new(config.seo.author_name.clone()),

            menu_1_label: Signal::new(config.menu_links.get(0).map(|m| m.label.clone()).unwrap_or_default()),
            menu_1_url: Signal::new(config.menu_links.get(0).map(|m| m.url.clone()).unwrap_or_default()),
            menu_2_label: Signal::new(config.menu_links.get(1).map(|m| m.label.clone()).unwrap_or_default()),
            menu_2_url: Signal::new(config.menu_links.get(1).map(|m| m.url.clone()).unwrap_or_default()),
            menu_3_label: Signal::new(config.menu_links.get(2).map(|m| m.label.clone()).unwrap_or_default()),
            menu_3_url: Signal::new(config.menu_links.get(2).map(|m| m.url.clone()).unwrap_or_default()),
            menu_4_label: Signal::new(config.menu_links.get(3).map(|m| m.label.clone()).unwrap_or_default()),
            menu_4_url: Signal::new(config.menu_links.get(3).map(|m| m.url.clone()).unwrap_or_default()),

            footer_text: Signal::new(config.footer.footer_text.clone()),
            footer_license_label: Signal::new(config.footer.footer_license_label.clone()),
            footer_license_url: Signal::new(config.footer.footer_license_url.clone()),

            custom_js: Signal::new(config.plugins.custom_js.clone()),
            preset_css: Signal::new(config.preset_css.clone()),
            static_pages: Signal::new(config.static_pages.clone()),
            ads: Signal::new(config.ads.clone()),
            icons: Signal::new(config.icons.clone()),
            template_pack: Signal::new(config.template_pack.clone()),
            scripts: Signal::new(config.scripts.clone()),
            enable_image_borders: Signal::new(config.enable_image_borders),
            custom_border_url: Signal::new(config.custom_border_url.clone()),
            svg_border_slice: Signal::new(config.svg_border_slice.clone()),
            image_border_width: Signal::new(config.image_border_width.clone()),
            target_sidebars: Signal::new(config.target_sidebars),
            target_canvas: Signal::new(config.target_canvas),
            glow_color: Signal::new(config.colors.glow_color.clone()),
            glow_logo: Signal::new(config.colors.glow_logo),
            glow_title: Signal::new(config.colors.glow_title),
            glow_toc: Signal::new(config.colors.glow_toc),
            glow_sidebar: Signal::new(config.colors.glow_sidebar),
            glow_logo_color: Signal::new(config.colors.glow_logo_color.clone()),
            glow_title_color: Signal::new(config.colors.glow_title_color.clone()),
            glow_toc_color: Signal::new(config.colors.glow_toc_color.clone()),
            glow_sidebar_color: Signal::new(config.colors.glow_sidebar_color.clone()),
            glow_text: Signal::new(config.colors.glow_text),
            glow_containers: Signal::new(config.colors.glow_containers),
            glow_icons: Signal::new(config.colors.glow_icons),
            glow_text_color: Signal::new(config.colors.glow_text_color.clone()),
            glow_containers_color: Signal::new(config.colors.glow_containers_color.clone()),
            glow_icons_color: Signal::new(config.colors.glow_icons_color.clone()),
            glow_footer: Signal::new(config.colors.glow_footer),
            glow_header: Signal::new(config.colors.glow_header),
            glow_main: Signal::new(config.colors.glow_main),
            glow_footer_color: Signal::new(config.colors.glow_footer_color.clone()),
            glow_header_color: Signal::new(config.colors.glow_header_color.clone()),
            glow_main_color: Signal::new(config.colors.glow_main_color.clone()),
            cursor_style: Signal::new(config.cursor_style.clone()),
            cursor_set: Signal::new(config.cursor_set.clone()),
            scrollbar_width: Signal::new(config.scrollbar_width.clone()),
            scrollbar_track_color: Signal::new(config.scrollbar_track_color.clone()),
            scrollbar_thumb_color: Signal::new(config.scrollbar_thumb_color.clone()),
            scrollbar_thumb_hover_color: Signal::new(config.scrollbar_thumb_hover_color.clone()),
        }
    }

    pub fn to_config(&self) -> ThemeConfig {
        ThemeConfig {
            site: SiteConfig {
                site_title: self.site_title.read().clone(),
                site_subtitle: self.site_subtitle.read().clone(),
                header_logo_url: self.header_logo_url.read().clone(),
                home_url: self.home_url.read().clone(),
            },
            colors: ColorConfig {
                bg_base: self.bg_base.read().clone(),
                bg_panel: self.bg_panel.read().clone(),
                bg_elevated: self.bg_elevated.read().clone(),
                header_fill: self.header_fill.read().clone(),
                fg_base: self.fg_base.read().clone(),
                fg_muted: self.fg_muted.read().clone(),
                accent: self.accent.read().clone(),
                border: self.border.read().clone(),
                panel_border_width: self.panel_border_width.read().clone(),
                glow_spread: self.glow_spread.read().clone(),
                hover_scale: self.hover_scale.read().clone(),
                glow_intensity: self.glow_intensity.read().clone(),
                glow_hover: *self.glow_hover.read(),
                panel_border_image_url: self.panel_border_image_url.read().clone(),
                panel_border_image_slice: self.panel_border_image_slice.read().clone(),
                panel_border_image_repeat: self.panel_border_image_repeat.read().clone(),
                glow_color: self.glow_color.read().clone(),
                glow_logo: *self.glow_logo.read(),
                glow_title: *self.glow_title.read(),
                glow_toc: *self.glow_toc.read(),
                glow_sidebar: *self.glow_sidebar.read(),
                glow_logo_color: self.glow_logo_color.read().clone(),
                glow_title_color: self.glow_title_color.read().clone(),
                glow_toc_color: self.glow_toc_color.read().clone(),
                glow_sidebar_color: self.glow_sidebar_color.read().clone(),
                glow_text: *self.glow_text.read(),
                glow_containers: *self.glow_containers.read(),
                glow_icons: *self.glow_icons.read(),
                glow_text_color: self.glow_text_color.read().clone(),
                glow_containers_color: self.glow_containers_color.read().clone(),
                glow_icons_color: self.glow_icons_color.read().clone(),
                glow_footer: *self.glow_footer.read(),
                glow_header: *self.glow_header.read(),
                glow_main: *self.glow_main.read(),
                glow_footer_color: self.glow_footer_color.read().clone(),
                glow_header_color: self.glow_header_color.read().clone(),
                glow_main_color: self.glow_main_color.read().clone(),
                ..Default::default()
            },
            icons: self.icons.read().clone(),
            buttons: self.buttons.read().clone(),
            typography: TypographyConfig {
                body_font_stack: self.body_font_stack.read().clone(),
                heading_font_stack: self.heading_font_stack.read().clone(),
                mono_font_stack: self.mono_font_stack.read().clone(),
                base_size: self.base_size.read().clone(),
                scale_ratio: self.scale_ratio.read().clone(),
                line_height: self.line_height.read().clone(),
                heading_weight: self.heading_weight.read().clone(),
                elements: self.type_elements.read().clone(),
            },
            background: self.background.read().clone(),
            assets: AssetConfig {
                favicon_url: self.favicon_url.read().clone(),
                social_card_image_url: self.social_card_image_url.read().clone(),
            },
            seo: SeoConfig {
                meta_description: self.meta_description.read().clone(),
                meta_keywords: self.meta_keywords.read().clone(),
                custom_robots: self.custom_robots.read().clone(),
                license_url: self.license_url.read().clone(),
                author_name: self.author_name.read().clone(),
            },
            menu_links: vec![
                MenuLink {
                    label: self.menu_1_label.read().clone(),
                    url: self.menu_1_url.read().clone(),
                },
                MenuLink {
                    label: self.menu_2_label.read().clone(),
                    url: self.menu_2_url.read().clone(),
                },
                MenuLink {
                    label: self.menu_3_label.read().clone(),
                    url: self.menu_3_url.read().clone(),
                },
                MenuLink {
                    label: self.menu_4_label.read().clone(),
                    url: self.menu_4_url.read().clone(),
                },
            ],
            footer: FooterConfig {
                footer_text: self.footer_text.read().clone(),
                footer_license_label: self.footer_license_label.read().clone(),
                footer_license_url: self.footer_license_url.read().clone(),
                ..Default::default()
            },
            plugins: PluginConfig {
                custom_js: self.custom_js.read().clone(),
            },
            static_pages: self.static_pages.read().clone(),
            ads: self.ads.read().clone(),
            template_pack: self.template_pack.read().clone(),
            scripts: self.scripts.read().clone(),
            blocks: Vec::new(),
            preset_css: self.preset_css.read().clone(),
            active_preset_id: None,
            active_variant_id: None,
            enable_image_borders: *self.enable_image_borders.read(),
            custom_border_url: self.custom_border_url.read().clone(),
            svg_border_slice: self.svg_border_slice.read().clone(),
            image_border_width: self.image_border_width.read().clone(),
            target_sidebars: *self.target_sidebars.read(),
            target_canvas: *self.target_canvas.read(),
            cursor_style: self.cursor_style.read().clone(),
            cursor_set: self.cursor_set.read().clone(),
            scrollbar_width: self.scrollbar_width.read().clone(),
            scrollbar_track_color: self.scrollbar_track_color.read().clone(),
            scrollbar_thumb_color: self.scrollbar_thumb_color.read().clone(),
            scrollbar_thumb_hover_color: self.scrollbar_thumb_hover_color.read().clone(),
        }
    }

    /// Apply a saved/imported config to every signal.
    pub fn apply_config(&self, config: &ThemeConfig) {
        self.set_colors(&config.colors);
        self.set_buttons(&config.buttons);
        self.set_typography(&config.typography);
        self.set_scrollbars(config);
        self.set_site(&config.site);
        self.set_seo(&config.seo);
        self.set_menu(&config.menu_links);
        self.set_footer(&config.footer);

        self.background.clone().set(config.background.clone());
        self.icons.clone().set(config.icons.clone());
        self.template_pack.clone().set(config.template_pack.clone());
        self.favicon_url.clone().set(config.assets.favicon_url.clone());
        self.social_card_image_url
            .clone()
            .set(config.assets.social_card_image_url.clone());
        self.custom_js.clone().set(config.plugins.custom_js.clone());
        self.preset_css.clone().set(config.preset_css.clone());
        self.static_pages.clone().set(config.static_pages.clone());
        self.ads.clone().set(config.ads.clone());
        self.scripts.clone().set(config.scripts.clone());
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
        self.cursor_set.clone().set(config.cursor_set.clone());
    }

    /// Apply a preset: visual theme only (colors, typography, buttons,
    /// scrollbars, icons) — leaves site content (title, menu, SEO…) untouched.
    pub fn apply_preset(&self, preset: &Preset) {
        let is_dark = *self.is_dark_mode.read();
        let palette = if is_dark { &preset.dark } else { &preset.light };

        self.set_colors(&palette.colors);
        self.background.clone().set(palette.background.clone());

        let base = &preset.base_config;
        self.set_buttons(&base.buttons);
        self.set_typography(&base.typography);
        self.set_scrollbars(base);
        self.icons.clone().set(base.icons.clone());
        self.apply_preset_css(preset);
    }

    pub fn apply_preset_css(&self, preset: &Preset) {
        self.preset_css.clone().set(preset.preset_css.to_string());
    }

    /// Dark/light toggle: swaps only the palette-varying colors plus the
    /// background, deliberately leaving structural fields (border widths,
    /// glow toggles, hover scale) in place. Narrower than [`set_colors`].
    pub fn swap_palette(&self, palette: &PresetPalette) {
        self.bg_base.clone().set(palette.colors.bg_base.clone());
        self.bg_panel.clone().set(palette.colors.bg_panel.clone());
        self.bg_elevated
            .clone()
            .set(palette.colors.bg_elevated.clone());
        self.header_fill
            .clone()
            .set(palette.colors.header_fill.clone());
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
        self.glow_text.clone().set(palette.colors.glow_text);
        self.glow_containers
            .clone()
            .set(palette.colors.glow_containers);
        self.glow_icons.clone().set(palette.colors.glow_icons);
        self.glow_text_color
            .clone()
            .set(palette.colors.glow_text_color.clone());
        self.glow_containers_color
            .clone()
            .set(palette.colors.glow_containers_color.clone());
        self.glow_icons_color
            .clone()
            .set(palette.colors.glow_icons_color.clone());
        self.glow_footer.clone().set(palette.colors.glow_footer);
        self.glow_header.clone().set(palette.colors.glow_header);
        self.glow_main.clone().set(palette.colors.glow_main);
        self.glow_footer_color
            .clone()
            .set(palette.colors.glow_footer_color.clone());
        self.glow_header_color
            .clone()
            .set(palette.colors.glow_header_color.clone());
        self.glow_main_color
            .clone()
            .set(palette.colors.glow_main_color.clone());

        self.background.clone().set(palette.background.clone());
    }

    // --- private domain setters -------------------------------------------
    // Each signal is assigned in exactly one of these. apply_config and
    // apply_preset compose them instead of re-listing fields, so adding a
    // theme field means touching one setter, not three methods.

    fn set_colors(&self, c: &ColorConfig) {
        self.bg_base.clone().set(c.bg_base.clone());
        self.bg_panel.clone().set(c.bg_panel.clone());
        self.bg_elevated.clone().set(c.bg_elevated.clone());
        self.header_fill.clone().set(c.header_fill.clone());
        self.fg_base.clone().set(c.fg_base.clone());
        self.fg_muted.clone().set(c.fg_muted.clone());
        self.accent.clone().set(c.accent.clone());
        self.border.clone().set(c.border.clone());
        self.panel_border_width
            .clone()
            .set(c.panel_border_width.clone());
        self.glow_spread.clone().set(c.glow_spread.clone());
        self.hover_scale.clone().set(c.hover_scale.clone());
        self.glow_intensity.clone().set(c.glow_intensity.clone());
        self.glow_hover.clone().set(c.glow_hover);
        self.panel_border_image_url
            .clone()
            .set(c.panel_border_image_url.clone());
        self.panel_border_image_slice
            .clone()
            .set(c.panel_border_image_slice.clone());
        self.panel_border_image_repeat
            .clone()
            .set(c.panel_border_image_repeat.clone());
        self.glow_color.clone().set(c.glow_color.clone());
        self.glow_logo.clone().set(c.glow_logo);
        self.glow_title.clone().set(c.glow_title);
        self.glow_toc.clone().set(c.glow_toc);
        self.glow_sidebar.clone().set(c.glow_sidebar);
        self.glow_logo_color.clone().set(c.glow_logo_color.clone());
        self.glow_title_color
            .clone()
            .set(c.glow_title_color.clone());
        self.glow_toc_color.clone().set(c.glow_toc_color.clone());
        self.glow_sidebar_color
            .clone()
            .set(c.glow_sidebar_color.clone());
        self.glow_text.clone().set(c.glow_text);
        self.glow_containers.clone().set(c.glow_containers);
        self.glow_icons.clone().set(c.glow_icons);
        self.glow_text_color.clone().set(c.glow_text_color.clone());
        self.glow_containers_color
            .clone()
            .set(c.glow_containers_color.clone());
        self.glow_icons_color
            .clone()
            .set(c.glow_icons_color.clone());
        self.glow_footer.clone().set(c.glow_footer);
        self.glow_header.clone().set(c.glow_header);
        self.glow_main.clone().set(c.glow_main);
        self.glow_footer_color
            .clone()
            .set(c.glow_footer_color.clone());
        self.glow_header_color
            .clone()
            .set(c.glow_header_color.clone());
        self.glow_main_color.clone().set(c.glow_main_color.clone());
    }

    fn set_buttons(&self, b: &ButtonConfig) {
        self.buttons.clone().set(b.clone());
    }

    fn set_typography(&self, t: &TypographyConfig) {
        self.body_font_stack.clone().set(t.body_font_stack.clone());
        self.heading_font_stack
            .clone()
            .set(t.heading_font_stack.clone());
        self.mono_font_stack.clone().set(t.mono_font_stack.clone());
        self.base_size.clone().set(t.base_size.clone());
        self.scale_ratio.clone().set(t.scale_ratio.clone());
        self.line_height.clone().set(t.line_height.clone());
        self.heading_weight.clone().set(t.heading_weight.clone());
        self.type_elements.clone().set(t.elements.clone());
    }

    fn set_scrollbars(&self, c: &ThemeConfig) {
        self.scrollbar_width.clone().set(c.scrollbar_width.clone());
        self.scrollbar_track_color
            .clone()
            .set(c.scrollbar_track_color.clone());
        self.scrollbar_thumb_color
            .clone()
            .set(c.scrollbar_thumb_color.clone());
        self.scrollbar_thumb_hover_color
            .clone()
            .set(c.scrollbar_thumb_hover_color.clone());
    }

    fn set_site(&self, s: &SiteConfig) {
        self.site_title.clone().set(s.site_title.clone());
        self.site_subtitle.clone().set(s.site_subtitle.clone());
        self.header_logo_url.clone().set(s.header_logo_url.clone());
        self.home_url.clone().set(s.home_url.clone());
    }

    fn set_seo(&self, s: &SeoConfig) {
        self.meta_description.clone().set(s.meta_description.clone());
        self.meta_keywords.clone().set(s.meta_keywords.clone());
        self.custom_robots.clone().set(s.custom_robots.clone());
        self.license_url.clone().set(s.license_url.clone());
        self.author_name.clone().set(s.author_name.clone());
    }

    fn set_footer(&self, f: &FooterConfig) {
        self.footer_text.clone().set(f.footer_text.clone());
        self.footer_license_label
            .clone()
            .set(f.footer_license_label.clone());
        self.footer_license_url
            .clone()
            .set(f.footer_license_url.clone());
    }

    fn set_menu(&self, links: &[MenuLink]) {
        let menu_pairs = [
            (self.menu_1_label, self.menu_1_url),
            (self.menu_2_label, self.menu_2_url),
            (self.menu_3_label, self.menu_3_url),
            (self.menu_4_label, self.menu_4_url),
        ];

        for (i, (mut label_sig, mut url_sig)) in menu_pairs.into_iter().enumerate() {
            let (label, url) = links
                .get(i)
                .map(|menu| (menu.label.clone(), menu.url.clone()))
                .unwrap_or_default();

            label_sig.set(label);
            url_sig.set(url);
        }
    }
}
