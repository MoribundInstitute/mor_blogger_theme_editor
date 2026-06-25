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

pub fn use_keyboard_shortcuts(layout: LayoutState) {
    let mut theme_palette_pos = layout.theme_palette_pos;
    let mut site_data_pos = layout.site_data_pos;

    use_effect(move || {
        let mut eval = dioxus::document::eval(
            r#"
            window.addEventListener('keydown', function(e) {
                let k = e.key.toLowerCase();

                if (e.ctrlKey || e.metaKey) {
                    if (k === 'b') { e.preventDefault(); dioxus.send("toggle_left"); }
                    if (k === 'e') { e.preventDefault(); dioxus.send("toggle_right"); }
                }

                if (e.altKey && !e.ctrlKey && !e.metaKey && !e.shiftKey) {
                    if (k === '1') { e.preventDefault(); dioxus.send("layout_split"); }
                    if (k === '2') { e.preventDefault(); dioxus.send("layout_wide"); }
                    if (k === '3') { e.preventDefault(); dioxus.send("layout_float"); }
                }
            });
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
                        "layout_split" | "layout_wide" => {
                            theme_palette_pos.set(DockPosition::mor_panel_left);
                            site_data_pos.set(DockPosition::mor_panel_right);
                        }
                        "layout_float" => {
                            theme_palette_pos.set(DockPosition::Floating);
                            site_data_pos.set(DockPosition::Floating);
                        }
                        _ => {}
                    }
                }
            }
        });
    });
}
