//! Theme presets.
//!
//! Presets are now loaded dynamically at runtime from TOML files in the `theme_presets` folder.
//! This completely detaches aesthetic definitions from the compiled Rust binary.

use crate::config::{BackgroundConfig, ColorConfig, ThemeConfig, TypographyConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

// Cached preset list; `None` means "reload from disk on next access" so
// presets saved at runtime (imports, GTK saves) appear without a restart.
static LOADED_PRESETS: RwLock<Option<Vec<Preset>>> = RwLock::new(None);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PresetPalette {
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
    pub background: BackgroundConfig,
}

/// A named alternate color scheme for a preset — same structure and CSS,
/// different palette pair. "Quick color options" in the preset browser.
#[derive(Clone, Debug, PartialEq)]
pub struct PresetVariant {
    pub name: &'static str,
    pub dark: PresetPalette,
    pub light: PresetPalette,
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
    pub variants: Vec<PresetVariant>,
}

impl Preset {
    /// Palette pair (light, dark) for a variant name; the base pair when
    /// `variant` is None or unknown.
    pub fn palette_pair(&self, variant: Option<&str>) -> (&PresetPalette, &PresetPalette) {
        if let Some(name) = variant {
            if let Some(v) = self.variants.iter().find(|v| v.name == name) {
                return (&v.light, &v.dark);
            }
        }
        (&self.light, &self.dark)
    }
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
    variants: Vec<TomlVariant>,

    #[serde(default)]
    scrollbars: TomlScrollbars,

    #[serde(default)]
    preset_css: String,
}

/// `[[variants]]` entry. A missing mode falls back to the preset's base
/// palette for that mode, so accent-only variants can skip one side.
#[derive(Clone, Debug, Deserialize)]
struct TomlVariant {
    name: String,
    light: Option<PresetPalette>,
    dark: Option<PresetPalette>,
}

/// Optional `[scrollbars]` table. Each field overrides the corresponding
/// ThemeConfig default only when present, so a preset can tune any subset.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

        let variants = self
            .variants
            .into_iter()
            .map(|v| PresetVariant {
                name: Box::leak(v.name.into_boxed_str()),
                light: v.light.unwrap_or_else(|| light.clone()),
                dark: v.dark.unwrap_or_else(|| dark.clone()),
            })
            .collect();

        Preset {
            id: Box::leak(id.into_boxed_str()),
            name: Box::leak(name.into_boxed_str()),
            description: Box::leak(self.description.into_boxed_str()),
            base_config: base,
            dark,
            light,
            preset_css: Box::leak(self.preset_css.into_boxed_str()),
            variants,
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
    if let Some(presets) = LOADED_PRESETS.read().unwrap().as_ref() {
        return presets.clone();
    }
    let presets = load_presets_from_disk();
    *LOADED_PRESETS.write().unwrap() = Some(presets.clone());
    presets
}

/// Drop the cached preset list so the next [`all_presets`] call re-reads the
/// presets directory. Call after writing a new preset file at runtime.
pub fn reload_presets() {
    *LOADED_PRESETS.write().unwrap() = None;
}

fn load_presets_from_disk() -> Vec<Preset> {
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
                            let id = path.file_stem().unwrap().to_str().unwrap().to_string();
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
}

/// Preset TOML written for themes saved at runtime (compendium imports, GTK
/// saves). Same shape `TomlPreset` reads back: explicit dark/light palettes
/// so the mode toggle restores the authored look instead of a generated one.
#[derive(Serialize)]
struct TomlPresetExport<'a> {
    name: &'a str,
    description: &'a str,
    colors: &'a ColorConfig,
    background: &'a BackgroundConfig,
    typography: &'a TypographyConfig,
    buttons: &'a crate::config::ButtonConfig,
    dark: PresetPalette,
    light: PresetPalette,
    scrollbars: TomlScrollbars,
    preset_css: &'a str,
}

fn build_preset_toml(name: &str, description: &str, config: &ThemeConfig) -> Result<String, String> {
    // Same luminance classification the dark-mode toggle uses: the config's
    // palette lands in its true slot, the opposite slot gets the derived pair.
    let (light, dark) = resolve_palette_pair(None, None, config);

    let export = TomlPresetExport {
        name,
        description,
        colors: &config.colors,
        background: &config.background,
        typography: &config.typography,
        buttons: &config.buttons,
        dark,
        light,
        scrollbars: TomlScrollbars {
            width: Some(config.scrollbar_width.clone()),
            track_color: Some(config.scrollbar_track_color.clone()),
            thumb_color: Some(config.scrollbar_thumb_color.clone()),
            thumb_hover_color: Some(config.scrollbar_thumb_hover_color.clone()),
        },
        preset_css: &config.preset_css,
    };

    toml::to_string_pretty(&export).map_err(|e| e.to_string())
}

