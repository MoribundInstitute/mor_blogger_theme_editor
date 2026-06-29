pub mod models;
pub mod persistence;
pub mod theme_resolver;

#[allow(unused_imports)]
pub use models::{
    CompendiumManifest, CustomEditorColors, EditorPrefs, LayoutPrefs, PluginState, ShortcutPrefs,
};
pub use theme_resolver::resolve_effective_theme;

#[allow(unused_imports)]
pub use crate::app::theme_hot_reload::{
    register_theme_reload_sender, spawn_editor_prefs_watcher, spawn_theme_presets_watcher,
};
