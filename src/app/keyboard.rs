use dioxus::prelude::*;

use crate::ui::workspace::layout::PanelLayout;

use super::layout_state::AppLayoutState;

pub fn use_keyboard_shortcuts(layout: AppLayoutState) {
    let mut left_layout = layout.left_layout;
    let mut right_layout = layout.right_layout;

    use_effect(move || {
        let mut eval = eval(
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
            while let Ok(value) = eval.recv().await {
                if let Some(cmd) = value.as_str() {
                    match cmd {
                        "toggle_left" => {
                            if left_layout() == PanelLayout::Hidden {
                                left_layout.set(PanelLayout::Split);
                            } else {
                                left_layout.set(PanelLayout::Hidden);
                            }
                        }
                        "toggle_right" => {
                            if right_layout() == PanelLayout::Hidden {
                                right_layout.set(PanelLayout::Split);
                            } else {
                                right_layout.set(PanelLayout::Hidden);
                            }
                        }
                        "layout_split" => {
                            left_layout.set(PanelLayout::Split);
                            right_layout.set(PanelLayout::Split);
                        }
                        "layout_wide" => {
                            left_layout.set(PanelLayout::Wide);
                            right_layout.set(PanelLayout::Wide);
                        }
                        "layout_float" => {
                            left_layout.set(PanelLayout::Floating);
                            right_layout.set(PanelLayout::Floating);
                        }
                        _ => {}
                    }
                }
            }
        });
    });
}
