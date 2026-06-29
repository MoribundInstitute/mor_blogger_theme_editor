use dioxus::prelude::*;

use crate::app::theme_signals::ThemeSignals;
use crate::ui::panels::theme_palette::presets::morph_preview_from_preset;
use mor_blogger_core::config::defaults::default_theme_config;
use mor_blogger_core::config::ColorConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeHistory {
    pub snapshots: Vec<Option<&'static str>>,
    pub cursor: usize,
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    pub show_undocked_presets: Signal<bool>,
    pub show_advanced_presets: Signal<bool>,
    pub show_advanced_glow: Signal<bool>,
    pub show_advanced_colors: Signal<bool>,
    pub show_advanced_cursors: Signal<bool>,
    pub show_advanced_buttons: Signal<bool>,
    pub show_advanced_typography: Signal<bool>,
    pub history: Signal<ThemeHistory>,
    pub last_imported_gtk: Signal<Option<mor_blogger_core::config::gtk_theme::ImportedGtkPreset>>,
    pub import_status: Signal<String>,
    pub enable_ai_bridge: Signal<bool>,
}

impl ThemeState {
    pub fn new() -> Self {
        let mut defaults = default_theme_config();
        if let Some(pack) = crate::app::config_bridge::EditorPrefs::load().default_template_pack {
            defaults.template_pack = pack;
        }

        // Seed a default preset at startup so the editor's bound `preset_css` buffer
        // (and the whole preview) is populated on first launch instead of empty.
        // Without this, `preset_css` stays "" until the user manually applies a preset,
        // so the CSS editor opens on a blank buffer ("[render_theme] preset_css bytes = 0").
        // ponytail: reuse apply_preset (it also sets preset_css). Reversible — delete the
        // apply call + restore `active_preset` to None to go back to an empty default.
        let all_presets = mor_blogger_core::presets::all_presets();
        // TEMP DEBUG: did presets load from the running binary's cwd? Empty here ->
        // default_preset = None -> preset_css never seeded -> "[render_theme] preset_css bytes = 0".
        eprintln!(
            "[seed] all_presets len = {} (cwd-dependent)",
            all_presets.len()
        );
        let default_preset = all_presets.into_iter().next();
        let default_preset_id = default_preset.as_ref().map(|p| p.id);

        let signals = use_hook(move || {
            let s = ThemeSignals::from_config(&defaults);
            if let Some(p) = &default_preset {
                s.apply_preset(p);
            }
            // TEMP DEBUG: byte length actually seeded into the editor-bound buffer.
            eprintln!(
                "[seed] preset_css after apply = {}",
                s.preset_css.peek().len()
            );
            s
        });

        let active_preset = use_signal(move || default_preset_id);
        let show_undocked_presets = use_signal(|| false);
        let show_advanced_presets = use_signal(|| false);
        let show_advanced_glow = use_signal(|| false);
        let show_advanced_colors = use_signal(|| false);
        let show_advanced_cursors = use_signal(|| false);
        let show_advanced_buttons = use_signal(|| false);
        let show_advanced_typography = use_signal(|| false);
        let history = use_signal(|| ThemeHistory {
            snapshots: vec![None],
            cursor: 0,
        });

        let last_imported_gtk =
            use_signal(|| None::<mor_blogger_core::config::gtk_theme::ImportedGtkPreset>);
        let import_status = use_signal(String::new);
        let enable_ai_bridge = use_signal(|| false);

        ThemeState {
            signals,
            active_preset,
            show_undocked_presets,
            show_advanced_presets,
            show_advanced_glow,
            show_advanced_colors,
            show_advanced_cursors,
            show_advanced_buttons,
            show_advanced_typography,
            history,
            last_imported_gtk,
            import_status,
            enable_ai_bridge,
        }
    }

    pub fn commit(&self) {
        let current = *self.active_preset.read();
        let mut history = self.history;
        let mut hist = history.write();
        if hist.snapshots.get(hist.cursor) == Some(&current) {
            return;
        }
        let cursor = hist.cursor;
        hist.snapshots.truncate(cursor + 1);
        hist.snapshots.push(current);
        if hist.snapshots.len() > 50 {
            hist.snapshots.remove(0);
        }
        hist.cursor = hist.snapshots.len() - 1;
    }

    pub fn undo(&self) {
        self._undo_internal();
    }

    pub fn redo(&self) {
        self._redo_internal();
    }

    fn _undo_internal(&self) {
        let mut history = self.history;
        let mut hist = history.write();
        if hist.cursor == 0 {
            return;
        }
        hist.cursor -= 1;
        let val = hist.snapshots[hist.cursor];
        let mut active_preset = self.active_preset;
        active_preset.set(val);
        self.restore_preset(val);
    }

    fn _redo_internal(&self) {
        let mut history = self.history;
        let mut hist = history.write();
        if hist.cursor + 1 >= hist.snapshots.len() {
            return;
        }
        hist.cursor += 1;
        let val = hist.snapshots[hist.cursor];
        let mut active_preset = self.active_preset;
        active_preset.set(val);
        self.restore_preset(val);
    }

