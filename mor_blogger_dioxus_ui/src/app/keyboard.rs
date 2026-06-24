use dioxus::prelude::*;

use super::state::{DockPosition, LayoutState};

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
