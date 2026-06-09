use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette};

const PRESET_CSS: &str = include_str!("css/mor_web_1_0_frames.css");

pub fn mor_web_1_0_frames() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "WELCOME TO MY HOMEPAGE!!".to_string(),
            site_subtitle: "Under Construction since 1998".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: "'Times New Roman', Times, serif".to_string(),
            heading_font_stack: "'Times New Roman', Times, serif".to_string(),
            mono_font_stack: "'Courier New', Courier, monospace".to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.15".to_string(),
            line_height: "1.45".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "0px".to_string(),
            border_width: "3px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: "My personal homepage on the World Wide Web.".to_string(),
            meta_keywords: "homepage, web, blog, personal, webring".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: "Mor Webmaster".to_string(),
        },
        vec![
            MenuLink {
                label: "Home".to_string(),
                url: "/".to_string(),
            },
            MenuLink {
                label: "About Me".to_string(),
                url: "/p/about.html".to_string(),
            },
            MenuLink {
                label: "Webring".to_string(),
                url: "/p/links.html".to_string(),
            },
            MenuLink {
                label: "Guestbook".to_string(),
                url: "#".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Best viewed in Netscape Navigator 4.0+ at 800x600".to_string(),
            footer_license_label: "Email Webmaster".to_string(),
            footer_license_url: "mailto:webmaster@example.com".to_string(),
            ..Default::default()
        },
    );

    Preset {
        id: "mor_web_1_0_frames",
        name: "Mor Web 1.0 Frames",
        description: "Tiled backgrounds, marquees, hit counters, and clashing colors. Like it's 1998.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#000000".to_string(), // Pure black
                bg_panel: SurfaceFill::solid("#000000"), // Pure black
                bg_elevated: SurfaceFill::solid("#111111"), // Slightly off-black
                fg_base: "#00ff00".to_string(), // Neon green text
                fg_muted: "#00cc00".to_string(), // Darker green
                accent: "#ff00ff".to_string(), // Clashing Magenta links
                border: "#00ff00".to_string(), // Green borders
                ..Default::default()
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Tile {
                    // A tiny base64 GIF of a classic 90s starfield
                    url: "data:image/gif;base64,R0lGODlhEAAQALMAAAAAAP///wAAAAAAACH5BAEAAAIALAAAAAAQABAAAAQzUMhJq7046yq0/2DAzkUWDGNmhmRnXGWqrigLwzB8x1Hk370P7Pw+Hw+I4w2PxyNyuWw2EwEAOw==".to_string(),
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#c0c0c0".to_string(), // Classic Silver
                bg_panel: SurfaceFill::solid("#c0c0c0"),
                bg_elevated: SurfaceFill::solid("#dfdfdf"),
                fg_base: "#000000".to_string(), // Pure black text
                fg_muted: "#404040".to_string(),
                accent: "#0000ee".to_string(), // Pure browser default blue
                border: "#808080".to_string(),
                ..Default::default()
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Tile {
                    // A tiny base64 GIF of classic 90s light marble texture
                    url: "data:image/gif;base64,R0lGODlhEAAQAKIAAMzMzP///8DAwAAAACH5BAEAAAAALAAAAAAQABAAAAIjhI+py+0PopxQzocA1rCz3nneF1okWZaomqrqygnFCgA7".to_string(),
                },
            },
        },
    }
}