/// Save a `ThemeConfig` as a reusable preset TOML in the presets directory
/// and refresh the preset list. Only the visual theme is captured — site
/// content (title, menus, SEO…) stays out, matching what applying a preset
/// touches. Overwrites an existing preset with the same id.
pub fn save_theme_config_as_preset(
    id: &str,
    name: &str,
    description: &str,
    config: &ThemeConfig,
) -> Result<PathBuf, String> {
    let toml_str = build_preset_toml(name, description, config)?;

    let preset_dir = get_canonical_presets_dir();
    if !preset_dir.exists() {
        fs::create_dir_all(&preset_dir).map_err(|e| e.to_string())?;
    }

    let file_path = preset_dir.join(format!("{}.toml", id));
    fs::write(&file_path, toml_str).map_err(|e| e.to_string())?;
    reload_presets();
    Ok(file_path)
}

/// Import a full preset document — TOML with authored `light`+`dark` palettes
/// and/or `[[variants]]` color schemes — saved verbatim so nothing a flat
/// `ThemeConfig` can't hold gets dropped. Errs when the text isn't a full
/// preset doc; callers fall back to the flat import path.
pub fn save_full_preset_text(text: &str) -> Result<Preset, String> {
    let parsed: TomlPreset = toml::from_str(text).map_err(|e| e.to_string())?;
    if parsed.variants.is_empty() && (parsed.light.is_none() || parsed.dark.is_none()) {
        return Err("not a full preset document".to_string());
    }
    if parsed.name.trim().is_empty() {
        return Err("preset document has no name".to_string());
    }
    let id: String = parsed
        .name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();

    let preset_dir = get_canonical_presets_dir();
    if !preset_dir.exists() {
        fs::create_dir_all(&preset_dir).map_err(|e| e.to_string())?;
    }
    fs::write(preset_dir.join(format!("{id}.toml")), text).map_err(|e| e.to_string())?;
    reload_presets();
    all_presets()
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| "imported preset did not load back from disk".to_string())
}

