//! Theme presets.
//!
//! Presets are now loaded dynamically at runtime from TOML files in the `theme_presets` folder.
//! This completely detaches aesthetic definitions from the compiled Rust binary.

use crate::config::{BackgroundConfig, ColorConfig, ThemeConfig, TypographyConfig};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

// Ensure UI compatibility by leaking strings safely only once at boot
static LOADED_PRESETS: OnceLock<Vec<Preset>> = OnceLock::new();

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PresetPalette {
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
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

#[derive(Clone, Debug, Deserialize)]
struct TomlPreset {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,

    #[serde(default)]
    colors: ColorConfig,
    #[serde(default)]
    background: BackgroundConfig,
    #[serde(default)]
    typography: TypographyConfig,
    #[serde(default)]
    buttons: crate::config::ButtonConfig,

    light: Option<PresetPalette>,
    dark: Option<PresetPalette>,

    #[serde(default)]
    scrollbars: TomlScrollbars,

    #[serde(default)]
    preset_css: String,
}

/// Optional `[scrollbars]` table. Each field overrides the corresponding
/// ThemeConfig default only when present, so a preset can tune any subset.
#[derive(Clone, Debug, Default, Deserialize)]
struct TomlScrollbars {
    width: Option<String>,
    track_color: Option<String>,
    thumb_color: Option<String>,
    thumb_hover_color: Option<String>,
}

impl TomlPreset {
    fn into_preset(self, id: String) -> Preset {
        let mut base = crate::config::defaults::default_theme_config();

        base.colors = self.colors.clone();
        base.background = self.background.clone();
        base.typography = self.typography;
        base.buttons = self.buttons;

        // Scrollbars: override only the fields the preset actually specifies.
        if let Some(v) = self.scrollbars.width {
            base.scrollbar_width = v;
        }
        if let Some(v) = self.scrollbars.track_color {
            base.scrollbar_track_color = v;
        }
        if let Some(v) = self.scrollbars.thumb_color {
            base.scrollbar_thumb_color = v;
        }
        if let Some(v) = self.scrollbars.thumb_hover_color {
            base.scrollbar_thumb_hover_color = v;
        }

        let base_pal = PresetPalette {
            colors: self.colors.clone(),
            background: self.background.clone(),
        };

        let has_light = self.light.is_some();
        let has_dark = self.dark.is_some();

        let light = self.light.clone().unwrap_or_else(|| base_pal.clone());

        let dark = self.dark.clone().unwrap_or_else(|| {
            if !has_light && !has_dark {
                // No explicit mode maps at all: generate the other contrast via swap for toggle to work
                PresetPalette {
                    colors: self.colors.inverted_contrast(),
                    background: self.background.clone(),
                }
            } else {
                base_pal.clone()
            }
        });

        let name = if self.name.is_empty() {
            "Unnamed Preset".to_string()
        } else {
            self.name
        };

        Preset {
            id: Box::leak(id.into_boxed_str()),
            name: Box::leak(name.into_boxed_str()),
            description: Box::leak(self.description.into_boxed_str()),
            base_config: base,
            dark,
            light,
            preset_css: Box::leak(self.preset_css.into_boxed_str()),
        }
    }
}

/// Load a workspace `ThemeConfig` or aesthetic preset TOML (e.g. `theme_presets/mor_retro_mmorpg.toml`)
/// into a render-ready [`ThemeConfig`].
pub fn theme_config_from_path(path: &Path) -> Result<ThemeConfig, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path.display(), e))?;

    if let Ok(config) = toml::from_str::<ThemeConfig>(&contents) {
        return Ok(config);
    }

    let toml_preset: TomlPreset = toml::from_str(&contents).map_err(|e| {
        format!(
            "Failed to parse '{}' as workspace or preset TOML: {}",
            path.display(),
            e
        )
    })?;

    let id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("preset")
        .to_string();

    let preset = toml_preset.into_preset(id);
    let mut config = preset.base_config.clone();
    config.preset_css = preset.preset_css.to_string();
    config.active_preset_id = Some(preset.id.to_string());
    Ok(config)
}

