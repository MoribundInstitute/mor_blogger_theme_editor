use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, TypographyConfig,
};

use super::{build_base, gradient, Preset, PresetPalette, STACK_SANS};

const PRESET_CSS: &str = include_str!("css/web_2_0_skeuo.css");

pub fn web_2_0_skeuo() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "Beta Blog 2.0".to_string(),
            site_subtitle: "Now with even more web!".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SANS.to_string(),
            heading_font_stack: STACK_SANS.to_string(),
            mono_font_stack: "'Courier New', Courier, monospace".to_string(),
            base_size: "15px".to_string(),
            scale_ratio: "1.22".to_string(),
            line_height: "1.55".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "12px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: "Welcome to the new web. Faster. Glossier. More social.".to_string(),
            meta_keywords: "blog, web2, mashup, ajax, social".to_string(),
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
                label: "Subscribe".to_string(),
                url: "/feeds/posts/default".to_string(),
            },
            MenuLink {
                label: "Tags".to_string(),
                url: "/p/tags.html".to_string(),
            },
            MenuLink {
                label: "About".to_string(),
                url: "/p/about.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Beta Blog 2.0 — Powered by the Web.".to_string(),
            footer_license_label: "RSS".to_string(),
            footer_license_url: "/feeds/posts/default".to_string(),
        },
    );

    Preset {
        id: "web_2_0_skeuo",
        name: "Web 2.0 Skeuomorphic",
        description: "Glossy gradients, beveled buttons, soft shadows. Circa 2007.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            // "Vista era" — Aero glass meets dark mode. Cool blues, gloss intact.
            colors: ColorConfig {
                bg_base: "#1c2330".to_string(),
                bg_panel: gradient("#2a3548", "#1a2030", 180),
                bg_elevated: gradient("#384660", "#28324a", 180),
                fg_base: "#eaf2ff".to_string(),
                fg_muted: "#9bb0d0".to_string(),
                accent: "#4aa3ff".to_string(),
                border: "#3d4d6a".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Gradient {
                    from: "#2a3548".to_string(),
                    to: "#0e1320".to_string(),
                    angle_deg: 180,
                },
            },
        },
        light: PresetPalette {
            // The signature "iWeb"/"iLife" blue-sky palette: pale sky blue
            // body, white panels with light blue gradient bevels.
            colors: ColorConfig {
                bg_base: "#c8dcf0".to_string(),
                bg_panel: gradient("#ffffff", "#e4ecf5", 180),
                bg_elevated: gradient("#f8fbff", "#d8e4f0", 180),
                fg_base: "#1a2840".to_string(),
                fg_muted: "#5a6b80".to_string(),
                accent: "#1e6fd9".to_string(),
                border: "#a8bcd4".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Gradient {
                    from: "#dfeaf5".to_string(),
                    to: "#a8c4e0".to_string(),
                    angle_deg: 180,
                },
            },
        },
    }
}
