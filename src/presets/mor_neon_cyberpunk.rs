use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO};

const PRESET_CSS: &str = include_str!("css/mor_neon_cyberpunk.css");

pub fn mor_neon_cyberpunk() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "NEON.LOG".to_string(),
            site_subtitle: ">> uplink established <<".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_MONO.to_string(),
            heading_font_stack: STACK_MONO.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "15px".to_string(),
            scale_ratio: "1.2".to_string(),
            line_height: "1.55".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "uppercase".to_string(),
        },
        SeoConfig {
            meta_description: "Transmissions from the perimeter.".to_string(),
            meta_keywords: "log, cyber, transmission".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: "operator".to_string(),
        },
        vec![
            MenuLink {
                label: "feed".to_string(),
                url: "/".to_string(),
            },
            MenuLink {
                label: "logs".to_string(),
                url: "/search/label/log".to_string(),
            },
            MenuLink {
                label: "tags".to_string(),
                url: "/p/tags.html".to_string(),
            },
            MenuLink {
                label: "ping".to_string(),
                url: "/p/contact.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "// signal integrity: nominal //".to_string(),
            footer_license_label: "wtfpl".to_string(),
            footer_license_url: "http://www.wtfpl.net/".to_string(),
        },
    );

    Preset {
        id: "mor_neon_cyberpunk",
        name: "Mor Neon Cyberpunk",
        description: "Pure black, magenta/cyan glow, scanlines. Loud on purpose.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#05010f".to_string(),
                bg_panel: SurfaceFill::solid("#0a0418"),
                bg_elevated: SurfaceFill::solid("#120828"),
                fg_base: "#00ffff".to_string(),
                fg_muted: "#6ee7ff".to_string(),
                accent: "#ff00ff".to_string(),
                border: "#ff00ff".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#05010f".to_string(),
                },
            },
        },
        // The "light" variant doesn't really work for cyberpunk — keep it
        // as a desaturated daylight version for accessibility.
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#1a1525".to_string(),
                bg_panel: SurfaceFill::solid("#23172e"),
                bg_elevated: SurfaceFill::solid("#2e1f3c"),
                fg_base: "#7ee0e0".to_string(),
                fg_muted: "#a89cb8".to_string(),
                accent: "#d957d9".to_string(),
                border: "#d957d9".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#1a1525".to_string(),
                },
            },
        },
    }
}