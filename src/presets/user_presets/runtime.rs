use std::fs;
use std::path::Path;

use crate::config::gtk_theme::svg_to_mask_uri;
use crate::config::{ColorConfig, IconConfig, ThemeConfig};
use crate::presets::{Preset, PresetPalette};

use super::icons::UserPresetIconAssets;
use super::io::{load_user_preset_bundle, load_user_preset_icon_assets};
use super::paths::default_user_presets_dir;
use super::UserPresetBundle;

/// Load all user preset bundles from the normal config directory and convert
/// them into the same runtime `Preset` shape used by built-in presets.
///
/// This intentionally returns best-effort results. A bad user preset should not
/// prevent the editor from opening; errors are printed to stderr for now.
pub fn load_user_presets_as_presets() -> Vec<Preset> {
    let root = default_user_presets_dir();

    let Ok(entries) = fs::read_dir(&root) else {
        return Vec::new();
    };

    let mut presets = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        match load_user_preset_from_dir(&path) {
            Ok(preset) => presets.push(preset),
            Err(err) => eprintln!("[user_presets] could not load {}: {}", path.display(), err),
        }
    }

    presets.sort_by(|a, b| a.name.cmp(b.name));
    presets
}

fn load_user_preset_from_dir(bundle_dir: &Path) -> Result<Preset, String> {
    let (bundle, mut report) = load_user_preset_bundle(bundle_dir)?;

    let icon_assets = match load_user_preset_icon_assets(bundle_dir) {
        Ok(icons) => Some(icons),
        Err(err) => {
            report.warnings.push(err);
            None
        }
    };

    Ok(bundle_to_runtime_preset(bundle, icon_assets))
}

fn bundle_to_runtime_preset(
    bundle: UserPresetBundle,
    icon_assets: Option<UserPresetIconAssets>,
) -> Preset {
    let mut config = bundle.config;

    if !bundle.preset_css.trim().is_empty() {
        config.preset_css = bundle.preset_css.clone();
    }

    if let Some(icon_assets) = icon_assets {
        merge_icon_assets_into_config(&mut config, icon_assets);
    }

    let palette = palette_from_config(&config);

    // The existing Preset type is optimized for built-in Rust presets and uses
    // &'static str fields. User presets are loaded from disk at runtime, so we
    // leak a tiny amount of string data per loaded preset to fit that API.
    // Later, this can be cleaned up by changing Preset to use Cow<'static, str>
    // or owned String fields.
    let id = leak_string(bundle.id);
    let name = leak_string(bundle.name);
    let description = leak_string(if bundle.description.trim().is_empty() {
        "User preset".to_string()
    } else {
        bundle.description
    });
    let preset_css = leak_string(config.preset_css.clone());

    Preset {
        id,
        name,
        description,
        base_config: config,
        dark: palette.clone(),
        light: palette,
        preset_css,
    }
}

fn palette_from_config(config: &ThemeConfig) -> PresetPalette {
    PresetPalette {
        colors: ColorConfig {
            bg_base: config.colors.bg_base.clone(),
            bg_panel: config.colors.bg_panel.clone(),
            bg_elevated: config.colors.bg_elevated.clone(),
            fg_base: config.colors.fg_base.clone(),
            fg_muted: config.colors.fg_muted.clone(),
            accent: config.colors.accent.clone(),
            border: config.colors.border.clone(),
        },
        background: config.background.clone(),
    }
}

fn merge_icon_assets_into_config(config: &mut ThemeConfig, assets: UserPresetIconAssets) {
    let mut icons: IconConfig = config.icons.clone();

    if let Some(svg) = assets.sidebar_left_svg.as_deref() {
        icons.sidebar_left = svg_to_mask_uri(svg);
    }

    if let Some(svg) = assets.sidebar_right_svg.as_deref() {
        icons.sidebar_right = svg_to_mask_uri(svg);
    }

    if let Some(svg) = assets.panel_close_svg.as_deref() {
        icons.panel_close = svg_to_mask_uri(svg);
    }

    if let Some(svg) = assets.search_svg.as_deref() {
        icons.search = svg_to_mask_uri(svg);
    }

    if let Some(svg) = assets.menu_svg.as_deref() {
        icons.menu = svg_to_mask_uri(svg);
    }

    config.icons = icons;
}

fn leak_string(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}
