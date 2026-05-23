use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink,
    SeoConfig, SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SERIF, STACK_SYSTEM_UI};

pub fn modern_editorial() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "Modern Editorial".to_string(),
            site_subtitle: "A clean Blogger starter for posts, notes, and pages.".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SERIF.to_string(),
            heading_font_stack: STACK_SYSTEM_UI.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "17px".to_string(),
            scale_ratio: "1.25".to_string(),
            line_height: "1.65".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: "A modern Blogger theme generated with Blogger Theme Architect."
                .to_string(),
            meta_keywords: "blog, writing, theme, editorial".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        },
        vec![
            MenuLink { label: "Home".to_string(), url: "/".to_string() },
            MenuLink { label: "Archive".to_string(), url: "/p/archive.html".to_string() },
            MenuLink { label: "About".to_string(), url: "/p/about.html".to_string() },
            MenuLink { label: "Contact".to_string(), url: "/p/contact.html".to_string() },
        ],
        FooterConfig {
            footer_text: "Published with Blogger.".to_string(),
            footer_license_label: "License".to_string(),
            footer_license_url: String::new(),
        },
    );

    Preset {
        id: "modern_editorial",
        name: "Modern Editorial",
        description: "Clean hero, cards, and editorial contrast. The default.",
        base_config: base,
        preset_css: "", // pure token swap, no extra rules
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#18161f".to_string(),
                bg_panel: SurfaceFill::solid("#211d2b"),
                bg_elevated: SurfaceFill::solid("#2a2435"),
                fg_base: "#f2eadf".to_string(),
                fg_muted: "#b9aebf".to_string(),
                accent: "#bc8d6b".to_string(),
                border: "#4a3f59".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid { color: "#18161f".to_string() },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#f5efe6".to_string(),
                bg_panel: SurfaceFill::solid("#fffaf2"),
                bg_elevated: SurfaceFill::solid("#ffffff"),
                fg_base: "#241f2d".to_string(),
                fg_muted: "#766b7f".to_string(),
                accent: "#8a5a3c".to_string(),
                border: "#d1c2b2".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid { color: "#f5efe6".to_string() },
            },
        },
    }
}
