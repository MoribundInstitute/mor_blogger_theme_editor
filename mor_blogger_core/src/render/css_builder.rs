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

    // 1. Build standard known variables
    let mut custom_vars = format!(
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
  --font-mono: {font_mono};"#,
        bg_base = config.colors.bg_base,
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

    // 2. Loop through dynamic infinite bucket and append to :root
    for (key, svg_data) in &config.icons.custom_icons {
        let safe_key = key.replace(' ', "-").to_lowercase();
        custom_vars.push_str(&format!("\n  --icon-{safe_key}: {svg_data};"));
    }

    // Close the :root block
    custom_vars.push_str("\n}\n");

    // 3. Override legacy pseudo-elements so dynamic UI icons render properly
    custom_vars.push_str(r#"
/* --- Force Legacy UI to use Dynamic Icons --- */
.header-panel-toggle-left::before {
  -webkit-mask: var(--icon-sidebar-left) center/20px no-repeat !important;
  mask: var(--icon-sidebar-left) center/20px no-repeat !important;
}
.header-panel-toggle-right::before {
  -webkit-mask: var(--icon-sidebar-right) center/20px no-repeat !important;
  mask: var(--icon-sidebar-right) center/20px no-repeat !important;
  transform: none !important; /* Kills the hardcoded flip bug */
}
.panel-header .panel-toggle::before {
  -webkit-mask: var(--icon-panel-close) center/18px no-repeat !important;
  mask: var(--icon-panel-close) center/18px no-repeat !important;
}
"#);

    combined_css.push_str(&custom_vars);

    // 4. Generate WYSIWYG utility classes for custom dictionary icons
    if !config.icons.custom_icons.is_empty() {
        combined_css.push_str("\n/* --- Custom Icon Utilities --- */\n");
        for key in config.icons.custom_icons.keys() {
            let safe_key = key.replace(' ', "-").to_lowercase();
            combined_css.push_str(&format!(
                ".custom-icon-{safe_key} {{\n  background-image: var(--icon-{safe_key});\n  background-size: contain;\n  background-repeat: no-repeat;\n  background-position: center;\n}}\n"
            ));
        }
    }
    
    // 5. Append User Preset CSS (Moved completely from XML Generator to avoid duplication)
    if !config.preset_css.is_empty() {
        combined_css.push_str("\n\n/* --- User Custom CSS --- */\n");
        combined_css.push_str(&config.preset_css);
    }

    combined_css
}