use dioxus::prelude::*;

use crate::app::theme_signals::ThemeSignals;
use crate::ui::panels::theme_palette::presets::morph_preview_from_preset;
use mor_blogger_core::config::defaults::default_theme_config;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeHistory {
    pub snapshots: Vec<Option<&'static str>>,
    pub cursor: usize,
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    /// Active color-variant name within the active preset (None = base palette).
    pub active_variant: Signal<Option<&'static str>>,
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
        let default_preset = all_presets.into_iter().next();
        let default_preset_id = default_preset.as_ref().map(|p| p.id);

        let signals = use_hook(move || {
            let s = ThemeSignals::from_config(&defaults);
            if let Some(p) = &default_preset {
                s.apply_preset(p);
            }
            s
        });

        let active_preset = use_signal(move || default_preset_id);
        let active_variant = use_signal(|| None::<&'static str>);
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
            active_variant,
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

    pub fn redo(&self) {
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
                let inv = signals.to_config().colors.inverted_contrast();
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
                let variant = *self.active_variant.read();
                let (light, dark) = preset.palette_pair(variant);
                signals.swap_palette(if new_dark { dark } else { light });
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
