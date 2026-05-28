use serde::{Deserialize, Serialize};

use crate::config::ThemeConfig;

/// Increment this when the on-disk user preset bundle shape changes.
pub const USER_PRESET_BUNDLE_VERSION: u32 = 1;

/// A reusable preset saved outside the Rust source tree.
///
/// Built-in presets still live in `src/presets/*.rs`.
/// User/imported presets should live under something like:
/// `~/.config/mor_blogger_theme_editor/presets/<id>/`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserPresetBundle {
    /// Bundle schema version, not app version.
    #[serde(default = "default_bundle_version")]
    pub version: u32,

    /// Stable filesystem-safe id, for example `whitesur-dark-solid-nord`.
    pub id: String,

    /// Human-facing display name, for example `WhiteSur Dark Solid Nord`.
    pub name: String,

    /// Short preset-card description.
    #[serde(default)]
    pub description: String,

    /// Full editor/theme configuration generated from the import.
    pub config: ThemeConfig,

    /// Blogger-specific CSS overrides saved as `preset.css` on disk.
    ///
    /// This field is duplicated in memory for convenience. On disk, the CSS is
    /// intentionally stored as a separate file so users can edit it directly.
    #[serde(default)]
    pub preset_css: String,

    /// Where this preset came from and what happened during import.
    #[serde(default)]
    pub source: PresetSourceInfo,
}

impl UserPresetBundle {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        config: ThemeConfig,
        preset_css: impl Into<String>,
        source: PresetSourceInfo,
    ) -> Self {
        Self {
            version: USER_PRESET_BUNDLE_VERSION,
            id: id.into(),
            name: name.into(),
            description: description.into(),
            config,
            preset_css: preset_css.into(),
            source,
        }
    }
}

/// Metadata for `source.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PresetSourceInfo {
    /// `gtk4`, `manual`, `json-import`, etc.
    #[serde(default = "default_source_kind")]
    pub source_kind: String,

    /// Original source path, if one exists and should be remembered.
    #[serde(default)]
    pub source_path: Option<String>,

    /// Simple timestamp string. Keep this dependency-free for now.
    #[serde(default)]
    pub imported_at: Option<String>,

    /// Version tag for the importer/converter logic.
    #[serde(default = "default_importer_version")]
    pub importer_version: String,

    /// Non-fatal issues from import/conversion.
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl Default for PresetSourceInfo {
    fn default() -> Self {
        Self {
            source_kind: default_source_kind(),
            source_path: None,
            imported_at: None,
            importer_version: default_importer_version(),
            warnings: Vec::new(),
        }
    }
}

impl PresetSourceInfo {
    pub fn gtk4(source_path: impl Into<String>, warnings: Vec<String>) -> Self {
        Self {
            source_kind: "gtk4".to_string(),
            source_path: Some(source_path.into()),
            imported_at: Some(current_unix_timestamp_string()),
            importer_version: default_importer_version(),
            warnings,
        }
    }
}

fn default_bundle_version() -> u32 {
    USER_PRESET_BUNDLE_VERSION
}

fn default_source_kind() -> String {
    "manual".to_string()
}

fn default_importer_version() -> String {
    "gtk-importer-v1".to_string()
}

fn current_unix_timestamp_string() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
