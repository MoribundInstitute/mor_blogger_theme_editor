use dioxus::prelude::*;

use crate::ui::components::accordion::EditorAccordion;
use crate::ui::panels::theme_palette::background_panel::BackgroundPanel;
use crate::ui::panels::theme_palette::buttons_panel::ButtonsPanel;
use crate::ui::panels::theme_palette::colors_panel::ColorsPanel;
use crate::ui::panels::theme_palette::effects_panel_2::EffectsPanel;
use crate::ui::panels::theme_palette::frames_panel::SvgFramesPanel;
use crate::ui::panels::theme_palette::presets::{PresetsPanel, ThemeSignals};
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesPanel;
use crate::ui::panels::theme_palette::template_modules::TemplateModulesPanel;
use crate::ui::panels::theme_palette::typography_panel::TypographyPanel;
use crate::ui::workspace::layout::PanelLayout;
use mor_blogger_core::config::ThemeConfig;

const LEFT_DOCK_CSS: &str = r#"
.editor-left-panel.is-floating {
    position: fixed !important;
    left: var(--left-dock-x, 20px) !important;
    top: var(--left-dock-y, 80px) !important;
    right: auto !important;
    bottom: auto !important;
    margin: 0 !important;
    z-index: 100 !important;
    width: 320px !important;
    max-height: 85vh !important;
    box-shadow: 0 10px 40px rgba(0,0,0,0.5) !important;
}
"#;

const DOCK_DRAG_JS: &str = r#"
(function () {
    if (window.__morCoreDockDragInstalled) return;
    window.__morCoreDockDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.floating-editor-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, a, select, textarea')) return;

        const panel = bar.closest('.editor-left-panel, .editor-right-panel');
        if (!panel) return;
        
        if (window.getComputedStyle(panel).position !== 'fixed' && window.getComputedStyle(panel).position !== 'absolute') return;

        e.preventDefault();
        
        const isLeft = panel.classList.contains('editor-left-panel');
        const varX = isLeft ? '--left-dock-x' : '--right-dock-x';
        const varY = isLeft ? '--left-dock-y' : '--right-dock-y';

        const rect = panel.getBoundingClientRect();
        const startX = e.clientX;
        const startY = e.clientY;
        const startLeft = rect.left;
        const startTop = rect.top;

        document.documentElement.style.setProperty(varX, startLeft + 'px');
        document.documentElement.style.setProperty(varY, startTop + 'px');
        document.body.classList.add('editor-floating-dragging');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;

            const nextLeft = Math.max(0, Math.min(startLeft + dx, window.innerWidth - 100));
            const nextTop = Math.max(0, Math.min(startTop + dy, window.innerHeight - 40));

            document.documentElement.style.setProperty(varX, nextLeft + 'px');
            document.documentElement.style.setProperty(varY, nextTop + 'px');
        };

        const onUp = function () {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
            document.body.classList.remove('editor-floating-dragging');
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

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
    show_undocked_pages: Signal<bool>,
    mut show_undocked_modules: Signal<bool>,
    mut preview_html: Signal<String>,
    base_preview_html: ReadSignal<String>,
) -> Element {
    let _ = show_preview;

    use_effect(move || {
        if active_tab() != "Pages" && !show_undocked_pages() {
            preview_html.set(base_preview_html());
        }
    });

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
        script { dangerous_inner_html: "{DOCK_DRAG_JS}" }

        style { "{LEFT_DOCK_CSS}" }

        aside {
            class: if layout() == PanelLayout::Floating { "editor-left-panel is-floating" } else { "editor-left-panel" },

            if layout() == PanelLayout::Floating {
                div { class: "floating-editor-window-bar",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", IconGrip {} }
                    span { class: "floating-editor-title", "Theme Palette" }

                    div { class: "floating-editor-window-actions", style: "display: flex; gap: 4px;",
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock to window",
                            onclick: move |_| layout.set(PanelLayout::Split),
                            IconDock {}
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Close",
                            onclick: move |_| layout.set(PanelLayout::Hidden),
                            IconClose {}
                        }
                    }
                }
            } else {
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

            div { class: "editor-panel-toolbar-actions",
                button {
                    class: if layout() == PanelLayout::Split { "editor-button is-active" } else { "editor-button" },
                    style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                    onclick: move |_| layout.set(PanelLayout::Split),
                    IconSplit {}
                    "Split"
                }
                button {
                    class: if layout() == PanelLayout::Wide { "editor-button is-active" } else { "editor-button" },
                    style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                    onclick: move |_| layout.set(PanelLayout::Wide),
                    IconWide {}
                    "Wide"
                }
                button {
                    class: if layout() == PanelLayout::Floating { "editor-button is-active" } else { "editor-button" },
                    style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                    onclick: move |_| layout.set(PanelLayout::Floating),
                    IconFloat {}
                    "Float"
                }
            }

            div { class: "editor-panel-tabs",
                EditorAccordion { id: "Modules", title: "Template Modules", active: active_tab,
                    TemplateModulesPanel {
                        current_config: current_config.clone(),
                        show_undocked_modules,
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

                EditorAccordion { id: "Effects", title: "Lighting & Motion", active: active_tab,
                    EffectsPanel {
                        glow_spread: signals.glow_spread,
                        hover_scale: signals.hover_scale,
                    }
                }

                EditorAccordion { id: "SvgFrames", title: "Borders & Frames", active: active_tab,
                    SvgFramesPanel {
                        current_config: current_config.clone(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            on_apply_theme.call(new_config);
                        }
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
                    StaticPagesPanel {
                        signals,
                        show_undocked_pages,
                        preview_html,
                        base_preview_html,
                    }
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
pub fn IconClose() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: "M4.5 4.5l7 7M11.5 4.5l-7 7" }
        }
    }
}

#[component]
pub fn IconGrip() -> Element {
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
