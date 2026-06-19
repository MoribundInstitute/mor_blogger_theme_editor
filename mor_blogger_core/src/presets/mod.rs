//! Theme presets.
//!
//! Presets are now loaded dynamically at runtime from TOML files in the `theme_presets` folder.
//! This completely detaches aesthetic definitions from the compiled Rust binary.

use crate::config::{BackgroundConfig, ColorConfig, ThemeConfig, TypographyConfig};
use serde::Deserialize;
use std::fs;
use std::path::Path;
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
    preset_css: String,
}

impl TomlPreset {
    fn into_preset(self, id: String) -> Preset {
        let mut base = crate::config::defaults::default_theme_config();

        base.colors = self.colors.clone();
        base.background = self.background.clone();
        base.typography = self.typography;
        base.buttons = self.buttons;

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

pub fn all_presets() -> Vec<Preset> {
    LOADED_PRESETS
        .get_or_init(|| {
            let mut presets = Vec::new();

            // Resilient path resolution: Check if we are running from the workspace root
            // or from inside the mor_blogger_dioxus_ui crate.
            let mut preset_dir = Path::new("theme_presets").to_path_buf();
            if !preset_dir.exists() && Path::new("../theme_presets").exists() {
                preset_dir = Path::new("../theme_presets").to_path_buf();
            }

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
