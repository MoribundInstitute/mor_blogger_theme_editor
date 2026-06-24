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
                edit_state.workbench_status.set("XML changes saved to buffer.".to_string());
            },
            on_close: move |_| {
                layout.xml_editor_pos.set(DockPosition::Hidden);
            }
        }
    }
}