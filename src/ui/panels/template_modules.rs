use dioxus::prelude::*;
use crate::config::ThemeConfig;

#[component]
pub fn TemplateModulesPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let pack = current_config.template_pack.clone();

    rsx! {
        div { class: "editor-panel-content", style: "display: flex; flex-direction: column; gap: 12px;",
            
            p { 
                style: "font-size: 13px; color: var(--editor-fg-muted); margin-bottom: 8px;",
                "Swap out the underlying HTML/XML layout blocks of the theme."
            }

            // 1. Header Variant
            div { class: "editor-card",
                label { class: "editor-label", style: "display: block; margin-bottom: 4px;", "Header Variant" }
                select {
                    class: "editor-input", style: "width: 100%;",
                    value: "{pack.header_variant}",
                    onchange: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();
                        move |evt| {
                            let mut new_config = current_config.clone();
                            new_config.template_pack.header_variant = evt.value().clone();
                            on_apply_theme.call(new_config);
                        }
                    },
                    option { value: "mor", "Mor (Default)" }
                    option { value: "gtk_headerbar", "GTK4 Headerbar" }
                }
            }

            // 2. Main Layout Variant
            div { class: "editor-card",
                label { class: "editor-label", style: "display: block; margin-bottom: 4px;", "Main Canvas Layout" }
                select {
                    class: "editor-input", style: "width: 100%;",
                    value: "{pack.main_variant}",
                    onchange: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();
                        move |evt| {
                            let mut new_config = current_config.clone();
                            new_config.template_pack.main_variant = evt.value().clone();
                            on_apply_theme.call(new_config);
                        }
                    },
                    option { value: "sidebars", "Three Column (Sidebars)" }
                    option { value: "single_column", "Single Column" }
                }
            }

            // 3. Left Sidebar Variant
            div { class: "editor-card",
                label { class: "editor-label", style: "display: block; margin-bottom: 4px;", "Left Sidebar Panel" }
                select {
                    class: "editor-input", style: "width: 100%;",
                    value: "{pack.left_sidebar_variant}",
                    onchange: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();
                        move |evt| {
                            let mut new_config = current_config.clone();
                            new_config.template_pack.left_sidebar_variant = evt.value().clone();
                            on_apply_theme.call(new_config);
                        }
                    },
                    option { value: "blogger_left", "Blogger Widgets (Labels, Archive)" }
                    // option { value: "gtk_dock", "GTK Tool Dock (Coming Soon)" }
                }
            }

            // 4. Right Sidebar Variant
            div { class: "editor-card",
                label { class: "editor-label", style: "display: block; margin-bottom: 4px;", "Right Sidebar Panel" }
                select {
                    class: "editor-input", style: "width: 100%;",
                    value: "{pack.right_sidebar_variant}",
                    onchange: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();
                        move |evt| {
                            let mut new_config = current_config.clone();
                            new_config.template_pack.right_sidebar_variant = evt.value().clone();
                            on_apply_theme.call(new_config);
                        }
                    },
                    option { value: "toc_right", "Table of Contents" }
                }
            }

            // 5. Footer Variant
            div { class: "editor-card",
                label { class: "editor-label", style: "display: block; margin-bottom: 4px;", "Footer Variant" }
                select {
                    class: "editor-input", style: "width: 100%;",
                    value: "{pack.footer_variant}",
                    onchange: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();
                        move |evt| {
                            let mut new_config = current_config.clone();
                            new_config.template_pack.footer_variant = evt.value().clone();
                            on_apply_theme.call(new_config);
                        }
                    },
                    option { value: "mor", "Mor (Default)" }
                }
            }

            // 6. Scripts Variant
            div { class: "editor-card",
                label { class: "editor-label", style: "display: block; margin-bottom: 4px;", "JavaScript Behaviors" }
                select {
                    class: "editor-input", style: "width: 100%;",
                    value: "{pack.script_variant}",
                    onchange: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();
                        move |evt| {
                            let mut new_config = current_config.clone();
                            new_config.template_pack.script_variant = evt.value().clone();
                            on_apply_theme.call(new_config);
                        }
                    },
                    option { value: "mor_panels", "Mor Collapsible Sidebars" }
                    option { value: "minimal", "None (Static Layout)" }
                }
            }
        }
    }
}