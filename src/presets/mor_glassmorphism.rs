use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SYSTEM_UI};

const PRESET_CSS: &str = include_str!("css/mor_glassmorphism.css");

pub fn mor_glassmorphism() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "Aurora".to_string(),
            site_subtitle: "Notes, light, and motion.".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SYSTEM_UI.to_string(),
            heading_font_stack: STACK_SYSTEM_UI.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.2".to_string(),
            line_height: "1.6".to_string(),
            heading_weight: "600".to_string(),
        },
        ButtonConfig {
            radius: "14px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: "A glass-paneled Blogger theme that catches the light.".to_string(),
            meta_keywords: "blog, design, glass, aurora".to_string(),
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
                label: "Writing".to_string(),
                url: "/p/writing.html".to_string(),
            },
            MenuLink {
                label: "Projects".to_string(),
                url: "/p/projects.html".to_string(),
            },
            MenuLink {
                label: "Contact".to_string(),
                url: "/p/contact.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Made with Blogger.".to_string(),
            footer_license_label: "CC BY".to_string(),
            footer_license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
        },
    );

    Preset {
        id: "mor_glassmorphism",
        name: "Mor Glassmorphism",
        description: "Frosted panels over an aurora gradient. backdrop-filter required.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#0b0d2b".to_string(),
                // Panels are deliberately mid-tone with alpha — the CSS file
                // overrides them with rgba() because alpha can't ride through
                // the SurfaceFill token cleanly. These values are just the
                // "fallback if preset_css is stripped" colors.
                bg_panel: SurfaceFill::solid("#1a1f4a"),
                bg_elevated: SurfaceFill::solid("#2a2f6a"),
                fg_base: "#f5f7ff".to_string(),
                fg_muted: "#a8aedd".to_string(),
                accent: "#8b5cf6".to_string(),
                border: "#3d4280".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Gradient {
                    from: "#1e1a4d".to_string(),
                    to: "#5b2c8a".to_string(),
                    angle_deg: 135,
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#e8efff".to_string(),
                bg_panel: SurfaceFill::solid("#ffffff"),
                bg_elevated: SurfaceFill::solid("#f4f6ff"),
                fg_base: "#1a1d3a".to_string(),
                fg_muted: "#6b7099".to_string(),
                accent: "#7c3aed".to_string(),
                border: "#c8d2f0".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Gradient {
                    from: "#fde8ff".to_string(),
                    to: "#a6e6ff".to_string(),
                    angle_deg: 135,
                },
            },
        },
    }
}
