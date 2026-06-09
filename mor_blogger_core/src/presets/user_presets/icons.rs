use serde::{Deserialize, Serialize};

/// Standard icon filenames inside a user preset bundle.
pub struct IconAssetNames;

impl IconAssetNames {
    pub const SIDEBAR_LEFT: &'static str = "sidebar-left.svg";
    pub const SIDEBAR_RIGHT: &'static str = "sidebar-right.svg";
    pub const PANEL_CLOSE: &'static str = "panel-close.svg";
    pub const SEARCH: &'static str = "search.svg";
    pub const MENU: &'static str = "menu.svg";
}

/// Optional raw SVG assets that can be written into `<preset>/icons/`.
///
/// These are raw SVG strings, not CSS `url(...)` data URIs. The runtime loader
/// can later convert them into `IconConfig` masks using the same helper used by
/// the GTK importer.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPresetIconAssets {
    #[serde(default)]
    pub sidebar_left_svg: Option<String>,

    #[serde(default)]
    pub sidebar_right_svg: Option<String>,

    #[serde(default)]
    pub panel_close_svg: Option<String>,

    #[serde(default)]
    pub search_svg: Option<String>,

    #[serde(default)]
    pub menu_svg: Option<String>,
}

impl UserPresetIconAssets {
    pub fn is_empty(&self) -> bool {
        self.sidebar_left_svg.is_none()
            && self.sidebar_right_svg.is_none()
            && self.panel_close_svg.is_none()
            && self.search_svg.is_none()
            && self.menu_svg.is_none()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &str)> {
        let mut items = Vec::new();

        if let Some(svg) = self.sidebar_left_svg.as_deref() {
            items.push((IconAssetNames::SIDEBAR_LEFT, svg));
        }
        if let Some(svg) = self.sidebar_right_svg.as_deref() {
            items.push((IconAssetNames::SIDEBAR_RIGHT, svg));
        }
        if let Some(svg) = self.panel_close_svg.as_deref() {
            items.push((IconAssetNames::PANEL_CLOSE, svg));
        }
        if let Some(svg) = self.search_svg.as_deref() {
            items.push((IconAssetNames::SEARCH, svg));
        }
        if let Some(svg) = self.menu_svg.as_deref() {
            items.push((IconAssetNames::MENU, svg));
        }

        items.into_iter()
    }
}
