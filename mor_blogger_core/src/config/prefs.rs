//! Headless view of the editor preferences file (`editor_prefs.json`).
//!
//! The full, UI-facing `EditorPrefs` lives in the Dioxus crate. The core
//! renderer only needs to know which plugins are switched on so it can
//! assemble the exported theme, so this is a deliberately small subset.
//! Serde ignores any unknown fields, so this stays compatible even as the
//! UI side grows new preference keys.

use serde::Deserialize;

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
