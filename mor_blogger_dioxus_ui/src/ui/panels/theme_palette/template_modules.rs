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
                CompactSelect { label: "Header Variant", val: pack.header_variant.clone(), options: vec![("mor", "Mor (Default)"), ("gtk_headerbar", "GTK4 Headerbar"), ("minimal", "Minimal Flexbox")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.header_variant = v; f.call(nc); } } }
                CompactSelect { label: "Main Canvas", val: pack.main_variant.clone(), options: vec![("sidebars", "Three Column (Sidebars)"), ("single_column", "Single Column"), ("two_column_right", "Two Column Right CSS Grid")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.main_variant = v; f.call(nc); } } }
                CompactSelect { label: "Content Layout", val: pack.content_variant.clone(), options: vec![("blog_standard", "Standard Feed (Default)"), ("mor_magazine", "Mor Magazine (Hero + Grid)"), ("mor_masonry", "Mor Masonry (Pinterest Grid)"), ("mor_minimal", "Mor Minimal (Dense List)")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.content_variant = v; f.call(nc); } } }
                CompactSelect { label: "Left Sidebar", val: pack.left_sidebar_variant.clone(), options: vec![("blogger_left", "Blogger Widgets (Labels, Archive)")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.left_sidebar_variant = v; f.call(nc); } } }
                CompactSelect { label: "Right Sidebar", val: pack.right_sidebar_variant.clone(), options: vec![("toc_right", "Table of Contents")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.right_sidebar_variant = v; f.call(nc); } } }
                CompactSelect { label: "Footer Variant", val: pack.footer_variant.clone(), options: vec![("mega", "Mega Grid (Default)"), ("basic", "Basic Columns"), ("compact", "Compact Centered"), ("social", "Social Centered Row")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.footer_variant = v; f.call(nc); } } }
                CompactSelect { label: "JS Behaviors", val: pack.script_variant.clone(), options: vec![("mor_panels", "Mor Collapsible Sidebars"), ("minimal", "None (Static Layout)")], on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.script_variant = v; f.call(nc); } } }
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
