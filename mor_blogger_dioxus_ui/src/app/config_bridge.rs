use mor_blogger_core::config::ThemeConfig;
use crate::ui::workspace::layout::PanelLayout;
use crate::ui::shell::theme::MorTheme;
use serde::{Deserialize, Serialize};

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
}

pub fn resolve_effective_theme(base: MorTheme, overrides: &CustomEditorColors) -> MorTheme {
    macro_rules! apply {
        ($field:ident) => {
            overrides.$field.clone().unwrap_or(base.$field)
        };
    }
    MorTheme {
        bg: apply!(bg),
        panel: apply!(panel),
        header: apply!(header),
        text: apply!(text),
        text_muted: apply!(text_muted),
        border: apply!(border),
        border_light: apply!(border_light),
        accent: apply!(accent),
        accent_hover: apply!(accent_hover),
        btn: apply!(btn),
        btn_hover: apply!(btn_hover),
        font_family: apply!(font_family),
        font_size_base: apply!(font_size_base),
        font_size_h1: apply!(font_size_h1),
        padding_base: apply!(padding_base),
        border_radius: apply!(border_radius),
        destructive: apply!(destructive),
        success: apply!(success),
        warning: apply!(warning),
    }
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
}

impl EditorPrefs {
    pub fn load() -> Self {
        let path = mor_blogger_core::config::prefs::editor_prefs_path();
        if !path.exists() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::copy("editor_prefs.json", &path);
        }
        let Ok(json) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&json).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = mor_blogger_core::config::prefs::editor_prefs_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        std::fs::write(&path, json)
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

    pub fn update_custom_btn(btn: String) {
        let mut prefs = Self::load();
        prefs.custom_editor_colors.btn = Some(btn);
        let _ = prefs.save();
    }
}

pub fn menu_label(config: &ThemeConfig, index: usize) -> String {
    config.menu_links.get(index).map(|link| link.label.clone()).unwrap_or_default()
}

pub fn menu_url(config: &ThemeConfig, index: usize) -> String {
    config.menu_links.get(index).map(|link| link.url.clone()).unwrap_or_default()
}

pub fn panel_layout_class(layout: PanelLayout) -> &'static str {
    match layout {
        PanelLayout::Split => "split",
        PanelLayout::Wide => "wide",
        PanelLayout::Floating => "floating",
        PanelLayout::Hidden => "hidden",
    }
}