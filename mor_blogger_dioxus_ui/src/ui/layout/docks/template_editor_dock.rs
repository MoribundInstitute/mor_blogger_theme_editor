use dioxus::prelude::*;
use mor_blogger_core::utils::fs_bridge;

use crate::app::shell::WorkbenchEditState;
use crate::app::state::{DockPosition, LayoutState};
use crate::ui::components::dock_chrome::DockChrome;

// De-facto module registry: there is no central registry type, this static list
// is the same one used by the workbench and smart-code docks.
const TEMPLATE_LAYOUTS: &[(&str, &str)] = &[
    ("Header Variant", "header_variant"),
    ("Main Canvas Variant", "main_variant"),
    ("Content Feed", "content_variant"),
    ("Left Sidebar", "left_sidebar_variant"),
    ("Right Sidebar", "right_sidebar_variant"),
    ("Footer Grid", "footer_variant"),
];

// The online template-module library: gallery, source repo, and machine index.
const COMPENDIUM_SITE_URL: &str = "https://morxml.blogspot.com/";
const COMPENDIUM_REPO_URL: &str = "https://github.com/MoribundInstitute/mor-xml-compendium";
const COMPENDIUM_INDEX_URL: &str =
    "https://raw.githubusercontent.com/MoribundInstitute/mor-xml-compendium/main/registry.json";

/// Map a workbench module slot to its templates sub-folder (mirrors the workbench).
fn key_to_category(key: &str) -> &'static str {
    match key {
        "header_variant" => "headers",
        "main_variant" => "layouts",
        "content_variant" => "content",
        "left_sidebar_variant" | "right_sidebar_variant" => "sidebars",
        "footer_variant" => "footers",
        _ => "custom",
    }
}

/// Guess which module slot a file belongs to from a lowercased hint (a filename,
/// or "<parent-folder>/<filename>" for picked files where the folder is reliable).
/// Falls back to the currently-selected slot when nothing matches. `layout` and
/// `column` are checked before `sidebar` so layout files like `sidebars.xml` /
/// `two_column_right.xml` aren't mistaken for sidebar modules.
pub(crate) fn infer_module_slot(
    hint_lower: &str,
    current: Option<&'static str>,
) -> Option<&'static str> {
    let h = hint_lower;
    if h.contains("footer") {
        Some("footer_variant")
    } else if h.contains("header") {
        Some("header_variant")
    } else if h.contains("layout") || h.contains("canvas") || h.contains("column") {
        Some("main_variant")
    } else if h.contains("content") || h.contains("feed") || h.contains("masonry") {
        Some("content_variant")
    } else if h.contains("right") && h.contains("sidebar") {
        Some("right_sidebar_variant")
    } else if h.contains("sidebar") || h.contains("left") {
        Some("left_sidebar_variant")
    } else {
        current
    }
}

/// Map a compendium entry `type` to (workbench slot, local templates category).
/// Returns None for non-module entries (e.g. css_snippet), which can't install
/// into a structural slot.
fn slot_for_type(ty: &str) -> Option<(&'static str, &'static str)> {
    match ty {
        "header" => Some(("header_variant", "headers")),
        "footer" => Some(("footer_variant", "footers")),
        "main_layout" => Some(("main_variant", "layouts")),
        "content" => Some(("content_variant", "content")),
        "sidebar_left" => Some(("left_sidebar_variant", "sidebars")),
        "sidebar_right" => Some(("right_sidebar_variant", "sidebars")),
        _ => None,
    }
}

/// One entry from the compendium's `registry.json`.
#[derive(serde::Deserialize, Clone, PartialEq)]
struct RemoteModule {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    payload_url: String,
}

#[cfg(test)]
mod tests {
    use super::infer_module_slot;

    #[test]
    fn infers_slot_from_folder_and_name_hints() {
        let f = |h: &str| infer_module_slot(h, None);
        assert_eq!(f("footers/morfootermega.xml"), Some("footer_variant"));
        assert_eq!(f("headers/mor_header_baseline.xml"), Some("header_variant"));
        assert_eq!(f("content/mor_masonry.xml"), Some("content_variant"));
        // layout/column beat "sidebar": a layout file named sidebars.xml is a layout.
        assert_eq!(f("layouts/sidebars.xml"), Some("main_variant"));
        assert_eq!(f("layouts/two_column_right.xml"), Some("main_variant"));
        // real sidebars still resolve, left vs right.
        assert_eq!(f("sidebars/toc_right.xml"), Some("right_sidebar_variant"));
        assert_eq!(f("sidebars/blogger_left.xml"), Some("left_sidebar_variant"));
        // no keyword → fall back to the currently-selected slot.
        assert_eq!(infer_module_slot("mystery.xml", Some("footer_variant")), Some("footer_variant"));
        assert_eq!(infer_module_slot("mystery.xml", None), None);
    }
}

