// src/ui/shell/theme.rs
// MOR TOML theme engine. Native OS Chameleon.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub const GTK4_DARK_TOML: &str = r##"
bg            = "#242424"
panel         = "#2d2d2d"
header        = "#1e1e1e"
text          = "#deddda"
text_muted    = "#77767b"
border        = "#171717"
border_light  = "#3d3d3d"
accent        = "#1c71d8"
accent_hover  = "#3584e4"
btn           = "#3d3d3d"
btn_hover     = "#4a4a4a"
font_family   = "Cantarell, system-ui, sans-serif"
font_size_base= "13px"
font_size_h1  = "20px"
padding_base  = "8px"
border_radius = "6px"
destructive   = "#ff5555"
success       = "#50fa7b"
warning       = "#f1fa8c"
"##;

pub const MAC_OS_LIGHT_TOML: &str = r##"
bg            = "#ececec"
panel         = "#f5f5f5"
header        = "#d8d8d8"
text          = "#1e1e1e"
text_muted    = "#7a7a7a"
border        = "#c8c8c8"
border_light  = "#e0e0e0"
accent        = "#007aff"
accent_hover  = "#005bb5"
btn           = "#ffffff"
btn_hover     = "#f0f0f0"
font_family   = "-apple-system, BlinkMacSystemFont, 'Helvetica Neue', sans-serif"
font_size_base= "13px"
font_size_h1  = "20px"
padding_base  = "8px"
border_radius = "8px"
destructive   = "#ff5555"
success       = "#50fa7b"
warning       = "#f1fa8c"
"##;

pub const WIN_11_DARK_TOML: &str = r##"
bg            = "#202020"
panel         = "#282828"
header        = "#181818"
text          = "#ffffff"
text_muted    = "#a0a0a0"
border        = "#333333"
border_light  = "#444444"
accent        = "#60cdff"
accent_hover  = "#3fb0e6"
btn           = "#2d2d2d"
btn_hover     = "#323232"
font_family   = "'Segoe UI Variable', 'Segoe UI', system-ui, sans-serif"
font_size_base= "13px"
font_size_h1  = "20px"
padding_base  = "8px"
border_radius = "4px"
destructive   = "#ff5555"
success       = "#50fa7b"
warning       = "#f1fa8c"
"##;

