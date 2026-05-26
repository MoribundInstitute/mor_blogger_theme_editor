use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_NEWSPAPER};

const PRESET_CSS: &str = include_str!("css/newspaper.css");

pub fn newspaper() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "The Daily Folio".to_string(),
            site_subtitle: "Established this morning · Vol. I, No. 1".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_NEWSPAPER.to_string(),
            // A blackletter heading stack with sensible serif fallbacks
            // for systems that don't have UnifrakturMaguntia or similar.
            heading_font_stack:
                "'UnifrakturMaguntia', 'Old English Text MT', 'Blackletter686 BT', \
                 'Times New Roman', Times, serif"
                    .to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "17px".to_string(),
            scale_ratio: "1.22".to_string(),
            line_height: "1.55".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "uppercase".to_string(),
        },
        SeoConfig {
            meta_description: "A longform blog published in the manner of a broadsheet newspaper."
                .to_string(),
            meta_keywords: "blog, longform, journalism, essays".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: "The Editors".to_string(),
        },
        vec![
            MenuLink {
                label: "Front Page".to_string(),
                url: "/".to_string(),
            },
            MenuLink {
                label: "Opinion".to_string(),
                url: "/search/label/opinion".to_string(),
            },
            MenuLink {
                label: "Archive".to_string(),
                url: "/p/archive.html".to_string(),
            },
            MenuLink {
                label: "Masthead".to_string(),
                url: "/p/about.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "© The Daily Folio. All rights reserved. Printed on the Internet."
                .to_string(),
            footer_license_label: "Terms".to_string(),
            footer_license_url: "/p/terms.html".to_string(),
        },
    );

    Preset {
        id: "newspaper",
        name: "Newspaper",
        description: "Broadsheet columns, drop caps, masthead. Black ink on cream paper.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            // "Microfiche" — the newspaper as seen on a library reader. Off-black
            // page, warm off-white ink, faded crimson for emphasis.
            colors: ColorConfig {
                bg_base: "#1a1814".to_string(),
                bg_panel: SurfaceFill::solid("#1f1d18"),
                bg_elevated: SurfaceFill::solid("#26241e"),
                fg_base: "#ebe4d3".to_string(),
                fg_muted: "#9a917f".to_string(),
                accent: "#c75148".to_string(),
                border: "#3a3730".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#1a1814".to_string(),
                },
            },
        },
        light: PresetPalette {
            // The signature broadsheet palette: cream paper, ink black,
            // hairline grey rules, one red for breaking-news flags.
            colors: ColorConfig {
                bg_base: "#f4ecd8".to_string(),
                bg_panel: SurfaceFill::solid("#fbf6e9"),
                bg_elevated: SurfaceFill::solid("#ffffff"),
                fg_base: "#161310".to_string(),
                fg_muted: "#5c554b".to_string(),
                accent: "#a01818".to_string(),
                border: "#c2b89e".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#f4ecd8".to_string(),
                },
            },
        },
    }
}
