//! Compact Template Modules UI.
//! Allows users to hot-swap structural XML components via a docked compact view.

use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

/// Single source of truth for the built-in module variants. Both the compact
/// docked panel and the Advanced Module Options dialog render from these lists.
pub(crate) struct ModuleDef {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
}

pub(crate) const HEADERS: &[ModuleDef] = &[
    ModuleDef { id: "mor", name: "Mor (Default)", desc: "The standard multi-row header with centered navigation." },
    ModuleDef { id: "mor_search_center", name: "Mor — Centered Search", desc: "Multi-row header with a centered, rounded, slightly larger search bar." },
    ModuleDef { id: "gtk_headerbar", name: "GTK4 Headerbar", desc: "A compact, desktop-style unified titlebar and navigation row." },
    ModuleDef { id: "minimal", name: "Minimal Flexbox", desc: "A lean single-stack flexbox header with branding and links." },
];

pub(crate) const MAIN_CANVASES: &[ModuleDef] = &[
    ModuleDef { id: "sidebars", name: "Three Column (Sidebars)", desc: "Classic blog layout with left and right docking panels." },
    ModuleDef { id: "single_column", name: "Single Column", desc: "A focused, distraction-free reading environment without sidebars." },
    ModuleDef { id: "two_column_right", name: "Two Column Right CSS Grid", desc: "Content with a single right rail on a CSS grid." },
];

pub(crate) const CONTENT_LAYOUTS: &[ModuleDef] = &[
    ModuleDef { id: "standard_feed", name: "Standard Feed (Default)", desc: "Chronological vertical list of full posts." },
    ModuleDef { id: "mor_magazine", name: "Mor Magazine (Hero + Grid)", desc: "A large featured hero post followed by a structured grid." },
    ModuleDef { id: "mor_masonry", name: "Mor Masonry (Pinterest Grid)", desc: "A dense, interlocking Pinterest-style grid of post cards." },
    ModuleDef { id: "mor_minimal", name: "Mor Minimal (Dense List)", desc: "Stripped-down, text-heavy list for rapid scanning." },
];

pub(crate) const LEFT_SIDEBARS: &[ModuleDef] = &[
    ModuleDef { id: "blogger_left", name: "Blogger Widgets (Labels, Archive)", desc: "Native Blogger widget wrappers for Archives and Labels." },
];

pub(crate) const RIGHT_SIDEBARS: &[ModuleDef] = &[
    ModuleDef { id: "toc_right", name: "Table of Contents", desc: "An empty socket ready for the Dewey Indexer plugin to inject the TOC." },
];

pub(crate) const FOOTERS: &[ModuleDef] = &[
    ModuleDef { id: "mega", name: "Mega Grid (Default)", desc: "Massive 6-column link directory for institutional sites." },
    ModuleDef { id: "basic", name: "Basic Columns", desc: "A standard 4-column layout for links and resources." },
    ModuleDef { id: "compact", name: "Compact Centered", desc: "A single minimal line for copyright and legal links." },
    ModuleDef { id: "social", name: "Social Centered Row", desc: "A centered row of social links with the copyright line." },
];

// Layout-specific scripts (e.g. the magazine grid reveal) are not behaviors —
// they ship automatically via the owning module's js_deps.
pub(crate) const JS_BEHAVIORS: &[ModuleDef] = &[
    ModuleDef { id: "mor_collapsible_sidebars", name: "Mor Collapsible Sidebars", desc: "Includes the core framework for mobile collapsible sidebars." },
    ModuleDef { id: "vanilla_base", name: "Vanilla Base (No JS)", desc: "No panel toggle behaviors. Purely static CSS grids." },
];

fn options_of(defs: &'static [ModuleDef]) -> Vec<(&'static str, &'static str)> {
    defs.iter().map(|m| (m.id, m.name)).collect()
}

