//! CSS Builder module for sanitizing and concatenating base CSS files.

use crate::config::{BackgroundMode, ThemeConfig};
use crate::render::util::escape_attr;
use crate::utils::svg_icons::svg_to_data_uri;

fn generate_workspace_background_value(bg: &BackgroundMode) -> String {
    match bg {
        BackgroundMode::Solid { color } => escape_attr(color),
        BackgroundMode::Gradient { from, to, angle_deg } => format!(
            "linear-gradient({}deg, {}, {})",
            angle_deg, escape_attr(from), escape_attr(to)
        ),
        BackgroundMode::Tile { url } if url.trim().is_empty() => "none".to_string(),
        BackgroundMode::Tile { url } => format!("url('{}')", escape_attr(url)),
    }
}

/// Cleans user-uploaded or modular CSS by stripping out existing Blogger XML wrappers
/// so we can safely concatenate and re-wrap it later without nesting errors.
pub fn clean_raw_css(input: &str) -> String {
    let mut cleaned = input.to_string();

    cleaned = cleaned.replace("<b:skin>", "");
    cleaned = cleaned.replace("</b:skin>", "");
    cleaned = cleaned.replace("<![CDATA[", "");
    cleaned = cleaned.replace("]]>", "");

    cleaned.trim().to_string()
}

pub const DEFAULT_ICON_SIDEBAR_LEFT:  &str = "M9 4v16M6 8h.01M6 12h.01 M3 4h18v16H3z";
pub const DEFAULT_ICON_SIDEBAR_RIGHT: &str = "M15 4v16M18 8h.01M18 12h.01 M3 4h18v16H3z";
pub const DEFAULT_ICON_PANEL_CLOSE:   &str = "M18 6 6 18M6 6l12 12";
pub const DEFAULT_ICON_SEARCH:        &str = "M11 18a7 7 0 100-14 7 7 0 000 14zM20 20l-3.5-3.5";
pub const DEFAULT_ICON_MENU:          &str = "M4 7h16M4 12h16M4 17h16";

/// Returns the configured icon if set, otherwise an inline-encoded fallback
/// generated from a built-in path string.
pub fn icon_or_default(value: &str, default_path_d: &str) -> String {
    if value.trim().is_empty() {
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#000" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><path d="{}"/></svg>"##,
            default_path_d
        );
        svg_to_data_uri(&svg)
    } else {
        value.to_string()
    }
}

/// Builds the master CSS baseline from modular chunks or user uploads,
/// ensuring it is perfectly wrapped for Blogger and dynamically applies user settings.
pub fn build_master_css(base_css_chunks: &[&str], config: &ThemeConfig) -> String {
    let mut combined_css = String::new();

    for chunk in base_css_chunks {
        let cleaned = clean_raw_css(chunk);
        if !cleaned.is_empty() {
            combined_css.push_str(&cleaned);
            combined_css.push_str("\n\n");
        }
    }

    let mut custom_vars = format!(
        r#":root {{
  --bg-base: {bg_base};
  --bg-panel: {bg_panel};
  --bg-elevated: {bg_elevated};
  --bg-highlight: {bg_elevated};
  --bg-workspace: {bg_workspace};
  --fg-base: {fg_base};
  --fg-dim: {fg_muted};
  --fg-muted: {fg_muted};
  --accent: {accent};
  --border-color: {border};
  --border-soft: {border};
  --theme-border-color: {border};
  --panel-border-width: {panel_border_width};
  --glow: 0 0 {glow_spread} {accent};
  --glow-strong: 0 0 calc({glow_spread} * 2) {accent};
  --btn-radius: {btn_radius};
  --btn-border-width: {btn_border_width};
  --btn-text-transform: {btn_text_transform};
  --font-mono: {font_mono};"#,
        bg_base = config.colors.bg_base,
        bg_panel = config.colors.bg_panel.to_css(),
        bg_elevated = config.colors.bg_elevated.to_css(),
        bg_workspace = generate_workspace_background_value(&config.background.mode),
        fg_base = config.colors.fg_base,
        fg_muted = config.colors.fg_muted,
        accent = config.colors.accent,
        border = config.colors.border,
        panel_border_width = config.colors.panel_border_width,
        glow_spread = config.colors.glow_spread,
        btn_radius = config.buttons.radius,
        btn_border_width = config.buttons.border_width,
        btn_text_transform = config.buttons.text_transform,
        font_mono = config.typography.mono_font_stack,
    );

    custom_vars.push_str(&format!(
        "\n  --icon-sidebar-left: {};",
        icon_or_default(&config.icons.sidebar_left, DEFAULT_ICON_SIDEBAR_LEFT)
    ));
    custom_vars.push_str(&format!(
        "\n  --icon-sidebar-right: {};",
        icon_or_default(&config.icons.sidebar_right, DEFAULT_ICON_SIDEBAR_RIGHT)
    ));
    custom_vars.push_str(&format!(
        "\n  --icon-panel-close: {};",
        icon_or_default(&config.icons.panel_close, DEFAULT_ICON_PANEL_CLOSE)
    ));
    custom_vars.push_str(&format!(
        "\n  --icon-search: {};",
        icon_or_default(&config.icons.search, DEFAULT_ICON_SEARCH)
    ));
    custom_vars.push_str(&format!(
        "\n  --icon-menu: {};",
        icon_or_default(&config.icons.menu, DEFAULT_ICON_MENU)
    ));

    for (key, svg_data) in &config.icons.custom_icons {
        if svg_data.trim().is_empty() {
            continue;
        }
        let safe_key = key.replace(' ', "-").to_lowercase();
        custom_vars.push_str(&format!("\n  --icon-{safe_key}: {svg_data};"));
    }

    custom_vars.push_str("\n}\n");

/* Widget header icons via CSS mask (scoped per widget ID/class for .widget h2::before) */
custom_vars.push_str("#Label1, .Label { --widget-icon: var(--icon-label); }\n");
custom_vars.push_str("#BlogArchive1, .BlogArchive { --widget-icon: var(--icon-archive); }\n");

    combined_css.push_str(&custom_vars);

    if !config.icons.custom_icons.is_empty() {
        combined_css.push_str("\n/* --- Custom Icon Utilities --- */\n");
        for (key, value) in &config.icons.custom_icons {
            if value.trim().is_empty() {
                continue;
            }
            let safe_key = key.replace(' ', "-").to_lowercase();
            combined_css.push_str(&format!(
                ".custom-icon-{safe_key} {{\n  background-image: var(--icon-{safe_key});\n  background-size: contain;\n  background-repeat: no-repeat;\n  background-position: center;\n}}\n"
            ));
        }
    }

    if !config.preset_css.is_empty() {
        combined_css.push_str("\n\n/* --- User Custom CSS --- */\n");
        combined_css.push_str(&config.preset_css);
    }

    combined_css
}