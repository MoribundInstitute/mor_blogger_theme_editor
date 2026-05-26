use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SERIF};

const PRESET_CSS: &str = include_str!("css/retro_mmorpg.css");

pub fn retro_mmorpg() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "The Adventurer's Log".to_string(),
            site_subtitle: "Buying iron ore 100ea".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SERIF.to_string(),
            heading_font_stack: STACK_SERIF.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.2".to_string(),
            line_height: "1.5".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "2px".to_string(),
            border_width: "2px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: "A retro MMORPG inspired theme.".to_string(),
            meta_keywords: "retro, mmorpg, gaming, blog".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        },
        vec![
            MenuLink {
                label: "Bank".to_string(),
                url: "/".to_string(),
            },
            MenuLink {
                label: "Quests".to_string(),
                url: "/p/quests.html".to_string(),
            },
            MenuLink {
                label: "Highscores".to_string(),
                url: "/p/highscores.html".to_string(),
            },
            MenuLink {
                label: "Log Out".to_string(),
                url: "/feeds/posts/default".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "You have 0 unread messages.".to_string(),
            footer_license_label: "Copyright".to_string(),
            footer_license_url: "#".to_string(),
        },
    );

    Preset {
        id: "retro_mmorpg",
        name: "Retro MMORPG",
        description:
            "Dark UI panels, red interface buttons, and gold text with black drop shadows.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#000000".to_string(),
                bg_panel: SurfaceFill::solid("#413525"),
                bg_elevated: SurfaceFill::solid("#4e402d"),
                fg_base: "#c8c0a8".to_string(),
                fg_muted: "#908673".to_string(),
                accent: "#ff981f".to_string(),
                border: "#19140d".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#000000".to_string(),
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#1a1a1a".to_string(),
                bg_panel: SurfaceFill::solid("#544531"),
                bg_elevated: SurfaceFill::solid("#63523b"),
                fg_base: "#e3dbbf".to_string(),
                fg_muted: "#a89d87".to_string(),
                accent: "#ffff00".to_string(),
                border: "#201a11".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#1a1a1a".to_string(),
                },
            },
        },
    }
}
