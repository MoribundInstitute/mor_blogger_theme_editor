use dioxus::prelude::*;

use crate::ui::accordion::EditorAccordion;
use crate::ui::background_panel::BackgroundPanel;
use crate::ui::buttons_panel::ButtonsPanel;
use crate::ui::colors_panel::ColorsPanel;
use crate::ui::layout::{set_panel_layout, PanelLayout};
use crate::ui::presets_panel::{PresetsPanel, ThemeSignals};
use crate::ui::static_pages_panel::StaticPagesPanel;
use crate::ui::typography_panel::TypographyPanel;

#[component]
pub fn LeftVisualsPanel(
    mut layout: Signal<PanelLayout>,
    active_tab: Signal<&'static str>,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
    show_preview: Signal<bool>,
) -> Element {
    let _ = show_preview;

    if layout() == PanelLayout::Hidden {
        return rsx! {
            div { class: "editor-left-panel-collapsed",
                button {
                    class: "editor-collapse-button",
                    onclick: move |_| layout.set(PanelLayout::Split),
                    "Theme Palette »"
                }
            }
        };
    }

    rsx! {
        aside { class: "editor-left-panel",

            // Render the draggable window bar when floating
            if layout() == PanelLayout::Floating {
                div { class: "floating-editor-window-bar",
                    span { class: "floating-editor-grip", "⠿" }
                    span { class: "floating-editor-title", "Theme Palette" }
                    div { class: "floating-editor-window-actions",
                        button {
                            class: "editor-mini-button",
                            onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Split),
                            "Dock"
                        }
                        button {
                            class: "editor-mini-button",
                            onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Hidden),
                            "Hide"
                        }
                    }
                }
            } else {
                // Otherwise render the normal header
                div { class: "editor-panel-header",
                    h2 { class: "editor-panel-title", "Theme Palette" }
                    button {
                        class: "editor-mini-button",
                        onclick: move |_| layout.set(PanelLayout::Hidden),
                        "« Hide"
                    }
                }
            }

            div { class: "editor-panel-toolbar-actions",
                button {
                    class: if layout() == PanelLayout::Split { "editor-button is-active" } else { "editor-button" },
                    onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Split),
                    "Split"
                }
                button {
                    class: if layout() == PanelLayout::Wide { "editor-button is-active" } else { "editor-button" },
                    onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Wide),
                    "Wide"
                }
                button {
                    class: if layout() == PanelLayout::Floating { "editor-button is-active" } else { "editor-button" },
                    onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Floating),
                    "Float"
                }
            }

            div { class: "editor-panel-tabs",
                EditorAccordion { id: "Presets", title: "Theme Presets", active: active_tab,
                    PresetsPanel { active_preset, signals }
                }
                EditorAccordion { id: "Colors", title: "Color Palette", active: active_tab,
                    ColorsPanel {
                        bg_base: signals.bg_base,
                        bg_panel: signals.bg_panel,
                        bg_elevated: signals.bg_elevated,
                        fg_base: signals.fg_base,
                        fg_muted: signals.fg_muted,
                        accent: signals.accent,
                        border: signals.border,
                    }
                }
                EditorAccordion { id: "Background", title: "Background", active: active_tab,
                    BackgroundPanel { background: signals.background }
                }
                EditorAccordion { id: "Typography", title: "Typography", active: active_tab,
                    TypographyPanel {
                        body_font_stack: signals.body_font_stack,
                        heading_font_stack: signals.heading_font_stack,
                        mono_font_stack: signals.mono_font_stack,
                        base_size: signals.base_size,
                        scale_ratio: signals.scale_ratio,
                        line_height: signals.line_height,
                        heading_weight: signals.heading_weight,
                    }
                }
                EditorAccordion { id: "Buttons", title: "Button Styles", active: active_tab,
                    ButtonsPanel {
                        btn_radius: signals.btn_radius,
                        btn_border_width: signals.btn_border_width,
                        btn_text_transform: signals.btn_text_transform,
                    }
                }
                EditorAccordion { id: "Pages", title: "Static Pages", active: active_tab,
                    StaticPagesPanel { signals }
                }
            }
        }
    }
}