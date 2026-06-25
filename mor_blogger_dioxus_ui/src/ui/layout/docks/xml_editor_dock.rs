use crate::app::shell::WorkbenchEditState;
use crate::app::state::{DockPosition, LayoutState, RenderState};
use crate::app::vfs::VfsDictionary;
use crate::ui::layout::docks::asset_editor_dock::AssetEditorDock;
use dioxus::prelude::*;

#[component]
pub fn XmlEditorDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let mut edit_state = use_context::<WorkbenchEditState>();
    let render = use_context::<RenderState>();
    // Default save target: the app data dir's theme.xml, so Ctrl+S persists
    // without prompting. (There is no canonical theme.xml elsewhere — it's
    // generated in-memory.) Falls back to an rfd prompt only if no data dir.
    let mut xml_path = use_signal(|| {
        mor_blogger_core::utils::fs_bridge::workspace_root().map(|r| r.join("theme.xml"))
    });

    // Seed global XML into editor buffer if it is empty
    use_effect(move || {
        if (layout.xml_editor_pos)() != DockPosition::Hidden {
            let current_theme_xml = (render.generated_xml)();
            if (edit_state.edited_xml)().is_empty() {
                edit_state.edited_xml.set(current_theme_xml);
            }
        }
    });

    if (layout.xml_editor_pos)() == DockPosition::Hidden {
        return rsx! {};
    }

    rsx! {
        AssetEditorDock {
            title: "XML EDITOR",
            mode: "xml",
            default_file: "theme.xml",
            available_files: vec!["theme.xml".to_string()],
            dock_position: layout.xml_editor_pos,
            content_signal: edit_state.edited_xml, // Fixed: Wired to global buffer
            vfs_signal: use_context::<VfsDictionary>().0,
            is_native_window: false,
            on_save: move |_| {
                let content = (edit_state.edited_xml)();
                let mut status = edit_state.workbench_status;
                if let Some(path) = xml_path() {
                    if let Some(parent) = path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    match std::fs::write(&path, &content) {
                        Ok(_) => status.set(format!("Saved theme.xml to {}", path.display())),
                        Err(e) => status.set(format!("Save failed: {}", e)),
                    }
                } else {
                    // No target yet — prompt once, remember the choice, then write.
                    spawn(async move {
                        if let Some(handle) = rfd::AsyncFileDialog::new()
                            .add_filter("Blogger theme", &["xml"])
                            .add_filter("HTML", &["html", "htm"])
                            .set_file_name("theme.xml")
                            .save_file()
                            .await
                        {
                            let path = handle.path().to_path_buf();
                            match std::fs::write(&path, &content) {
                                Ok(_) => status.set(format!("Saved theme.xml to {}", path.display())),
                                Err(e) => status.set(format!("Save failed: {}", e)),
                            }
                            xml_path.set(Some(path));
                        }
                    });
                }
            },
            on_close: move |_| {
                layout.xml_editor_pos.set(DockPosition::Hidden);
            }
        }
    }
}