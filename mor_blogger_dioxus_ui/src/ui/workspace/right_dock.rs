use dioxus::prelude::*;

use super::left_dock::{IconClose, IconGrip};
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::panels::site_data::ads_panel::AdsPanel;
use crate::ui::panels::site_data::assets_panel::AssetsPanel;
use crate::ui::panels::site_data::menu_panel::MenuPanel;
use crate::ui::panels::site_data::plugins_panel::PluginsPanel;
use crate::ui::panels::site_data::seo_panel::{FooterPanel, SeoPanel};
use crate::ui::panels::site_data::site_panel::SitePanel;
use crate::ui::panels::theme_palette::presets::ThemeSignals;
use crate::ui::workspace::layout::PanelLayout;
use mor_blogger_core::config::ThemeConfig;

const RIGHT_DOCK_CSS: &str = r#"
.editor-right-panel.is-floating {
    position: fixed !important;
    left: var(--right-dock-x, calc(100vw - 340px)) !important;
    top: var(--right-dock-y, 80px) !important;
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
pub fn RightDataPanel(
    mut layout: Signal<PanelLayout>,
    active_tab: Signal<&'static str>,
    signals: ThemeSignals,
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    if layout() == PanelLayout::Hidden {
        return rsx! {
            div { class: "editor-right-panel-collapsed",
                button {
                    class: "editor-collapse-button",
                    onclick: move |_| layout.set(PanelLayout::Split),
                    "« Site Data"
                }
            }
        };
    }

    rsx! {
        script { dangerous_inner_html: "{DOCK_DRAG_JS}" }

        style { "{RIGHT_DOCK_CSS}" }

        aside {
            class: if layout() == PanelLayout::Floating { "editor-right-panel is-floating" } else { "editor-right-panel" },

            if layout() == PanelLayout::Floating {
                div { class: "floating-editor-window-bar",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", IconGrip {} }
                    span { class: "floating-editor-title", "Site Data" }
                    div { class: "floating-editor-window-actions",
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock to window",
                            onclick: move |_| layout.set(PanelLayout::Split),
                            "Dock"
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
                    h2 { class: "editor-panel-title", "Site Data" }
                    button {
                        class: "editor-mini-button",
                        onclick: move |_| layout.set(PanelLayout::Hidden),
                        "Hide »"
                    }
                }
            }

            div { class: "editor-panel-toolbar-actions",
                button {
                    class: if layout() == PanelLayout::Split { "editor-button is-active" } else { "editor-button" },
                    onclick: move |_| layout.set(PanelLayout::Split),
                    "Split"
                }
                button {
                    class: if layout() == PanelLayout::Wide { "editor-button is-active" } else { "editor-button" },
                    onclick: move |_| layout.set(PanelLayout::Wide),
                    "Wide"
                }
                button {
                    class: if layout() == PanelLayout::Floating { "editor-button is-active" } else { "editor-button" },
                    onclick: move |_| layout.set(PanelLayout::Floating),
                    "Float"
                }
            }

            div { class: "editor-panel-tabs",
                EditorAccordion { id: "Site", title: "Site Identity", active: active_tab,
                    SitePanel {
                        site_title: signals.site_title,
                        site_subtitle: signals.site_subtitle,
                        header_logo_url: signals.header_logo_url,
                        home_url: signals.home_url,
                    }
                }

                EditorAccordion { id: "Assets", title: "Images & Assets", active: active_tab,
                    AssetsPanel {
                        favicon_url: signals.favicon_url,
                        social_card_image_url: signals.social_card_image_url,
                        current_config: current_config.clone(),
                        on_apply_theme,
                    }
                }

                EditorAccordion { id: "Menu", title: "Navigation Menu", active: active_tab,
                    MenuPanel {
                        menu_1_label: signals.menu_1_label,
                        menu_1_url: signals.menu_1_url,
                        menu_2_label: signals.menu_2_label,
                        menu_2_url: signals.menu_2_url,
                        menu_3_label: signals.menu_3_label,
                        menu_3_url: signals.menu_3_url,
                        menu_4_label: signals.menu_4_label,
                        menu_4_url: signals.menu_4_url,
                    }
                }

                EditorAccordion { id: "SEO", title: "SEO & Footer", active: active_tab,
                    SeoPanel {
                        meta_description: signals.meta_description,
                        meta_keywords: signals.meta_keywords,
                        custom_robots: signals.custom_robots,
                        author_name: signals.author_name,
                        license_url: signals.license_url,
                    }
                    FooterPanel {
                        footer_text: signals.footer_text,
                        footer_license_label: signals.footer_license_label,
                        footer_license_url: signals.footer_license_url,
                    }
                }

                EditorAccordion { id: "Ads", title: "Advertising", active: active_tab,
                    AdsPanel { ads: signals.ads }
                }

                EditorAccordion { id: "Plugins", title: "Custom Scripts", active: active_tab,
                    PluginsPanel { custom_js: signals.custom_js }
                }
            }
        }
    }
}
