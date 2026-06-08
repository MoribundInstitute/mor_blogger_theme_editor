use crate::config::ThemeConfig;
use crate::ui::workspace::layout::PanelLayout;
use serde::{Deserialize, Serialize};

/// A straightforward record to remember if a specific plugin is turned on or off.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginState {
    pub id: String,
    pub enabled: bool,
}

/// Application preferences state, including active plugins.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorPrefs {
    /// A list of plugins and their current on/off status.
    /// The #[serde(default)] tag ensures that if a user loads an older save file 
    /// from before the Plugin Manager existed, the app will safely load an empty list 
    /// instead of crashing.
    #[serde(default)]
    pub plugins: Vec<PluginState>,
}

pub fn menu_label(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|link| link.label.clone())
        .unwrap_or_default()
}

pub fn menu_url(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|link| link.url.clone())
        .unwrap_or_default()
}

pub fn panel_layout_class(layout: PanelLayout) -> &'static str {
    match layout {
        PanelLayout::Split => "split",
        PanelLayout::Wide => "wide",
        PanelLayout::Floating => "floating",
        PanelLayout::Hidden => "hidden",
    }
}