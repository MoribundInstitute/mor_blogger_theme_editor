use super::models::{EditorPrefs, LayoutPrefs, ShortcutPrefs};
use std::fs;
use std::path::Path;

fn load_toml_generic<T: serde::de::DeserializeOwned + Default>(path: &Path) -> T {
    let Ok(toml_str) = fs::read_to_string(path) else {
        return T::default();
    };
    toml::from_str(&toml_str).unwrap_or_default()
}

fn save_toml_generic<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let toml_str = toml::to_string_pretty(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    fs::write(path, toml_str)
}

impl ShortcutPrefs {
    pub fn load() -> Self {
        let path = mor_blogger_core::config::prefs::shortcuts_path();
        if !path.exists() {
            let default_prefs = Self::default();
            let _ = default_prefs.save();
            default_prefs
        } else {
            load_toml_generic(&path)
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = mor_blogger_core::config::prefs::shortcuts_path();
        save_toml_generic(&path, self)
    }
}

impl LayoutPrefs {
    pub fn load() -> Self {
        let path = mor_blogger_core::config::prefs::layout_prefs_path();
        if path.exists() {
            return load_toml_generic(&path);
        }

        let legacy_pins = EditorPrefs::load().pinned_docks;
        let prefs = Self {
            pinned_docks: legacy_pins,
            quick_launch_hidden: Vec::new(),
            activity_icons: std::collections::HashMap::new(),
        };
        let _ = prefs.save();
        prefs
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = mor_blogger_core::config::prefs::layout_prefs_path();
        save_toml_generic(&path, self)
    }
}

impl EditorPrefs {
    pub fn load() -> Self {
        let path = mor_blogger_core::config::prefs::editor_prefs_path();
        load_toml_generic(&path)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = mor_blogger_core::config::prefs::editor_prefs_path();
        save_toml_generic(&path, self)
    }

    pub fn update_ui_mode(mode: String) {
        let mut prefs = Self::load();
        prefs.ui_mode = Some(mode);
        let _ = prefs.save();
    }

    pub fn update_workspace_theme(theme: String) {
        let mut prefs = Self::load();
        prefs.workspace_theme = Some(theme);
        let _ = prefs.save();
    }

    pub fn update_minimap(show: bool) {
        let mut prefs = Self::load();
        prefs.show_minimap = Some(show);
        let _ = prefs.save();
    }

    pub fn update_minimap_override(key: String, show: bool) {
        let mut prefs = Self::load();
        prefs.minimap_overrides.insert(key, show);
        let _ = prefs.save();
    }

    pub fn update_wrap_override(key: String, wrap: bool) {
        let mut prefs = Self::load();
        prefs.wrap_overrides.insert(key, wrap);
        let _ = prefs.save();
    }

    pub fn update_default_template_pack(pack: mor_blogger_core::config::TemplatePackConfig) {
        let mut prefs = Self::load();
        prefs.default_template_pack = Some(pack);
        let _ = prefs.save();
    }

    pub fn clear_default_template_pack() {
        let mut prefs = Self::load();
        prefs.default_template_pack = None;
        let _ = prefs.save();
    }
}
