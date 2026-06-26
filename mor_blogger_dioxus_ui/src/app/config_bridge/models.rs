use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginState {
    pub id: String,
    pub enabled: bool,
    #[serde(default = "default_plugin_version")]
    pub version: String,
}

fn default_plugin_version() -> String {
    "1.0.0".to_string()
}

/// The blueprint for the data fetched from your decentralized Blogger/GitHub compendium.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompendiumManifest {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub payload_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CustomEditorColors {
    pub bg: Option<String>,
    pub panel: Option<String>,
    pub header: Option<String>,
    pub text: Option<String>,
    pub text_muted: Option<String>,
    pub border: Option<String>,
    pub border_light: Option<String>,
    pub accent: Option<String>,
    pub accent_hover: Option<String>,
    pub btn: Option<String>,
    pub btn_hover: Option<String>,
    pub font_family: Option<String>,
    pub font_size_base: Option<String>,
    pub font_size_h1: Option<String>,
    pub padding_base: Option<String>,
    pub border_radius: Option<String>,
    pub destructive: Option<String>,
    pub success: Option<String>,
    pub warning: Option<String>,
    #[serde(default)]
    pub panel_title_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutPrefs {
    pub undo: Option<String>,
    pub redo: Option<String>,
    pub copy_raw_xml: Option<String>,
    pub toggle_left_dock: Option<String>,
    pub toggle_right_dock: Option<String>,
    #[serde(default)]
    pub user_prefs: Option<String>,
    #[serde(default)]
    pub theme_diagnostics: Option<String>,
    #[serde(default)]
    pub toggle_preview: Option<String>,
    #[serde(default)]
    pub exit_architect: Option<String>,
    #[serde(default)]
    pub open_project: Option<String>,
    #[serde(default)]
    pub save_project: Option<String>,
    #[serde(default)]
    pub export_xml: Option<String>,
    #[serde(default)]
    pub reset_zoom: Option<String>,
}

impl Default for ShortcutPrefs {
    fn default() -> Self {
        Self {
            undo: Some("Ctrl+Z".to_string()),
            redo: Some("Ctrl+Y".to_string()),
            copy_raw_xml: Some("Ctrl+C".to_string()),
            toggle_left_dock: Some("Ctrl+B".to_string()),
            toggle_right_dock: Some("Ctrl+E".to_string()),
            user_prefs: Some("Ctrl+P".to_string()),
            theme_diagnostics: Some("Ctrl+D".to_string()),
            toggle_preview: Some("F9".to_string()),
            exit_architect: Some("Ctrl+Q".to_string()),
            open_project: Some("Ctrl+O".to_string()),
            save_project: Some("Ctrl+S".to_string()),
            export_xml: Some("Shift+Ctrl+E".to_string()),
            reset_zoom: Some("Ctrl+0".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutPrefs {
    /// Dock IDs pinned to the activity bar, in display order.
    #[serde(default)]
    pub pinned_docks: Vec<String>,
    /// Dock IDs hidden from the activity / quick-launch bar (e.g. "js_editor").
    #[serde(default)]
    pub quick_launch_hidden: Vec<String>,
    /// Per-dock activity-bar icon overrides, keyed by dock ID. Value is a tagged
    /// string: "emoji:🎨", "svg:palette", or "raw:<svg…>". Absent = built-in default.
    #[serde(default)]
    pub activity_icons: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorPrefs {
    #[serde(default)]
    pub plugins: Vec<PluginState>,
    #[serde(default)]
    pub ui_mode: Option<String>,
    #[serde(default)]
    pub workspace_theme: Option<String>,
    #[serde(default)]
    pub workspace_theme_preset: Option<String>,
    #[serde(default)]
    pub custom_editor_colors: CustomEditorColors,
    #[serde(default)]
    pub default_template_pack: Option<mor_blogger_core::config::TemplatePackConfig>,
    #[serde(default)]
    pub pinned_docks: Vec<String>,
    /// VS Code-style code minimap: global default for editors with no per-workspace
    /// override. Unset = on.
    #[serde(default)]
    pub show_minimap: Option<bool>,
    /// Per-workspace minimap overrides, keyed by editor/workspace id. Takes
    /// precedence over `show_minimap` for that editor.
    #[serde(default)]
    pub minimap_overrides: HashMap<String, bool>,
}