pub fn resolve_palette_pair(
    preset_id: Option<&str>,
    variant_id: Option<&str>,
    fallback_config: &crate::config::ThemeConfig,
) -> (PresetPalette, PresetPalette) {
    if let Some(id) = preset_id {
        if let Some(preset) = all_presets().into_iter().find(|p| p.id == id) {
            let (light, dark) = preset.palette_pair(variant_id);
            return (light.clone(), dark.clone());
        }
    }

    // Default theme canonical pair logic.
    // Measure the canvas: a theme is "dark" when its background is actually
    // dark, not when it equals one of our own default hex strings — imported
    // themes (e.g. compendium JSON with bg_base #080808) never matched those,
    // got classified light, and wore a garbage inverted dark palette.
    let bg_probe = match &fallback_config.background.mode {
        crate::config::BackgroundMode::Gradient { from, .. } => from.as_str(),
        crate::config::BackgroundMode::Solid { color } => color.as_str(),
        _ => fallback_config.colors.bg_base.as_str(),
    };
    let is_currently_dark = perceived_luminance(bg_probe)
        .or_else(|| perceived_luminance(&fallback_config.colors.bg_base))
        .map(|l| l < 0.5)
        .unwrap_or(false);

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

/// Perceived luminance (0.0 black – 1.0 white) of a `#rgb`/`#rrggbb` color.
/// None for anything unparseable (var(), gradients, named colors).
fn perceived_luminance(color: &str) -> Option<f32> {
    let hex = color.trim().strip_prefix('#')?;
    let (r, g, b) = match hex.len() {
        3 => (
            u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?,
            u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?,
            u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?,
        ),
        6 => (
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        ),
        _ => return None,
    };
    Some((0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Importing a full preset document must preserve its [[variants]] color
    // schemes — the flat ThemeConfig import path drops them (the compendium
    // "MorBranding has no color options" bug).
    #[test]
    fn full_preset_import_preserves_variants() {
        let text = r##"
name = "Import Fixture Zz"
description = "test fixture"
preset_css = "body { color: red; }"

[colors]
bg_base = "#eadfc2"
accent = "#8a6a1f"

[light.colors]
bg_base = "#eadfc2"
accent = "#8a6a1f"

[dark.colors]
bg_base = "#0b1b1d"
accent = "#d2a94f"

[[variants]]
name = "Ultramarine Deep"
[variants.light]
colors = { bg_base = "#d8dfe6", accent = "#1d4f9c" }
[variants.dark]
colors = { bg_base = "#0b141f", accent = "#6fa1e8" }
"##;

        let preset = save_full_preset_text(text).expect("full preset import");
        let cleanup = get_canonical_presets_dir().join("import_fixture_zz.toml");
        let variants = preset.variants.len();
        let has_ultramarine = preset.variants.iter().any(|v| v.name == "Ultramarine Deep");
        let _ = fs::remove_file(&cleanup);
        reload_presets();

        assert_eq!(variants, 1, "expected the fixture's color scheme");
        assert!(has_ultramarine, "Ultramarine Deep scheme missing");

        // A flat workspace ThemeConfig TOML must NOT be claimed by this path.
        let flat = toml::to_string(&crate::config::defaults::default_theme_config()).unwrap();
        assert!(save_full_preset_text(&flat).is_err());
    }

    // The preset format must carry scrollbar settings into base_config; overrides
    // are per-field, unspecified fields keep the ThemeConfig default.
    #[test]
    fn imported_dark_theme_lands_in_dark_palette_slot() {
        // Compendium import regression: no active preset + a near-black canvas
        // that matches none of our default hex strings must still classify as
        // dark, keeping the imported colors in the dark slot (the old
        // equality heuristic called it light and derived a garbage dark pair).
        let mut cfg = crate::config::ThemeConfig::default();
        cfg.colors.bg_base = "#080808".into();
        cfg.colors.fg_base = "#ECECEB".into();
        cfg.background.mode = crate::config::BackgroundMode::Solid { color: "#080808".into() };
        let (light, dark) = resolve_palette_pair(None, None, &cfg);
        assert_eq!(dark.colors.bg_base, "#080808", "imported colors must own the dark slot");
        assert_ne!(light.colors.bg_base, "#080808", "light slot must be the derived pair");

        // And a genuinely light import stays light.
        cfg.colors.bg_base = "#f5f2ea".into();
        cfg.background.mode = crate::config::BackgroundMode::Solid { color: "#f5f2ea".into() };
        let (light, dark) = resolve_palette_pair(None, None, &cfg);
        assert_eq!(light.colors.bg_base, "#f5f2ea");
        assert_ne!(dark.colors.bg_base, "#f5f2ea");
    }

    #[test]
    fn saved_preset_toml_round_trips_with_correct_palette_slots() {
        let mut config = crate::config::defaults::default_theme_config();
        config.colors.bg_base = "#0a0a0a".to_string();
        config.colors.fg_base = "#eeeeee".to_string();
        config.background.mode = crate::config::BackgroundMode::Solid {
            color: "#0a0a0a".into(),
        };
        config.preset_css = ".x { color: red; }".to_string();
        config.scrollbar_width = "13px".to_string();

        let toml_str = build_preset_toml("Imported", "test", &config).unwrap();
        let parsed: TomlPreset = toml::from_str(&toml_str).unwrap();
        let preset = parsed.into_preset("imported".to_string());

        // Dark-authored palette lands in the dark slot; light is derived.
        assert_eq!(preset.dark.colors.bg_base, "#0a0a0a");
        assert_ne!(preset.light.colors.bg_base, "#0a0a0a");
        assert_eq!(preset.preset_css, ".x { color: red; }");
        assert_eq!(preset.base_config.scrollbar_width, "13px");
    }

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
            // Skip transient fixtures from the full-preset import test.
            if name.starts_with("import_fixture") {
                continue;
            }
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
        // 5 presets ship in-app; the rest moved to the preset compendium repo.
        assert!(checked >= 5, "expected >= 5 presets, found {checked} in {dir:?}");
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
            // Skip transient fixtures from the full-preset import test.
            if p.id.starts_with("import_fixture") {
                continue;
            }
            assert_ne!(
                p.base_config.scrollbar_thumb_color, def,
                "preset '{}' does not customize its scrollbar",
                p.name
            );
        }
    }

    // Variants: names must be present and unique, every shipped preset offers
    // at least one alternate scheme, and each variant actually changes the
    // accent (a variant that looks identical to base is authoring error).
    #[test]
    fn shipped_presets_have_distinct_color_variants() {
        let presets = all_presets();
        if presets.is_empty() {
            return;
        }
        for p in &presets {
            assert!(
                !p.variants.is_empty(),
                "preset '{}' has no color variants",
                p.name
            );
            let mut seen = std::collections::HashSet::new();
            for v in &p.variants {
                assert!(!v.name.trim().is_empty(), "{}: unnamed variant", p.name);
                assert!(seen.insert(v.name), "{}: duplicate variant {}", p.name, v.name);
                assert_ne!(
                    v.dark.colors.accent, p.dark.colors.accent,
                    "{}: variant '{}' keeps the base dark accent",
                    p.name, v.name
                );
            }
            // palette_pair resolves variants and falls back on unknown names.
            let (l, _) = p.palette_pair(Some(p.variants[0].name));
            assert_eq!(l, &p.variants[0].light);
            let (l, d) = p.palette_pair(Some("no-such-variant"));
            assert_eq!((l, d), (&p.light, &p.dark));
        }
    }

    // The variant id in the config must reach the rendered palette pair — the
    // exported CSS's light/dark variable blocks carry the VARIANT colors, not
    // the preset's base palette (the "scheme half-applies" regression).
    #[test]
    fn variant_id_reaches_rendered_palette_pair() {
        let presets = all_presets();
        if presets.is_empty() {
            return;
        }
        let preset = presets
            .iter()
            .find(|p| !p.variants.is_empty())
            .expect("at least one preset ships variants");
        let variant = &preset.variants[0];

        let mut config = preset.base_config.clone();
        config.preset_css = preset.preset_css.to_string();
        config.active_preset_id = Some(preset.id.to_string());
        config.active_variant_id = Some(variant.name.to_string());

        let xml = crate::render::theme::render_theme(&config, &std::collections::HashMap::new());
        assert!(
            xml.contains(&variant.dark.colors.accent),
            "{}/{}: variant dark accent missing from rendered theme",
            preset.name,
            variant.name
        );
    }

    // The secondary phosphor (palette glow_color) must be emitted as the
    // --glow-color CSS var so two-tone presets keep their pair per scheme.
    #[test]
    fn palette_glow_color_lands_as_css_var() {
        let presets = all_presets();
        let Some(neon) = presets.iter().find(|p| p.id == "mor_neon_cyberpunk") else {
            return;
        };
        let variant = neon
            .variants
            .iter()
            .find(|v| v.name == "Cyan Overdrive")
            .expect("neon ships Cyan Overdrive");

        let mut config = neon.base_config.clone();
        config.preset_css = neon.preset_css.to_string();
        config.active_preset_id = Some(neon.id.to_string());
        config.active_variant_id = Some(variant.name.to_string());

        let xml = crate::render::theme::render_theme(&config, &std::collections::HashMap::new());
        // Cyan Overdrive swaps the pair: cyan primary, pink secondary.
        assert!(xml.contains("--glow-color: #ff2bd6"), "secondary phosphor var missing");
        assert!(xml.contains("--accent: #00e5ff"), "primary phosphor missing");
    }

    // Every cursor slot must land in the rendered CSS as its --theme-cursor-*
    // variable (the exported theme's pure-CSS cursor set).
    #[test]
    fn cursor_set_lands_in_rendered_css() {
        let mut config = ThemeConfig::default();
        config.cursor_set.zoom_in = "url('https://x/z.png') 4 4, zoom-in".to_string();
        config.cursor_set.not_allowed = "no-drop".to_string();
        let xml = crate::render::theme::render_theme(&config, &std::collections::HashMap::new());
        assert!(xml.contains("--theme-cursor-zoom: url('https://x/z.png') 4 4, zoom-in"));
        assert!(xml.contains("--theme-cursor-not-allowed: no-drop"));
        // The mapping sheet ships with the defaults.
        assert!(xml.contains("var(--theme-cursor-help, help)"));
        assert!(xml.contains("var(--theme-cursor-wait, wait)"));
    }

    // Preset cursor SVGs use {{ACCENT_URLENCODED}}, so the pen/arrow ink
    // follows the active color scheme instead of baking one hex forever.
    #[test]
    fn preset_cursors_follow_the_scheme() {
        let presets = all_presets();
        let glass = presets
            .iter()
            .find(|p| p.id == "mor_fluid_interactive")
            .expect("mor_fluid_interactive ships with the app");
        let variant = glass
            .variants
            .iter()
            .find(|v| v.name == "Ultraviolet Deep")
            .expect("fluid interactive ships Ultraviolet Deep");

        // Simulate the app applying the variant: current colors = variant palette.
        let mut config = glass.base_config.clone();
        config.colors = variant.dark.colors.clone();
        config.preset_css = glass.preset_css.to_string();
        config.active_preset_id = Some(glass.id.to_string());
        config.active_variant_id = Some(variant.name.to_string());

        let xml = crate::render::theme::render_theme(&config, &std::collections::HashMap::new());
        let enc = variant.dark.colors.accent.replace('#', "%23");
        assert!(
            xml.contains(&format!("fill='{enc}'")),
            "cursor fill did not follow the scheme accent"
        );
        assert!(!xml.contains("{{ACCENT_URLENCODED}}"), "token left unreplaced");
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