pub fn get_canonical_presets_dir() -> PathBuf {
    let local = Path::new("theme_presets");
    let parent = Path::new("../theme_presets");

    // Antigravity check: Does the local directory actually contain matter?
    let local_has_files = fs::read_dir(local)
        .map(|mut iter| iter.any(|entry| {
            entry.ok()
                .map(|e| e.path().extension() == Some(std::ffi::OsStr::new("toml")))
                .unwrap_or(false)
        }))
        .unwrap_or(false);

    if !local_has_files && parent.exists() {
        parent.to_path_buf()
    } else {
        local.to_path_buf()
    }
}

pub fn all_presets() -> Vec<Preset> {
    LOADED_PRESETS
        .get_or_init(|| {
            let mut presets = Vec::new();

            let preset_dir = get_canonical_presets_dir();

            if !preset_dir.exists() {
                let _ = fs::create_dir_all(&preset_dir);
                return presets;
            }

            if let Ok(entries) = fs::read_dir(&preset_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            match toml::from_str::<TomlPreset>(&contents) {
                                Ok(toml_preset) => {
                                    let id =
                                        path.file_stem().unwrap().to_str().unwrap().to_string();
                                    presets.push(toml_preset.into_preset(id));
                                }
                                Err(e) => eprintln!("Failed to parse preset {:?}: {}", path, e),
                            }
                        }
                    }
                }
            }

            presets.sort_by(|a, b| a.name.cmp(b.name));
            presets
        })
        .clone()
}

pub fn resolve_palette_pair(
    preset_id: Option<&str>,
    fallback_config: &crate::config::ThemeConfig,
) -> (PresetPalette, PresetPalette) {
    if let Some(id) = preset_id {
        if let Some(preset) = all_presets().into_iter().find(|p| p.id == id) {
            return (preset.light.clone(), preset.dark.clone());
        }
    }

    // Default theme canonical pair logic
    // Heuristic: Is the current fallback_config representing a dark or light mode?
    let is_currently_dark = match &fallback_config.background.mode {
        crate::config::BackgroundMode::Gradient { from, .. } => {
            // Default dark purple workspace gradient
            from == "#1e1a4d"
        }
        crate::config::BackgroundMode::Solid { color } => {
            // Default dark background color
            color == "#0f1026" || color == "#222129"
        }
        _ => {
            // Fallback: check bg_base directly
            fallback_config.colors.bg_base == "#0f1026"
                || fallback_config.colors.bg_base == "#222129"
        }
    };

    if is_currently_dark {
        // Fallback is dark: assign current to dark slot, and invert for light slot
        let dark = PresetPalette {
            colors: fallback_config.colors.clone(),
            background: fallback_config.background.clone(),
        };
        let light = PresetPalette {
            colors: fallback_config.colors.inverted_contrast(),
            background: fallback_config.background.inverted_contrast(),
        };
        (light, dark)
    } else {
        // Fallback is light: assign current to light slot, and invert for dark slot
        let light = PresetPalette {
            colors: fallback_config.colors.clone(),
            background: fallback_config.background.clone(),
        };
        let dark = PresetPalette {
            colors: fallback_config.colors.inverted_contrast(),
            background: fallback_config.background.inverted_contrast(),
        };
        (light, dark)
    }
}

// ---------------------------------------------------------------------------
// Shared font stacks & Helpers
// ---------------------------------------------------------------------------

pub const STACK_MONO: &str = "'Courier New', Courier, monospace";
pub const STACK_SERIF: &str = "Georgia, 'Times New Roman', Times, serif";
pub const STACK_SANS: &str =
    "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif";
pub const STACK_NEWSPAPER: &str = "'Times New Roman', Times, Georgia, serif";
pub const STACK_SYSTEM_UI: &str = "system-ui, -apple-system, sans-serif";
pub const STACK_WIN95: &str = "'MS Sans Serif', 'Microsoft Sans Serif', Tahoma, Geneva, sans-serif";

