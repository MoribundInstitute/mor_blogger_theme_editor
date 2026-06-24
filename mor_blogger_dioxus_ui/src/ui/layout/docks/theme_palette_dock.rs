use dioxus::prelude::*;

use crate::app::state::{ContextMenuPayload, DockPosition, LayoutState};
use crate::app::theme_signals::ThemeSignals;
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::components::icons::{IconClose, IconDockLeft, IconDockRight, IconFloat, IconGrip};
use crate::ui::panels::theme_palette::background_panel::BackgroundPanel;
use crate::ui::panels::theme_palette::buttons_panel::ButtonsPanel;
use crate::ui::panels::theme_palette::colors_panel::ColorsPanel;
use crate::ui::panels::theme_palette::cursor_panel::CursorPanel;
use crate::ui::panels::theme_palette::effects_panel_2::EffectsPanel;
use crate::ui::panels::theme_palette::frames_panel::SvgFramesPanel;
use crate::ui::panels::theme_palette::presets;
use crate::ui::panels::theme_palette::scrollbar_panel::ScrollbarPanel;
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesPanel;
use crate::ui::panels::theme_palette::template_modules::TemplateModulesPanel;
use crate::ui::panels::theme_palette::typography_panel::TypographyPanel;
use mor_blogger_core::config::ThemeConfig;

const LEFT_PANE_CSS: &str = r#"
.mor_panel_left.is-floating {
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
.mor_panel_right.is-floating {
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
.mor_panel_left:not(.is-floating) { width: var(--left-pane-width, 320px) !important; position: relative; }
.mor_panel_right:not(.is-floating) { width: var(--right-pane-width, 320px) !important; position: relative; }
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

        const panel = bar.closest('.mor_panel_left, .mor_panel_right');
        if (!panel) return;
        
        if (window.getComputedStyle(panel).position !== 'fixed' && window.getComputedStyle(panel).position !== 'absolute') return;

        e.preventDefault();
        
        const isLeft = panel.classList.contains('mor_panel_left');
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
        const panel = resizer.closest('.mor_panel_left, .mor_panel_right');
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
    show_advanced_glow: Signal<bool>,
    mut preview_html: Signal<String>,
    base_preview_html: ReadSignal<String>,
) -> Element {
    let _ = show_preview;
    let mut layout = use_context::<LayoutState>();
    let pos = (layout.theme_palette_pos)();

    use_effect(move || {
        if active_tab() != "Pages" && !show_undocked_pages() {
            preview_html.set(base_preview_html());
        }
    });

    if pos == DockPosition::Hidden {
        return rsx! { div { style: "display: none;" } };
    }

    let header_actions = rsx! {
        div { class: "floating-editor-window-actions",
            if pos != DockPosition::mor_panel_left {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Left",
                    onclick: move |_| {
                        layout.request_exclusive_dock("theme", DockPosition::mor_panel_left);
                    },
                    IconDockLeft {}
                }
            }
            if pos != DockPosition::mor_panel_right {
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Dock Right",
                    onclick: move |_| {
                        layout.request_exclusive_dock("theme", DockPosition::mor_panel_right);
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
                h2 {
                    class: "editor-panel-title",
                    oncontextmenu: move |evt| {
                        evt.prevent_default();
                        evt.stop_propagation();
                        let coords = evt.client_coordinates();
                        layout.active_context_menu.set(Some(ContextMenuPayload {
                            x: coords.x,
                            y: coords.y,
                            kind: "ui_typography".to_string(),
                            target_id: "ui-header".to_string(),
                        }));
                    },
                    "Theme Palette"
                }
                {header_actions}
            }
        }

        div { class: "editor-panel-tabs",
            EditorAccordion { id: "Presets", title: "Theme Presets", active: active_tab,
                presets::PresetsPanel {
                    is_embedded: true,
                    active_preset,
                    signals,
                    current_config: current_config.clone(),
                    on_apply_theme: move |new_config: ThemeConfig| {
                        on_apply_theme.call(new_config);
                    },
                    show_undocked_presets,
                }
            }

            EditorAccordion { id: "Modules", title: "Template Modules", active: active_tab,
                TemplateModulesPanel {
                    current_config: current_config.clone(),
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
                    base_size: signals.base_size,
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

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            {inner_content}
        }
    }
}
