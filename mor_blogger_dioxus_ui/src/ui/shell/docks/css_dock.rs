use crate::app::state::{CenterView, DockPosition, LayoutState, RenderState};
use crate::app::vfs::VfsDictionary;
use crate::ui::shell::docks::AssetEditorDock;
use dioxus::prelude::*;

fn get_css_deps(render: &RenderState, registry_type: &str, target_id: &str) -> Vec<&'static str> {
    render
        .get_manifest(registry_type, target_id)
        .map(|c| c.css_deps.to_vec())
        .unwrap_or_default()
}

#[component]
pub fn CssEditorPanel() -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();

    let pos = (layout.css_editor_pos)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let view = (layout.center_view)();
    let config = (render.current_config)();
    let pack = &config.template_pack;

    let mut available_files = vec!["preset_css.css".to_string()];

    if view == CenterView::ModuleWorkbench {
        if let Some(key) = (layout.active_workbench_module)() {
            let deps = match key {
                "header_variant" => {
                    let mut d = get_css_deps(&render, "header", &pack.header_variant);
                    if d.is_empty() {
                        d.push("header.css");
                    }
                    d
                }
                "main_variant" => {
                    let mut d = get_css_deps(&render, "layout", &pack.main_variant);
                    if d.is_empty() {
                        d.push("layout.css");
                    }
                    d
                }
                "content_variant" => {
                    let mut d = get_css_deps(&render, "content", &pack.content_variant);
                    if d.is_empty() {
                        d.push("content.css");
                    }
                    d
                }
                "left_sidebar_variant" | "right_sidebar_variant" => {
                    let mut d = if key == "left_sidebar_variant" {
                        get_css_deps(&render, "sidebar_left", &pack.left_sidebar_variant)
                    } else {
                        get_css_deps(&render, "sidebar_right", &pack.right_sidebar_variant)
                    };
                    if d.is_empty() {
                        d.push("sidebar.css");
                    }
                    d
                }
                "footer_variant" => {
                    let mut d = get_css_deps(&render, "footer", &pack.footer_variant);
                    if d.is_empty() {
                        d.push("footer.css");
                    }
                    d
                }
                _ => vec![],
            };
            available_files.extend(deps.into_iter().map(|s| s.to_string()));
        }
    } else if view == CenterView::Preview
        || view == CenterView::CodeEditor
        || view == CenterView::Split
        || view == CenterView::StaticPageEditor
    {
        let mut all_deps = Vec::new();

        // Add baseline CSS files
        all_deps.extend(&[
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
        ]);

        // Aggregate all CSS dependencies across the active theme
        all_deps.extend(get_css_deps(&render, "header", &pack.header_variant));
        all_deps.extend(get_css_deps(&render, "layout", &pack.main_variant));
        all_deps.extend(get_css_deps(&render, "content", &pack.content_variant));
        all_deps.extend(get_css_deps(
            &render,
            "sidebar_left",
            &pack.left_sidebar_variant,
        ));
        all_deps.extend(get_css_deps(
            &render,
            "sidebar_right",
            &pack.right_sidebar_variant,
        ));
        all_deps.extend(get_css_deps(&render, "footer", &pack.footer_variant));

        // Deduplicate and sort
        all_deps.sort();
        all_deps.dedup();

        available_files.extend(all_deps.into_iter().map(|s| s.to_string()));
    }

    let content_signal = use_signal(|| String::new());

    rsx! {
        AssetEditorDock {
            title: "CSS EDITOR",
            mode: "css",
            default_file: "preset_css.css",
            available_files,
            dock_position: layout.css_editor_pos,
            content_signal,
            vfs_signal: use_context::<VfsDictionary>().0,
            is_native_window: false,
            on_save: move |_| {
                spawn(async move {
                    let vfs = use_context::<VfsDictionary>().0;
                    let current_vfs = vfs.read().clone();
                    for (filename, content) in current_vfs.iter() {
                        if filename == "preset_css.css" {
                            continue;
                        }
                        match mor_blogger_core::utils::fs_bridge::save_custom_css(filename, content) {
                            Ok(path) => log::info!("Successfully synced {} to OS at {}", filename, path.display()),
                            Err(e) => log::error!("Failed to sync {} to OS: {}", filename, e),
                        }
                    }
                });
            },
            on_close: move |_| {
                let mut pos = layout.css_editor_pos;
                pos.set(DockPosition::Hidden);
            }
        }
    }
}
