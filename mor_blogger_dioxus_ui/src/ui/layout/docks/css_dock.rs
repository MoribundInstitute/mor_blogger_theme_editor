use crate::app::state::{CenterView, DockPosition, LayoutState, RenderState, ThemeState};
use crate::app::vfs::VfsDictionary;
use crate::ui::layout::docks::{
    resolve_theme_dependencies, resolve_workbench_dependencies, AssetEditorDock,
};
use dioxus::prelude::*;

/// Shared shell for the CSS/JS editor panels: everything but the file list.
/// `mode` is the editor mode ("css" / "javascript"); the persist kind derives from it.
pub(super) fn asset_editor_panel(
    title: &'static str,
    mode: &'static str,
    default_file: &'static str,
    available_files: Vec<String>,
    dock_position: Signal<DockPosition>,
    content_signal: Signal<String>,
    open_file_request: Option<Signal<Option<String>>>,
) -> Element {
    let theme = use_context::<ThemeState>();
    let persist_kind = if mode == "css" { "css" } else { "js" };

    rsx! {
        AssetEditorDock {
            title,
            mode,
            default_file,
            available_files,
            dock_position,
            content_signal,
            vfs_signal: use_context::<VfsDictionary>().0,
            is_native_window: false,
            open_file_request,
            on_save: move |_| {
                let vfs = use_context::<VfsDictionary>().0;
                let current_vfs = vfs.read().clone();
                crate::app::workspace_ops::persist_asset_editor(theme, &current_vfs, persist_kind);
            },
            on_close: move |_| {
                let mut pos = dock_position;
                pos.set(DockPosition::Hidden);
            }
        }
    }
}

#[component]
pub fn CssEditorPanel() -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let theme = use_context::<ThemeState>();

    let pos = (layout.css_editor_pos)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let view = (layout.center_view)();
    let mut available_files = vec!["preset_css.css".to_string()];

    if view == CenterView::ModuleWorkbench {
        if let Some(key) = (layout.active_workbench_module)() {
            let deps = resolve_workbench_dependencies(&render, key, "css");
            available_files.extend(deps);
        }
    } else if view == CenterView::Preview
        || view == CenterView::CodeEditor
        || view == CenterView::Split
        || view == CenterView::StaticPageEditor
    {
        let mut baseline: Vec<String> = vec![
            "00-Root-Section.css",
            "01-Reset-Base.css",
            "02-Typography-Links.css",
            "03-Buttons.css",
            "12-Terminal-Post-Styling.css",
            "13-Pagination.css",
            "17-Scrollbars.css",
            "19-Responsive-Mobile-Tablet.css",
            "20-Responsive-Very-Small-Screens.css",
            "21-Responsive-Desktop.css",
            "22-Export-Safety.css",
            "23-Comments.css",
            "24-Author-Profile.css",
            "25-Share-Menu.css",
            "26-Analytics-Dashboard.css",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        // Aggregate all CSS dependencies across the active theme
        baseline.extend(resolve_theme_dependencies(&render, "css"));

        // Deduplicate and sort
        baseline.sort();
        baseline.dedup();

        available_files.extend(baseline);
    }

    asset_editor_panel(
        "CSS EDITOR",
        "css",
        "preset_css.css",
        available_files,
        layout.css_editor_pos,
        theme.signals.preset_css,
        None,
    )
}
