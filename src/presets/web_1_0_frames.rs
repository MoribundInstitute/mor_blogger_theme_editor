use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_WIN95};

const PRESET_CSS: &str = include_str!("css/web_1_0_frames.css");

pub fn web_1_0_frames() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "WELCOME TO MY HOMEPAGE!!".to_string(),
            site_subtitle: "Under Construction since 1998".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_WIN95.to_string(),
            heading_font_stack: STACK_WIN95.to_string(),
            mono_font_stack: "'Courier New', Courier, monospace".to_string(),
            base_size: "14px".to_string(),
            scale_ratio: "1.15".to_string(),
            line_height: "1.45".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "0px".to_string(),
            border_width: "2px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description: "My personal homepage on the World Wide Web.".to_string(),
            meta_keywords: "homepage, web, blog, personal".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: "Webmaster".to_string(),
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
                label: "Guestbook".to_string(),
                url: "/p/guestbook.html".to_string(),
            },
            MenuLink {
                label: "Links".to_string(),
                url: "/p/links.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Best viewed in Netscape Navigator 4.0+ at 800x600".to_string(),
            footer_license_label: "Email Me".to_string(),
            footer_license_url: "mailto:webmaster@example.com".to_string(),
        },
    );

    Preset {
        id: "web_1_0_frames",
        name: "Web 1.0 Frames",
        description: "Ridged borders, grey panels, MS Sans Serif. Like it's 1998.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            // Web 1.0 didn't really do dark mode — use a deep teal screensaver-y palette
            // as the "dark" variant for users who want low-light. Still chunky/ridged.
            colors: ColorConfig {
                bg_base: "#002b36".to_string(),
                bg_panel: SurfaceFill::solid("#073642"),
                bg_elevated: SurfaceFill::solid("#586e75"),
                fg_base: "#eee8d5".to_string(),
                fg_muted: "#93a1a1".to_string(),
                accent: "#b58900".to_string(),
                border: "#002b36".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#002b36".to_string(),
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                // The signature Windows 9x grey.
                bg_base: "#008080".to_string(), // Teal desktop background
                bg_panel: SurfaceFill::solid("#c0c0c0"),
                bg_elevated: SurfaceFill::solid("#dfdfdf"),
                fg_base: "#000000".to_string(),
                fg_muted: "#404040".to_string(),
                accent: "#0000ee".to_string(), // Unvisited link blue
                border: "#808080".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#008080".to_string(),
                },
            },
        },
    }
}