pub fn gradient(from: &str, to: &str, angle_deg: u16) -> crate::config::SurfaceFill {
    crate::config::SurfaceFill {
        mode: crate::config::SurfaceMode::Gradient,
        color: from.to_string(),
        gradient_from: from.to_string(),
        gradient_to: to.to_string(),
        gradient_angle_deg: angle_deg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The preset format must carry scrollbar settings into base_config; overrides
    // are per-field, unspecified fields keep the ThemeConfig default.
    #[test]
    fn toml_scrollbars_map_into_base_config() {
        let src = "name = \"T\"\n[scrollbars]\nwidth = \"13px\"\nthumb_color = \"#abcdef\"\n";
        let p: TomlPreset = toml::from_str(src).unwrap();
        let preset = p.into_preset("t".into());
        assert_eq!(preset.base_config.scrollbar_width, "13px");
        assert_eq!(preset.base_config.scrollbar_thumb_color, "#abcdef");
        let def = crate::config::ThemeConfig::default();
        assert_eq!(
            preset.base_config.scrollbar_track_color,
            def.scrollbar_track_color
        );
    }

    // Regression gate: every shipped preset must load, carry non-empty CSS with
    // no CDATA hazard, and survive a full render with its CSS landing in the
    // exported XML. Fails loudly if the presets dir can't be found — a silent
    // skip here would defeat the gate.
    #[test]
    fn every_shipped_preset_loads_renders_and_lands_its_css() {
        let dir = get_canonical_presets_dir();
        let mut checked = 0;
        for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("presets dir {dir:?}: {e}")) {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|s| s.to_str()) != Some("toml") {
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let config = theme_config_from_path(&path)
                .unwrap_or_else(|e| panic!("{name}: failed to load: {e}"));

            let css = config.preset_css.trim();
            assert!(!css.is_empty(), "{name}: empty preset_css");
            assert!(!css.contains("]]>"), "{name}: CDATA hazard ]]> in preset_css");

            let xml = crate::render::theme::render_theme(&config, &std::collections::HashMap::new());
            // A distinctive slice from the first rule proves the CSS landed
            // verbatim in the export (comments before the first brace skipped).
            let brace = css.find('{').unwrap_or_else(|| panic!("{name}: css has no rule"));
            let slice = &css[brace..css.len().min(brace + 40)];
            assert!(xml.contains(slice), "{name}: preset_css missing from rendered XML");
            checked += 1;
        }
        assert!(checked >= 8, "expected >= 8 presets, found {checked} in {dir:?}");
    }

    // Every shipped preset must actually customize its scrollbar (not leave the
    // default thumb). Guards the theme_presets/*.toml [scrollbars] tables.
    #[test]
    fn shipped_presets_customize_scrollbars() {
        let presets = all_presets();
        if presets.is_empty() {
            return; // preset dir not resolvable from this cwd; nothing to check.
        }
        let def = crate::config::ThemeConfig::default().scrollbar_thumb_color;
        for p in &presets {
            assert_ne!(
                p.base_config.scrollbar_thumb_color, def,
                "preset '{}' does not customize its scrollbar",
                p.name
            );
        }
    }

    // The themed [buttons] fields must reach base_config (TOML -> ButtonConfig).
    #[test]
    fn shipped_presets_have_themed_buttons() {
        let presets = all_presets();
        if presets.is_empty() {
            return;
        }
        let has = |pred: fn(&crate::config::ButtonConfig) -> bool| {
            presets.iter().any(|p| pred(&p.base_config.buttons))
        };
        // Presets now express their button looks through varied fills/effects.
        assert!(has(|b| b.fill == "gradient"), "no preset uses gradient fill");
        assert!(has(|b| b.fill == "glass"), "no preset uses glass fill");
        assert!(has(|b| b.fill == "neon"), "no preset uses neon fill");
        assert!(has(|b| !b.box_shadow.is_empty()), "no preset sets a custom box-shadow");
        assert!(has(|b| b.hover_effect == "glow"), "no preset uses glow hover");
    }
}
