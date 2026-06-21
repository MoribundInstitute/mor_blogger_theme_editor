use dioxus::prelude::*;

use crate::app::state::layout_state::LayoutState;
use crate::app::state::theme_state::ThemeState;
use crate::app::vfs::VfsDictionary;
use mor_blogger_core::config::{MenuLink, ThemeConfig};
use mor_blogger_core::diagnostics::{check_integrity, DiagnosticResult};
use mor_blogger_core::render::template_resolver::{
    ComponentManifest, CONTENT_REGISTRY, FOOTER_REGISTRY, HEADER_REGISTRY, LAYOUT_REGISTRY,
    SIDEBAR_LEFT_REGISTRY, SIDEBAR_RIGHT_REGISTRY,
};
use mor_blogger_core::render::{render_preview_html, render_theme};

#[derive(Clone, Copy)]
pub struct RenderState {
    pub current_config: Memo<ThemeConfig>,
    pub generated_xml: Memo<String>,
    pub preview_html: Memo<String>,
    pub diag: Signal<DiagnosticResult>,
}

impl RenderState {
    pub fn new(theme: ThemeState, layout: LayoutState) -> Self {
        let signals = theme.signals;
        let active_preset = theme.active_preset;

        let current_config = use_memo(move || ThemeConfig {
            site: mor_blogger_core::config::SiteConfig {
                site_title: (signals.site_title)(),
                site_subtitle: (signals.site_subtitle)(),
                header_logo_url: (signals.header_logo_url)(),
                home_url: (signals.home_url)(),
            },
            colors: mor_blogger_core::config::ColorConfig {
                bg_base: (signals.bg_base)(),
                bg_panel: (signals.bg_panel)(),
                bg_elevated: (signals.bg_elevated)(),
                fg_base: (signals.fg_base)(),
                fg_muted: (signals.fg_muted)(),
                accent: (signals.accent)(),
                border: (signals.border)(),
                panel_border_width: (signals.panel_border_width)(),
                glow_spread: (signals.glow_spread)(),
                hover_scale: (signals.hover_scale)(),
                panel_border_image_url: (signals.panel_border_image_url)(),
                panel_border_image_slice: (signals.panel_border_image_slice)(),
                panel_border_image_repeat: (signals.panel_border_image_repeat)(),
                glow_color: (signals.glow_color)(),
                glow_logo: (signals.glow_logo)(),
                glow_title: (signals.glow_title)(),
                glow_toc: (signals.glow_toc)(),
                glow_sidebar: (signals.glow_sidebar)(),
                glow_logo_color: (signals.glow_logo_color)(),
                glow_title_color: (signals.glow_title_color)(),
                glow_toc_color: (signals.glow_toc_color)(),
                glow_sidebar_color: (signals.glow_sidebar_color)(),
                glow_text: (signals.glow_text)(),
                glow_containers: (signals.glow_containers)(),
                glow_icons: (signals.glow_icons)(),
                glow_text_color: (signals.glow_text_color)(),
                glow_containers_color: (signals.glow_containers_color)(),
                glow_icons_color: (signals.glow_icons_color)(),
                ..Default::default()
            },
            icons: (signals.icons)(),
            buttons: mor_blogger_core::config::ButtonConfig {
                radius: (signals.btn_radius)(),
                border_width: (signals.btn_border_width)(),
                text_transform: (signals.btn_text_transform)(),
            },
            typography: mor_blogger_core::config::TypographyConfig {
                body_font_stack: (signals.body_font_stack)(),
                heading_font_stack: (signals.heading_font_stack)(),
                mono_font_stack: (signals.mono_font_stack)(),
                base_size: (signals.base_size)(),
                scale_ratio: (signals.scale_ratio)(),
                line_height: (signals.line_height)(),
                heading_weight: (signals.heading_weight)(),
            },
            background: (signals.background)(),
            assets: mor_blogger_core::config::AssetConfig {
                favicon_url: (signals.favicon_url)(),
                social_card_image_url: (signals.social_card_image_url)(),
            },
            seo: mor_blogger_core::config::SeoConfig {
                meta_description: (signals.meta_description)(),
                meta_keywords: (signals.meta_keywords)(),
                custom_robots: (signals.custom_robots)(),
                license_url: (signals.license_url)(),
                author_name: (signals.author_name)(),
            },
            menu_links: vec![
                MenuLink {
                    label: (signals.menu_1_label)(),
                    url: (signals.menu_1_url)(),
                },
                MenuLink {
                    label: (signals.menu_2_label)(),
                    url: (signals.menu_2_url)(),
                },
                MenuLink {
                    label: (signals.menu_3_label)(),
                    url: (signals.menu_3_url)(),
                },
                MenuLink {
                    label: (signals.menu_4_label)(),
                    url: (signals.menu_4_url)(),
                },
            ],
            footer: mor_blogger_core::config::FooterConfig {
                footer_text: (signals.footer_text)(),
                footer_license_label: (signals.footer_license_label)(),
                footer_license_url: (signals.footer_license_url)(),
                ..Default::default()
            },
            plugins: mor_blogger_core::config::PluginConfig {
                custom_js: (signals.custom_js)(),
            },
            static_pages: (signals.static_pages)(),
            ads: (signals.ads)(),
            template_pack: (signals.template_pack)(),
            scripts: (signals.scripts)(),
            blocks: Vec::new(),
            preset_css: (signals.preset_css)(),
            active_preset_id: active_preset().map(|s| s.to_string()),
            enable_image_borders: (signals.enable_image_borders)(),
            custom_border_url: (signals.custom_border_url)(),
            svg_border_slice: (signals.svg_border_slice)(),
            image_border_width: (signals.image_border_width)(),
            target_sidebars: (signals.target_sidebars)(),
            target_canvas: (signals.target_canvas)(),
            cursor_style: signals.cursor_style.read().clone(),
            scrollbar_width: (signals.scrollbar_width)(),
            scrollbar_track_color: (signals.scrollbar_track_color)(),
            scrollbar_thumb_color: (signals.scrollbar_thumb_color)(),
            scrollbar_thumb_hover_color: (signals.scrollbar_thumb_hover_color)(),
        });

        let current_config_for_xml = current_config;
        let vfs = use_context::<VfsDictionary>().0;

        let generated_xml = use_memo(move || {
            let config = current_config_for_xml();
            let rendered_xml = render_theme(&config, &*vfs.read());
            match mor_blogger_core::utils::rehydration::inject_state(&rendered_xml, &config) {
                Ok(xml) => xml,
                Err(err) => {
                    log::error!("Failed to inject rehydration state: {}", err);
                    rendered_xml
                }
            }
        });

        let current_config_for_preview = current_config;
        let preview_template_mode = layout.preview_template_mode;
        let is_dark_mode = theme.signals.is_dark_mode;

        let preview_html = use_memo(move || {
            render_preview_html(
                &current_config_for_preview(),
                preview_template_mode(),
                is_dark_mode(),
                &*vfs.read(),
            )
        });

        let current_config_for_diag_init = current_config;
        let current_config_for_diag_effect = current_config;

        let generated_xml_for_diag_init = generated_xml;
        let generated_xml_for_diag_effect = generated_xml;

        let mut diag = use_signal(move || {
            let config = current_config_for_diag_init();
            check_integrity(&generated_xml_for_diag_init(), &config.template_pack)
        });

        use_effect(move || {
            let config = current_config_for_diag_effect();
            diag.set(check_integrity(
                &generated_xml_for_diag_effect(),
                &config.template_pack,
            ));
        });

        Self {
            current_config,
            generated_xml,
            preview_html,
            diag,
        }
    }

    pub fn get_manifest(&self, registry_type: &str, id: &str) -> Option<ComponentManifest> {
        let registry = match registry_type {
            "header" => HEADER_REGISTRY,
            "layout" => LAYOUT_REGISTRY,
            "content" => CONTENT_REGISTRY,
            "sidebar_left" => SIDEBAR_LEFT_REGISTRY,
            "sidebar_right" => SIDEBAR_RIGHT_REGISTRY,
            "footer" => FOOTER_REGISTRY,
            _ => return None,
        };
        registry.iter().find(|c| c.id == id).cloned()
    }
}
