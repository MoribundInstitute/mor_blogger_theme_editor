use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub glow_color: String,

    pub panel_border_width: String,
    pub glow_spread: String,
    pub hover_scale: String,
    /// Number of stacked glow layers (1–4). More layers = deeper neon bloom.
    pub glow_intensity: String,
    /// Glow trigger. `true` (default) = enabled targets glow only on
    /// hover/focus; `false` = glow always on, with a brighter hover bloom.
    pub glow_hover: bool,

    pub panel_border_image_url: String,
    pub panel_border_image_slice: String,
    pub panel_border_image_repeat: String,

    pub glow_text: bool,
    pub glow_containers: bool,
    pub glow_icons: bool,
    pub glow_logo: bool,
    pub glow_title: bool,
    pub glow_toc: bool,
    pub glow_sidebar: bool,
    pub glow_footer: bool,
    pub glow_header: bool,
    pub glow_main: bool,
    pub glow_logo_color: String,
    pub glow_title_color: String,
    pub glow_toc_color: String,
    pub glow_sidebar_color: String,
    pub glow_text_color: String,
    pub glow_containers_color: String,
    pub glow_icons_color: String,
    pub glow_footer_color: String,
    pub glow_header_color: String,
    pub glow_main_color: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        let default_bg = BackgroundMode::default().to_surface_fill();
        Self {
            bg_base: "#222129".to_string(),
            bg_panel: default_bg.clone(),
            bg_elevated: default_bg,
            fg_base: "#f2eadf".to_string(),
            fg_muted: "#bc8d6b".to_string(),
            accent: "#a9aae2".to_string(),
            border: "#6f6078".to_string(),
            glow_color: String::new(),
            panel_border_width: "1px".to_string(),
            glow_spread: "10px".to_string(),
            hover_scale: "1.02".to_string(),
            glow_intensity: "2".to_string(),
            glow_hover: true,
            panel_border_image_url: String::new(),
            panel_border_image_slice: "30%".to_string(),
            panel_border_image_repeat: "stretch".to_string(),
            glow_text: false,
            glow_containers: false,
            glow_icons: false,
            glow_logo: false,
            glow_title: false,
            glow_toc: false,
            glow_sidebar: false,
            glow_footer: false,
            glow_header: false,
            glow_main: false,
            glow_logo_color: String::new(),
            glow_title_color: String::new(),
            glow_toc_color: String::new(),
            glow_sidebar_color: String::new(),
            glow_text_color: String::new(),
            glow_containers_color: String::new(),
            glow_icons_color: String::new(),
            glow_footer_color: String::new(),
            glow_header_color: String::new(),
            glow_main_color: String::new(),
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
        Self::solid("#222129")
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

impl ColorConfig {
    /// Generate the contrasting (light<->dark) variant dynamically.
    /// Swaps bg_base / bg_panel (as solid) with fg_base / fg_muted values.
    /// Preserves accent, border, and all scalar/asset fields exactly.
    /// Used when an active TOML preset lacks an explicit [light.colors] / [dark.colors] block for the requested mode.
    pub fn inverted_contrast(&self) -> Self {
        let mut c = self.clone();

        let orig_bg_base = self.bg_base.clone();
        let orig_bg_panel_color = self.bg_panel.color.clone();
        let orig_fg_base = self.fg_base.clone();
        let orig_fg_muted = self.fg_muted.clone();

        // Swap roles: former fg tones become the new bg tones (as solid fills)
        // former bg tones become the new fg tones. This avoids raw RGB invert garbage.
        c.bg_base = orig_fg_base.clone();
        c.bg_panel = SurfaceFill::solid(orig_fg_muted.clone());
        c.bg_elevated = SurfaceFill::solid(orig_fg_base.clone());

        c.fg_base = orig_bg_base.clone();
        c.fg_muted = orig_bg_panel_color.clone();

        // accent, border, widths, glow, hover_scale, border-image etc. are left as cloned (preserved)
        c
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
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

impl BackgroundMode {
    /// Converts the main workspace background mode into an equivalent SurfaceFill
    /// so that sidebars (and other panels) can inherit the exact same gradient/solid
    /// by default for visual matching with the main workspace area.
    pub fn to_surface_fill(&self) -> SurfaceFill {
        match self {
            BackgroundMode::Solid { color } => SurfaceFill::solid(color.clone()),
            BackgroundMode::Gradient {
                from,
                to,
                angle_deg,
            } => SurfaceFill {
                mode: SurfaceMode::Gradient,
                color: from.clone(),
                gradient_from: from.clone(),
                gradient_to: to.clone(),
                gradient_angle_deg: *angle_deg,
            },
            BackgroundMode::Tile { .. } => SurfaceFill::solid("#0a0c18"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BackgroundConfig {
    pub mode: BackgroundMode,
}

impl BackgroundConfig {
    pub fn inverted_contrast(&self) -> Self {
        Self {
            mode: self.mode.inverted_contrast(),
        }
    }
}

impl BackgroundMode {
    pub fn inverted_contrast(&self) -> Self {
        match self {
            Self::Gradient { from, to, .. } => {
                // Failure 1 Fix: Detect the default dark purple workspace gradient
                // and swap it for the default light blue gradient.
                if from == "#1e1a4d" && to == "#5b2c8a" {
                    return Self::Gradient {
                        from: "#e8efff".to_string(),
                        to: "#cdd8f5".to_string(),
                        angle_deg: 135,
                    };
                }
                // Detect the default light blue and swap back to dark purple
                if from == "#e8efff" && to == "#cdd8f5" {
                    return Self::Gradient {
                        from: "#1e1a4d".to_string(),
                        to: "#5b2c8a".to_string(),
                        angle_deg: 135,
                    };
                }
                self.clone()
            }
            Self::Solid { color: _ } => self.clone(),
            _ => self.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ButtonConfig {
    // Geometry
    pub radius: String,
    pub border_width: String,
    pub border_style: String, // solid | dashed
    pub padding_x: String,
    pub padding_y: String,
    pub full_width: bool,
    // Type
    pub text_transform: String,
    pub font_size: String,    // "" = inherit
    pub font_weight: String,  // "" = inherit
    pub letter_spacing: String,
    // Appearance
    pub fill: String,         // outline | solid | soft | ghost | glass | neon | glossy | gradient
    pub elevation: String,    // flat | subtle | raised
    // Effect parameters (used by glass/neon fills and the glow hover)
    pub glow_color: String,   // "" = derive from accent
    pub glow_strength: String, // glow/neon spread, e.g. "12px"
    pub glass_blur: String,   // backdrop blur for glass, e.g. "12px"
    pub glass_opacity: String, // glass surface alpha 0..1, e.g. "0.4"
    // Gradient fill (fill = "gradient"). Values may be raw colors or CSS
    // expressions like color-mix(...)/var(...).
    pub gradient_from: String,
    pub gradient_to: String,
    pub gradient_angle: String, // e.g. "180deg"
    // Raw box-shadow override ("" = derive from elevation). Escape hatch for
    // bevels/insets the elevation presets can't express.
    pub box_shadow: String,
    // Color overrides ("" = derive from theme)
    pub bg_color: String,
    pub text_color: String,
    pub border_color: String,
    pub hover_bg_color: String,
    // Interaction & motion
    pub hover_effect: String, // none | lift | grow | brighten | glow
    pub transition_ms: String,
    /// CSS transition-timing-function. "" or "ease" = browser default; accepts any
    /// keyword or cubic-bezier(...). Curated named curves are offered in the UI.
    pub easing: String,
    pub pressed_feedback: bool,
    // Focus ring ("" color = derive)
    pub focus_ring_color: String,
    pub focus_ring_width: String,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        // Defaults reproduce the historical neutral bordered button so existing
        // themes don't shift until a field is deliberately changed.
        Self {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            border_style: "solid".to_string(),
            padding_x: "8px".to_string(),
            padding_y: "4px".to_string(),
            full_width: false,
            text_transform: "none".to_string(),
            font_size: String::new(),
            font_weight: String::new(),
            letter_spacing: String::new(),
            fill: "outline".to_string(),
            elevation: "flat".to_string(),
            glow_color: String::new(),
            glow_strength: "12px".to_string(),
            glass_blur: "12px".to_string(),
            glass_opacity: "0.4".to_string(),
            gradient_from: String::new(),
            gradient_to: String::new(),
            gradient_angle: "180deg".to_string(),
            box_shadow: String::new(),
            bg_color: String::new(),
            text_color: String::new(),
            border_color: String::new(),
            hover_bg_color: String::new(),
            hover_effect: "none".to_string(),
            transition_ms: "0ms".to_string(),
            easing: "ease".to_string(),
            pressed_feedback: false,
            focus_ring_color: String::new(),
            focus_ring_width: "2px".to_string(),
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
    /// Per-element overrides (LibreOffice-style). Empty by default; each entry
    /// targets a fixed selector ("h1"|"h2"|"h3"|"p"|"blockquote"|"code") and
    /// only emits CSS for its non-empty properties. See
    /// [`crate::render::css_builder::build_element_typography_css`].
    pub elements: Vec<ElementStyle>,
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
            elements: Vec::new(),
        }
    }
}

/// Per-element typography override. An empty string means "inherit / leave the
/// base stylesheet's value". `selector` is one of the fixed element keys the
/// Typography panel offers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ElementStyle {
    pub selector: String,
    pub font_size: String,
    pub font_weight: String,
    pub line_height: String,
    pub letter_spacing: String,
    pub italic: bool,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IconConfig {
    pub sidebar_left: String,
    pub sidebar_right: String,
    pub panel_close: String,
    pub search: String,
    pub menu: String,

    // Standard blog action icons (expanded library, first-class fields)
    #[serde(default = "default_archive_icon")]
    pub archive: String,
    #[serde(default = "default_label_icon")]
    pub label: String,
    #[serde(default = "default_toc_icon")]
    pub toc: String,
    #[serde(default = "default_share_icon")]
    pub share: String,
    #[serde(default = "default_user_icon")]
    pub user: String,
    #[serde(default = "default_comment_icon")]
    pub comment: String,
    #[serde(default = "default_arrow_up_icon")]
    pub arrow_up: String,
    #[serde(default = "default_external_link_icon")]
    pub external_link: String,

    /// Arbitrary SVG mask icons keyed by name.
    pub custom_icons: HashMap<String, String>,

    /// Per-slot render mode keyed by slot field name (e.g. "panel_close").
    /// Absent or `true` = recolor to theme (mask). `false` = show the icon
    /// as-is in full colour (background image, no tint) — for colour emoji,
    /// multicolour SVGs, logos, etc.
    #[serde(default)]
    pub recolor: HashMap<String, bool>,
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            sidebar_left: svg_mask(ICON_SIDEBAR_LEFT_PATH),
            sidebar_right: svg_mask(ICON_SIDEBAR_RIGHT_PATH),
            panel_close: svg_mask(ICON_CLOSE_PATH),
            search: svg_mask(ICON_SEARCH_PATH),
            menu: svg_mask(ICON_MENU_PATH),
            // defaults for new standard blog action icons
            archive: svg_mask(ICON_ARCHIVE_PATH),
            label: svg_mask(ICON_LABEL_PATH),
            toc: default_toc_icon(),
            share: svg_mask(ICON_SHARE_PATH),
            user: svg_mask(ICON_USER_PATH),
            comment: svg_mask(ICON_COMMENT_PATH),
            arrow_up: svg_mask(ICON_ARROW_UP_PATH),
            external_link: svg_mask(ICON_EXTERNAL_LINK_PATH),
            custom_icons: HashMap::new(),
            recolor: HashMap::new(),
        }
    }
}

const ICON_SIDEBAR_LEFT_PATH: &str = "M9 4v16M6 8h.01M6 12h.01 M3 4h18v16H3z";

const ICON_SIDEBAR_RIGHT_PATH: &str = "M15 4v16M18 8h.01M18 12h.01 M3 4h18v16H3z";

const ICON_CLOSE_PATH: &str =
    "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z";

const ICON_SEARCH_PATH: &str = "M15.5 10.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0Z M21 21l-5.5-5.5";

const ICON_MENU_PATH: &str = "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z";

// New standard blog action icon paths (minimal Lucide/Feather style)
const ICON_ARCHIVE_PATH: &str = "M5 8v12h14V8 M10 12h4 M3 4h18v4H3z";
const ICON_LABEL_PATH: &str =
    "M20 13 12 21 3 12V3h9l8 8z M7.5 7.5A1.5 1.5 0 107.5 4a1.5 1.5 0 000 3.5z";
const ICON_TOC_PATH: &str = "M8 6h13 M8 12h13 M8 18h13 M3 6h.01 M3 12h.01 M3 18h.01";
const ICON_SHARE_PATH: &str = "M4 12v8a2 2 0 002 2h12a2 2 0 002-2v-8 M16 6l-4-4-4 4 M12 2v13";
const ICON_USER_PATH: &str =
    "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2 M12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8";
const ICON_COMMENT_PATH: &str = "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z";
const ICON_ARROW_UP_PATH: &str = "M12 19V5 M5 12l7-7 7 7";
const ICON_EXTERNAL_LINK_PATH: &str =
    "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6 M15 3h6v6 M10 14L21 3";

fn default_archive_icon() -> String {
    svg_mask(ICON_ARCHIVE_PATH)
}
fn default_label_icon() -> String {
    svg_mask(ICON_LABEL_PATH)
}
fn default_toc_icon() -> String {
    svg_mask(ICON_TOC_PATH)
}
fn default_share_icon() -> String {
    svg_mask(ICON_SHARE_PATH)
}
fn default_user_icon() -> String {
    svg_mask(ICON_USER_PATH)
}
fn default_comment_icon() -> String {
    svg_mask(ICON_COMMENT_PATH)
}
fn default_arrow_up_icon() -> String {
    svg_mask(ICON_ARROW_UP_PATH)
}
fn default_external_link_icon() -> String {
    svg_mask(ICON_EXTERNAL_LINK_PATH)
}

pub fn svg_mask(path_d: &str) -> String {
    let encoded = path_d
        .replace('"', "%22")
        .replace('#', "%23")
        .replace(' ', "%20");

    format!(
        r#"url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='{}'/%3E%3C/svg%3E")"#,
        encoded
    )
}
