use crate::app::state::ThemeState;
use crate::ui::components::modal::Modal;
use dioxus::prelude::*;

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

#[derive(Props, Clone, PartialEq)]
pub struct TemplateGridDialogProps {
    pub open: Signal<bool>,
}

#[component]
pub fn TemplateGridDialog(props: TemplateGridDialogProps) -> Element {
    let mut open_signal = props.open;
    let theme = use_context::<ThemeState>();
    let mut active_category = use_signal(|| ModuleCategory::Header);
    let pack = theme.signals.template_pack.read().clone();

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
        Modal {
            open: open_signal,
            title: "Advanced Module Options",
            style: "width: 800px; height: 600px; max-width: 800px;".to_string(),
            on_close: move |_| open_signal.set(false),

            div {
                class: "split-pane-container",
                style: "display: flex; height: 500px; min-height: 0;",

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
                                    let id = module.id.to_string();
                                    let mut theme = theme;
                                    let cat = active_category();
                                    move |_| {
                                        let mut pack = theme.signals.template_pack.read().clone();
                                        match cat {
                                            ModuleCategory::Header => pack.header_variant = id.clone(),
                                            ModuleCategory::MainCanvas => pack.main_variant = id.clone(),
                                            ModuleCategory::Content => pack.content_variant = id.clone(),
                                            ModuleCategory::LeftSidebar => pack.left_sidebar_variant = id.clone(),
                                            ModuleCategory::RightSidebar => pack.right_sidebar_variant = id.clone(),
                                            ModuleCategory::Footer => pack.footer_variant = id.clone(),
                                            ModuleCategory::Scripts => pack.script_variant = id.clone(),
                                        }
                                        theme.signals.template_pack.set(pack);
                                        theme.active_preset.set(None);
                                        theme.commit();
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
