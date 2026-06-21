use dioxus::prelude::*;

use crate::app::state::{DockPosition, LayoutState};
use crate::app::theme_signals::ThemeSignals;
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::components::icons::{IconClose, IconDockLeft, IconDockRight, IconFloat};
use crate::ui::panels::site_data::ads_panel::AdsPanel;
use crate::ui::panels::site_data::assets_panel::AssetsPanel;
use crate::ui::panels::site_data::menu_panel::MenuPanel;
use crate::ui::panels::site_data::plugins_panel::PluginsPanel;
use crate::ui::panels::site_data::seo_panel::{FooterPanel, SeoPanel};
use crate::ui::panels::site_data::site_panel::SitePanel;
use mor_blogger_core::config::ThemeConfig;

const RIGHT_PANE_CSS: &str = r#"
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
pub fn SiteDataDock(
    active_tab: Signal<&'static str>,
    signals: ThemeSignals,
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let mut layout = use_context::<LayoutState>();
    let pos = (layout.site_data_pos)();

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
                        layout.request_exclusive_dock("site", DockPosition::Left);
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
                        layout.request_exclusive_dock("site", DockPosition::Right);
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
                        layout.site_data_pos.set(DockPosition::Floating);
                    },
                    IconFloat {}
                }
            }
            button {
                class: "editor-mini-button",
                style: "display: flex; align-items: center; padding: 4px;",
                title: "Close",
                onclick: move |_| layout.site_data_pos.set(DockPosition::Hidden),
                IconClose {}
            }
        }
    };

    let inner_content = rsx! {
        script { dangerous_inner_html: "{PANE_DRAG_JS}" }
        script { dangerous_inner_html: "{PANE_RESIZE_JS}" }
        style { "{RIGHT_PANE_CSS}" }

        if pos == DockPosition::Floating {
            div {
                class: "floating-editor-window-bar",
                div {
                    class: "floating-editor-grip-group",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", IconGrip {} }
                    span {
                        class: "floating-editor-title",
                        "Site Data"
                    }
                }
                {header_actions}
            }
        } else {
            div { class: "editor-panel-header",
                h2 { class: "editor-panel-title", "Site Data" }
                {header_actions}
            }
        }

        div { class: "editor-panel-tabs",
            EditorAccordion { id: "Site", title: "Site Identity", active: active_tab,
                SitePanel {}
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
                SeoPanel {}
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
            aside { class: "editor-right-panel is-floating",
                {inner_content}
            }
        },
        DockPosition::Hidden => rsx! {},
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
