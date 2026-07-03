use crate::app::state::ThemeState;
use crate::ui::dialogs::modal::Modal;
use crate::ui::panels::theme_palette::template_modules::{
    ModuleFileButton, CONTENT_LAYOUTS, FOOTERS, HEADERS, JS_BEHAVIORS, LEFT_SIDEBARS,
    MAIN_CANVASES, RIGHT_SIDEBARS,
};
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

#[derive(Props, Clone, PartialEq)]
pub struct TemplateGridDialogProps {
    pub open: Signal<bool>,
}

#[component]
pub fn TemplateGridDialog(props: TemplateGridDialogProps) -> Element {
    let mut open_signal = props.open;
    let mut theme = use_context::<ThemeState>();
    let mut active_category = use_signal(|| ModuleCategory::Header);
    let file_status = use_signal(String::new);
    let pack = theme.signals.template_pack.read().clone();

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
        ModuleCategory::Header => HEADERS,
        ModuleCategory::MainCanvas => MAIN_CANVASES,
        ModuleCategory::Content => CONTENT_LAYOUTS,
        ModuleCategory::LeftSidebar => LEFT_SIDEBARS,
        ModuleCategory::RightSidebar => RIGHT_SIDEBARS,
        ModuleCategory::Footer => FOOTERS,
        ModuleCategory::Scripts => JS_BEHAVIORS,
    };

    // No file override for Scripts: the resolver's module_override doesn't cover script_variant.
    let (file_slot_key, file_slot_label) = match active_category() {
        ModuleCategory::Header => (Some("header_variant"), "Header Variant"),
        ModuleCategory::MainCanvas => (Some("main_variant"), "Main Canvas"),
        ModuleCategory::Content => (Some("content_variant"), "Content Layout"),
        ModuleCategory::LeftSidebar => (Some("left_sidebar_variant"), "Left Sidebar"),
        ModuleCategory::RightSidebar => (Some("right_sidebar_variant"), "Right Sidebar"),
        ModuleCategory::Footer => (Some("footer_variant"), "Footer Variant"),
        ModuleCategory::Scripts => (None, "JS Behaviors"),
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
                            border-right: 1px solid var(--editor-border-soft); padding: 18px;",

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
                        style: "margin-top: 18px; padding-top: 18px; border-top: 1px dashed var(--editor-border-soft);",
                        p { style: "margin: 0 0 10px; font-size: 0.8rem; color: var(--editor-text); font-weight: 600;", "Download More Layouts" }
                        p { style: "margin: 0 0 10px; font-size: 0.75rem; color: var(--editor-muted); line-height: 1.4;", "Browse the official compendiums for new XML snippets." }
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
                            style: "display: block; text-align: center; background: transparent; color: var(--editor-muted); \
                                    text-decoration: underline; padding: 4px; font-size: 0.75rem;",
                            "GitHub Repository"
                        }
                    }
                }

                // Right Grid Area
                div {
                    style: "flex: 1 1 auto; overflow-y: auto; padding: 18px; background: var(--editor-bg);",

                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 14px;",
                        if let Some(slot_key) = file_slot_key {
                            p { style: "margin: 0; font-size: 0.8rem; color: var(--editor-muted);",
                                "Pick a built-in variant, or load your own XML from a file."
                            }
                            ModuleFileButton {
                                module_key: slot_key,
                                label: file_slot_label,
                                status: file_status,
                                on_loaded: move |_| {
                                    // Rewriting the pack forces a preview re-render so the
                                    // freshly saved custom_<key>.xml override is picked up.
                                    let pack = theme.signals.template_pack.read().clone();
                                    theme.signals.template_pack.set(pack);
                                },
                            }
                        } else {
                            p { style: "margin: 0; font-size: 0.8rem; color: var(--editor-muted);",
                                "Pick a behavior, or override a bundled script file with your own."
                            }
                            JsFileButton { status: file_status }
                        }
                    }

                    if !file_status().is_empty() {
                        div { class: "restore-status", style: "margin-bottom: 14px;", "{file_status}" }
                    }

                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 16px;",
                        for module in active_list {
                            div {
                                key: "{module.id}",
                                style: format!("display: flex; flex-direction: column; justify-content: space-between; \
                                                padding: 16px; background: var(--editor-panel); border-radius: 4px; cursor: pointer; \
                                                transition: all 0.2s; border: 2px solid {};",
                                                if current_selection == module.id { "var(--mor-accent-hover)" } else { "var(--editor-border-soft)" }),
                                onclick: {
                                    let id = module.id.to_string();
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
                                    h3 { style: "margin: 0 0 6px; font-size: 1.05rem; font-weight: 600; color: var(--editor-text);", "{module.name}" }
                                    p { style: "margin: 0; font-size: 0.85rem; line-height: 1.5; color: var(--editor-muted);", "{module.desc}" }
                                }

                                div {
                                    style: "margin-top: 16px; font-family: monospace; font-size: 0.75rem; text-align: right;",
                                    if current_selection == module.id {
                                        span { style: "color: var(--editor-good); font-weight: bold;", "● Active" }
                                    } else {
                                        span { style: "color: var(--editor-muted);", "○ Select" }
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

/// Native picker for a JS override. Scripts ship as an aggregated bundle of known
/// filenames, so the picked file must be named after the bundle file it replaces
/// (e.g. 07-Theme-Toggler.js). It lands in the VFS (instant preview) and the js/
/// folder on disk (persists), same as saving from the JS Editor.
#[component]
fn JsFileButton(status: Signal<String>) -> Element {
    use mor_blogger_core::render::template_resolver::{CORE_JS_FILES, MAGAZINE_GRID_JS};
    let vfs = use_context::<crate::app::vfs::VfsDictionary>().0;
    rsx! {
        button {
            class: "editor-mini-button",
            style: "padding: 2px 8px; min-height: 22px; font-size: 0.75rem;",
            title: "Load a .js file overriding a bundled script (must keep the bundle filename, e.g. 01-Core-Helpers.js)",
            onclick: move |_| {
                let mut status = status;
                let mut vfs = vfs;
                let start_dir = mor_blogger_core::utils::fs_bridge::js_root();
                spawn(async move {
                    let mut dlg = rfd::AsyncFileDialog::new()
                        .add_filter("JavaScript", &["js"])
                        .set_title("Load JS bundle override");
                    if let Some(dir) = start_dir {
                        dlg = dlg.set_directory(dir);
                    }
                    let Some(handle) = dlg.pick_file().await else { return };
                    let name = handle.file_name();
                    if !CORE_JS_FILES.contains(&name.as_str()) && name != MAGAZINE_GRID_JS {
                        status.set(format!(
                            "{name} won't ship: JS overrides must be named after a bundled file ({}, {MAGAZINE_GRID_JS}).",
                            CORE_JS_FILES.join(", ")
                        ));
                        return;
                    }
                    let content = match std::fs::read_to_string(handle.path()) {
                        Ok(c) => c,
                        Err(e) => { status.set(format!("Could not read file: {e}")); return; }
                    };
                    match mor_blogger_core::utils::fs_bridge::save_custom_js(&name, &content) {
                        Ok(_) => {
                            vfs.write().insert(name.clone(), content);
                            status.set(format!("{name} loaded — overrides the bundled script. Reset it in the JS Editor."));
                        }
                        Err(e) => status.set(format!("Load failed: {e}")),
                    }
                });
            },
            "📂"
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
                            if active { "var(--mor-btn-hover)" } else { "transparent" },
                            if active { "var(--editor-text)" } else { "var(--editor-muted)" },
                            if active { "600" } else { "400" }),
            onclick: on_click,
            "{label}"
        }
    }
}
