use crate::config::{
    BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig, MenuLink, SeoConfig,
    SiteConfig, SurfaceFill, TypographyConfig,
};

use super::{build_base, Preset, PresetPalette, STACK_MONO, STACK_SYSTEM_UI};

const PRESET_CSS: &str = include_str!("css/mor_fluid_interactive.css");

pub fn mor_fluid_interactive() -> Preset {
    let base = build_base(
        SiteConfig {
            site_title: "Mor Fluid Ledger".to_string(),
            site_subtitle: "A living surface for notes, essays, dashboards, and project telemetry."
                .to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        TypographyConfig {
            body_font_stack: STACK_SYSTEM_UI.to_string(),
            heading_font_stack: STACK_SYSTEM_UI.to_string(),
            mono_font_stack: STACK_MONO.to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.22".to_string(),
            line_height: "1.65".to_string(),
            heading_weight: "700".to_string(),
        },
        ButtonConfig {
            radius: "8px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        SeoConfig {
            meta_description:
                "A fluid interactive Blogger theme preset with readable motion, luminous panels, and responsive dashboard-friendly surfaces."
                    .to_string(),
            meta_keywords: "blogger, theme, fluid, interactive, dashboard, writing".to_string(),
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
                label: "Archive".to_string(),
                url: "/p/archive.html".to_string(),
            },
            MenuLink {
                label: "Dashboard".to_string(),
                url: "/p/analytics-dashboard.html".to_string(),
            },
            MenuLink {
                label: "About".to_string(),
                url: "/p/about.html".to_string(),
            },
        ],
        FooterConfig {
            footer_text: "Rendered through the Mor Blogger Theme Editor.".to_string(),
            footer_license_label: "Source".to_string(),
            footer_license_url: "https://github.com/MoribundInstitute/mor_blogger_theme_editor"
                .to_string(),
            ..Default::default()
        },
    );

    Preset {
        id: "mor_fluid_interactive",
        name: "Mor Fluid Interactive",
        description:
            "Readable liquid-motion UI: luminous panels, gentle hover physics, and dashboard-friendly surfaces.",
        base_config: base,
        preset_css: PRESET_CSS,
        dark: PresetPalette {
            colors: ColorConfig {
                bg_base: "#080d12".to_string(),
                bg_panel: SurfaceFill::solid("#101923"),
                bg_elevated: SurfaceFill::solid("#172635"),
                fg_base: "#edf7f3".to_string(),
                fg_muted: "#9fb8b0".to_string(),
                accent: "#05a581".to_string(),
                border: "#27423d".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#080d12".to_string(),
                },
            },
        },
        light: PresetPalette {
            colors: ColorConfig {
                bg_base: "#ecf8f2".to_string(),
                bg_panel: SurfaceFill::solid("#ffffff"),
                bg_elevated: SurfaceFill::solid("#dff2eb"),
                fg_base: "#132520".to_string(),
                fg_muted: "#58706a".to_string(),
                accent: "#087f67".to_string(),
                border: "#a9cbc0".to_string(),
            },
            background: BackgroundConfig {
                mode: BackgroundMode::Solid {
                    color: "#ecf8f2".to_string(),
                },
            },
        },
    }
}
