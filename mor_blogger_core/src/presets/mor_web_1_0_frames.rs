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
            body_font_stack: "\"Times New Roman\", Times, serif".to_string(),
            heading_font_stack: "\"Times New Roman\", Times, serif".to_string(),
            mono_font_stack: "\"Courier New\", Courier, monospace".to_string(),
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
            meta_keywords: "homepage, webring, guestbook".to_string(),
            custom_robots: "index, follow".to_string(),
            author_name: "Webmaster".to_string(),
            license_url: String::new(),
        },
        // NEW: Providing the missing empty Menu Links and default Footer
        vec![],
        FooterConfig::default(),
    );

    Preset {
        id: "mor_web_1_0_frames",
        name: "Web 1.0 (Frames)",
        // NEW: Swapped 'category' for 'description'
        description: "Classic 90s aesthetic. Teal desktop in light mode, hacker terminal in dark mode.",
        // NEW: Swapped 'base' for 'base_config'
        base_config: base,
        // NEW: Removed .to_string() to satisfy &str requirement
        preset_css: PRESET_CSS,
        
        // DARK MODE: Hacker / Matrix Terminal
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#000000".to_string(), 
                bg_panel: SurfaceFill::solid("#000000"), 
                bg_elevated: SurfaceFill::solid("#111111"), 
                fg_base: "#00ff00".to_string(), 
                fg_muted: "#008800".to_string(), 
                accent: "#ff00ff".to_string(), 
                border: "#00ff00".to_string(), 
                panel_border_width: "3px".to_string(),
                glow_spread: "0px".to_string(),
                hover_scale: "1.0".to_string(),
                ..Default::default()
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Tile {
                    url: "data:image/gif;base64,R0lGODlhEAAQALMAAAAAAP///wAAAAAAACH5BAEAAAIALAAAAAAQABAAAAQzUMhJq7046yq0/2DAzkUWDGNmhmRnXGWqrigLwzB8x1Hk370P7Pw+Hw+I4w2PxyNyuWw2EwEAOw==".to_string(),
                },
            },
        },
        
        // LIGHT MODE: Classic Windows 95 / Geocities Teal
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#008080".to_string(), // Classic Win95 Desktop Teal
                bg_panel: SurfaceFill::solid("#c0c0c0"), // Silver/Gray panels
                bg_elevated: SurfaceFill::solid("#dfdfdf"), // Light gray elevated
                fg_base: "#000000".to_string(), // Black text
                fg_muted: "#404040".to_string(), // Dark gray muted text
                accent: "#0000ee".to_string(), // Classic standard hyperlink blue
                border: "#dfdfdf".to_string(), // Creates the 3D ridge effect
                panel_border_width: "3px".to_string(),
                glow_spread: "0px".to_string(),
                hover_scale: "1.0".to_string(),
                ..Default::default()
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid { 
                    color: "#008080".to_string() 
                },
            },
        },
    }
}