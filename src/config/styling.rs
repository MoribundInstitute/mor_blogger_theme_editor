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
