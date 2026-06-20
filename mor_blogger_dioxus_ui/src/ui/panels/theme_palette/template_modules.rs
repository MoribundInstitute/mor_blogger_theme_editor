//! Undockable Template Modules UI.
//! Allows users to hot-swap structural XML components via a docked compact view
//! or an expanded, floating grid layout.

use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

const MODULE_FLOATING_DRAG_JS: &str = r#"
(function () {
    if (window.__morModuleFloatingDragInstalled) return;
    window.__morModuleFloatingDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.module-floating-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, textarea, select, a, label')) return;

        const panel = bar.closest('.module-floating-window');
        if (!panel) return;

        e.preventDefault();

        const startX = e.clientX;
        const startY = e.clientY;
        const rect = panel.getBoundingClientRect();
        const startLeft = rect.left;
        const startTop = rect.top;

        document.body.classList.add('editor-floating-dragging');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;

            const maxLeft = Math.max(0, document.documentElement.clientWidth - rect.width);
            const maxTop = Math.max(0, document.documentElement.clientHeight - rect.height);

            panel.style.left = Math.min(maxLeft, Math.max(0, startLeft + dx)) + 'px';
            panel.style.top = Math.min(maxTop, Math.max(0, startTop + dy)) + 'px';
            panel.style.transform = 'none';
        };

        const onUp = function () {
            document.body.classList.remove('editor-floating-dragging');
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

#[derive(Clone, Copy, PartialEq)]
enum ModuleCategory {
    Header,
    MainCanvas,
    Content,
    LeftSidebar,
    RightSidebar,
    Footer,
    Scripts,
}

struct ModuleDef {
    id: &'static str,
    name: &'static str,
    desc: &'static str,
}

#[component]
pub fn TemplateModulesPanel(
    current_config: ThemeConfig,
    mut show_undocked_modules: Signal<bool>,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let pack = current_config.template_pack.clone();

    if show_undocked_modules() {
        return rsx! {
            div { class: "editor-note",
                p { class: "editor-note-title", "Panel Undocked" }
                p { class: "editor-note-body", "The template modules grid is currently floating." }
                button {
                    class: "editor-button", style: "margin-top: 8px;",
                    onclick: move |_| show_undocked_modules.set(false),
                    "Dock to Sidebar"
                }
            }
        };
    }

    rsx! {
        div { class: "editor-panel-content", style: "display: flex; flex-direction: column; gap: 16px;",

            div { style: "display: flex; align-items: flex-start; justify-content: space-between; gap: 12px;",
                p {
                    style: "margin: 0; font-size: 13px; color: var(--editor-fg-muted); line-height: 1.4;",
                    "Swap out the underlying HTML/XML layout blocks of the theme."
                }
                button {
                    style: "flex: 0 0 auto; background: rgba(236, 231, 218, 0.1); color: #ece7da; \
                            border: 1px solid rgba(236, 231, 218, 0.2); border-radius: 4px; \
                            padding: 6px 12px; font-size: 0.8rem; cursor: pointer; transition: all 0.2s;",
                    onclick: move |_| show_undocked_modules.set(true),
                    "⤢ Float Grid"
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

#[component]
pub fn TemplateModulesFloatingWindow(
    current_config: ThemeConfig,
    mut show_undocked_modules: Signal<bool>,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let mut active_category = use_signal(|| ModuleCategory::Header);
    let pack = current_config.template_pack.clone();

    // Data maps for the visual grid
    let headers = vec![
        ModuleDef {
            id: "mor",
            name: "Mor Baseline",
            desc: "The standard multi-row header with centered navigation.",
        },
        ModuleDef {
            id: "gtk_headerbar",
            name: "GTK4 Headerbar",
            desc: "A compact, desktop-style unified titlebar and navigation row.",
        },
    ];
    let main_layouts = vec![
        ModuleDef {
            id: "sidebars",
            name: "Three Column (Sidebars)",
            desc: "Classic blog layout with left and right docking panels.",
        },
        ModuleDef {
            id: "single_column",
            name: "Single Column",
            desc: "A focused, distraction-free reading environment without sidebars.",
        },
    ];
    let contents = vec![
        ModuleDef {
            id: "blog_standard",
            name: "Standard Feed",
            desc: "Chronological vertical list of full posts.",
        },
        ModuleDef {
            id: "mor_magazine",
            name: "Magazine",
            desc: "A large featured hero post followed by a structured grid.",
        },
        ModuleDef {
            id: "mor_masonry",
            name: "Masonry",
            desc: "A dense, interlocking Pinterest-style grid of post cards.",
        },
        ModuleDef {
            id: "mor_minimal",
            name: "Minimal",
            desc: "Stripped-down, text-heavy list for rapid scanning.",
        },
    ];
    let left_bars = vec![ModuleDef {
        id: "blogger_left",
        name: "Blogger Widgets",
        desc: "Native Blogger widget wrappers for Archives and Labels.",
    }];
    let right_bars = vec![ModuleDef {
        id: "toc_right",
        name: "Table of Contents",
        desc: "An empty socket ready for the Dewey Indexer plugin to inject the TOC.",
    }];
    let footers = vec![
        ModuleDef {
            id: "mega",
            name: "Mega Grid",
            desc: "Massive 6-column link directory for institutional sites.",
        },
        ModuleDef {
            id: "basic",
            name: "Basic Columns",
            desc: "A standard 4-column layout for links and resources.",
        },
        ModuleDef {
            id: "compact",
            name: "Compact Centered",
            desc: "A single minimal line for copyright and legal links.",
        },
    ];
    let scripts = vec![
        ModuleDef {
            id: "mor_panels",
            name: "Mor Sidebars",
            desc: "Includes the core framework for mobile collapsible sidebars.",
        },
        ModuleDef {
            id: "minimal",
            name: "Static Layout",
            desc: "No panel toggle behaviors. Purely static CSS grids.",
        },
    ];

    let current_selection = match active_category() {
        ModuleCategory::Header => pack.header_variant.clone(),
        ModuleCategory::MainCanvas => pack.main_variant.clone(),
        ModuleCategory::Content => pack.content_variant.clone(),
        ModuleCategory::LeftSidebar => pack.left_sidebar_variant.clone(),
        ModuleCategory::RightSidebar => pack.right_sidebar_variant.clone(),
        ModuleCategory::Footer => pack.footer_variant.clone(),
        ModuleCategory::Scripts => pack.script_variant.clone(),
    };

    let active_list = match active_category() {
        ModuleCategory::Header => headers,
        ModuleCategory::MainCanvas => main_layouts,
        ModuleCategory::Content => contents,
        ModuleCategory::LeftSidebar => left_bars,
        ModuleCategory::RightSidebar => right_bars,
        ModuleCategory::Footer => footers,
        ModuleCategory::Scripts => scripts,
    };

    rsx! {
        section {
            class: "module-floating-window floating-window",
            role: "dialog",
            style: "position: fixed; top: 88px; left: max(320px, 50% - 420px); \
                    z-index: 3100; width: 840px; height: 640px; \
                    display: flex; flex-direction: column; overflow: hidden; \
                    background: #11100e; color: #ece7da; \
                    border: 1px solid rgba(236, 231, 218, 0.22); border-radius: 4px; \
                    box-shadow: 0 24px 60px rgba(0, 0, 0, 0.7), 0 0 0 1px rgba(0, 0, 0, 0.45); \
                    font-family: 'Iowan Old Style', 'Palatino Linotype', 'Book Antiqua', Palatino, Georgia, 'Times New Roman', serif;",

            script { "{MODULE_FLOATING_DRAG_JS}" }

            // Title bar
            header {
                class: "module-floating-window-bar",
                style: "flex: 0 0 auto; display: flex; align-items: center; justify-content: space-between; padding: 14px 18px; \
                        background: #0b0a09; border-bottom: 1px solid rgba(236, 231, 218, 0.18); cursor: grab;",
                div {
                    p { style: "margin: 0 0 2px; font-family: monospace; font-size: 0.62rem; letter-spacing: 0.22em; text-transform: uppercase; color: #8c8678;", "Moribund · Structure" }
                    h2 { style: "margin: 0; font-size: 1.12rem; font-weight: 600; color: #f4f1ea;", "Template Modules" }
                }
                div { style: "display: flex; gap: 8px;",
                    button {
                        style: "background: transparent; color: #b8b2a4; border: 1px solid rgba(236, 231, 218, 0.22); border-radius: 3px; padding: 4px 10px; font-size: 0.8rem; cursor: pointer;",
                        onclick: move |_| show_undocked_modules.set(false),
                        "Dock"
                    }
                    button {
                        style: "width: 28px; height: 28px; background: transparent; color: #b8b2a4; border: 1px solid rgba(236, 231, 218, 0.22); border-radius: 3px; cursor: pointer;",
                        onclick: move |_| show_undocked_modules.set(false),
                        "✕"
                    }
                }
            }

            // Split Layout
            div {
                style: "flex: 1 1 auto; display: flex; min-height: 0;",

                // Left Navigation
                nav {
                    style: "flex: 0 0 220px; display: flex; flex-direction: column; \
                            border-right: 1px solid rgba(236, 231, 218, 0.18); padding: 18px;",

                    div { style: "display: flex; flex-direction: column; gap: 4px; flex: 1 1 auto;",
                        CategoryButton { label: "Header Variant", active: active_category() == ModuleCategory::Header, on_click: move |_| active_category.set(ModuleCategory::Header) }
                        CategoryButton { label: "Main Canvas", active: active_category() == ModuleCategory::MainCanvas, on_click: move |_| active_category.set(ModuleCategory::MainCanvas) }
                        CategoryButton { label: "Content Layout", active: active_category() == ModuleCategory::Content, on_click: move |_| active_category.set(ModuleCategory::Content) }
                        CategoryButton { label: "Left Sidebar", active: active_category() == ModuleCategory::LeftSidebar, on_click: move |_| active_category.set(ModuleCategory::LeftSidebar) }
                        CategoryButton { label: "Right Sidebar", active: active_category() == ModuleCategory::RightSidebar, on_click: move |_| active_category.set(ModuleCategory::RightSidebar) }
                        CategoryButton { label: "Footer Variant", active: active_category() == ModuleCategory::Footer, on_click: move |_| active_category.set(ModuleCategory::Footer) }
                        CategoryButton { label: "JS Behaviors", active: active_category() == ModuleCategory::Scripts, on_click: move |_| active_category.set(ModuleCategory::Scripts) }
                    }

                    // Discover Link Block
                    div {
                        style: "margin-top: 18px; padding-top: 18px; border-top: 1px dashed rgba(236, 231, 218, 0.18);",
                        p { style: "margin: 0 0 10px; font-size: 0.8rem; color: #f4f1ea; font-weight: 600;", "Download More Layouts" }
                        p { style: "margin: 0 0 10px; font-size: 0.75rem; color: #a8a294; line-height: 1.4;", "Browse the official compendiums for new XML snippets." }
                        a {
                            href: "https://morxml.blogspot.com/",
                            target: "_blank",
                            style: "display: block; text-align: center; background: #ece7da; color: #11100e; \
                                    text-decoration: none; padding: 8px; border-radius: 3px; font-weight: 600; font-size: 0.85rem; margin-bottom: 8px;",
                            "⇱ View XML Catalog"
                        }
                        a {
                            href: "https://github.com/MoribundInstitute/mor-xml-compendium",
                            target: "_blank",
                            style: "display: block; text-align: center; background: transparent; color: #a8a294; \
                                    text-decoration: underline; padding: 4px; font-size: 0.75rem;",
                            "GitHub Repository"
                        }
                    }
                }

                // Right Grid Area
                div {
                    style: "flex: 1 1 auto; overflow-y: auto; padding: 18px; background: #0f0e0c;",

                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 16px;",
                        for module in active_list {
                            div {
                                key: "{module.id}",
                                style: format!("display: flex; flex-direction: column; justify-content: space-between; \
                                                padding: 16px; background: #16140f; border-radius: 4px; cursor: pointer; \
                                                transition: all 0.2s; border: 2px solid {};",
                                                if current_selection == module.id { "#ece7da" } else { "rgba(236, 231, 218, 0.12)" }),
                                onclick: {
                                    let id = module.id;
                                    let config_clone = current_config.clone();
                                    let on_apply = on_apply_theme.clone();
                                    let cat = active_category();
                                    move |_| {
                                        let mut new_config = config_clone.clone();
                                        match cat {
                                            ModuleCategory::Header => new_config.template_pack.header_variant = id.to_string(),
                                            ModuleCategory::MainCanvas => new_config.template_pack.main_variant = id.to_string(),
                                            ModuleCategory::Content => new_config.template_pack.content_variant = id.to_string(),
                                            ModuleCategory::LeftSidebar => new_config.template_pack.left_sidebar_variant = id.to_string(),
                                            ModuleCategory::RightSidebar => new_config.template_pack.right_sidebar_variant = id.to_string(),
                                            ModuleCategory::Footer => new_config.template_pack.footer_variant = id.to_string(),
                                            ModuleCategory::Scripts => new_config.template_pack.script_variant = id.to_string(),
                                        }
                                        on_apply.call(new_config);
                                    }
                                },

                                div {
                                    h3 { style: "margin: 0 0 6px; font-size: 1.05rem; font-weight: 600; color: #f4f1ea;", "{module.name}" }
                                    p { style: "margin: 0; font-size: 0.85rem; line-height: 1.5; color: #a8a294;", "{module.desc}" }
                                }

                                div {
                                    style: "margin-top: 16px; font-family: monospace; font-size: 0.75rem; text-align: right;",
                                    if current_selection == module.id {
                                        span { style: "color: #73c991; font-weight: bold;", "● Active" }
                                    } else {
                                        span { style: "color: #8c8678;", "○ Select" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CategoryButton(
    label: &'static str,
    active: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            style: format!("text-align: left; padding: 8px 12px; border: none; cursor: pointer; border-radius: 3px; \
                            font-family: inherit; font-size: 0.95rem; transition: background 0.2s; \
                            background: {}; color: {}; font-weight: {};",
                            if active { "rgba(236, 231, 218, 0.08)" } else { "transparent" },
                            if active { "#f4f1ea" } else { "#a8a294" },
                            if active { "600" } else { "400" }),
            onclick: move |evt| on_click.call(evt),
            "{label}"
        }
    }
}
