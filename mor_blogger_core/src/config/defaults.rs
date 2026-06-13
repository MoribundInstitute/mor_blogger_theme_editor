use crate::config::ads::AdsConfig;
use crate::config::pages::StaticPagesConfig;
use crate::config::{
    AssetConfig,
    BackgroundConfig,
    ButtonConfig,
    ColorConfig,
    FooterConfig,
    PluginConfig,
    SeoConfig,
    SiteConfig,
    SurfaceFill,
    TemplatePackConfig,
    ThemeConfig,
    TypographyConfig,
};
use std::collections::HashMap;

pub fn default_theme_config() -> ThemeConfig {
    ThemeConfig {
        site: SiteConfig {
            site_title: "My Custom Theme".to_string(),
            site_subtitle: "A minimal workspace for ideas.".to_string(),
            header_logo_url: "".to_string(),
            home_url: "/".to_string(),
        },
        colors: ColorConfig {
            bg_base:     "#1a1a17".to_string(), 
            bg_panel:    SurfaceFill::solid("#242421"),
            bg_elevated: SurfaceFill::solid("#2e2e2a"),
            fg_base:     "#e6e4d5".to_string(), 
            fg_muted:    "#a3a08d".to_string(),
            accent:      "#c25e5e".to_string(), 
            border:      "#3d3d38".to_string(),
            panel_border_width: "1px".to_string(),
            glow_spread: "0px".to_string(),
            hover_scale: "1.0".to_string(),
            panel_border_image_url: String::new(),
            panel_border_image_slice: "30%".to_string(),
            panel_border_image_repeat: "stretch".to_string(),
        },
        typography: TypographyConfig {
            body_font_stack:    "Montserrat".to_string(), 
            heading_font_stack: "IM Fell English".to_string(), 
            mono_font_stack:    "Fira Code".to_string(),
            base_size:          "15px".to_string(),
            scale_ratio:        "1.25".to_string(),
            line_height:        "1.65".to_string(),
            heading_weight:     "400".to_string(), 
        },
        buttons: ButtonConfig {
            radius:         "2px".to_string(),
            border_width:   "1px".to_string(),
            text_transform: "uppercase".to_string(),
        },
        background:   BackgroundConfig::default(),
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
            footer_variant:        "MorFooterMega".to_string(),
            script_variant:        "mor_panels".to_string(),
            icon_pack:             "default".to_string(),
            widget_map: HashMap::from([
                ("sidebar-left".to_string(), vec!["Label1".to_string(), "BlogArchive1".to_string()]),
                ("sidebar-right".to_string(), vec!["HTML1".to_string()]),
            ]),
        },

        preset_css: "".to_string(),
        icons: crate::config::styling::IconConfig {
            sidebar_left: "url('data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20width=%2716%27%20height=%2716%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27currentColor%27%20stroke-width=%271.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Crect%20x=%271.5%27%20y=%272.5%27%20width=%2713%27%20height=%2711%27%20rx=%272%27%20/%3E%3Cpath%20d=%27M5.5%202.5v11%27%20/%3E%3C/svg%3E')".to_string(),
            sidebar_right: "url('data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20width=%2716%27%20height=%2716%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27currentColor%27%20stroke-width=%271.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Crect%20x=%271.5%27%20y=%272.5%27%20width=%2713%27%20height=%2711%27%20rx=%272%27%20/%3E%3Cpath%20d=%27M10.5%202.5v11%27%20/%3E%3C/svg%3E')".to_string(),
            panel_close: "url('data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20width=%2716%27%20height=%2716%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27currentColor%27%20stroke-width=%271.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M4.5%204.5l7%207M11.5%204.5l-7%207%27%20/%3E%3C/svg%3E')".to_string(),
            search: "url('data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20width=%2716%27%20height=%2716%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27currentColor%27%20stroke-width=%271.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Ccircle%20cx=%277.5%27%20cy=%277.5%27%20r=%275%27%20/%3E%3Cpath%20d=%27M11%2011l3.5%203.5%27%20/%3E%3C/svg%3E')".to_string(),
            menu: "url('data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20width=%2716%27%20height=%2716%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27currentColor%27%20stroke-width=%271.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M2.5%208h11M2.5%204h11M2.5%2012h11%27%20/%3E%3C/svg%3E')".to_string(),
            custom_icons: std::collections::HashMap::new(),
        },
    }
}