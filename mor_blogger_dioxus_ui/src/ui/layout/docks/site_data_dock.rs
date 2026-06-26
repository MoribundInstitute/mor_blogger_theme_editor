use dioxus::prelude::*;

use crate::app::state::{DockPosition, LayoutState};
use crate::app::theme_signals::ThemeSignals;
use crate::ui::components::accordion::EditorAccordion;
use crate::ui::components::dock_chrome::DockChrome;
use crate::ui::panels::site_data::ads_panel::AdsPanel;
use crate::ui::panels::site_data::assets_panel::AssetsPanel;
use crate::ui::panels::site_data::menu_panel::MenuPanel;
use crate::ui::panels::site_data::plugins_panel::PluginsPanel;
use crate::ui::panels::site_data::seo_panel::{FooterPanel, SeoPanel};
use crate::ui::panels::site_data::site_panel::SitePanel;
use mor_blogger_core::config::ThemeConfig;

use super::shared::{PANE_CSS, PANE_DRAG_JS, PANE_RESIZE_JS};

#[derive(Props, Clone, PartialEq)]
pub struct SiteDataDockProps {
    pub active_tab: Signal<&'static str>,
    pub signals: ThemeSignals,
    pub current_config: ThemeConfig,
    pub on_apply_theme: EventHandler<ThemeConfig>,
}

#[component]
pub fn SiteDataDock(props: SiteDataDockProps) -> Element {
    let mut layout = use_context::<LayoutState>();
    let active_tab = props.active_tab;
    let signals = props.signals;
    let current_config = props.current_config;
    let on_apply_theme = props.on_apply_theme;

    let pos = (layout.site_data_pos)();

    if pos == DockPosition::Hidden {
        return rsx! { div { style: "display: none;" } };
    }

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_right,
            script { dangerous_inner_html: "{PANE_DRAG_JS}" }
            script { dangerous_inner_html: "{PANE_RESIZE_JS}" }
            style { "{PANE_CSS}" }

            DockChrome {
                title: "Site Data".to_string(),
                dock_id: "site".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.site_data_pos.set(DockPosition::Hidden);
                },
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
            }
        }
    }
}
