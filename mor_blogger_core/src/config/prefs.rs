//! Headless view of the editor preferences file (`editor_prefs.toml`).
//!
//! The full, UI-facing `EditorPrefs` lives in the Dioxus crate. The core
//! renderer only needs to know which plugins are switched on so it can
//! assemble the exported theme, so this is a deliberately small subset.
//! Serde ignores any unknown fields, so this stays compatible even as the
//! UI side grows new preference keys.

use directories::ProjectDirs;
use serde::Deserialize;

pub fn editor_prefs_path() -> std::path::PathBuf {
    ProjectDirs::from("io", "Moribund", "MorBloggerThemeEditor")
        .unwrap()
        .config_dir()
        .join("editor_prefs.toml")
}

pub fn shortcuts_path() -> std::path::PathBuf {
    ProjectDirs::from("io", "Moribund", "MorBloggerThemeEditor")
        .unwrap()
        .config_dir()
        .join("shortcuts.toml")
}

pub fn layout_prefs_path() -> std::path::PathBuf {
    ProjectDirs::from("io", "Moribund", "MorBloggerThemeEditor")
        .unwrap()
        .config_dir()
        .join("layout_prefs.toml")
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RenderPrefs {
    #[serde(default)]
    pub plugins: Vec<PluginPref>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PluginPref {
    pub id: String,
    #[serde(default)]
    pub enabled: bool,
}