#[component]
pub fn TemplateModulesPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let mut theme_state = use_context::<crate::app::state::LayoutState>();
    let mut site_data = use_context::<Signal<crate::app::state::SiteData>>();
    let mut injected_warning = use_signal(|| false);
    let file_status = use_signal(String::new);
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
                CompactSelect { label: "Header Variant", val: pack.header_variant.clone(), options: options_of(HEADERS), module_key: Some("header_variant"), status: Some(file_status), on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.header_variant = v; f.call(nc); } } }
                CompactSelect { label: "Main Canvas", val: pack.main_variant.clone(), options: options_of(MAIN_CANVASES), module_key: Some("main_variant"), status: Some(file_status), on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.main_variant = v; f.call(nc); } } }

                div { class: "editor-card", style: "padding: 8px 12px;",
                    div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 4px;",
                        label { class: "editor-label", style: "font-size: 0.75rem;", "Content Layout" }
                        ModuleFileButton {
                            module_key: "content_variant",
                            label: "Content Layout",
                            status: file_status,
                            on_loaded: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |_| f.call(c.clone()) },
                        }
                    }
                    select {
                        class: "editor-input", style: "width: 100%; font-size: 0.8rem; padding: 4px;",
                        value: "{pack.content_variant}",
                        onchange: {
                            let current_config = current_config.clone();
                            let on_apply_theme = on_apply_theme.clone();
                            move |evt| {
                                let selected = evt.value();

                                // 1. Update the core layout state. Layout-specific JS
                                // ships via the module's js_deps — no behavior swap needed.
                                let mut nc = current_config.clone();
                                nc.template_pack.content_variant = selected.clone();

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
                        for m in CONTENT_LAYOUTS {
                            option { value: "{m.id}", "{m.name}" }
                        }
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

                CompactSelect { label: "Left Sidebar", val: pack.left_sidebar_variant.clone(), options: options_of(LEFT_SIDEBARS), module_key: Some("left_sidebar_variant"), status: Some(file_status), on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.left_sidebar_variant = v; f.call(nc); } } }

                CompactSelect { label: "Right Sidebar", val: pack.right_sidebar_variant.clone(), options: options_of(RIGHT_SIDEBARS), module_key: Some("right_sidebar_variant"), status: Some(file_status), on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.right_sidebar_variant = v; f.call(nc); } } }
                CompactSelect { label: "Footer Variant", val: pack.footer_variant.clone(), options: options_of(FOOTERS), module_key: Some("footer_variant"), status: Some(file_status), on_change: { let c = current_config.clone(); let f = on_apply_theme.clone(); move |v| { let mut nc = c.clone(); nc.template_pack.footer_variant = v; f.call(nc); } } }
                if !file_status().is_empty() {
                    div { class: "restore-status", "{file_status}" }
                }
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
                        for m in JS_BEHAVIORS {
                            option { value: "{m.id}", "{m.name}" }
                        }
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
    #[props(default)] module_key: Option<&'static str>,
    #[props(default)] status: Option<Signal<String>>,
) -> Element {
    // Re-applying the unchanged value forces a preview re-render, which picks up
    // the freshly written custom_<key>.xml override from disk.
    let val_for_reload = val.clone();
    rsx! {
        div { class: "editor-card", style: "padding: 8px 12px;",
            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 4px;",
                label { class: "editor-label", style: "font-size: 0.75rem;", "{label}" }
                if let (Some(key), Some(status)) = (module_key, status) {
                    ModuleFileButton {
                        module_key: key,
                        label,
                        status,
                        on_loaded: move |_| on_change.call(val_for_reload.clone()),
                    }
                }
            }
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

/// Native file picker that installs the chosen XML as this slot's
/// custom_<key>.xml override (same file the Module Workbench saves to).
#[component]
pub(crate) fn ModuleFileButton(
    module_key: &'static str,
    label: &'static str,
    status: Signal<String>,
    on_loaded: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "editor-mini-button",
            style: "padding: 2px 8px; min-height: 22px; font-size: 0.75rem;",
            title: "Load {label} XML from a file (overrides the dropdown until reset in the Module Workbench)",
            onclick: move |_| {
                let mut status = status;
                let category = crate::ui::workspace::module_workbench::module_key_to_category(module_key);
                let start_dir = mor_blogger_core::utils::fs_bridge::category_dir(category)
                    .or_else(mor_blogger_core::utils::fs_bridge::templates_root);
                spawn(async move {
                    let mut dlg = rfd::AsyncFileDialog::new()
                        .add_filter("Blogger module XML", &["xml"])
                        .set_title(format!("Load {label} module XML"));
                    if let Some(dir) = start_dir {
                        dlg = dlg.set_directory(dir);
                    }
                    let Some(handle) = dlg.pick_file().await else { return };
                    let content = match std::fs::read_to_string(handle.path()) {
                        Ok(c) => c,
                        Err(e) => { status.set(format!("Could not read file: {e}")); return; }
                    };
                    match mor_blogger_core::utils::fs_bridge::save_custom_module(category, &format!("custom_{module_key}"), &content) {
                        Ok(_) => {
                            status.set(format!("{label} loaded from file — overrides the dropdown until reset in the Module Workbench."));
                            on_loaded.call(());
                        }
                        Err(e) => status.set(format!("Load failed: {e}")),
                    }
                });
            },
            "📂"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mor_blogger_core::render::template_resolver::{
        ComponentManifest, CONTENT_REGISTRY, FOOTER_REGISTRY, HEADER_REGISTRY, LAYOUT_REGISTRY,
        SIDEBAR_LEFT_REGISTRY, SIDEBAR_RIGHT_REGISTRY,
    };

    // The dropdown ids here and the resolver registry ids are two lists that
    // must agree; a dangling id renders a blank dropdown (the
    // "mor_header_baseline" bug). Every UI option must resolve in core.
    // (Core may carry extra non-user-facing variants, e.g. GTK-import ones,
    // so this is subset, not equality.)
    #[test]
    fn ui_module_ids_exist_in_core_registries() {
        let check = |defs: &[ModuleDef], reg: &[ComponentManifest], slot: &str| {
            for d in defs {
                assert!(
                    reg.iter().any(|c| c.id == d.id),
                    "{slot} option '{}' has no core registry entry",
                    d.id
                );
            }
        };
        check(HEADERS, HEADER_REGISTRY, "header");
        check(MAIN_CANVASES, LAYOUT_REGISTRY, "main");
        check(CONTENT_LAYOUTS, CONTENT_REGISTRY, "content");
        check(LEFT_SIDEBARS, SIDEBAR_LEFT_REGISTRY, "left sidebar");
        check(RIGHT_SIDEBARS, SIDEBAR_RIGHT_REGISTRY, "right sidebar");
        check(FOOTERS, FOOTER_REGISTRY, "footer");
    }
}
