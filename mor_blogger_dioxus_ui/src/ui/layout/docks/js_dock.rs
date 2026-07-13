use crate::app::state::{CenterView, DockPosition, LayoutState, RenderState, ThemeState};
use crate::ui::layout::docks::css_dock::asset_editor_panel;
use crate::ui::layout::docks::resolve_workbench_dependencies;
use dioxus::prelude::*;
use mor_blogger_core::render::template_resolver::{CORE_JS_FILES, MAGAZINE_GRID_JS};

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
        || view == CenterView::JsWorkbench
    {
        let mut deps = vec!["custom_js.js".to_string()];
        for file in CORE_JS_FILES {
            deps.push(file.to_string());
        }
        // Not in CORE_JS_FILES, but the JS workspace cross-links to it.
        deps.push(MAGAZINE_GRID_JS.to_string());
        available_files.extend(deps);
    } else {
        available_files.push("custom_js.js".to_string());
    }

    asset_editor_panel(
        "JS EDITOR",
        "javascript",
        "custom_js.js",
        available_files,
        layout.js_editor_pos,
        theme.signals.custom_js,
        Some(layout.js_editor_open_file),
    )
}
