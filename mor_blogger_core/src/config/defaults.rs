use crate::config::ads::AdsConfig;
use crate::config::pages::StaticPagesConfig;
use crate::config::{
    AssetConfig, BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig,
    PluginConfig, SeoConfig, SiteConfig, SurfaceFill, TemplatePackConfig, ThemeConfig,
    TypographyConfig,
};
use std::collections::HashMap;

pub fn light_color_config() -> ColorConfig {
    ColorConfig {
        bg_base: "#e8efff".to_string(),
        bg_panel: SurfaceFill::solid("#ffffff"),
        bg_elevated: SurfaceFill::solid("#f4f6ff"),
        fg_base: "#1a1d3a".to_string(),
        fg_muted: "#6b7099".to_string(),
        accent: "#7c3aed".to_string(),
        border: "#c8d2f0".to_string(),
        panel_border_width: "1px".to_string(),
        glow_spread: "10px".to_string(),
        hover_scale: "1.02".to_string(),
        panel_border_image_url: String::new(),
        panel_border_image_slice: "30%".to_string(),
        panel_border_image_repeat: "stretch".to_string(),
    }
}

pub fn dark_color_config() -> ColorConfig {
    let bg_fill = dark_background_config().mode.to_surface_fill();
    ColorConfig {
        bg_base: "#0f1026".to_string(),
        bg_panel: bg_fill.clone(),
        bg_elevated: bg_fill,
        fg_base: "#f5f7ff".to_string(),
        fg_muted: "#a8aedd".to_string(),
        accent: "#8b5cf6".to_string(),
        border: "#3d4280".to_string(),
        panel_border_width: "1px".to_string(),
        glow_spread: "10px".to_string(),
        hover_scale: "1.02".to_string(),
        panel_border_image_url: String::new(),
        panel_border_image_slice: "30%".to_string(),
        panel_border_image_repeat: "stretch".to_string(),
    }
}

pub fn light_background_config() -> BackgroundConfig {
    BackgroundConfig {
        mode: BackgroundMode::Gradient {
            from: "#e8efff".to_string(),
            to: "#cdd8f5".to_string(),
            angle_deg: 135,
        },
    }
}

pub fn dark_background_config() -> BackgroundConfig {
    BackgroundConfig {
        mode: BackgroundMode::Gradient {
            from: "#1e1a4d".to_string(),
            to: "#5b2c8a".to_string(),
            angle_deg: 135,
        },
    }
}

pub fn default_theme_config() -> ThemeConfig {
    ThemeConfig {
        site: SiteConfig {
            site_title: "Moribund Workspace".to_string(),
            site_subtitle: "A fluid, interactive environment.".to_string(),
            header_logo_url: "".to_string(),
            home_url: "/".to_string(),
        },
        colors: dark_color_config(),
        typography: TypographyConfig {
            body_font_stack:    "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(), 
            heading_font_stack: "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(), 
            mono_font_stack:    "'Courier New', Courier, monospace".to_string(),
            base_size:          "16px".to_string(),
            scale_ratio:        "1.2".to_string(),
            line_height:        "1.6".to_string(),
            heading_weight:     "600".to_string(), 
        },
        buttons: ButtonConfig {
            radius:         "14px".to_string(),
            border_width:   "1px".to_string(),
            text_transform: "none".to_string(),
        },
        background: dark_background_config(),
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
            widget_titles: TemplatePackConfig::default().widget_titles,
        },

        blocks: vec![],
        preset_css: "".to_string(),
        active_preset_id: None,
        // Drops the 15 lines of raw string repetition in favor of styling.rs defaults.
        // Explicit standard blog action icons are now first-class on IconConfig (with svg_mask defaults).
        icons: {
            let mut i = crate::config::styling::IconConfig::default();
            // ensure label/archive/toc still populate custom_icons for --icon-* widget vars (css_builder)
            i.custom_icons.insert("label".to_string(), i.label.clone());
            i.custom_icons.insert("archive".to_string(), i.archive.clone());
            i.custom_icons.insert("toc".to_string(), i.toc.clone());
            i
        }, 
    }
}