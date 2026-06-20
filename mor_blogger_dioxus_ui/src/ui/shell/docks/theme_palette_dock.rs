use dioxus::prelude::*;

use crate::app::layout_state::{AppLayoutState, DockPosition};
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::panels::theme_palette::background_panel::BackgroundPanel;
use crate::ui::panels::theme_palette::buttons_panel::ButtonsPanel;
use crate::ui::panels::theme_palette::colors_panel::ColorsPanel;
use crate::ui::panels::theme_palette::cursor_panel::CursorPanel;
use crate::ui::panels::theme_palette::effects_panel_2::EffectsPanel;
use crate::ui::panels::theme_palette::frames_panel::SvgFramesPanel;
use crate::ui::panels::theme_palette::presets::ThemeSignals;
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesPanel;
use crate::ui::panels::theme_palette::template_modules::TemplateModulesPanel;
use crate::ui::panels::theme_palette::scrollbar_panel::ScrollbarPanel;
use crate::ui::panels::theme_palette::typography_panel::TypographyPanel;
use mor_blogger_core::config::ThemeConfig;

const LEFT_PANE_CSS: &str = r#"
.editor-left-panel.is-floating {
    position: fixed !important;
    left: var(--left-pane-x, 20px) !important;
    top: var(--left-pane-y, 80px) !important;
    right: auto !important;
    bottom: auto !important;
    margin: 0 !important;
    z-index: 100 !important;
    width: 320px !important;
    max-height: 85vh !important;
    box-shadow: 0 10px 40px rgba(0,0,0,0.5) !important;
}
.editor-right-panel.is-floating {
    position: fixed !important;
    left: var(--right-pane-x, calc(100vw - 340px)) !important;
    top: var(--right-pane-y, 80px) !important;
    right: auto !important;
    bottom: auto !important;
    margin: 0 !important;
    z-index: 100 !important;
    width: 320px !important;
    max-height: 85vh !important;
    box-shadow: 0 10px 40px rgba(0,0,0,0.5) !important;
}
.editor-left-panel:not(.is-floating) { width: var(--left-pane-width, 320px) !important; position: relative; }
.editor-right-panel:not(.is-floating) { width: var(--right-pane-width, 320px) !important; position: relative; }
.pane-resizer { position: absolute; top: 0; bottom: 0; width: 6px; z-index: 999; cursor: ew-resize; background: transparent; transition: background 0.1s; }
.pane-resizer:hover, .pane-resizer:active { background: var(--editor-accent, rgba(255,255,255,0.2)); }
.pane-resizer-right { right: 0; }
.pane-resizer-left { left: 0; }
"#;

