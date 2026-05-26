use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SYSTEM_UI};

const PRESET_CSS: &str = include_str!("css/fluid_interactive.css");

pub fn fluid_interactive() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "Drift".to_string(),
            site_subtitle: "A blog that moves with you.".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SYSTEM_UI.to_string(),
            heading_font_stack: STACK_SYSTEM_UI.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.25".to_string(),
            line_height: "1.6".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "999px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description:
                "An interactive blog with scroll-driven animations and animated gradients."
                    .to_string(),
            meta_keywords: "blog, motion, scroll, modern, interactive".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        },
        vec![
            MenuLink {
                label: "Home".to_string(),
                url: "/".to_string(),
            },
            MenuLink {
                label: "Read".to_string(),
                url: "/p/read.html".to_string(),
            },
            MenuLink {
                label: "Watch".to_string(),
                url: "/p/watch.html".to_string(),
            },
            MenuLink {
                label: "Subscribe".to_string(),
                url: "/feeds/posts/default".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Made with Blogger. Animated with CSS.".to_string(),
            footer_license_label: "CC".to_string(),
            footer_license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
        },
    );

    Preset {
        id: "fluid_interactive",
        name: "Fluid Interactive",
        description: "Animated gradient mesh, scroll-driven reveals, focus-within rings.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#0a0e1a".to_string(),
                bg_panel: SurfaceFill::solid("#141a2b"),
                bg_elevated: SurfaceFill::solid("#1c2438"),
                fg_base: "#f0f4ff".to_string(),
                fg_muted: "#8c98b8".to_string(),
                accent: "#6ee7ff".to_string(),
                border: "#2a3654".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#0a0e1a".to_string(),
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#fafbff".to_string(),
                bg_panel: SurfaceFill::solid("#ffffff"),
                bg_elevated: SurfaceFill::solid("#f4f6fb".to_string()),
                fg_base: "#0e1424".to_string(),
                fg_muted: "#5b6688".to_string(),
                accent: "#0ea5b7".to_string(),
                border: "#dce2ee".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#fafbff".to_string(),
                },
            },
        },
    }
}
