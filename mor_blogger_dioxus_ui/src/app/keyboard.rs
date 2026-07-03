use dioxus::prelude::*;

use super::state::{DockPosition, LayoutState};

/// Synchronous (capture-phase) keydown guard for the editor windows. Dioxus's
/// `evt.prevent_default()` runs after the event round-trips to Rust, which is
/// too late to stop the webview's built-in actions (save-page, close-tab,
/// history back/forward). This cancels those defaults inline so the muda menu /
/// onkeydown handlers can own the shortcuts. It only prevents defaults — it
/// performs no actions. Inject once per editor DOM via a `<script>`.
pub const EDITOR_KEY_GUARD_JS: &str = r#"
(function(){
  if (window.__morEditorKeyGuard) return;
  window.__morEditorKeyGuard = true;
  window.addEventListener('keydown', function(e){
    var k = (e.key || '').toLowerCase();
    if ((e.ctrlKey || e.metaKey) && (k === 's' || k === 'w')) { e.preventDefault(); }
    if (e.altKey && (k === 'arrowleft' || k === 'arrowright')) { e.preventDefault(); }
  }, true);
})();
"#;

pub fn use_keyboard_shortcuts(
    layout: LayoutState,
    prefs: Signal<crate::app::config_bridge::ShortcutPrefs>,
) {
    let mut theme_palette_pos = layout.theme_palette_pos;
    let mut site_data_pos = layout.site_data_pos;
    let mut layout = layout;

    // Dock-command combos come from the (customizable) prefs. The listener reads
    // window.__morDockKeys at each keydown, so a rebind only has to update the
    // map — re-adding listeners would leave the old combos firing.
    use_effect(move || {
        let p = prefs();
        let mut combo_map = std::collections::HashMap::new();
        for (combo, cmd) in [
            (&p.toggle_left_dock, "toggle_left"),
            (&p.toggle_right_dock, "toggle_right"),
            (&p.close_left_dock, "close_left"),
            (&p.close_right_dock, "close_right"),
        ] {
            if let Some(c) = combo {
                combo_map.insert(crate::ui::layout::shortcut::normalize_combo(c), cmd);
            }
        }
        let _ = dioxus::document::eval(&format!(
            "window.__morDockKeys = {};",
            serde_json::to_string(&combo_map).unwrap_or_else(|_| "{}".to_string())
        ));
    });

    use_effect(move || {
        let mut eval = dioxus::document::eval(
            r#"
            if (!window.__morDockKeysListener) {
                window.__morDockKeysListener = true;
                window.addEventListener('keydown', function(e) {
                    let k = e.key.toLowerCase();

                    // Canonical combo, matching Rust's normalize_combo():
                    // CTRL/SHIFT/ALT order, uppercase, Arrow prefix stripped.
                    let combo = (e.ctrlKey || e.metaKey ? 'CTRL+' : '')
                        + (e.shiftKey ? 'SHIFT+' : '')
                        + (e.altKey ? 'ALT+' : '')
                        + e.key.toUpperCase().replace(/^ARROW/, '');
                    let cmd = (window.__morDockKeys || {})[combo];
                    if (cmd) { e.preventDefault(); dioxus.send(cmd); return; }

                    if ((e.ctrlKey || e.metaKey) && e.shiftKey && /^Digit[1-9]$/.test(e.code)) {
                        // Ctrl/Cmd+Shift+1..9 toggles the Nth pinned dock (e.code so shifted digits still match).
                        e.preventDefault(); dioxus.send("dock_" + e.code.slice(5));
                    }

                    if (e.altKey && !e.ctrlKey && !e.metaKey && !e.shiftKey) {
                        if (k === '1') { e.preventDefault(); dioxus.send("layout_split"); }
                        if (k === '2') { e.preventDefault(); dioxus.send("layout_wide"); }
                        if (k === '3') { e.preventDefault(); dioxus.send("layout_float"); }
                    }
                });
            }
            "#,
        );

        spawn(async move {
            while let Ok(value) = eval.recv::<serde_json::Value>().await {
                if let Some(cmd) = value.as_str() {
                    match cmd {
                        "toggle_left" => {
                            if theme_palette_pos() == DockPosition::Hidden {
                                theme_palette_pos.set(DockPosition::mor_panel_left);
                            } else {
                                theme_palette_pos.set(DockPosition::Hidden);
                            }
                        }
                        "toggle_right" => {
                            if site_data_pos() == DockPosition::Hidden {
                                site_data_pos.set(DockPosition::mor_panel_right);
                            } else {
                                site_data_pos.set(DockPosition::Hidden);
                            }
                        }
                        "close_left" => theme_palette_pos.set(DockPosition::Hidden),
                        "close_right" => site_data_pos.set(DockPosition::Hidden),
                        "layout_split" | "layout_wide" => {
                            theme_palette_pos.set(DockPosition::mor_panel_left);
                            site_data_pos.set(DockPosition::mor_panel_right);
                        }
                        "layout_float" => {
                            theme_palette_pos.set(DockPosition::Floating);
                            site_data_pos.set(DockPosition::Floating);
                        }
                        other => {
                            if let Some(n) = other
                                .strip_prefix("dock_")
                                .and_then(|s| s.parse::<usize>().ok())
                                .filter(|&n| n >= 1)
                            {
                                layout.toggle_dock_by_index(n - 1);
                            }
                        }
                    }
                }
            }
        });
    });
}
