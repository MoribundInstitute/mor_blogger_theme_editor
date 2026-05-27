use dioxus::prelude::*;

use crate::ui::accordion::EditorAccordion;
use crate::ui::ads_panel::AdsPanel;
use crate::ui::assets_panel::AssetsPanel;
use crate::ui::layout::{set_panel_layout, PanelLayout};
use crate::ui::menu_panel::MenuPanel;
use crate::ui::plugins_panel::PluginsPanel;
use crate::ui::presets_panel::ThemeSignals;
use crate::ui::seo_panel::{FooterPanel, SeoPanel};
use crate::ui::site_panel::SitePanel;

#[component]
pub fn RightDataPanel(
    mut layout: Signal<PanelLayout>,
    active_tab: Signal<&'static str>,
    signals: ThemeSignals,
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
        aside { class: "editor-right-panel",

            // Render the draggable window bar when floating
            if layout() == PanelLayout::Floating {
                div { class: "floating-editor-window-bar",
                    span { class: "floating-editor-grip", "⠿" }
                    span { class: "floating-editor-title", "Site Data" }
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