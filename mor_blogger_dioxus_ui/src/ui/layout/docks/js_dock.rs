use crate::app::state::{CenterView, DockPosition, LayoutState, RenderState, ThemeState};
use crate::app::vfs::VfsDictionary;
use crate::ui::layout::docks::{resolve_workbench_dependencies, AssetEditorDock};
use dioxus::prelude::*;
use mor_blogger_core::render::template_resolver::CORE_JS_FILES;

#[component]
pub fn JsEditorPanel() -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let theme = use_context::<ThemeState>();

    let pos = (layout.js_editor_pos)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let view = (layout.center_view)();
    let mut available_files = vec![];

    if view == CenterView::ModuleWorkbench {
        available_files.push("custom_js.js".to_string());
        if let Some(key) = (layout.active_workbench_module)() {
            let deps = resolve_workbench_dependencies(&render, key, "js");
            available_files.extend(deps);
        }
    } else if view == CenterView::Preview
        || view == CenterView::CodeEditor
        || view == CenterView::Split
        || view == CenterView::StaticPageEditor
    {
        let mut deps = vec!["custom_js.js".to_string()];
        for file in CORE_JS_FILES {
            deps.push(file.to_string());
        }
        available_files.extend(deps);
    } else {
        available_files.push("custom_js.js".to_string());
    }

    rsx! {
        AssetEditorDock {
            title: "JS EDITOR",
            mode: "javascript",
            default_file: "custom_js.js",
            available_files,
            dock_position: layout.js_editor_pos,
            content_signal: theme.signals.custom_js,
            vfs_signal: use_context::<VfsDictionary>().0,
            is_native_window: false,
            on_save: move |_| {
                let vfs = use_context::<VfsDictionary>().0;
                let current_vfs = vfs.read().clone();
                crate::app::services::workspace_service::persist_asset_editor(theme, &current_vfs, "js");
            },
            on_close: move |_| {
                let mut pos = layout.js_editor_pos;
                pos.set(DockPosition::Hidden);
            }
        }
    }
}
