use mor_blogger_core::config::ThemeConfig;
use crate::ui::workspace::layout::PanelLayout;
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorPrefs {
    #[serde(default)]
    pub plugins: Vec<PluginState>,
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