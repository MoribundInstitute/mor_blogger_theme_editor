//! Theme presets.
//!
//! Each preset is one file in this module. Presets are grouped into two layers:
//!
//! 1. **Tokens**: palette, typography, button shape, menu, footer.
//!    These map into the existing `{{COLOR_*}}`, `{{FONT_*}}`, etc. placeholders.
//! 2. **Preset CSS**: optional bundle of additional CSS rules (`preset_css`)
//!    loaded *after* the token `:root`.
//!
//! To add a new preset:
//!   * create `src/presets/<name>.rs` with a public `fn <name>() -> Preset`
//!   * optionally create `src/presets/css/<name>.css` and `include_str!` it
//!   * register the function in `all_presets()` below

use crate::config::{
    AssetConfig, BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig,
    IconConfig, MenuLink, PluginConfig, SeoConfig, SiteConfig, SurfaceFill, SurfaceMode,
    TemplatePackConfig, ThemeConfig, TypographyConfig,
};

pub mod mor_fluid_interactive;
pub mod mor_glassmorphism;
pub mod mor_minimal;
pub mod mor_modern_editorial;
pub mod mor_neon_cyberpunk;
pub mod mor_newspaper;
pub mod mor_retro_mmorpg;
pub mod mor_terminal_classic;
pub mod mor_web_1_0_frames;
pub mod mor_web_2_0_skeuo;
pub mod user_presets;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
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
    pub preset_css: &'static str,
}

pub fn all_builtin_presets() -> Vec<Preset> {
    vec![
        mor_modern_editorial::mor_modern_editorial(),
        mor_web_1_0_frames::mor_web_1_0_frames(),
        mor_newspaper::mor_newspaper(),
        mor_web_2_0_skeuo::mor_web_2_0_skeuo(),
        mor_glassmorphism::mor_glassmorphism(),
        mor_neon_cyberpunk::mor_neon_cyberpunk(),
        mor_fluid_interactive::mor_fluid_interactive(),
        mor_terminal_classic::mor_terminal_classic(),
        mor_retro_mmorpg::mor_retro_mmorpg(),
        mor_minimal::mor_minimal(),
    ]
}

pub fn all_presets() -> Vec<Preset> {
    let mut presets = all_builtin_presets();
    presets.extend(user_presets::load_user_presets_as_presets());
    presets
}

// ---------------------------------------------------------------------------
// Shared font stacks
// ---------------------------------------------------------------------------

pub const STACK_MONO: &str = "'Courier New', Courier, monospace";
pub const STACK_SERIF: &str = "Georgia, 'Times New Roman', Times, serif";

#[allow(dead_code)]
pub const STACK_SANS: &str =
    "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif";

#[allow(dead_code)]
pub const STACK_NEWSPAPER: &str = "'Times New Roman', Times, Georgia, serif";

pub const STACK_SYSTEM_UI: &str = "system-ui, -apple-system, sans-serif";
pub const STACK_WIN95: &str = "'MS Sans Serif', 'Microsoft Sans Serif', Tahoma, Geneva, sans-serif";

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

#[allow(dead_code)]
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
        icons: IconConfig::default(),
        background: BackgroundConfig {
            mode: BackgroundMode::Solid {
                color: "#000".to_string(),
            },
        },
        assets: AssetConfig {
            favicon_url: String::new(),
            social_card_image_url: String::new(),
        },
        plugins: PluginConfig {
            custom_js: String::new(),
        },
        static_pages: crate::config::StaticPagesConfig::default(),
        ads: crate::config::AdsConfig::default(),
        template_pack: TemplatePackConfig::default(),
        preset_css: String::new(),
    }
}
// ---------------------------------------------------------------------------
// Engine resolution helpers
// ---------------------------------------------------------------------------

/// Return the `(light, dark)` palette pair for the named preset.
///
/// If `preset_id` is `None`, or the ID is not found in the registry, both
/// palettes fall back to the current active colors from `fallback_config`.
/// This means the engine always gets valid palettes regardless of state.
pub fn resolve_palette_pair(
    preset_id: Option<&str>,
    fallback_config: &crate::config::ThemeConfig,
) -> (PresetPalette, PresetPalette) {
    if let Some(id) = preset_id {
        if let Some(preset) = all_presets().into_iter().find(|p| p.id == id) {
            return (preset.light.clone(), preset.dark.clone());
        }
    }

    // Fallback: wrap the live config colors so the engine never receives empty palettes.
    let fallback = PresetPalette {
        colors: fallback_config.colors.clone(),
        background: fallback_config.background.clone(),
    };
    (fallback.clone(), fallback)
}