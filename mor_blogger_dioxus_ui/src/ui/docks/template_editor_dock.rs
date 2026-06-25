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

#[component]
pub fn TemplateModulesDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let mut edit_state = use_context::<WorkbenchEditState>();
    let pos = (layout.template_modules_pos)();

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

                    div {
                        style: "margin-top: auto; padding-top: 10px; border-top: 1px solid var(--border-color);",
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
                    }
                }
            }
        }
    }
}
