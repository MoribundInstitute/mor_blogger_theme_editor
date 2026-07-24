use dioxus::prelude::*;

use crate::app::theme_signals::ThemeSignals;
use crate::ui::panels::theme_palette::presets::morph_preview_from_preset;
use mor_blogger_core::config::defaults::default_theme_config;
use mor_blogger_core::config::ThemeConfig;

/// One undo step: the full theme document at a commit point (penpot-style —
/// history stores real state, not just which preset was active, so field
/// edits survive undo/redo instead of snapping back to the pristine preset).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEntry {
    pub preset: Option<&'static str>,
    pub config: ThemeConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeHistory {
    pub snapshots: Vec<HistoryEntry>,
    pub cursor: usize,
}

/// Push an entry: dedupe against the cursor entry, truncate any redo tail,
/// cap at 50 (drop oldest). Returns true when the history changed.
// ponytail: full ThemeConfig clones (~50 max). Switch to diffs if memory matters.
fn push_history(hist: &mut ThemeHistory, entry: HistoryEntry) -> bool {
    if hist.snapshots.get(hist.cursor) == Some(&entry) {
        return false;
    }
    let cursor = hist.cursor;
    hist.snapshots.truncate(cursor + 1);
    hist.snapshots.push(entry);
    if hist.snapshots.len() > 50 {
        hist.snapshots.remove(0);
    }
    hist.cursor = hist.snapshots.len() - 1;
    true
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
    /// Recently committed colors, newest first (penpot-style palette memory).
    pub recent_colors: Signal<Vec<String>>,
}

/// Penpot's recent-colors model: dedupe (case-insensitive) to front, cap 15.
/// Returns false when the color is blank and nothing changed.
fn push_recent(list: &mut Vec<String>, color: &str) -> bool {
    let color = color.trim().to_string();
    if color.is_empty() {
        return false;
    }
    list.retain(|c| !c.eq_ignore_ascii_case(&color));
    list.insert(0, color);
    list.truncate(15);
    true
}

#[cfg(test)]
mod tests {
    use super::{push_history, push_recent, HistoryEntry, ThemeHistory};

    #[test]
    fn history_dedupes_truncates_redo_tail_and_caps() {
        let entry = |bg: &str| {
            let mut c = super::default_theme_config();
            c.colors.bg_base = bg.to_string();
            HistoryEntry {
                preset: None,
                config: c,
            }
        };
        let mut hist = ThemeHistory {
            snapshots: Vec::new(),
            cursor: 0,
        };
        assert!(push_history(&mut hist, entry("#000")));
        assert!(!push_history(&mut hist, entry("#000")), "dedupe vs cursor");
        assert!(push_history(&mut hist, entry("#111")));
        assert!(push_history(&mut hist, entry("#222")));
        assert_eq!(hist.cursor, 2);

        // after undoing to the middle, a new edit drops the redo tail
        hist.cursor = 1;
        assert!(push_history(&mut hist, entry("#333")));
        assert_eq!(hist.snapshots.len(), 3);
        assert_eq!(hist.snapshots[2].config.colors.bg_base, "#333");
        assert_eq!(hist.cursor, 2);

        for i in 0..60 {
            push_history(&mut hist, entry(&format!("#{i:03}")));
        }
        assert_eq!(hist.snapshots.len(), 50);
        assert_eq!(hist.cursor, 49);
    }

    #[test]
    fn recent_colors_dedupe_and_cap() {
        let mut list = Vec::new();
        assert!(!push_recent(&mut list, "  "));
        for i in 0..20 {
            push_recent(&mut list, &format!("#{i:06x}"));
        }
        assert_eq!(list.len(), 15);
        assert_eq!(list[0], "#000013");
        // re-picking an existing color moves it to the front, case-insensitively
        push_recent(&mut list, "#00000A");
        assert_eq!(list[0], "#00000A");
        assert_eq!(list.len(), 15);
    }
}

impl ThemeState {
    pub fn new() -> Self {
        let mut defaults = default_theme_config();
        let prefs = crate::app::config_bridge::EditorPrefs::load();
        let saved_recent_colors = prefs.recent_colors;
        if let Some(pack) = prefs.default_template_pack {
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
            snapshots: Vec::new(),
            cursor: 0,
        });

        let last_imported_gtk =
            use_signal(|| None::<mor_blogger_core::config::gtk_theme::ImportedGtkPreset>);
        let import_status = use_signal(String::new);
        let enable_ai_bridge = use_signal(|| false);
        let recent_colors = use_signal(move || saved_recent_colors);

        let state = ThemeState {
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
            recent_colors,
        };
        // Seed history with the startup document so the first edit is undoable.
        use_hook(move || state.commit());
        state
    }

    /// Record a committed color pick: dedupe to front, cap at 15, persist.
    pub fn push_recent_color(&self, color: &str) {
        let mut recents = self.recent_colors;
        let mut list = recents.write();
        if push_recent(&mut list, color) {
            crate::app::config_bridge::EditorPrefs::update_recent_colors(list.clone());
        }
    }

    /// Record the current full theme document as an undo step.
    pub fn commit(&self) {
        let entry = HistoryEntry {
            preset: *self.active_preset.read(),
            config: self.signals.to_config(),
        };
        let mut history = self.history;
        push_history(&mut history.write(), entry);
    }

    pub fn undo(&self) {
        let entry = {
            let mut history = self.history;
            let mut hist = history.write();
            if hist.cursor == 0 {
                return;
            }
            hist.cursor -= 1;
            hist.snapshots[hist.cursor].clone()
        };
        self.restore_entry(entry);
    }

    pub fn redo(&self) {
        let entry = {
            let mut history = self.history;
            let mut hist = history.write();
            if hist.cursor + 1 >= hist.snapshots.len() {
                return;
            }
            hist.cursor += 1;
            hist.snapshots[hist.cursor].clone()
        };
        self.restore_entry(entry);
    }

    pub fn can_undo(&self) -> bool {
        self.history.read().cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        let hist = self.history.read();
        hist.cursor + 1 < hist.snapshots.len()
    }

    fn restore_entry(&self, entry: HistoryEntry) {
        let mut active_preset = self.active_preset;
        active_preset.set(entry.preset);
        self.signals.apply_config(&entry.config);
        // Keep the editor chrome morphing in sync when the step had a preset.
        if let Some(id) = entry.preset {
            let is_dark = *self.signals.is_dark_mode.read();
            let presets = mor_blogger_core::presets::all_presets();
            if let Some(p) = presets.iter().find(|p| p.id == id) {
                morph_preview_from_preset(p, is_dark);
            }
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
