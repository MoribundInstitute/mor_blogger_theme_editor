//! Compact Template Modules UI.
//! Allows users to hot-swap structural XML components via a docked compact view.

use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

#[component]
pub fn TemplateModulesPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let mut theme_state = use_context::<crate::app::state::LayoutState>();
    let mut site_data = use_context::<Signal<crate::app::state::SiteData>>();
    let mut injected_warning = use_signal(|| false);
    let pack = current_config.template_pack.clone();

    rsx! {
        div { class: "editor-panel-content", style: "display: flex; flex-direction: column; gap: 16px;",

            div { style: "display: flex; align-items: flex-start; justify-content: space-between; gap: 12px;",
                p {
                    style: "margin: 0; font-size: 13px; color: var(--editor-fg-muted); line-height: 1.4;",
                    "Swap out the underlying HTML/XML layout blocks of the theme."
                }
                button {
                    class: if (theme_state.show_advanced_modules)() { "editor-button editor-button-small editor-button-active" } else { "editor-button editor-button-small" },
                    onclick: move |_| theme_state.show_advanced_modules.set(true),
                    "⚙ Advanced"
                }
            }

            // Compact Docked View (Fallback)
            div { style: "display: flex; flex-direction: column; gap: 12px;",
                CompactSelect { label: "Header Variant", val: pack.header_variant.clone(), options: vec![("mor", "Mor (Default)"), ("mor_search_center", "Mor — Centered Search"), ("gtk_headerbar", "GTK4 Headerbar"), ("minimal", "Minimal Flexbox")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.header_variant = v; f.call(nc); } } }
                CompactSelect { label: "Main Canvas", val: pack.main_variant.clone(), options: vec![("sidebars", "Three Column (Sidebars)"), ("single_column", "Single Column"), ("two_column_right", "Two Column Right CSS Grid")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.main_variant = v; f.call(nc); } } }

                div { class: "editor-card", style: "padding: 8px 12px;",
                    label { class: "editor-label", style: "display: block; margin-bottom: 4px; font-size: 0.75rem;", "Content Layout" }
                    select {
                        class: "editor-input", style: "width: 100%; font-size: 0.8rem; padding: 4px;",
                        value: "{pack.content_variant}",
                        onchange: {
                            let current_config = current_config.clone();
                            let on_apply_theme = on_apply_theme.clone();
                            move |evt| {
                                let selected = evt.value();

                                // 1. Update the core layout state
                                let mut nc = current_config.clone();
                                nc.template_pack.content_variant = selected.clone();

                                // Apply the soft override for JS behaviors
                                if selected == "mor_magazine" {
                                    nc.template_pack.script_variant = "magazine_grid_logic".to_string();
                                } else if selected == "standard_feed" {
                                    nc.template_pack.script_variant = "mor_collapsible_sidebars".to_string();
                                }

                                // 2. The Auto-Injector Logic
                                let requires_grid = selected == "mor_magazine" || selected == "mor_masonry" || selected.contains("Grid");

                                if requires_grid {
                                    // Force at least 4 posts into the global site data
                                    let injected = site_data.write().ensure_minimum_posts(4);
                                    injected_warning.set(injected);
                                } else {
                                    injected_warning.set(false);
                                }

                                on_apply_theme.call(nc);
                            }
                        },
                        option { value: "standard_feed", "Standard Feed (Default)" }
                        option { value: "mor_magazine", "Mor Magazine (Hero + Grid)" }
                        option { value: "mor_masonry", "Mor Masonry (Pinterest Grid)" }
                        option { value: "mor_minimal", "Mor Minimal (Dense List)" }
                    }

                    // 3. The Contextual UI Warning
                    if *injected_warning.read() {
                        div {
                            class: "injection-notice",
                            style: "font-size: 0.8em; color: var(--warning-color, #eab308); margin-top: 4px;",
                            "⚠️ Added dummy posts to demonstrate grid."
                        }
                    }
                }

                CompactSelect { label: "Left Sidebar", val: pack.left_sidebar_variant.clone(), options: vec![("blogger_left", "Blogger Widgets (Labels, Archive)")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.left_sidebar_variant = v; f.call(nc); } } }

                CompactSelect { label: "Right Sidebar", val: pack.right_sidebar_variant.clone(), options: vec![("toc_right", "Table of Contents")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.right_sidebar_variant = v; f.call(nc); } } }
                CompactSelect { label: "Footer Variant", val: pack.footer_variant.clone(), options: vec![("mega", "Mega Grid (Default)"), ("basic", "Basic Columns"), ("compact", "Compact Centered"), ("social", "Social Centered Row")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.footer_variant = v; f.call(nc); } } }
                div { class: "editor-card", style: "padding: 8px 12px;",
                    label { class: "editor-label", style: "display: block; margin-bottom: 4px; font-size: 0.75rem;", "JS Behaviors" }
                    select {
                        class: "editor-input", style: "width: 100%; font-size: 0.8rem; padding: 4px;",
                        value: "{pack.script_variant}",
                        onchange: {
                            let current_config = current_config.clone();
                            let on_apply_theme = on_apply_theme.clone();
                            move |evt| {
                                let mut nc = current_config.clone();
                                nc.template_pack.script_variant = evt.value();
                                on_apply_theme.call(nc);
                            }
                        },
                        option { value: "mor_collapsible_sidebars", "Mor Collapsible Sidebars" }
                        option { value: "magazine_grid_logic", "Magazine Grid Logic" }
                        option { value: "vanilla_base", "Vanilla Base (No JS)" }
                    }
                }
                button {
                    class: "editor-button",
                    onclick: move |_| crate::app::config_bridge::EditorPrefs::update_default_template_pack(current_config.template_pack.clone()),
                    "Save as Default Template"
                }
            }
        }
    }
}

#[component]
fn CompactSelect(
    label: &'static str,
    val: String,
    options: Vec<(&'static str, &'static str)>,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "editor-card", style: "padding: 8px 12px;",
            label { class: "editor-label", style: "display: block; margin-bottom: 4px; font-size: 0.75rem;", "{label}" }
            select {
                class: "editor-input", style: "width: 100%; font-size: 0.8rem; padding: 4px;",
                value: "{val}",
                onchange: move |evt| on_change.call(evt.value().clone()),
                for (id, name) in options {
                    option { value: "{id}", "{name}" }
                }
            }
        }
    }
}
