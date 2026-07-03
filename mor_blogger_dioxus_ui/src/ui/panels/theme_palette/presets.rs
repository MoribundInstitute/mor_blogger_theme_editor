pub(crate) mod importers;
mod panel;

use dioxus::prelude::*;

pub use mor_blogger_core::presets::Preset as ThemePreset;
pub use panel::{PresetFloatingWindow, PresetsPanel};

use mor_blogger_core::config::BackgroundMode;

/// Blog-theme preset → inline `--bg-*` / `--fg-*` / `--accent` … declarations
/// (no selector). Fed to [`crate::app::hotswap::execute_theme_morph`], which
/// injects them into the preview iframe's `:root`. These are the *blog* tokens;
/// the app shell's own tokens are `--mor-*` (see `layout/theme.rs`).
pub fn build_css_vars(preset: &ThemePreset, is_light: bool) -> String {
    let palette = if is_light {
        &preset.light
    } else {
        &preset.dark
    };
    let colors = &palette.colors;

    // Gradient falls back to a solid pair; tiles paint their own background.
    let (grad_from, grad_to) = match &palette.background.mode {
        BackgroundMode::Gradient { from, to, .. } => (from.clone(), to.clone()),
        BackgroundMode::Solid { color } => (color.clone(), color.clone()),
        BackgroundMode::Tile { .. } => ("transparent".to_string(), "transparent".to_string()),
    };

    let bg_panel = colors.bg_panel.to_css();
    let bg_elevated = colors.bg_elevated.to_css();

    let active_glow_color = if colors.glow_color.is_empty() {
        &colors.accent
    } else {
        &colors.glow_color
    };

    // Live UI signals win over the preset's saved values (context is absent in
    // headless/export paths, hence the fallbacks).
    let theme_state = dioxus::prelude::try_consume_context::<crate::app::state::ThemeState>();
    let live = |read: fn(&crate::app::theme_signals::ThemeSignals) -> String| {
        theme_state.as_ref().map(|s| read(&s.signals))
    };

    let cursor_val = live(|s| s.cursor_style.read().to_string())
        .unwrap_or_else(|| "default".to_string());
    let scrollbar_width_val = live(|s| s.scrollbar_width.read().to_string())
        .unwrap_or_else(|| preset.base_config.scrollbar_width.clone());
    let scrollbar_track_val = live(|s| s.scrollbar_track_color.read().to_string())
        .unwrap_or_else(|| preset.base_config.scrollbar_track_color.clone());
    let scrollbar_thumb_val = live(|s| s.scrollbar_thumb_color.read().to_string())
        .unwrap_or_else(|| preset.base_config.scrollbar_thumb_color.clone());
    let scrollbar_thumb_hover_val = live(|s| s.scrollbar_thumb_hover_color.read().to_string())
        .unwrap_or_else(|| preset.base_config.scrollbar_thumb_hover_color.clone());

    format!(
        "--bg-base: {bg}; --bg-panel: {panel}; --bg-highlight: {elevated}; --bg-soft: {panel}; --bg-elevated: {elevated}; --bg-workspace: {bg}; --fg-base: {fg}; --fg-dim: {muted}; --fg-muted: {muted}; --accent: {acc}; --border-color: {border}; --border-soft: {border}; --theme-border-color: {border}; --panel-border-width: {border_w}; --bg-gradient-from: {g_from}; --bg-gradient-to: {g_to}; --glow: 0 0 {glow} {acc}; --glow-strong: 0 0 calc({glow} * 2) {acc}; --theme-cursor: {cursor}; --theme-scrollbar-width: {scrollbar_width}; --theme-scrollbar-track: {scrollbar_track}; --theme-scrollbar-thumb: {scrollbar_thumb}; --theme-scrollbar-thumb-hover: {scrollbar_thumb_hover};",
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
        scrollbar_width = scrollbar_width_val,
        scrollbar_track = scrollbar_track_val,
        scrollbar_thumb = scrollbar_thumb_val,
        scrollbar_thumb_hover = scrollbar_thumb_hover_val,
    )
}

pub fn morph_preview_from_preset(preset: &ThemePreset, is_dark: bool) {
    let is_light_mode = !is_dark;
    crate::app::hotswap::execute_theme_morph(&build_css_vars(preset, is_light_mode), is_light_mode);
}
