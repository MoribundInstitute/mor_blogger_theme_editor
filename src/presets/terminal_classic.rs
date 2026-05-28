use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO};

pub fn terminal_classic() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "My Blogger Site".to_string(),
            site_subtitle: "A high-contrast mor-style weblog".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_MONO.to_string(),
            heading_font_stack: STACK_MONO.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.2".to_string(),
            line_height: "1.55".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "lowercase".to_string(),
        },
        SeoConfig {
            meta_description: "A custom Blogger theme generated with Blogger Theme Architect."
                .to_string(),
            meta_keywords: "blog, writing, theme".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        },
        vec![
            MenuLink {
                label: "home".to_string(),
                url: "/".to_string(),
            },
            MenuLink {
                label: "archive".to_string(),
                url: "/p/archive.html".to_string(),
            },
            MenuLink {
                label: "about".to_string(),
                url: "/p/about.html".to_string(),
            },
            MenuLink {
                label: "contact".to_string(),
                url: "/p/contact.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Powered by Blogger.".to_string(),
            footer_license_label: String::new(),
            footer_license_url: String::new(),
        },
    );

    Preset {
        id: "terminal_classic",
        name: "Terminal Classic",
        description: "High-contrast monospace. The original sidebar-era look.",
        base_config: base,
        preset_css: "",
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#222129".to_string(),
                bg_panel: SurfaceFill::solid("#2b2933"),
                bg_elevated: SurfaceFill::solid("#343140"),
                fg_base: "#f2eadf".to_string(),
                fg_muted: "#bc8d6b".to_string(),
                accent: "#a9aae2".to_string(),
                border: "#6f6078".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#222129".to_string(),
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#f2eadf".to_string(),
                bg_panel: SurfaceFill::solid("#ffffff"),
                bg_elevated: SurfaceFill::solid("#faf6ef"),
                fg_base: "#222129".to_string(),
                fg_muted: "#8b6a4d".to_string(),
                accent: "#5b5db0".to_string(),
                border: "#c9bea7".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#f2eadf".to_string(),
                },
            },
        },
    }
}
