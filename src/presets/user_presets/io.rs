use std::fs;
use std::path::{Path, PathBuf};

use super::bundle::{PresetSourceInfo, UserPresetBundle};
use super::icons::{IconAssetNames, UserPresetIconAssets};
use super::paths::user_preset_bundle_dir;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UserPresetDiskReport {
    pub bundle_dir: PathBuf,
    pub files_written: Vec<PathBuf>,
    pub files_read: Vec<PathBuf>,
    pub warnings: Vec<String>,
}

/// Save a reusable user preset bundle.
///
/// Disk layout:
/// - `preset.toml` stores metadata plus ThemeConfig, without inline CSS
/// - `preset.css` stores Blogger-specific CSS
/// - `source.toml` stores import provenance and warnings
/// - `icons/*.svg` stores optional raw SVG icon assets
pub fn save_user_preset_bundle(
    bundle: &UserPresetBundle,
    icons: Option<&UserPresetIconAssets>,
) -> Result<UserPresetDiskReport, String> {
    let bundle_dir = user_preset_bundle_dir(&bundle.id);
    save_user_preset_bundle_to_dir(bundle, icons, &bundle_dir)
}

pub fn save_user_preset_bundle_to_dir(
    bundle: &UserPresetBundle,
    icons: Option<&UserPresetIconAssets>,
    bundle_dir: &Path,
) -> Result<UserPresetDiskReport, String> {
    let mut report = UserPresetDiskReport {
        bundle_dir: bundle_dir.to_path_buf(),
        ..UserPresetDiskReport::default()
    };

    fs::create_dir_all(bundle_dir)
        .map_err(|err| format!("could not create {}: {}", bundle_dir.display(), err))?;

    let mut preset_for_toml = bundle.clone();
    preset_for_toml.preset_css.clear();

    let preset_toml = toml::to_string_pretty(&preset_for_toml)
        .map_err(|err| format!("could not serialize preset.toml: {}", err))?;
    let preset_path = bundle_dir.join("preset.toml");
    fs::write(&preset_path, preset_toml)
        .map_err(|err| format!("could not write {}: {}", preset_path.display(), err))?;
    report.files_written.push(preset_path);

    let css_path = bundle_dir.join("preset.css");
    fs::write(&css_path, &bundle.preset_css)
        .map_err(|err| format!("could not write {}: {}", css_path.display(), err))?;
    report.files_written.push(css_path);

    let source_toml = toml::to_string_pretty(&bundle.source)
        .map_err(|err| format!("could not serialize source.toml: {}", err))?;
    let source_path = bundle_dir.join("source.toml");
    fs::write(&source_path, source_toml)
        .map_err(|err| format!("could not write {}: {}", source_path.display(), err))?;
    report.files_written.push(source_path);

    if let Some(icons) = icons {
        if !icons.is_empty() {
            let icons_dir = bundle_dir.join("icons");
            fs::create_dir_all(&icons_dir)
                .map_err(|err| format!("could not create {}: {}", icons_dir.display(), err))?;

            for (filename, svg) in icons.iter() {
                let path = icons_dir.join(filename);
                fs::write(&path, svg)
                    .map_err(|err| format!("could not write {}: {}", path.display(), err))?;
                report.files_written.push(path);
            }
        }
    }

    Ok(report)
}

/// Load a user preset bundle from a bundle directory.
///
/// This reads `preset.toml`, then attaches `preset.css` if present. Missing CSS
/// is non-fatal because some presets may be config-only.
pub fn load_user_preset_bundle(bundle_dir: &Path) -> Result<(UserPresetBundle, UserPresetDiskReport), String> {
    let mut report = UserPresetDiskReport {
        bundle_dir: bundle_dir.to_path_buf(),
        ..UserPresetDiskReport::default()
    };

    let preset_path = bundle_dir.join("preset.toml");
    let preset_toml = fs::read_to_string(&preset_path)
        .map_err(|err| format!("could not read {}: {}", preset_path.display(), err))?;
    report.files_read.push(preset_path);

    let mut bundle = toml::from_str::<UserPresetBundle>(&preset_toml)
        .map_err(|err| format!("could not parse preset.toml: {}", err))?;

    let css_path = bundle_dir.join("preset.css");
    match fs::read_to_string(&css_path) {
        Ok(css) => {
            bundle.preset_css = css;
            report.files_read.push(css_path);
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            report
                .warnings
                .push(format!("{} missing; using config-only preset", css_path.display()));
        }
        Err(err) => return Err(format!("could not read {}: {}", css_path.display(), err)),
    }

    let source_path = bundle_dir.join("source.toml");
    match fs::read_to_string(&source_path) {
        Ok(source_toml) => {
            match toml::from_str::<PresetSourceInfo>(&source_toml) {
                Ok(source) => bundle.source = source,
                Err(err) => report
                    .warnings
                    .push(format!("could not parse {}: {}", source_path.display(), err)),
            }
            report.files_read.push(source_path);
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(format!("could not read {}: {}", source_path.display(), err)),
    }

    Ok((bundle, report))
}

/// Load raw SVG files from `<preset>/icons/`.
pub fn load_user_preset_icon_assets(bundle_dir: &Path) -> Result<UserPresetIconAssets, String> {
    let icons_dir = bundle_dir.join("icons");

    let read_optional = |filename: &str| -> Result<Option<String>, String> {
        let path = icons_dir.join(filename);
        match fs::read_to_string(&path) {
            Ok(svg) => Ok(Some(svg)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(format!("could not read {}: {}", path.display(), err)),
        }
    };

    Ok(UserPresetIconAssets {
        sidebar_left_svg: read_optional(IconAssetNames::SIDEBAR_LEFT)?,
        sidebar_right_svg: read_optional(IconAssetNames::SIDEBAR_RIGHT)?,
        panel_close_svg: read_optional(IconAssetNames::PANEL_CLOSE)?,
        search_svg: read_optional(IconAssetNames::SEARCH)?,
        menu_svg: read_optional(IconAssetNames::MENU)?,
    })
}