pub fn get_native_os_theme() -> &'static str {
    match std::env::consts::OS {
        "macos" => MAC_OS_LIGHT_TOML,
        "windows" => WIN_11_DARK_TOML,
        "linux" | _ => GTK4_DARK_TOML,
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MorTheme {
    pub bg: String,
    pub panel: String,
    pub header: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
    pub border_light: String,
    pub accent: String,
    pub accent_hover: String,
    pub btn: String,
    pub btn_hover: String,
    pub font_family: String,
    pub font_size_base: String,
    pub font_size_h1: String,
    pub padding_base: String,
    pub border_radius: String,
    pub destructive: String,
    pub success: String,
    pub warning: String,
    #[serde(default)]
    pub enable_image_borders: bool,
    #[serde(default)]
    pub custom_border_url: Option<String>,
    #[serde(default = "default_slice")]
    pub svg_border_slice: String,
    #[serde(default = "default_image_width")]
    pub image_border_width: String,
    #[serde(default = "default_true")]
    pub target_sidebars: bool,
    #[serde(default)]
    pub target_canvas: bool,
}

fn default_slice() -> String {
    "30".to_string()
}

fn default_image_width() -> String {
    "20px".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for MorTheme {
    fn default() -> Self {
        Self {
            bg: "#2c2c2c".to_string(),
            panel: "#2e2e2e".to_string(),
            header: "#252525".to_string(),
            text: "#e0e0e0".to_string(),
            text_muted: "#888888".to_string(),
            border: "#1f1f1f".to_string(),
            border_light: "#444444".to_string(),
            accent: "#4a6984".to_string(),
            accent_hover: "#4a90d9".to_string(),
            btn: "#3a3a3a".to_string(),
            btn_hover: "#4a4a4a".to_string(),
            font_family:
                "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif"
                    .to_string(),
            font_size_base: "13px".to_string(),
            font_size_h1: "20px".to_string(),
            padding_base: "8px".to_string(),
            border_radius: "3px".to_string(),
            destructive: "#ff5555".to_string(),
            success: "#50fa7b".to_string(),
            warning: "#f1fa8c".to_string(),
            enable_image_borders: false,
            custom_border_url: None,
            svg_border_slice: default_slice(),
            image_border_width: default_image_width(),
            target_sidebars: default_true(),
            target_canvas: false,
        }
    }
}

impl MorTheme {
    pub fn from_toml(src: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(src)
    }

    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_default()
    }

    pub fn to_css_vars(&self) -> String {
        let image_vars = if self.enable_image_borders && self.custom_border_url.is_some() {
            let url = self.custom_border_url.as_ref().unwrap();
            format!(
                "  --mor-border-image: url(\"{}\");\n  --mor-border-slice: {};\n  --mor-border-width-img: {};",
                url, self.svg_border_slice, self.image_border_width
            )
        } else {
            "  --mor-border-image: none;\n  --mor-border-slice: 0;\n  --mor-border-width-img: var(--panel-border-width);"
                .to_string()
        };
        format!(
            ":root {{\n  --mor-bg: {};\n  --mor-panel: {};\n  --mor-header: {};\n  --mor-text: {};\n  --mor-text-muted: {};\n  --mor-border: {};\n  --mor-border-light: {};\n  --mor-accent: {};\n  --mor-accent-hover: {};\n  --mor-btn: {};\n  --mor-btn-hover: {};\n  --mor-font: {};\n  --mor-font-size: {};\n  --mor-font-h1: {};\n  --mor-padding: {};\n  --mor-radius: {};\n  --mor-destructive: {};\n  --mor-success: {};\n  --mor-warning: {};\n{}\n}}",
            self.bg, self.panel, self.header, self.text, self.text_muted, self.border, self.border_light, self.accent, self.accent_hover, self.btn, self.btn_hover, self.font_family, self.font_size_base, self.font_size_h1, self.padding_base, self.border_radius, self.destructive, self.success, self.warning, image_vars
        )
    }
}

pub const MOR_CSS: &str = r#"
/* Base */
.mor-root {
  font-family: var(--mor-font);
  font-size: var(--mor-font-size);
  color: var(--mor-text);
  background-color: transparent;
}

/* ── Global App Menu Bar (OBS Style Default) ── */
.mor-menu-bar {
  display: flex;
  background-color: var(--mor-bg);
  border-bottom: 1px solid var(--mor-border);
  height: 30px;
  align-items: center;
  font-family: var(--mor-font);
  font-size: var(--mor-font-size);
  color: var(--mor-text);
  user-select: none;
  position: relative;
  z-index: 9999;
}

.mor-menu-item {
  position: relative;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 10px;
  cursor: default;
  color: var(--mor-text);
}

.mor-menu-item:hover {
  background-color: var(--mor-btn-hover);
  color: var(--mor-text);
}

.mor-menu-dropdown {
  display: none;
  position: absolute;
  top: 30px;
  left: 0;
  background-color: var(--mor-panel);
  border: 1px solid var(--mor-border);
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.65);
  min-width: 200px;
  padding: 4px 0;
}

.mor-menu-item:hover .mor-menu-dropdown {
  display: block;
}

/* Force standard mor-menu-item inside the dropdown to behave like an action list */
.mor-menu-dropdown .mor-menu-item {
  display: flex;
  width: 100%;
  text-align: left;
  background: transparent;
  border: none;
  color: var(--mor-text);
  padding: 6px 16px;
  font-size: var(--mor-font-size);
  cursor: pointer;
  height: auto;
  transition: none;
}

.mor-menu-dropdown .mor-menu-item:hover {
  background-color: var(--mor-accent-hover);
  color: #ffffff;
}

.mor-menu-divider, .mor-separator {
  height: 1px;
  background-color: var(--mor-border-light);
  margin: 4px 0;
}

/* ── Modals ── */
.mor-modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 200;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(0, 0, 0, 0.7);
}

.mor-modal {
  background-color: var(--mor-bg);
  color: var(--mor-text);
  border: 1px solid var(--mor-border-light);
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.7);
  border-radius: var(--mor-radius);
  overflow: hidden;
  min-width: 380px;
  max-width: 620px;
  font-family: var(--mor-font);
  /* Column layout so the body fills/scrolls when the modal is resized. */
  display: flex;
  flex-direction: column;
}

