use dioxus::prelude::*;

use crate::app::state::{ContextMenuPayload, DockPosition, LayoutState};
use crate::app::theme_signals::ThemeSignals;
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::components::dock_chrome::DockChrome;
use crate::ui::panels::theme_palette::background_panel::BackgroundPanel;
use crate::ui::panels::theme_palette::buttons_panel::ButtonsPanel;
use crate::ui::panels::theme_palette::colors_panel::ColorsPanel;
use crate::ui::panels::theme_palette::cursor_panel::CursorPanel;
use crate::ui::panels::theme_palette::effects_panel_2::EffectsPanel;
use crate::ui::panels::theme_palette::frames_panel::SvgFramesPanel;
use crate::ui::panels::theme_palette::logo_panel::LogoPanel;
use crate::ui::panels::theme_palette::presets;
use crate::ui::panels::theme_palette::scrollbar_panel::ScrollbarPanel;
use crate::ui::panels::theme_palette::template_modules::TemplateModulesPanel;
use crate::ui::panels::theme_palette::typography_panel::TypographyPanel;
use mor_blogger_core::config::ThemeConfig;

use super::shared::{PANE_CSS, PANE_DRAG_JS, PANE_RESIZE_JS};

#[derive(Props, Clone, PartialEq)]
pub struct ThemePaletteDockProps {
    pub active_tab: Signal<&'static str>,
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    pub show_preview: Signal<bool>,
    pub current_config: ThemeConfig,
    pub on_apply_theme: EventHandler<ThemeConfig>,
    pub show_undocked_presets: Signal<bool>,
    pub show_undocked_pages: Signal<bool>,
    pub show_advanced_glow: Signal<bool>,
    pub preview_html: Signal<String>,
    pub base_preview_html: ReadSignal<String>,
}

#[component]
pub fn ThemePaletteDock(props: ThemePaletteDockProps) -> Element {
    let active_tab = props.active_tab;
    let signals = props.signals;
    let active_preset = props.active_preset;
    let show_preview = props.show_preview;
    let current_config = props.current_config;
    let on_apply_theme = props.on_apply_theme;
    let show_undocked_presets = props.show_undocked_presets;
    let show_undocked_pages = props.show_undocked_pages;
    let show_advanced_glow = props.show_advanced_glow;
    let mut preview_html = props.preview_html;
    let base_preview_html = props.base_preview_html;

    let _ = show_preview;
    let mut layout = use_context::<LayoutState>();
    let pos = (layout.theme_palette_pos)();

    use_effect(move || {
        // The Static Page editor owns the preview while it's the active center
        // view (it injects the page). Resetting to the base here would clobber
        // it on every base re-render (e.g. dark/light toggle).
        let static_page_active =
            *layout.center_view.read() == crate::app::state::CenterView::StaticPageEditor;
        if active_tab() != "Pages" && !show_undocked_pages() && !static_page_active {
            preview_html.set(base_preview_html());
        }
    });

    if pos == DockPosition::Hidden {
        return rsx! { div { style: "display: none;" } };
    }

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            script { dangerous_inner_html: "{PANE_DRAG_JS}" }
            script { dangerous_inner_html: "{PANE_RESIZE_JS}" }
            style { "{PANE_CSS}" }

            DockChrome {
                title: "Theme Palette".to_string(),
                dock_id: "theme".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.theme_palette_pos.set(DockPosition::Hidden);
                },
                div { class: "editor-panel-tabs",
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

                    EditorAccordion { id: "Logo", title: "Logo", active: active_tab,
                        LogoPanel { header_logo_url: signals.header_logo_url }
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
                            glow_intensity: signals.glow_intensity,
                            glow_hover: signals.glow_hover,
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
                        TypographyPanel {}
                    }

                    EditorAccordion { id: "Scrollbars", title: "Scrollbars", active: active_tab,
                        ScrollbarPanel {}
                    }

                    EditorAccordion { id: "Buttons", title: "Button Styles", active: active_tab,
                        ButtonsPanel {}
                    }
                }
            }
        }
    }
}
