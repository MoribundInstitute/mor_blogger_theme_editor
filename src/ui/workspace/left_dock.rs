use dioxus::prelude::*;

use crate::config::ThemeConfig;
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::panels::background_panel::BackgroundPanel;
use crate::ui::panels::buttons_panel::ButtonsPanel;
use crate::ui::panels::colors_panel::ColorsPanel;
use crate::ui::workspace::layout::{set_panel_layout, PanelLayout};
use crate::ui::panels::presets_panel::{PresetsPanel, ThemeSignals};
use crate::ui::panels::static_pages_panel::StaticPagesPanel;
use crate::ui::panels::template_modules::TemplateModulesPanel;
use crate::ui::panels::typography_panel::TypographyPanel;

#[component]
pub fn LeftVisualsPanel(
    mut layout: Signal<PanelLayout>,
    active_tab: Signal<&'static str>,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
    show_preview: Signal<bool>,
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
    show_undocked_presets: Signal<bool>,
) -> Element {
    let _ = show_preview;

    if layout() == PanelLayout::Hidden {
        return rsx! {
            div { class: "editor-left-panel-collapsed",
                button {
                    class: "editor-collapse-button",
                    style: "display: flex; align-items: center; gap: 6px;",
                    onclick: move |_| layout.set(PanelLayout::Split),
                    IconSidebarLeft {}
                    "Theme Palette"
                }
            }
        };
    }

    rsx! {
        aside { class: "editor-left-panel",

            // Render the draggable window bar when floating
            if layout() == PanelLayout::Floating {
                div { class: "floating-editor-window-bar",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", IconGrip {} }
                    span { class: "floating-editor-title", "Theme Palette" }
                    
                    // GTK-style icon-only window controls
                    div { class: "floating-editor-window-actions", style: "display: flex; gap: 4px;",
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock to window",
                            onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Split),
                            IconDock {}
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Close",
                            onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Hidden),
                            IconClose {}
                        }
                    }
                }
            } else {
                // Normal header
                div { class: "editor-panel-header",
                    h2 { class: "editor-panel-title", "Theme Palette" }
                    button {
                        class: "editor-mini-button",
                        style: "display: flex; align-items: center; gap: 6px;",
                        onclick: move |_| layout.set(PanelLayout::Hidden),
                        IconSidebarLeft {}
                        "Hide"
                    }
                }
            }

            // Toolbar with Icon + Text combinations
            div { class: "editor-panel-toolbar-actions",
                button {
                    class: if layout() == PanelLayout::Split { "editor-button is-active" } else { "editor-button" },
                    style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                    onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Split),
                    IconSplit {}
                    "Split"
                }
                button {
                    class: if layout() == PanelLayout::Wide { "editor-button is-active" } else { "editor-button" },
                    style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                    onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Wide),
                    IconWide {}
                    "Wide"
                }
                button {
                    class: if layout() == PanelLayout::Floating { "editor-button is-active" } else { "editor-button" },
                    style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                    onclick: move |_| set_panel_layout(&mut layout.clone(), PanelLayout::Floating),
                    IconFloat {}
                    "Float"
                }
            }

            div { class: "editor-panel-tabs",
                EditorAccordion { id: "Modules", title: "Template Modules", active: active_tab,
                    TemplateModulesPanel {
                        current_config: current_config.clone(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            on_apply_theme.call(new_config);
                        }
                    }
                }
                EditorAccordion { id: "Presets", title: "Theme Presets", active: active_tab,
                    PresetsPanel {
                        active_preset,
                        signals,
                        current_config: current_config.clone(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            on_apply_theme.call(new_config);
                        },
                        show_undocked_presets,
                    }
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

// ============================================================================
// GTK / NORD SYMBOLIC ICONS
// ============================================================================

#[component]
fn IconSidebarLeft() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M5.5 2.5v11" }
        }
    }
}

#[component]
fn IconClose() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: "M4.5 4.5l7 7M11.5 4.5l-7 7" }
        }
    }
}

#[component]
fn IconGrip() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "currentColor",
            circle { cx: "6", cy: "4", r: "1" }
            circle { cx: "10", cy: "4", r: "1" }
            circle { cx: "6", cy: "8", r: "1" }
            circle { cx: "10", cy: "8", r: "1" }
            circle { cx: "6", cy: "12", r: "1" }
            circle { cx: "10", cy: "12", r: "1" }
        }
    }
}

#[component]
fn IconDock() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M5.5 2.5v11" }
            path { d: "M11 6l-2 2 2 2" }
            path { d: "M14 8H9" }
        }
    }
}

#[component]
fn IconSplit() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M8 2.5v11" }
        }
    }
}

#[component]
fn IconWide() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
        }
    }
}

#[component]
fn IconFloat() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "1.5", width: "10", height: "8", rx: "1.5" }
            rect { x: "4.5", y: "6.5", width: "10", height: "8", rx: "1.5" }
        }
    }
}