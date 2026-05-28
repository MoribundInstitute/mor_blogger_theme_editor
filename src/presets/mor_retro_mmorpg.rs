use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SERIF};

const PRESET_CSS: &str = include_str!("css/mor_retro_mmorpg.css");

pub fn mor_retro_mmorpg() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "The Adventurer's Log".to_string(),
            site_subtitle: "Quest board • guild notices • trading post".to_string(),
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
            meta_description:
                "A retro MMORPG-inspired theme with warm brown panels and ember-red controls."
                    .to_string(),
            meta_keywords: "retro, mmorpg, fantasy, guild, gaming, blog".to_string(),
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
            footer_text: "Guild notices updated. Adventure continues.".to_string(),
            footer_license_label: "Copyright".to_string(),
            footer_license_url: "#".to_string(),
        },
    );

    Preset {
        id: "mor_retro_mmorpg",
        name: "Mor Retro MMORPG",
        description:
            "Warm brown fantasy-game panels, ember-red buttons, and gold text with chunky shadows.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#24170f".to_string(),
                bg_panel: SurfaceFill::solid("#4f3b2a"),
                bg_elevated: SurfaceFill::solid("#5e4732"),
                fg_base: "#e7dcc0".to_string(),
                fg_muted: "#b8a88d".to_string(),
                accent: "#ffae42".to_string(),
                border: "#22160f".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Gradient {
                    from: "#362317".to_string(),
                    to: "#1b120d".to_string(),
                    angle_deg: 180,
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#3a281d".to_string(),
                bg_panel: SurfaceFill::solid("#6a5139"),
                bg_elevated: SurfaceFill::solid("#7a5e42"),
                fg_base: "#f1e6c8".to_string(),
                fg_muted: "#c9b99a".to_string(),
                accent: "#ffd15a".to_string(),
                border: "#2b1d14".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Gradient {
                    from: "#5a3d29".to_string(),
                    to: "#2a1b12".to_string(),
                    angle_deg: 180,
                },
            },
        },
    }
}
