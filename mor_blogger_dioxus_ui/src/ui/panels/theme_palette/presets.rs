mod importers;
mod panel;
mod signals;

use dioxus::prelude::*;

pub use mor_blogger_core::presets::Preset as ThemePreset;
pub use panel::{PresetFloatingWindow, PresetsPanel};
pub use signals::ThemeSignals;

use mor_blogger_core::config::BackgroundMode;

// Ownership: Borrow preset. Return raw CSS string.
pub fn build_css_vars(preset: &ThemePreset, is_light: bool) -> String {
    let palette = if is_light {
        &preset.light
    } else {
        &preset.dark
    };
    let colors = &palette.colors;

    // Extract gradient data securely. Fallback to solid color if no gradient exists.
    let (grad_from, grad_to) = match &palette.background.mode {
        BackgroundMode::Gradient { from, to, .. } => (from.clone(), to.clone()),
        BackgroundMode::Solid { color } => (color.clone(), color.clone()),
        BackgroundMode::Tile { .. } => ("transparent".to_string(), "transparent".to_string()),
    };

    let bg_panel = colors.bg_panel.to_css();
    let bg_elevated = colors.bg_elevated.to_css();

    let active_glow_color = if colors.glow_color.is_empty() { &colors.accent } else { &colors.glow_color };

    let cursor_val = dioxus::prelude::try_consume_context::<crate::app::state::ThemeAppState>()
        .map(|state| state.signals.cursor_style.read().to_string())
        .unwrap_or_else(|| "default".to_string());

    format!(
        "--bg-base: {bg}; --bg-panel: {panel}; --bg-highlight: {elevated}; --bg-soft: {panel}; --bg-elevated: {elevated}; --bg-workspace: {bg}; --fg-base: {fg}; --fg-dim: {muted}; --fg-muted: {muted}; --accent: {acc}; --border-color: {border}; --border-soft: {border}; --theme-border-color: {border}; --panel-border-width: {border_w}; --bg-gradient-from: {g_from}; --bg-gradient-to: {g_to}; --glow: 0 0 {glow} {acc}; --glow-strong: 0 0 calc({glow} * 2) {acc}; --theme-cursor: {cursor};",
        bg = colors.bg_base,
        panel = bg_panel,
        elevated = bg_elevated,
        fg = colors.fg_base,
        muted = colors.fg_muted,
        acc = active_glow_color,
        border = colors.border,
        border_w = colors.panel_border_width,
        g_from = grad_from,
        g_to = grad_to,
        glow = colors.glow_spread,
        cursor = cursor_val,
    )
}

pub fn morph_preview_from_preset(preset: &ThemePreset, is_dark: bool) {
    let is_light_mode = !is_dark;
    crate::app::hotswap::execute_theme_morph(&build_css_vars(preset, is_light_mode), is_light_mode);
}