.mor-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--mor-padding) calc(var(--mor-padding) * 2);
  background-color: var(--mor-header);
  border-bottom: 1px solid var(--mor-border-light);
  font-size: var(--mor-font-size);
  font-weight: 500;
  letter-spacing: 0.3px;
  /* Drag handle for moving the dialog. */
  cursor: move;
  flex-shrink: 0;
  user-select: none;
}

.mor-modal-close {
  padding: 0 var(--mor-padding);
  font-size: 22px;
  line-height: 1;
  color: var(--mor-text-muted);
  cursor: pointer;
  transition: color 80ms ease;
}
.mor-modal-close:hover { color: white; }

.mor-modal-body {
  padding: calc(var(--mor-padding) * 2.5);
  font-size: var(--mor-font-size);
  line-height: 1.5;
  /* Fill the resizable modal and scroll internally instead of a fixed cap. */
  flex: 1 1 auto;
  min-height: 0;
  overflow-y: auto;
}

.mor-tabs {
  display: flex;
  border-bottom: 1px solid var(--mor-border-light);
  margin-bottom: calc(var(--mor-padding) * 2);
  font-size: var(--mor-font-size);
}

.mor-tab {
  padding: var(--mor-padding) calc(var(--mor-padding) * 2);
  cursor: pointer;
  color: var(--mor-text-muted);
  border-bottom: 2px solid transparent;
  transition: all 80ms ease;
  background: transparent;
  border-top: none; border-left: none; border-right: none;
}
.mor-tab:hover { color: var(--mor-text); }
.mor-tab.active {
  color: var(--mor-accent-hover);
  border-bottom-color: var(--mor-accent-hover);
  font-weight: 500;
}

.mor-btn {
  padding: 6px calc(var(--mor-padding) * 2);
  font-size: var(--mor-font-size);
  border-radius: var(--mor-radius);
  border: 1px solid var(--mor-border-light);
  background-color: var(--mor-btn);
  color: var(--mor-text);
  font-family: var(--mor-font);
  cursor: pointer;
  transition: background-color 80ms ease;
}
.mor-btn:hover { background-color: var(--mor-btn-hover); }
.mor-btn:active { background-color: var(--mor-bg); }

.mor-btn.primary, .mor-btn-primary {
  padding: 6px calc(var(--mor-padding) * 2);
  font-size: var(--mor-font-size);
  border-radius: var(--mor-radius);
  border: 1px solid var(--mor-accent-hover);
  background-color: var(--mor-accent-hover);
  color: white;
  font-family: var(--mor-font);
  cursor: pointer;
  transition: background-color 80ms ease;
}
.mor-btn.primary:hover, .mor-btn-primary:hover { background-color: var(--mor-accent); }
.mor-btn.primary:active, .mor-btn-primary:active { background-color: var(--mor-bg); }

.mor-btn-secondary {
  padding: 6px calc(var(--mor-padding) * 2);
  font-size: var(--mor-font-size);
  border-radius: var(--mor-radius);
  border: 1px solid var(--mor-border-light);
  background-color: var(--mor-bg);
  color: var(--mor-text-muted);
  font-family: var(--mor-font);
  cursor: pointer;
  transition: all 80ms ease;
}
.mor-btn-secondary:hover {
  background-color: var(--mor-btn);
  color: var(--mor-text);
}

.mor-btn-outline {
  padding: 6px calc(var(--mor-padding) * 2);
  font-size: var(--mor-font-size);
  border-radius: var(--mor-radius);
  border: 1px solid var(--mor-border);
  background-color: transparent;
  color: var(--mor-text);
  font-family: var(--mor-font);
  cursor: pointer;
  transition: all 80ms ease;
}
.mor-btn-outline:hover {
  background-color: var(--mor-btn-hover);
  border-color: var(--mor-accent-hover);
}


/* ── Forms ── */
.mor-checkbox-wrapper {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    font-size: var(--mor-font-size);
    color: var(--mor-text);
    user-select: none;
    margin: 6px 0;
    font-family: var(--mor-font);
}

