use crate::config::ads::AdsConfig;
use crate::config::pages::StaticPagesConfig;
use crate::config::{
    AssetConfig, BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig,
    PluginConfig, SeoConfig, SiteConfig, SurfaceFill, TemplatePackConfig, ThemeConfig,
    TypographyConfig,
};
use std::collections::HashMap;

pub fn default_theme_config() -> ThemeConfig {
    ThemeConfig {
        site: SiteConfig {
            site_title: "Moribund Workspace".to_string(),
            site_subtitle: "A fluid, interactive environment.".to_string(),
            header_logo_url: "".to_string(),
            home_url: "/".to_string(),
        },
        colors: ColorConfig {
            bg_base:     "#0b0f19".to_string(), 
            bg_panel:    SurfaceFill::solid("#111827"),
            bg_elevated: SurfaceFill::solid("#1f2937"),
            fg_base:     "#f3f4f6".to_string(), 
            fg_muted:    "#9ca3af".to_string(),
            accent:      "#8b5cf6".to_string(), // Vivid Violet
            border:      "#374151".to_string(),
            panel_border_width: "1px".to_string(),
            glow_spread: "15px".to_string(),
            hover_scale: "1.015".to_string(),
            panel_border_image_url: String::new(),
            panel_border_image_slice: "30%".to_string(),
            panel_border_image_repeat: "stretch".to_string(),
        },
        typography: TypographyConfig {
            body_font_stack:    "Inter".to_string(), 
            heading_font_stack: "Inter".to_string(), 
            mono_font_stack:    "JetBrains Mono".to_string(),
            base_size:          "15px".to_string(),
            scale_ratio:        "1.25".to_string(),
            line_height:        "1.6".to_string(),
            heading_weight:     "700".to_string(), 
        },
        buttons: ButtonConfig {
            radius:         "8px".to_string(),
            border_width:   "1px".to_string(),
            text_transform: "none".to_string(),
        },
        background: BackgroundConfig {
            mode: BackgroundMode::Gradient {
                from: "#0f172a".to_string(),
                to: "#1e1b4b".to_string(),
                angle_deg: 135,
            }
        },
        assets:       AssetConfig::default(),
        seo:          SeoConfig::default(),
        menu_links:   vec![],
        footer:       FooterConfig::default(),
        plugins:      PluginConfig::default(),
        static_pages: StaticPagesConfig::default(),
        ads:          AdsConfig::default(),

        template_pack: TemplatePackConfig {
            header_variant:        "mor_header_baseline".to_string(),
            main_variant:          "sidebars".to_string(),
            content_variant:       "blog_standard".to_string(),
            left_sidebar_variant:  "blogger_left".to_string(),
            right_sidebar_variant: "toc_right".to_string(),
            footer_variant:        "mega".to_string(),
            script_variant:        "mor_panels".to_string(),
            icon_pack:             "default".to_string(),
            widget_map: HashMap::from([
                ("sidebar-left".to_string(), vec!["Label1".to_string(), "BlogArchive1".to_string()]),
                ("sidebar-right".to_string(), vec!["HTML1".to_string()]),
            ]),
        },

        preset_css: "".to_string(),
        // Drops the 15 lines of raw string repetition in favor of styling.rs defaults
        icons: crate::config::styling::IconConfig::default(), 
    }
}