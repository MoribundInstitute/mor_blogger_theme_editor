//! CSS Builder module for sanitizing and concatenating base CSS files.

use crate::config::ThemeConfig;

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

    // Generate the dynamic CSS cascade override block based on user UI selections.
    // Notice: We are using the correct struct fields (e.g., `config.buttons.radius`)
    let custom_vars = format!(
        r#":root {{
  --bg-base: {bg_base};
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
  --font-mono: {font_mono};
}}"#,
        bg_base = config.colors.bg_base,
        fg_base = config.colors.fg_base,
        fg_muted = config.colors.fg_muted,
        accent = config.colors.accent,
        border = config.colors.border,
        panel_border_width = config.colors.panel_border_width,
        glow_spread = config.colors.glow_spread,
        btn_radius = config.buttons.radius,             // FIXED
        btn_border_width = config.buttons.border_width, // FIXED
        btn_text_transform = config.buttons.text_transform, // FIXED
        font_mono = config.typography.mono_font_stack,
    );

    // Re-inject both the legacy PRESET_CSS token and our new Dynamic variables
    format!(
        "<b:skin><![CDATA[\n{}\n/* ===== Active Preset CSS ===== */\n{{{{PRESET_CSS}}}}\n/* ===== Dynamic User CSS Variables ===== */\n{}\n]]></b:skin>",
        combined_css, custom_vars
    )
}