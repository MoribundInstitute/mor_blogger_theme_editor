use dioxus::prelude::*;

use crate::app::layout_state::CenterView;
use crate::ui::panels::theme_palette::presets::{morph_preview_from_preset, ThemeSignals};
use mor_blogger_core::config::defaults::default_theme_config;
use mor_blogger_core::config::{ColorConfig, MenuLink, ThemeConfig};

use super::config_bridge::{menu_label, menu_url};

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeHistory {
    pub snapshots: Vec<Option<&'static str>>,
    pub cursor: usize,
}

#[derive(Clone, Copy)]
pub struct ThemeAppState {
    pub signals: ThemeSignals,
    pub current_config: Memo<ThemeConfig>,
    pub active_preset: Signal<Option<&'static str>>,
    pub center_view: Signal<CenterView>,
    pub show_undocked_presets: Signal<bool>,
    pub show_advanced_glow: Signal<bool>,
    pub history: Signal<ThemeHistory>,
}

pub fn use_theme_app_state() -> ThemeAppState {
    let mut defaults = default_theme_config();
    if let Some(pack) = crate::app::config_bridge::EditorPrefs::load().default_template_pack {
        defaults.template_pack = pack;
    }

    let site_title = use_signal(|| defaults.site.site_title.clone());
    let site_subtitle = use_signal(|| defaults.site.site_subtitle.clone());
    let header_logo_url = use_signal(|| defaults.site.header_logo_url.clone());
    let home_url = use_signal(|| defaults.site.home_url.clone());

    let bg_base = use_signal(|| defaults.colors.bg_base.clone());
    let bg_panel = use_signal(|| defaults.colors.bg_panel.clone());
    let bg_elevated = use_signal(|| defaults.colors.bg_elevated.clone());
    let fg_base = use_signal(|| defaults.colors.fg_base.clone());
    let fg_muted = use_signal(|| defaults.colors.fg_muted.clone());
    let accent = use_signal(|| defaults.colors.accent.clone());
    let border = use_signal(|| defaults.colors.border.clone());
    let panel_border_width = use_signal(|| defaults.colors.panel_border_width.clone());
    let glow_spread = use_signal(|| defaults.colors.glow_spread.clone());
    let hover_scale = use_signal(|| defaults.colors.hover_scale.clone());
    let panel_border_image_url = use_signal(|| defaults.colors.panel_border_image_url.clone());
    let panel_border_image_slice = use_signal(|| defaults.colors.panel_border_image_slice.clone());
    let panel_border_image_repeat =
        use_signal(|| defaults.colors.panel_border_image_repeat.clone());

    let btn_radius = use_signal(|| defaults.buttons.radius.clone());
    let btn_border_width = use_signal(|| defaults.buttons.border_width.clone());
    let btn_text_transform = use_signal(|| defaults.buttons.text_transform.clone());

    let body_font_stack = use_signal(|| defaults.typography.body_font_stack.clone());
    let heading_font_stack = use_signal(|| defaults.typography.heading_font_stack.clone());
    let mono_font_stack = use_signal(|| defaults.typography.mono_font_stack.clone());
    let base_size = use_signal(|| defaults.typography.base_size.clone());
    let scale_ratio = use_signal(|| defaults.typography.scale_ratio.clone());
    let line_height = use_signal(|| defaults.typography.line_height.clone());
    let heading_weight = use_signal(|| defaults.typography.heading_weight.clone());

    let background = use_signal(|| defaults.background.clone());
    let favicon_url = use_signal(|| defaults.assets.favicon_url.clone());
    let social_card_image_url = use_signal(|| defaults.assets.social_card_image_url.clone());

    let meta_description = use_signal(|| defaults.seo.meta_description.clone());
    let meta_keywords = use_signal(|| defaults.seo.meta_keywords.clone());
    let custom_robots = use_signal(|| defaults.seo.custom_robots.clone());
    let license_url = use_signal(|| defaults.seo.license_url.clone());
    let author_name = use_signal(|| defaults.seo.author_name.clone());

    let menu_1_label = use_signal(|| menu_label(&defaults, 0));
    let menu_1_url = use_signal(|| menu_url(&defaults, 0));
    let menu_2_label = use_signal(|| menu_label(&defaults, 1));
    let menu_2_url = use_signal(|| menu_url(&defaults, 1));
    let menu_3_label = use_signal(|| menu_label(&defaults, 2));
    let menu_3_url = use_signal(|| menu_url(&defaults, 2));
    let menu_4_label = use_signal(|| menu_label(&defaults, 3));
    let menu_4_url = use_signal(|| menu_url(&defaults, 3));

    let footer_text = use_signal(|| defaults.footer.footer_text.clone());
    let footer_license_label = use_signal(|| defaults.footer.footer_license_label.clone());
    let footer_license_url = use_signal(|| defaults.footer.footer_license_url.clone());

    let custom_js = use_signal(|| defaults.plugins.custom_js.clone());
    let template_pack = use_signal(|| defaults.template_pack.clone());
    let static_pages = use_signal(|| defaults.static_pages.clone());
    let ads = use_signal(|| defaults.ads.clone());
    let icons = use_signal(|| defaults.icons.clone());
    let preset_css = use_signal(String::new);
    let enable_image_borders = use_signal(|| defaults.enable_image_borders);
    let custom_border_url = use_signal(|| defaults.custom_border_url.clone());
    let svg_border_slice = use_signal(|| defaults.svg_border_slice.clone());
    let image_border_width = use_signal(|| defaults.image_border_width.clone());
    let target_sidebars = use_signal(|| defaults.target_sidebars);
    let target_canvas = use_signal(|| defaults.target_canvas);
    let glow_color = use_signal(|| defaults.colors.glow_color.clone());
    let glow_logo = use_signal(|| defaults.colors.glow_logo);
    let glow_title = use_signal(|| defaults.colors.glow_title);
    let glow_toc = use_signal(|| defaults.colors.glow_toc);
    let glow_sidebar = use_signal(|| defaults.colors.glow_sidebar);
    let glow_logo_color = use_signal(|| defaults.colors.glow_logo_color.clone());
    let glow_title_color = use_signal(|| defaults.colors.glow_title_color.clone());
    let glow_toc_color = use_signal(|| defaults.colors.glow_toc_color.clone());
    let glow_sidebar_color = use_signal(|| defaults.colors.glow_sidebar_color.clone());
    let glow_text = use_signal(|| defaults.colors.glow_text);
    let glow_containers = use_signal(|| defaults.colors.glow_containers);
    let glow_icons = use_signal(|| defaults.colors.glow_icons);
    let glow_text_color = use_signal(|| defaults.colors.glow_text_color.clone());
    let glow_containers_color = use_signal(|| defaults.colors.glow_containers_color.clone());
    let glow_icons_color = use_signal(|| defaults.colors.glow_icons_color.clone());
    let cursor_style = use_signal(|| defaults.cursor_style.clone());

    let center_view = use_signal(|| CenterView::Preview);
    let active_preset = use_signal(|| None::<&'static str>);
    let is_dark_mode = use_signal(|| true);
    let show_undocked_presets = use_signal(|| false);
    let show_advanced_glow = use_signal(|| false);
    let history = use_signal(|| ThemeHistory {
        snapshots: vec![None],
        cursor: 0,
    });

    let signals = ThemeSignals {
        is_dark_mode,
        site_title,
        site_subtitle,
        header_logo_url,
        home_url,
        bg_base,
        bg_panel,
        bg_elevated,
        fg_base,
        fg_muted,
        accent,
        border,
        panel_border_width,
        glow_spread,
        hover_scale,
        panel_border_image_url,
        panel_border_image_slice,
        panel_border_image_repeat,
        btn_radius,
        btn_border_width,
        btn_text_transform,
        body_font_stack,
        heading_font_stack,
        mono_font_stack,
        base_size,
        scale_ratio,
        line_height,
        heading_weight,
        background,
        favicon_url,
        social_card_image_url,
        meta_description,
        meta_keywords,
        custom_robots,
        license_url,
        author_name,
        menu_1_label,
        menu_1_url,
        menu_2_label,
        menu_2_url,
        menu_3_label,
        menu_3_url,
        menu_4_label,
        menu_4_url,
        footer_text,
        footer_license_label,
        footer_license_url,
        custom_js,
        template_pack,
        preset_css,
        static_pages,
        ads,
        icons,
        enable_image_borders,
        custom_border_url,
        svg_border_slice,
        image_border_width,
        target_sidebars,
        target_canvas,
        glow_color,
        glow_logo,
        glow_title,
        glow_toc,
        glow_sidebar,
        glow_logo_color,
        glow_title_color,
        glow_toc_color,
        glow_sidebar_color,
        glow_text,
        glow_containers,
        glow_icons,
        glow_text_color,
        glow_containers_color,
        glow_icons_color,
        cursor_style,
    };

    let current_config = use_memo(move || ThemeConfig {
        site: mor_blogger_core::config::SiteConfig {
            site_title: site_title(),
            site_subtitle: site_subtitle(),
            header_logo_url: header_logo_url(),
            home_url: home_url(),
        },
        colors: mor_blogger_core::config::ColorConfig {
            bg_base: bg_base(),
            bg_panel: bg_panel(),
            bg_elevated: bg_elevated(),
            fg_base: fg_base(),
            fg_muted: fg_muted(),
            accent: accent(),
            border: border(),
            panel_border_width: panel_border_width(),
            glow_spread: glow_spread(),
            hover_scale: hover_scale(),
            panel_border_image_url: panel_border_image_url(),
            panel_border_image_slice: panel_border_image_slice(),
            panel_border_image_repeat: panel_border_image_repeat(),
            glow_color: glow_color(),
            glow_logo: glow_logo(),
            glow_title: glow_title(),
            glow_toc: glow_toc(),
            glow_sidebar: glow_sidebar(),
            glow_logo_color: glow_logo_color(),
            glow_title_color: glow_title_color(),
            glow_toc_color: glow_toc_color(),
            glow_sidebar_color: glow_sidebar_color(),
            glow_text: glow_text(),
            glow_containers: glow_containers(),
            glow_icons: glow_icons(),
            glow_text_color: glow_text_color(),
            glow_containers_color: glow_containers_color(),
            glow_icons_color: glow_icons_color(),
            ..Default::default()
        },
        icons: icons(),
        buttons: mor_blogger_core::config::ButtonConfig {
            radius: btn_radius(),
            border_width: btn_border_width(),
            text_transform: btn_text_transform(),
        },
        typography: mor_blogger_core::config::TypographyConfig {
            body_font_stack: body_font_stack(),
            heading_font_stack: heading_font_stack(),
            mono_font_stack: mono_font_stack(),
            base_size: base_size(),
            scale_ratio: scale_ratio(),
            line_height: line_height(),
            heading_weight: heading_weight(),
        },
        background: background(),
        assets: mor_blogger_core::config::AssetConfig {
            favicon_url: favicon_url(),
            social_card_image_url: social_card_image_url(),
        },
        seo: mor_blogger_core::config::SeoConfig {
            meta_description: meta_description(),
            meta_keywords: meta_keywords(),
            custom_robots: custom_robots(),
            license_url: license_url(),
            author_name: author_name(),
        },
        menu_links: vec![
            MenuLink {
                label: menu_1_label(),
                url: menu_1_url(),
            },
            MenuLink {
                label: menu_2_label(),
                url: menu_2_url(),
            },
            MenuLink {
                label: menu_3_label(),
                url: menu_3_url(),
            },
            MenuLink {
                label: menu_4_label(),
                url: menu_4_url(),
            },
        ],
        footer: mor_blogger_core::config::FooterConfig {
            footer_text: footer_text(),
            footer_license_label: footer_license_label(),
            footer_license_url: footer_license_url(),
            ..Default::default()
        },
        plugins: mor_blogger_core::config::PluginConfig {
            custom_js: custom_js(),
        },
        static_pages: static_pages(),
        ads: ads(),
        template_pack: template_pack(),
        blocks: Vec::new(),
        preset_css: preset_css(),
        active_preset_id: active_preset().map(|s| s.to_string()),
        enable_image_borders: enable_image_borders(),
        custom_border_url: custom_border_url(),
        svg_border_slice: svg_border_slice(),
        image_border_width: image_border_width(),
        target_sidebars: target_sidebars(),
        target_canvas: target_canvas(),
        cursor_style: cursor_style(),
    });

    ThemeAppState {
        signals,
        current_config,
        active_preset,
        center_view,
        show_undocked_presets,
        show_advanced_glow,
        history,
    }
}

impl ThemeAppState {
    pub fn commit(&self) {
        let current = *self.active_preset.read();
        let mut history = self.history;
        let mut hist = history.write();
        if hist.snapshots.get(hist.cursor) == Some(&current) {
            return;
        }
        let cursor = hist.cursor;
        hist.snapshots.truncate(cursor + 1);
        hist.snapshots.push(current);
        if hist.snapshots.len() > 50 {
            hist.snapshots.remove(0);
        }
        hist.cursor = hist.snapshots.len() - 1;
    }

    pub fn undo(&self) {
        self._undo_internal();
    }

    pub fn redo(&self) {
        self._redo_internal();
    }

    fn _undo_internal(&self) {
        let mut history = self.history;
        let mut hist = history.write();
        if hist.cursor == 0 {
            return;
        }
        hist.cursor -= 1;
        let val = hist.snapshots[hist.cursor];
        let mut active_preset = self.active_preset;
        active_preset.set(val);
        self.restore_preset(val);
    }

    fn _redo_internal(&self) {
        let mut history = self.history;
        let mut hist = history.write();
        if hist.cursor + 1 >= hist.snapshots.len() {
            return;
        }
        hist.cursor += 1;
        let val = hist.snapshots[hist.cursor];
        let mut active_preset = self.active_preset;
        active_preset.set(val);
        self.restore_preset(val);
    }

    pub fn can_undo(&self) -> bool {
        self.history.read().cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        let hist = self.history.read();
        hist.cursor + 1 < hist.snapshots.len()
    }

    fn restore_preset(&self, val: Option<&'static str>) {
        let is_dark = *self.signals.is_dark_mode.read();
        if val.is_none() {
            let defaults = default_theme_config();
            self.signals.apply_config(&defaults);
            return;
        }
        let id = val.unwrap();
        let presets = mor_blogger_core::presets::all_presets();
        let preset = presets.iter().find(|p| p.id == id);
        if let Some(p) = preset {
            self.signals.apply_preset(p);
            morph_preview_from_preset(p, is_dark);
        }
    }

    /// Toggle is_dark_mode. If active preset has no explicit distinct [light.colors]/[dark.colors]
    /// (i.e. light and dark palettes are identical), generate the opposite via ColorConfig::inverted_contrast
    /// (bg/fg value swap, preserve accent+border). Otherwise use the preset's designed pal.
    pub fn perform_dark_mode_toggle(&self) {
        let mut signals = self.signals;
        let new_dark = !*signals.is_dark_mode.read();
        signals.is_dark_mode.set(new_dark);

        let active_id = *self.active_preset.read();

        let no_explicit = if let Some(id) = active_id {
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(preset) = presets.iter().find(|p| p.id == id) {
                let lc = &preset.light.colors;
                let dc = &preset.dark.colors;
                lc.bg_base == dc.bg_base && lc.fg_base == dc.fg_base
            } else {
                true
            }
        } else {
            true
        };

        if no_explicit {
            if active_id.is_none() {
                // Failure 1: Default theme inversion. Use explicit light/dark configs.
                let pal = if new_dark {
                    mor_blogger_core::presets::PresetPalette {
                        colors: mor_blogger_core::config::defaults::dark_color_config(),
                        background: mor_blogger_core::config::defaults::dark_background_config(),
                    }
                } else {
                    mor_blogger_core::presets::PresetPalette {
                        colors: mor_blogger_core::config::defaults::light_color_config(),
                        background: mor_blogger_core::config::defaults::light_background_config(),
                    }
                };
                signals.swap_palette(&pal);
            } else {
                // Build a ColorConfig snapshot from current signals (accent/border included so preserve works)
                let cur = ColorConfig {
                    bg_base: signals.bg_base.read().clone(),
                    bg_panel: signals.bg_panel.read().clone(),
                    bg_elevated: signals.bg_elevated.read().clone(),
                    fg_base: signals.fg_base.read().clone(),
                    fg_muted: signals.fg_muted.read().clone(),
                    accent: signals.accent.read().clone(),
                    border: signals.border.read().clone(),
                    panel_border_width: signals.panel_border_width.read().clone(),
                    glow_spread: signals.glow_spread.read().clone(),
                    hover_scale: signals.hover_scale.read().clone(),
                    panel_border_image_url: signals.panel_border_image_url.read().clone(),
                    panel_border_image_slice: signals.panel_border_image_slice.read().clone(),
                    panel_border_image_repeat: signals.panel_border_image_repeat.read().clone(),
                    glow_color: signals.glow_color.read().clone(),
                    glow_logo: *signals.glow_logo.read(),
                    glow_title: *signals.glow_title.read(),
                    glow_toc: *signals.glow_toc.read(),
                    glow_sidebar: *signals.glow_sidebar.read(),
                    glow_logo_color: signals.glow_logo_color.read().clone(),
                    glow_title_color: signals.glow_title_color.read().clone(),
                    glow_toc_color: signals.glow_toc_color.read().clone(),
                    glow_sidebar_color: signals.glow_sidebar_color.read().clone(),
                    glow_text: *signals.glow_text.read(),
                    glow_containers: *signals.glow_containers.read(),
                    glow_icons: *signals.glow_icons.read(),
                    glow_text_color: signals.glow_text_color.read().clone(),
                    glow_containers_color: signals.glow_containers_color.read().clone(),
                    glow_icons_color: signals.glow_icons_color.read().clone(),
                    ..Default::default()
                };
                let inv = cur.inverted_contrast();
                signals.bg_base.set(inv.bg_base);
                signals.bg_panel.set(inv.bg_panel);
                signals.bg_elevated.set(inv.bg_elevated);
                signals.fg_base.set(inv.fg_base);
                signals.fg_muted.set(inv.fg_muted);

                // Also invert background!
                let bg_cur = signals.background.read().clone();
                signals.background.set(bg_cur.inverted_contrast());
            }
        } else if let Some(id) = active_id {
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(preset) = presets.iter().find(|p| p.id == id) {
                let pal = if new_dark {
                    &preset.dark
                } else {
                    &preset.light
                };
                signals.swap_palette(pal);
            }
        }

        if let Some(id) = active_id {
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(preset) = presets.iter().find(|p| p.id == id) {
                morph_preview_from_preset(preset, new_dark);
            }
        }
    }
}
