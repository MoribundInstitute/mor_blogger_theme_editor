use dioxus::prelude::*;
use mor_blogger_core::utils::fs_bridge;

use crate::app::shell::WorkbenchEditState;
use crate::app::state::LayoutState;
use crate::ui_kit::MorDock;

const TEMPLATE_LAYOUTS: &[(&str, &str)] = &[
    ("Header Variant", "header_variant"),
    ("Main Canvas Variant", "main_variant"),
    ("Content Feed", "content_variant"),
    ("Left Sidebar", "left_sidebar_variant"),
    ("Right Sidebar", "right_sidebar_variant"),
    ("Footer Grid", "footer_variant"),
];

pub(crate) fn module_key_to_category(key: &str) -> &'static str {
    match key {
        "header_variant" => "headers",
        "main_variant" => "layouts",
        "content_variant" => "content",
        "left_sidebar_variant" | "right_sidebar_variant" => "sidebars",
        "footer_variant" => "footers",
        _ => "custom",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TemplateModulesDockProps {
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
}

#[component]
pub fn TemplateModulesDock(props: TemplateModulesDockProps) -> Element {
    let mut layout = use_context::<LayoutState>();
    let mut edit_state = use_context::<WorkbenchEditState>();

    rsx! {
        MorDock {
            title: "Template Modules",
            width: "280px".to_string(),
            left: "20px".to_string(),
            top: "60px".to_string(),
            background: "var(--bg-surface, #1e1e1e)".to_string(),
            on_close: move |_| {
                if let Some(ref handler) = props.on_close {
                    handler.call(());
                }
            },

            div {
                class: "palette-content",
                style: "padding: 12px; overflow-y: auto; display: flex; flex-direction: column; gap: 6px;",
                for (label, key) in TEMPLATE_LAYOUTS {
                    button {
                        class: if (layout.active_workbench_module)() == Some(key) { "editor-button editor-button-active" } else { "editor-button" },
                        style: "text-align: left; font-size: 0.85rem; width: 100%; border: none; background: transparent; cursor: pointer; padding: 6px 8px; border-radius: 4px; color: var(--fg-base);",
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
                        class: "editor-button",
                        style: "width: 100%; justify-content: center; font-size: 0.8rem; border: none; background: var(--bg-base); color: var(--fg-base); cursor: pointer; padding: 6px; border-radius: 4px;",
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
