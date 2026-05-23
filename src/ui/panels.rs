use dioxus::prelude::*;

use crate::config::{BackgroundConfig, SurfaceFill, ThemeConfig};
use crate::diagnostics::DiagnosticResult;
use crate::ui::assets_panel::AssetsPanel;
use crate::ui::background_panel::BackgroundPanel;
use crate::ui::buttons_panel::ButtonsPanel;
use crate::ui::colors_panel::ColorsPanel;
use crate::ui::export_panel::ExportPanel;
use crate::ui::inputs::PanelNote;
use crate::ui::layout::{
    set_workbench_layout, PreviewTemplateMode, PreviewViewport, WorkbenchLayout,
};
use crate::ui::menu_panel::MenuPanel;
use crate::ui::plugins_panel::PluginsPanel;
use crate::ui::presets_panel::{PresetsPanel, ThemeSignals};
use crate::ui::seo_panel::{FooterPanel, SeoPanel};
use crate::ui::site_panel::SitePanel;
use crate::ui::typography_panel::TypographyPanel;

#[component]
pub fn LeftPanel(
    is_collapsed: Signal<bool>,
    workbench_layout: Signal<WorkbenchLayout>,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,

    site_title: Signal<String>,
    site_subtitle: Signal<String>,
    header_logo_url: Signal<String>,
    home_url: Signal<String>,

    bg_base: Signal<String>,
    bg_panel: Signal<SurfaceFill>,
    bg_elevated: Signal<SurfaceFill>,
    fg_base: Signal<String>,
    fg_muted: Signal<String>,
    accent: Signal<String>,
    border: Signal<String>,

    btn_radius: Signal<String>,
    btn_border_width: Signal<String>,
    btn_text_transform: Signal<String>,

    body_font_stack: Signal<String>,
    heading_font_stack: Signal<String>,
    mono_font_stack: Signal<String>,
    base_size: Signal<String>,
    scale_ratio: Signal<String>,
    line_height: Signal<String>,
    heading_weight: Signal<String>,

    background: Signal<BackgroundConfig>,
    favicon_url: Signal<String>,
    social_card_image_url: Signal<String>,

    menu_1_label: Signal<String>,
    menu_1_url: Signal<String>,
    menu_2_label: Signal<String>,
    menu_2_url: Signal<String>,
    menu_3_label: Signal<String>,
    menu_3_url: Signal<String>,
    menu_4_label: Signal<String>,
    menu_4_url: Signal<String>,

    meta_description: Signal<String>,
    meta_keywords: Signal<String>,
    custom_robots: Signal<String>,
    author_name: Signal<String>,
    license_url: Signal<String>,

    footer_text: Signal<String>,
    footer_license_label: Signal<String>,
    footer_license_url: Signal<String>,

    custom_js: Signal<String>,
) -> Element {
    let mut collapsed = is_collapsed;

    let open_section = use_signal(|| "site");

    if collapsed() {
        return rsx! {
            div {
                class: "editor-left-panel-collapsed",
                button {
                    class: "editor-collapse-button",
                    title: "Expand settings panel (Ctrl+B)",
                    onclick: move |_| collapsed.set(false),
                    "[ open ] »"
                }
            }
        };
    }

    rsx! {
        div {
            class: "editor-left-panel",

            if workbench_layout() == WorkbenchLayout::FloatingEditor {
                div {
                    class: "floating-editor-window-bar",
                    "data-floating-window-bar": "true",
                    span {
                        class: "floating-editor-grip",
                        "data-floating-window-grip": "true",
                        "⠿"
                    }
                    span {
                        class: "floating-editor-title",
                        "data-floating-window-title": "true",
                        "Floating Editor"
                    }
                    div {
                        class: "floating-editor-window-actions",
                        button {
                            class: "editor-mini-button",
                            title: "Reset floating panel position",
                            onclick: move |_| {
                                let _ = eval(
                                    r#"
                                    try {
                                        window.localStorage.removeItem(
                                            "mor_blogger_theme_editor.floating_editor_position"
                                        );
                                        const panel = document.querySelector(".editor-left-panel");
                                        if (panel) {
                                            panel.style.setProperty("--floating-editor-x", "18px");
                                            panel.style.setProperty("--floating-editor-y", "18px");
                                        }
                                    } catch (e) {}
                                    "#,
                                );
                            },
                            "Reset"
                        }
                        button {
                            class: "editor-mini-button",
                            title: "Dock the editor panel back to split layout",
                            onclick: move |_| {
                                set_workbench_layout(workbench_layout, WorkbenchLayout::Split);
                            },
                            "Dock"
                        }
                    }
                }
            }

            div {
                class: "editor-panel-heading-row",
                h2 {
                    class: "editor-panel-title",
                    "Blogger Theme Architect"
                }
                button {
                    class: "editor-mini-button",
                    title: "Collapse settings panel (Ctrl+B)",
                    onclick: move |_| collapsed.set(true),
                    "« hide"
                }
            }

            p {
                class: "editor-panel-copy",
                "Customize the blank Blogger XML template without touching the raw theme code."
            }

            div {
                class: "editor-panel-toolbar",
                p {
                    class: "editor-panel-toolbar-label",
                    "Workbench Layout · Alt+1–4"
                }

                div {
                    class: "editor-panel-toolbar-actions",

                    button {
                        class: if workbench_layout() == WorkbenchLayout::Split {
                            "editor-mini-button editor-mini-button-active"
                        } else {
                            "editor-mini-button"
                        },
                        onclick: move |_| {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::Split);
                        },
                        "Normal"
                    }

                    button {
                        class: if workbench_layout() == WorkbenchLayout::WideEditor {
                            "editor-mini-button editor-mini-button-active"
                        } else {
                            "editor-mini-button"
                        },
                        onclick: move |_| {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::WideEditor);
                        },
                        "Wide"
                    }

                    button {
                        class: if workbench_layout() == WorkbenchLayout::FloatingEditor {
                            "editor-mini-button editor-mini-button-active"
                        } else {
                            "editor-mini-button"
                        },
                        onclick: move |_| {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::FloatingEditor);
                        },
                        "Float"
                    }

                    button {
                        class: if workbench_layout() == WorkbenchLayout::PreviewTakeover {
                            "editor-mini-button editor-mini-button-active"
                        } else {
                            "editor-mini-button"
                        },
                        onclick: move |_| {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::PreviewTakeover);
                        },
                        "Preview"
                    }

                    if workbench_layout() == WorkbenchLayout::FloatingEditor {
                        button {
                            class: "editor-mini-button",
                            onclick: move |_| {
                                set_workbench_layout(workbench_layout, WorkbenchLayout::Split);
                            },
                            "Dock"
                        }
                    }
                }
            }

            PresetsPanel { signals, active_preset }

            div {
                class: "editor-accordion",

                AccordionSection {
                    id: "site".to_string(),
                    title: "Site Identity".to_string(),
                    open_section,
                    SitePanel {
                        site_title,
                        site_subtitle,
                        header_logo_url,
                        home_url,
                    }
                }

                AccordionSection {
                    id: "palette".to_string(),
                    title: "Palette".to_string(),
                    open_section,
                    ColorsPanel {
                        bg_base,
                        bg_panel,
                        bg_elevated,
                        fg_base,
                        fg_muted,
                        accent,
                        border,
                    }
                }

                AccordionSection {
                    id: "typography".to_string(),
                    title: "Typography".to_string(),
                    open_section,
                    TypographyPanel {
                        body_font_stack,
                        heading_font_stack,
                        mono_font_stack,
                        base_size,
                        scale_ratio,
                        line_height,
                        heading_weight,
                    }
                }

                AccordionSection {
                    id: "buttons".to_string(),
                    title: "Buttons".to_string(),
                    open_section,
                    ButtonsPanel {
                        btn_radius,
                        btn_border_width,
                        btn_text_transform,
                    }
                }

                AccordionSection {
                    id: "background".to_string(),
                    title: "Background".to_string(),
                    open_section,
                    BackgroundPanel { background }
                }

                AccordionSection {
                    id: "assets".to_string(),
                    title: "Assets".to_string(),
                    open_section,
                    AssetsPanel {
                        favicon_url,
                        social_card_image_url,
                    }
                }

                AccordionSection {
                    id: "menu".to_string(),
                    title: "Menu".to_string(),
                    open_section,
                    MenuPanel {
                        menu_1_label,
                        menu_1_url,
                        menu_2_label,
                        menu_2_url,
                        menu_3_label,
                        menu_3_url,
                        menu_4_label,
                        menu_4_url,
                    }
                }

                AccordionSection {
                    id: "seo".to_string(),
                    title: "SEO".to_string(),
                    open_section,
                    SeoPanel {
                        meta_description,
                        meta_keywords,
                        custom_robots,
                        author_name,
                        license_url,
                    }
                }

                AccordionSection {
                    id: "footer".to_string(),
                    title: "Footer".to_string(),
                    open_section,
                    FooterPanel {
                        footer_text,
                        footer_license_label,
                        footer_license_url,
                    }
                }

                AccordionSection {
                    id: "plugins".to_string(),
                    title: "Plugins".to_string(),
                    open_section,
                    PluginsPanel { custom_js }
                }
            }

            PanelNote {
                title: "Blogger Notes".to_string(),
                body: "Some Blogger widgets, profile settings, and layout gadgets may still need to be adjusted inside Blogger itself.".to_string(),
            }

            ul {
                style: "font-size: 0.85rem; color: #d9b58c; padding-left: 20px;",
                li {
                    a {
                        href: "https://www.blogger.com",
                        target: "_blank",
                        style: "color: inherit;",
                        "Edit Widgets / Gadgets"
                    }
                }
                li {
                    a {
                        href: "https://www.blogger.com",
                        target: "_blank",
                        style: "color: inherit;",
                        "Change Blog Title"
                    }
                }
                li {
                    a {
                        href: "https://www.blogger.com",
                        target: "_blank",
                        style: "color: inherit;",
                        "Edit Author Profile"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AccordionSectionProps {
    id: String,
    title: String,
    open_section: Signal<&'static str>,
    children: Element,
}

#[component]
fn AccordionSection(props: AccordionSectionProps) -> Element {
    let id_static: &'static str = match props.id.as_str() {
        "site" => "site",
        "palette" => "palette",
        "typography" => "typography",
        "buttons" => "buttons",
        "background" => "background",
        "assets" => "assets",
        "menu" => "menu",
        "seo" => "seo",
        "footer" => "footer",
        "plugins" => "plugins",
        _ => "site",
    };

    let is_open = (props.open_section)() == id_static;

    rsx! {
        section {
            class: if is_open {
                "editor-accordion-section editor-accordion-section-open"
            } else {
                "editor-accordion-section"
            },

            button {
                class: "editor-accordion-trigger",
                onclick: move |_| {
                    let mut open = props.open_section;
                    if open() == id_static {
                        open.set("");
                    } else {
                        open.set(id_static);
                    }
                },

                span {
                    class: "editor-accordion-title",
                    "{props.title}"
                }

                span {
                    class: "editor-accordion-icon",
                    if is_open { "−" } else { "+" }
                }
            }

            if is_open {
                div {
                    class: "editor-accordion-body",
                    {props.children}
                }
            }
        }
    }
}

#[component]
pub fn RightPanel(
    workbench_layout: Signal<WorkbenchLayout>,
    preview_viewport: Signal<PreviewViewport>,
    preview_width: Signal<u32>,
    preview_template_mode: Signal<PreviewTemplateMode>,

    generated_xml: String,
    preview_html: String,
    show_preview: Signal<bool>,
    diag: Signal<DiagnosticResult>,
    config_toml: String,
    on_load_theme: EventHandler<String>,
    on_restore: EventHandler<ThemeConfig>,
) -> Element {
    rsx! {
        div {
            class: "editor-right-panel",

            ExportPanel {
                workbench_layout,
                preview_viewport,
                preview_width,
                preview_template_mode,

                generated_xml,
                preview_html,
                show_preview,
                diag,
                config_toml,
                on_load_theme,
                on_restore,
            }
        }
    }
}