const PANE_DRAG_JS: &str = r#"
(function () {
    if (window.__morCorePaneDragInstalled) return;
    window.__morCorePaneDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.floating-editor-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, a, select, textarea')) return;

        const panel = bar.closest('.editor-left-panel, .editor-right-panel');
        if (!panel) return;
        
        if (window.getComputedStyle(panel).position !== 'fixed' && window.getComputedStyle(panel).position !== 'absolute') return;

        e.preventDefault();
        
        const isLeft = panel.classList.contains('editor-left-panel');
        const varX = isLeft ? '--left-pane-x' : '--right-pane-x';
        const varY = isLeft ? '--left-pane-y' : '--right-pane-y';

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

const PANE_RESIZE_JS: &str = r#"
(function() {
    if (window.__morPaneResizeInstalled) return;
    window.__morPaneResizeInstalled = true;
    document.addEventListener('pointerdown', function(e) {
        const resizer = e.target.closest('.pane-resizer');
        if (!resizer) return;
        e.preventDefault();
        const isLeft = resizer.classList.contains('pane-resizer-right');
        const startX = e.clientX;
        const panel = resizer.closest('.editor-left-panel, .editor-right-panel');
        const startWidth = panel.getBoundingClientRect().width;
        
        const onMove = function(moveEvt) {
            const dx = moveEvt.clientX - startX;
            const newWidth = isLeft ? startWidth + dx : startWidth - dx;
            const clamped = Math.max(200, Math.min(newWidth, window.innerWidth / 2.5));
            const varName = isLeft ? '--left-pane-width' : '--right-pane-width';
            document.documentElement.style.setProperty(varName, clamped + 'px');
        };
        const onUp = function() {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
        };
        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

#[component]
pub fn ThemePaletteDock(
    active_tab: Signal<&'static str>,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
    show_preview: Signal<bool>,
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
    show_undocked_presets: Signal<bool>,
    show_undocked_pages: Signal<bool>,
    mut show_undocked_modules: Signal<bool>,
    show_advanced_glow: Signal<bool>,
    mut preview_html: Signal<String>,
    base_preview_html: ReadSignal<String>,
) -> Element {
    let _ = show_preview;
    let mut layout = use_context::<AppLayoutState>();
    let pos = (layout.theme_palette_pos)();

    use_effect(move || {
        if active_tab() != "Pages" && !show_undocked_pages() {
            preview_html.set(base_preview_html());
        }
    });

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let header_actions = rsx! {
        div { class: "floating-editor-window-actions",
            if pos != DockPosition::Left {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Left",
                    onclick: move |_| {
                        layout.request_exclusive_dock("theme", DockPosition::Left);
                    },
                    IconDockLeft {}
                }
            }
            if pos != DockPosition::Right {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Right",
                    onclick: move |_| {
                        layout.request_exclusive_dock("theme", DockPosition::Right);
                    },
                    IconDockRight {}
                }
            }
            if pos != DockPosition::Floating {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Float Window",
                    onclick: move |_| {
                        layout.theme_palette_pos.set(DockPosition::Floating);
                    },
                    IconFloat {}
                }
            }
            button {
                class: "editor-mini-button",
                style: "display: flex; align-items: center; padding: 4px;",
                title: "Close",
                onclick: move |_| layout.theme_palette_pos.set(DockPosition::Hidden),
                IconClose {}
            }
        }
    };

    let inner_content = rsx! {
        script { dangerous_inner_html: "{PANE_DRAG_JS}" }
        script { dangerous_inner_html: "{PANE_RESIZE_JS}" }
        style { "{LEFT_PANE_CSS}" }

        if pos == DockPosition::Floating {
            div {
                class: "floating-editor-window-bar",
                div {
                    class: "floating-editor-grip-group",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", IconGrip {} }
                    span {
                        class: "floating-editor-title",
                        "Theme Palette"
                    }
                }
                {header_actions}
            }
        } else {
            div { class: "editor-panel-header",
                h2 { class: "editor-panel-title", "Theme Palette" }
                {header_actions}
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

            EditorAccordion { id: "Cursors", title: "Cursors", active: active_tab,
                CursorPanel {}
            }

            EditorAccordion { id: "Effects", title: "Lighting & Motion", active: active_tab,
                EffectsPanel {
                    glow_spread: signals.glow_spread,
                    hover_scale: signals.hover_scale,
                    show_advanced_glow,
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

            EditorAccordion { id: "Scrollbars", title: "Scrollbars", active: active_tab,
                ScrollbarPanel {}
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
    };

    match pos {
        DockPosition::Left => rsx! {
            aside { class: "editor-left-panel",
                {inner_content}
                div { class: "pane-resizer pane-resizer-right" }
            }
        },
        DockPosition::Right => rsx! {
            aside { class: "editor-right-panel",
                {inner_content}
                div { class: "pane-resizer pane-resizer-left" }
            }
        },
        DockPosition::Floating => rsx! {
            aside { class: "editor-left-panel is-floating",
                {inner_content}
            }
        },
        DockPosition::Hidden => rsx! {}
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
fn IconFloat() -> Element {
    rsx! {
        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "1.5", width: "10", height: "8", rx: "1.5" }
            rect { x: "4.5", y: "6.5", width: "10", height: "8", rx: "1.5" }
        }
    }
}

#[component]
fn IconDockLeft() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M5.5 2.5v11" }
        }
    }
}

#[component]
fn IconDockRight() -> Element {
    rsx! {
        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            rect { x: "1.5", y: "2.5", width: "13", height: "11", rx: "2" }
            path { d: "M10.5 2.5v11" }
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