    pub fn can_undo(&self) -> bool {
        self.history.read().cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        let hist = self.history.read();
        hist.cursor + 1 < hist.snapshots.len()
    }

    fn restore_preset(&self, val: Option<&'static str>) {
        let is_dark = *self.signals.is_dark_mode.read();
        if val.is_none() {
            let defaults = default_theme_config();
            self.signals.apply_config(&defaults);
            return;
        }
        let id = val.unwrap();
        let presets = mor_blogger_core::presets::all_presets();
        let preset = presets.iter().find(|p| p.id == id);
        if let Some(p) = preset {
            self.signals.apply_preset(p);
            morph_preview_from_preset(p, is_dark);
        }
    }

    pub fn perform_dark_mode_toggle(&self) {
        let mut signals = self.signals;
        let new_dark = !*signals.is_dark_mode.read();
        signals.is_dark_mode.set(new_dark);

        let active_id = *self.active_preset.read();

        let no_explicit = if let Some(id) = active_id {
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(preset) = presets.iter().find(|p| p.id == id) {
                let lc = &preset.light.colors;
                let dc = &preset.dark.colors;
                lc.bg_base == dc.bg_base && lc.fg_base == dc.fg_base
            } else {
                true
            }
        } else {
            true
        };

        if no_explicit {
            if active_id.is_none() {
                let pal = if new_dark {
                    mor_blogger_core::presets::PresetPalette {
                        colors: mor_blogger_core::config::defaults::dark_color_config(),
                        background: mor_blogger_core::config::defaults::dark_background_config(),
                    }
                } else {
                    mor_blogger_core::presets::PresetPalette {
                        colors: mor_blogger_core::config::defaults::light_color_config(),
                        background: mor_blogger_core::config::defaults::light_background_config(),
                    }
                };
                signals.swap_palette(&pal);
            } else {
                let cur = ColorConfig {
                    bg_base: signals.bg_base.read().clone(),
                    bg_panel: signals.bg_panel.read().clone(),
                    bg_elevated: signals.bg_elevated.read().clone(),
                    fg_base: signals.fg_base.read().clone(),
                    fg_muted: signals.fg_muted.read().clone(),
                    accent: signals.accent.read().clone(),
                    border: signals.border.read().clone(),
                    panel_border_width: signals.panel_border_width.read().clone(),
                    glow_spread: signals.glow_spread.read().clone(),
                    hover_scale: signals.hover_scale.read().clone(),
                    panel_border_image_url: signals.panel_border_image_url.read().clone(),
                    panel_border_image_slice: signals.panel_border_image_slice.read().clone(),
                    panel_border_image_repeat: signals.panel_border_image_repeat.read().clone(),
                    glow_color: signals.glow_color.read().clone(),
                    glow_logo: *signals.glow_logo.read(),
                    glow_title: *signals.glow_title.read(),
                    glow_toc: *signals.glow_toc.read(),
                    glow_sidebar: *signals.glow_sidebar.read(),
                    glow_logo_color: signals.glow_logo_color.read().clone(),
                    glow_title_color: signals.glow_title_color.read().clone(),
                    glow_toc_color: signals.glow_toc_color.read().clone(),
                    glow_sidebar_color: signals.glow_sidebar_color.read().clone(),
                    glow_text: *signals.glow_text.read(),
                    glow_containers: *signals.glow_containers.read(),
                    glow_icons: *signals.glow_icons.read(),
                    glow_text_color: signals.glow_text_color.read().clone(),
                    glow_containers_color: signals.glow_containers_color.read().clone(),
                    glow_icons_color: signals.glow_icons_color.read().clone(),
                    glow_footer: *signals.glow_footer.read(),
                    glow_header: *signals.glow_header.read(),
                    glow_main: *signals.glow_main.read(),
                    glow_footer_color: signals.glow_footer_color.read().clone(),
                    glow_header_color: signals.glow_header_color.read().clone(),
                    glow_main_color: signals.glow_main_color.read().clone(),
                    ..Default::default()
                };
                let inv = cur.inverted_contrast();
                signals.bg_base.set(inv.bg_base);
                signals.bg_panel.set(inv.bg_panel);
                signals.bg_elevated.set(inv.bg_elevated);
                signals.fg_base.set(inv.fg_base);
                signals.fg_muted.set(inv.fg_muted);

                let bg_cur = signals.background.read().clone();
                signals.background.set(bg_cur.inverted_contrast());
            }
        } else if let Some(id) = active_id {
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(preset) = presets.iter().find(|p| p.id == id) {
                let pal = if new_dark {
                    &preset.dark
                } else {
                    &preset.light
                };
                signals.swap_palette(pal);
                signals.apply_preset_css(preset);
            }
        }

        if let Some(id) = active_id {
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(preset) = presets.iter().find(|p| p.id == id) {
                morph_preview_from_preset(preset, new_dark);
            }
        }
    }
}
