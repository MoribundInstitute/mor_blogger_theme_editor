//! Theme presets.
//!
//! Each preset is one file in this module. Presets are grouped into two layers:
//!
//! 1. **Tokens**: palette, typography, button shape, menu, footer.
//!    These map into the existing `{{COLOR_*}}`, `{{FONT_*}}`, etc. placeholders.
//! 2. **Preset CSS**: optional bundle of additional CSS rules (`preset_css`)
//!    loaded *after* the token `:root`. This is where presets express their
//!    structural personality (ridged borders, backdrop blurs, neon glows, etc.).
//!
//! To add a new preset:
//!   - create `src/presets/<name>.rs` with a public `fn <name>() -> Preset`
//!   - optionally create `src/presets/css/<name>.css` and `include_str!` it
//!   - register the function in `all_presets()` below

use crate::config::{
    AssetConfig, BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig,
    MenuLink, PluginConfig, SeoConfig, SiteConfig, SurfaceFill, SurfaceMode, ThemeConfig,
    TypographyConfig,
};

pub mod modern_editorial;
pub mod minimal;
pub mod terminal_classic;
pub mod web_1_0_frames;
pub mod glassmorphism;
pub mod neon_cyberpunk;
pub mod newspaper;
pub mod web_2_0_skeuo;
pub mod fluid_interactive;

#[derive(Clone, Debug, PartialEq)]
pub struct PresetPalette {
    pub colors: ColorConfig,
    pub background: BackgroundConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub base_config: ThemeConfig,
    pub dark: PresetPalette,
    pub light: PresetPalette,

    /// Optional bundle of CSS rules that ship with this preset. Loaded
    /// inside the exported theme's `<b:skin>` block, after the token-driven
    /// `:root`, so preset rules can override or extend the base style.
    /// Empty string means "no extra CSS; pure token swap."
    pub preset_css: &'static str,
}

pub fn all_presets() -> Vec<Preset> {
    vec![
        modern_editorial::modern_editorial(),
        web_1_0_frames::web_1_0_frames(),
        newspaper::newspaper(),
        web_2_0_skeuo::web_2_0_skeuo(),
        glassmorphism::glassmorphism(),
        neon_cyberpunk::neon_cyberpunk(),
        fluid_interactive::fluid_interactive(),
        terminal_classic::terminal_classic(),
        minimal::minimal(),
    ]
}

// ---------------------------------------------------------------------------
// Shared font stacks
// ---------------------------------------------------------------------------

pub const STACK_MONO: &str = "'Courier New', Courier, monospace";
pub const STACK_SERIF: &str = "Georgia, 'Times New Roman', Times, serif";
#[allow(dead_code)] // reserved for upcoming presets (web_2_0_skeuo, newspaper)
pub const STACK_SANS: &str =
    "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif";
#[allow(dead_code)] // reserved for newspaper preset
pub const STACK_NEWSPAPER: &str = "'Times New Roman', Times, Georgia, serif";
pub const STACK_SYSTEM_UI: &str = "system-ui, -apple-system, sans-serif";
pub const STACK_WIN95: &str = "'MS Sans Serif', 'Microsoft Sans Serif', Tahoma, Geneva, sans-serif";

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

#[allow(dead_code)] // helper for gradient SurfaceFills, used by future presets
pub fn gradient(from: &str, to: &str, angle_deg: u16) -> SurfaceFill {
    SurfaceFill {
        mode: SurfaceMode::Gradient,
        color: from.to_string(),
        gradient_from: from.to_string(),
        gradient_to: to.to_string(),
        gradient_angle_deg: angle_deg,
    }
}

pub fn build_base(
    site: SiteConfig,
    typography: TypographyConfig,
    buttons: ButtonConfig,
    seo: SeoConfig,
    menu_links: Vec<MenuLink>,
    footer: FooterConfig,
) -> ThemeConfig {
    ThemeConfig {
        site,
        typography,
        buttons,
        seo,
        menu_links,
        footer,
        // Palette fields overwritten by swap_palette at apply time.
        colors: ColorConfig {
            bg_base: "#000".to_string(),
            bg_panel: SurfaceFill::solid("#000"),
            bg_elevated: SurfaceFill::solid("#000"),
            fg_base: "#fff".to_string(),
            fg_muted: "#aaa".to_string(),
            accent: "#fff".to_string(),
            border: "#444".to_string(),
        },
        background: BackgroundConfig {
            mode: BackgroundMode::Solid { color: "#000".to_string() },
        },
        assets: AssetConfig {
            favicon_url: String::new(),
            social_card_image_url: String::new(),
        },
        plugins: PluginConfig { custom_js: String::new() },
        static_pages: crate::config::StaticPagesConfig::default(),
        preset_css: String::new(),
    }
}