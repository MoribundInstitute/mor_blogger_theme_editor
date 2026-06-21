use crate::app::state::{CenterView, DockPosition, LayoutState, RenderState};
use crate::app::vfs::VfsDictionary;
use crate::ui::shell::docks::AssetEditorDock;
use dioxus::prelude::*;
use mor_blogger_core::render::template_resolver::CORE_JS_FILES;

fn get_js_deps(render: &RenderState, registry_type: &str, target_id: &str) -> Vec<&'static str> {
    render
        .get_manifest(registry_type, target_id)
        .map(|c| c.js_deps.to_vec())
        .unwrap_or_default()
}

#[component]
pub fn JsEditorPanel() -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();

    let pos = (layout.js_editor_pos)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let view = (layout.center_view)();
    let config = (render.current_config)();
    let pack = &config.template_pack;

    let mut available_files = vec![];

    if view == CenterView::ModuleWorkbench {
        available_files.push("custom_js.js".to_string());
        if let Some(key) = (layout.active_workbench_module)() {
            let deps = match key {
                "header_variant" => {
                    let mut d = get_js_deps(&render, "header", &pack.header_variant);
                    if d.is_empty() {
                        d.push("header.js");
                    }
                    d
                }
                "main_variant" => {
                    let mut d = get_js_deps(&render, "layout", &pack.main_variant);
                    if d.is_empty() {
                        d.push("layout.js");
                    }
                    d
                }
                "content_variant" => {
                    let mut d = get_js_deps(&render, "content", &pack.content_variant);
                    if d.is_empty() {
                        d.push("content.js");
                    }
                    d
                }
                "left_sidebar_variant" | "right_sidebar_variant" => {
                    let mut d = if key == "left_sidebar_variant" {
                        get_js_deps(&render, "sidebar_left", &pack.left_sidebar_variant)
                    } else {
                        get_js_deps(&render, "sidebar_right", &pack.right_sidebar_variant)
                    };
                    if d.is_empty() {
                        d.push("sidebar.js");
                    }
                    d
                }
                "footer_variant" => {
                    let mut d = get_js_deps(&render, "footer", &pack.footer_variant);
                    if d.is_empty() {
                        d.push("footer.js");
                    }
                    d
                }
                _ => vec![],
            };
            available_files.extend(deps.into_iter().map(|s| s.to_string()));
        }
    } else if matches!(view, CenterView::Preview | CenterView::CodeEditor) {
        let mut deps = vec!["custom_js.js".to_string()];
        for file in CORE_JS_FILES {
            deps.push(file.to_string());
        }
        available_files.extend(deps);
    } else {
        available_files.push("custom_js.js".to_string());
    }

    let content_signal = use_signal(|| String::new());

    rsx! {
        AssetEditorDock {
            title: "JS EDITOR",
            mode: "javascript",
            default_file: "custom_js.js",
            available_files,
            dock_position: layout.js_editor_pos,
            content_signal,
            vfs_signal: use_context::<VfsDictionary>().0,
            is_native_window: false,
            on_save: move |_| {
                spawn(async move {
                    let vfs = use_context::<VfsDictionary>().0;
                    let current_vfs = vfs.read().clone();
                    for (filename, content) in current_vfs.iter() {
                        if filename.ends_with(".css") || filename == "custom_js.js" {
                            continue;
                        }
                        match mor_blogger_core::utils::fs_bridge::save_custom_js(filename, content) {
                            Ok(path) => log::info!("Successfully synced {} to OS at {}", filename, path.display()),
                            Err(e) => log::error!("Failed to sync {} to OS: {}", filename, e),
                        }
                    }
                });
            },
            on_close: move |_| {
                let mut pos = layout.js_editor_pos;
                pos.set(DockPosition::Hidden);
            }
        }
    }
}