#[component]
pub fn TemplateModulesDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let mut edit_state = use_context::<WorkbenchEditState>();
    let pos = (layout.template_modules_pos)();
    let mut import_url = use_signal(String::new);
    let mut compendium = use_signal(Vec::<RemoteModule>::new);

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "TEMPLATE MODULES".to_string(),
                dock_id: "template_modules".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.template_modules_pos.set(DockPosition::Hidden);
                },
                div {
                    class: "palette-content template-modules",
                    style: "padding: 12px; height: calc(100% - 45px); overflow-y: auto; display: flex; flex-direction: column; gap: 6px; background: var(--bg-panel); color: var(--fg-base);",
                    for (label, key) in TEMPLATE_LAYOUTS {
                        button {
                            class: if (layout.active_workbench_module)() == Some(key) { "template-module-btn editor-button editor-button-active" } else { "template-module-btn editor-button" },
                            onclick: {
                                let key = *key;
                                move |_| {
                                    layout.active_workbench_module.set(Some(key));
                                    edit_state.edited_xml.set(String::new());
                                }
                            },
                            "{label}"
                        }
                    }

                    // ── Import from the XML compendium ────────────────────
                    div {
                        style: "margin-top: 14px; padding-top: 10px; border-top: 1px solid var(--border-color); display: flex; flex-direction: column; gap: 6px;",
                        span { style: "color: var(--fg-muted); font-size: 0.64rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;", "Import module" }
                        input {
                            class: "editor-input",
                            style: "width: 100%; font-size: 0.78rem; padding: 5px 6px;",
                            r#type: "text",
                            placeholder: "Paste module XML URL…",
                            value: "{import_url}",
                            oninput: move |e| import_url.set(e.value()),
                        }
                        button {
                            class: "editor-button editor-button-small",
                            title: "Fetch the module XML and load it into the selected slot's override",
                            onclick: move |_| {
                                let Some(key) = (layout.active_workbench_module)() else {
                                    edit_state.workbench_status.set("Select a module slot above first.".to_string());
                                    return;
                                };
                                let raw = import_url();
                                if raw.trim().is_empty() {
                                    edit_state.workbench_status.set("Paste a module XML URL first.".to_string());
                                    return;
                                }
                                let url = crate::ui::panels::theme_palette::presets::importers::normalize_preset_url(&raw);
                                let category = key_to_category(key);
                                let mut status = edit_state.workbench_status;
                                let mut layout2 = layout;
                                spawn(async move {
                                    status.set("Importing…".to_string());
                                    match crate::ui::panels::theme_palette::static_pages_panel::fetch_raw_page(&url).await {
                                        Ok(xml) => match fs_bridge::save_custom_module(category, &format!("custom_{}", key), &xml) {
                                            Ok(_) => {
                                                status.set(format!("Imported into {key}."));
                                                layout2.enter_workspace(crate::app::state::CenterView::ModuleWorkbench);
                                            }
                                            Err(e) => status.set(format!("Save failed: {e}")),
                                        },
                                        Err(e) => status.set(format!("Import failed: {e}")),
                                    }
                                });
                            },
                            "Import into Selected Slot"
                        }
                    }

                    // ── Compendium install picker (appears once loaded) ──
                    if !compendium().is_empty() {
                        div {
                            style: "margin-top: 10px; padding-top: 8px; border-top: 1px solid var(--border-color); display: flex; flex-direction: column; gap: 4px;",
                            div {
                                style: "display: flex; align-items: center; gap: 6px;",
                                span { style: "flex: 1; color: var(--fg-muted); font-size: 0.62rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;", "Compendium ({compendium().len()})" }
                                button { class: "editor-mini-button", title: "Hide", onclick: move |_| compendium.set(Vec::new()), "×" }
                            }
                            for rm in compendium() {
                                {
                                    let rm_install = rm.clone();
                                    let mapped = slot_for_type(&rm.kind);
                                    let type_label = rm.kind.replace('_', " ");
                                    rsx! {
                                        div {
                                            key: "{rm.kind}-{rm.display_name}",
                                            style: "display: flex; align-items: center; gap: 6px; padding: 3px 4px; font-size: 0.76rem;",
                                            span { style: "flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", title: "{rm.kind}", "{rm.display_name}" }
                                            span { style: "color: var(--fg-muted); font-size: 0.62rem; font-family: var(--font-mono);", "{type_label}" }
                                            if let Some((slot, category)) = mapped {
                                                button {
                                                    class: "editor-mini-button editor-mini-button-active",
                                                    title: "Install into this slot's override",
                                                    onclick: move |_| {
                                                        let rm = rm_install.clone();
                                                        let mut status = edit_state.workbench_status;
                                                        let mut layout2 = layout;
                                                        spawn(async move {
                                                            status.set(format!("Installing {}…", rm.display_name));
                                                            match crate::ui::panels::theme_palette::static_pages_panel::fetch_raw_page(&rm.payload_url).await {
                                                                Ok(xml) => match fs_bridge::save_custom_module(category, &format!("custom_{}", slot), &xml) {
                                                                    Ok(_) => {
                                                                        layout2.active_workbench_module.set(Some(slot));
                                                                        status.set(format!("Installed {}", rm.display_name));
                                                                        layout2.enter_workspace(crate::app::state::CenterView::ModuleWorkbench);
                                                                    }
                                                                    Err(e) => status.set(format!("Save failed: {e}")),
                                                                },
                                                                Err(e) => status.set(format!("Fetch failed: {e}")),
                                                            }
                                                        });
                                                    },
                                                    "Install"
                                                }
                                            } else {
                                                span { style: "color: var(--fg-muted); font-size: 0.62rem; font-style: italic;", "CSS · n/a" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Folder + compendium links ─────────────────────────
                    div {
                        style: "margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--border-color); display: flex; flex-direction: column; gap: 6px;",
                        button {
                            class: "template-modules-folder-btn editor-button",
                            title: "Open the user-space templates directory in the system file manager",
                            onclick: move |_| {
                                match fs_bridge::open_templates_folder() {
                                    Ok(()) => edit_state.workbench_status.set("Templates folder opened.".to_string()),
                                    Err(e) => edit_state.workbench_status.set(format!("Could not open folder: {}", e)),
                                }
                            },
                            "Open Templates Folder"
                        }
                        button {
                            class: "editor-button",
                            title: "Open a module .xml file from disk into the workspace for editing",
                            onclick: move |_| {
                                let mut layout2 = layout;
                                let mut edit2 = edit_state;
                                spawn(async move {
                                    let mut dlg = rfd::AsyncFileDialog::new().add_filter("Module XML", &["xml"]);
                                    if let Some(dir) = fs_bridge::templates_root() { dlg = dlg.set_directory(dir); }
                                    let Some(handle) = dlg.pick_file().await else { return; };
                                    let name = handle.file_name();
                                    // Parent folder (headers/footers/…) is a reliable category hint.
                                    let parent = handle.path().parent()
                                        .and_then(|p| p.file_name())
                                        .and_then(|s| s.to_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let xml = String::from_utf8_lossy(&handle.read().await).into_owned();
                                    let hint = format!("{parent}/{name}").to_lowercase();
                                    match infer_module_slot(&hint, (layout2.active_workbench_module)()) {
                                        Some(slot) => {
                                            layout2.active_workbench_module.set(Some(slot));
                                            edit2.load_module_request.set(Some(xml));
                                            layout2.enter_workspace(crate::app::state::CenterView::ModuleWorkbench);
                                            edit2.workbench_status.set(format!("Loaded {name} → {slot}."));
                                        }
                                        None => edit2.workbench_status.set(
                                            format!("Couldn't detect module type for {name}; select a slot first.")
                                        ),
                                    }
                                });
                            },
                            "Open Module File…"
                        }
                        button {
                            class: "editor-button editor-button-active",
                            title: "Load the online module index so you can install modules directly",
                            onclick: move |_| {
                                let mut comp = compendium;
                                let mut status = edit_state.workbench_status;
                                spawn(async move {
                                    status.set("Loading compendium…".to_string());
                                    match crate::ui::panels::theme_palette::static_pages_panel::fetch_raw_page(COMPENDIUM_INDEX_URL).await {
                                        Ok(json) => match serde_json::from_str::<Vec<RemoteModule>>(&json) {
                                            Ok(list) => { status.set(format!("{} module(s) available.", list.len())); comp.set(list); }
                                            Err(e) => status.set(format!("Bad index.json: {e}")),
                                        },
                                        Err(e) => status.set(format!("Load failed: {e}")),
                                    }
                                });
                            },
                            "⬇ Install from Compendium"
                        }
                        button {
                            class: "editor-button editor-button-small",
                            title: "Browse the XML / template-module compendium gallery",
                            onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMPENDIUM_SITE_URL).spawn(); },
                            "Browse XML Compendium ↗"
                        }
                        button {
                            class: "editor-button editor-button-small",
                            title: "View the XML compendium source on GitHub",
                            onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMPENDIUM_REPO_URL).spawn(); },
                            "Source on GitHub ↗"
                        }
                    }
                }
            }
        }
    }
}
