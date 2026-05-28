use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorConfig {
    pub bg_base: String,
    pub bg_panel: SurfaceFill,
    pub bg_elevated: SurfaceFill,
    pub fg_base: String,
    pub fg_muted: String,
    pub accent: String,
    pub border: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            bg_base: "#222129".to_string(),
            bg_panel: SurfaceFill::solid("#2b2933"),
            bg_elevated: SurfaceFill::solid("#343140"),
            fg_base: "#f2eadf".to_string(),
            fg_muted: "#bc8d6b".to_string(),
            accent: "#a9aae2".to_string(),
            border: "#6f6078".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SurfaceFill {
    pub mode: SurfaceMode,
    pub color: String,
    pub gradient_from: String,
    pub gradient_to: String,
    pub gradient_angle_deg: u16,
}

impl Default for SurfaceFill {
    fn default() -> Self {
        Self::solid("#2b2933")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SurfaceMode {
    #[default]
    Solid,
    Gradient,
}

impl SurfaceFill {
    pub fn solid(color: impl Into<String>) -> Self {
        let c = color.into();
        Self {
            mode: SurfaceMode::Solid,
            gradient_from: c.clone(),
            gradient_to: c.clone(),
            gradient_angle_deg: 180,
            color: c,
        }
    }

    pub fn to_css(&self) -> String {
        match self.mode {
            SurfaceMode::Solid => self.color.clone(),
            SurfaceMode::Gradient => format!(
                "linear-gradient({}deg, {}, {})",
                self.gradient_angle_deg, self.gradient_from, self.gradient_to
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct AssetConfig {
    pub favicon_url: String,
    pub social_card_image_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BackgroundMode {
    Solid {
        color: String,
    },
    Gradient {
        from: String,
        to: String,
        angle_deg: u16,
    },
    Tile {
        url: String,
    },
}

impl Default for BackgroundMode {
    fn default() -> Self {
        Self::Solid {
            color: "#222129".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct BackgroundConfig {
    pub mode: BackgroundMode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ButtonConfig {
    pub radius: String,
    pub border_width: String,
    pub text_transform: String,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TypographyConfig {
    pub body_font_stack: String,
    pub heading_font_stack: String,
    pub mono_font_stack: String,
    pub base_size: String,
    pub scale_ratio: String,
    pub line_height: String,
    pub heading_weight: String,
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            body_font_stack: "serif".to_string(),
            heading_font_stack: "serif".to_string(),
            mono_font_stack: "monospace".to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.2".to_string(),
            line_height: "1.6".to_string(),
            heading_weight: "700".to_string(),
        }
    }
}
/// CSS mask icon — stored as a ready-to-embed data URI string.
/// e.g. "url(\"data:image/svg+xml,...\")"
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IconConfig {
    /// sidebar-left open button (panel-open-symbolic equivalent)
    pub sidebar_left: String,
    /// sidebar-right open button
    pub sidebar_right: String,
    /// panel close button (window-close-symbolic equivalent)
    pub panel_close: String,
    /// search icon
    pub search: String,
    /// catalog / grid menu trigger
    pub menu: String,
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            sidebar_left: svg_mask(ICON_SIDEBAR_LEFT_PATH),
            sidebar_right: svg_mask(ICON_SIDEBAR_RIGHT_PATH),
            panel_close: svg_mask(ICON_CLOSE_PATH),
            search: svg_mask(ICON_SEARCH_PATH),
            menu: svg_mask(ICON_MENU_PATH),
        }
    }
}

// The same path data you already have hardcoded, now as named constants:
const ICON_SIDEBAR_LEFT_PATH: &str = "M3 3h18v18H3V3zm16 16V5H9v14h10z";
const ICON_SIDEBAR_RIGHT_PATH: &str = "M3 3h18v18H3V3zm2 16V5h10v14H5z";
const ICON_CLOSE_PATH:         &str = "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z";
const ICON_SEARCH_PATH:        &str = "M15.5 14h-.79l-.28-.27A6.47 6.47 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z";
const ICON_MENU_PATH: &str = "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z";

/// Build a CSS-ready data-URI mask value from a 24×24 viewBox path.
pub fn svg_mask(path_d: &str) -> String {
    let encoded = path_d
        .replace('"', "%22")
        .replace('#', "%23")
        .replace(' ', "%20");
    format!(
        "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath d='{}'/%3E%3C/svg%3E\")",
        encoded
    )
}