.mor-checkbox {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    border: 1px solid var(--mor-border);
    border-radius: 3px;
    background: var(--mor-bg);
    cursor: pointer;
    position: relative;
    transition: all 0.1s ease;
    margin: 0;
}

.mor-checkbox:checked {
    background: var(--mor-accent-hover);
    border-color: var(--mor-accent-hover);
}

.mor-checkbox:checked::after {
    content: '';
    position: absolute;
    left: 4px;
    top: 1px;
    width: 4px;
    height: 8px;
    border: solid var(--mor-bg);
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
}

.mor-select-wrapper {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin: var(--mor-padding) 0;
    width: 100%;
    font-family: var(--mor-font);
}

.mor-select-label {
    font-size: calc(var(--mor-font-size) - 1px);
    color: var(--mor-text-muted);
    font-weight: 500;
}

.mor-select {
    -webkit-appearance: none;
    appearance: none;
    background: var(--mor-bg);
    color: var(--mor-text);
    border: 1px solid var(--mor-border);
    border-radius: var(--mor-radius);
    padding: 6px 28px 6px 10px;
    font-size: var(--mor-font-size);
    font-family: var(--mor-font);
    cursor: pointer;
    outline: none;
    transition: border-color 0.1s ease;
    background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%2377767b%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E");
    background-repeat: no-repeat;
    background-position: right 10px top 50%;
    background-size: 8px auto;
}

.mor-select:hover, .mor-select:focus-visible {
    border-color: var(--mor-accent-hover);
}

.mor-input-wrapper {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin: var(--mor-padding) 0;
    width: 100%;
}

.mor-input-label {
    font-size: calc(var(--mor-font-size) - 1px);
    color: var(--mor-text-muted);
    font-weight: 500;
}

.mor-input {
    background: var(--mor-bg);
    color: var(--mor-text);
    border: 1px solid var(--mor-border);
    border-radius: var(--mor-radius);
    padding: var(--mor-padding);
    font-size: var(--mor-font-size);
    font-family: var(--mor-font);
    outline: none;
    transition: border-color 0.1s ease;
}

.mor-input:focus-visible {
    border-color: var(--mor-accent-hover);
    box-shadow: 0 0 0 1px var(--mor-accent-hover);
}

/* ── MOR Slider ── */
.mor-slider-shell {
    display: flex;
    flex-direction: column;
    gap: 6px;
    width: 100%;
    user-select: none;
    font-family: var(--mor-font);
}

.mor-slider-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    font-size: calc(var(--mor-font-size) - 1px);
    color: var(--mor-text-muted);
}

.mor-slider-label {
    font-weight: 500;
    color: var(--mor-text);
}

.mor-slider-value {
    font-variant-numeric: tabular-nums;
    min-width: 3ch;
    text-align: right;
}

.mor-slider-input {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    border-radius: var(--mor-radius);
    background: var(--mor-bg);
    outline: none;
    cursor: pointer;
    transition: background 0.15s ease;
    margin: 4px 0;
}

.mor-slider-input:hover { background: var(--mor-btn); }
.mor-slider-input:focus-visible { outline: 1px solid var(--mor-accent-hover); outline-offset: 3px; }

.mor-slider-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--mor-accent-hover);
    border: 2px solid var(--mor-border);
    cursor: grab;
}

.mor-slider-input::-moz-range-thumb {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--mor-accent-hover);
    border: 2px solid var(--mor-border);
    cursor: grab;
}
"#;

#[component]
pub fn MorStyleProvider(
    #[props(default = GTK4_DARK_TOML.to_string())] theme_toml: String,
) -> Element {
    let base = MorTheme::from_toml(&theme_toml).unwrap_or_default();
    let prefs = crate::app::config_bridge::EditorPrefs::load();
    let theme =
        crate::app::config_bridge::resolve_effective_theme(base, &prefs.custom_editor_colors);
    let mut css_vars = theme.to_css_vars();
    if let Some(ref c) = prefs.custom_editor_colors.panel_title_color {
        let mut safe_css = css_vars.trim_end().to_string();
        if safe_css.ends_with("}") {
            safe_css.pop();
            safe_css.push_str(&format!("  --panel-title-color: {};\n}}", c));
            css_vars = safe_css;
        }
    }

    rsx! {
        style { "{css_vars}" }
        style { "{MOR_CSS}" }
    }
}
