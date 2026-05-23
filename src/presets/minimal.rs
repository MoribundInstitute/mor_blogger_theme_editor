use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink,
    SeoConfig, SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SYSTEM_UI};

pub fn minimal() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "notes".to_string(),
            site_subtitle: String::new(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SYSTEM_UI.to_string(),
            heading_font_stack: STACK_SYSTEM_UI.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.125".to_string(),
            line_height: "1.6".to_string(),
            heading_weight: "600".to_string(),
        },
        ButtonConfig {
            radius: "4px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: String::new(),
            meta_keywords: String::new(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        },
        vec![
            MenuLink { label: "Home".to_string(), url: "/".to_string() },
            MenuLink { label: "Archive".to_string(), url: "/p/archive.html".to_string() },
            MenuLink { label: "About".to_string(), url: "/p/about.html".to_string() },
            MenuLink { label: String::new(), url: String::new() },
        ],
        FooterConfig {
            footer_text: String::new(),
            footer_license_label: String::new(),
            footer_license_url: String::new(),
        },
    );

    Preset {
        id: "minimal",
        name: "Minimal",
        description: "System sans, tight scale, no decoration.",
        base_config: base,
        preset_css: "",
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#111111".to_string(),
                bg_panel: SurfaceFill::solid("#1a1a1a"),
                bg_elevated: SurfaceFill::solid("#222222"),
                fg_base: "#eeeeee".to_string(),
                fg_muted: "#999999".to_string(),
                accent: "#66aaff".to_string(),
                border: "#333333".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid { color: "#111111".to_string() },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#ffffff".to_string(),
                bg_panel: SurfaceFill::solid("#ffffff"),
                bg_elevated: SurfaceFill::solid("#fafafa"),
                fg_base: "#111111".to_string(),
                fg_muted: "#666666".to_string(),
                accent: "#0066cc".to_string(),
                border: "#e5e5e5".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid { color: "#ffffff".to_string() },
            },
        },
    }
}
