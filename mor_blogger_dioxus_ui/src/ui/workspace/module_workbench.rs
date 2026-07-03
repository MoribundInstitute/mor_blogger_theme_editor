use std::collections::HashMap;

use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::icons::{
    IconChevronDown, IconChevronUp, IconEye, IconEyeOff, IconGrip, IconPencil, IconTrash,
};
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::render::xml_parts::css_generator::render_css_sockets;

use mor_blogger_core::utils::fs_bridge;

use dioxus::html::HasFileData;

use crate::app::shell::WorkbenchEditState;
use crate::app::state::{DockPosition, LayoutState, RenderState};
use crate::app::vfs::VfsDictionary;
use crate::ui::workspace::layout::{apply_preview_viewport, clamp_preview_width, PreviewViewport};
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use crate::ui::workspace::widget_layout;

fn pick_xml(render: &RenderState, registry_type: &str, id: &str) -> String {
    render
        .get_manifest(registry_type, id)
        .or_else(|| {
            let fallback_id = match registry_type {
                "header" => "mor",
                "layout" => "sidebars",
                "content" => "blog_standard",
                "sidebar_left" => "blogger_left",
                "sidebar_right" => "toc_right",
                "footer" => "mega",
                _ => "",
            };
            render.get_manifest(registry_type, fallback_id)
        })
        .map(|c| c.xml_content.to_string())
        .unwrap_or_default()
}

fn resolve_module_xml(render: &RenderState, module_key: &str, config: &ThemeConfig) -> String {
    // A saved override (custom_<key>.xml) is the live source for this slot.
    if let Some(custom) = mor_blogger_core::render::template_resolver::module_override(module_key) {
        return custom;
    }
    let pack = &config.template_pack;
    match module_key {
        "header_variant" => pick_xml(render, "header", &pack.header_variant),
        "main_variant" => pick_xml(render, "layout", &pack.main_variant),
        "content_variant" => pick_xml(render, "content", &pack.content_variant),
        "left_sidebar_variant" => pick_xml(render, "sidebar_left", &pack.left_sidebar_variant),
        "right_sidebar_variant" => pick_xml(render, "sidebar_right", &pack.right_sidebar_variant),
        "footer_variant" => pick_xml(render, "footer", &pack.footer_variant),
        _ => String::new(),
    }
}

fn render_module_preview(
    module_key: &str,
    config: &ThemeConfig,
    vfs: &HashMap<String, String>,
    slots: &[widget_layout::WidgetSlot],
) -> String {
    let parts = mor_blogger_core::render::template_resolver::resolve_template_parts(config, vfs);
    // Decode XML entities: Blogger-escaped CSS into a browser <style>.
    let true_css =
        mor_blogger_core::render::util::unescape_for_style(&render_css_sockets(parts.css, config));

    use mor_blogger_core::render::preview;

    // Every module preview reuses the SAME section builders as the live preview, so
    // the two never drift. Content gating is the only module-specific behavior.
    let body = match module_key {
        "header_variant" => preview::preview_header_html(config),
        "main_variant" => preview::preview_workspace_html(config, &[]),
        // Content feed is driven by the Blog widget; all hidden/removed → empty.
        "content_variant" if !slots.iter().any(|s| s.visible) => {
            r#"<div style="display:flex;align-items:center;justify-content:center;height:100%;min-height:240px;padding:48px;text-align:center;color:#888;font-family:monospace;">No visible content widget — the post feed is hidden.</div>"#
                .to_string()
        }
        "content_variant" => format!(
            r#"<main class="canvas-core">{}</main>"#,
            preview::preview_content_html(config, &[])
        ),
        "left_sidebar_variant" => preview::preview_left_panel_html(config),
        "right_sidebar_variant" => preview::preview_right_panel_html(config),
        "footer_variant" => preview::preview_footer_html(config),
        _ => r#"<div style="padding:40px;text-align:center;color:var(--fg-muted);">Select a module.</div>"#
            .to_string(),
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style id="mor-true-css">
{true_css}
html, body {{ overflow: hidden; margin: 0; }}
</style>
</head>
<body>
{body}
</body>
</html>"#
    )
}

/// Route a module file (dropped on the workspace) into the matching slot's editor
/// buffer. Infers the slot from the filename; falls back to the active slot.
fn apply_loaded_module(name: &str, xml: String, mut layout: LayoutState, mut edit_state: WorkbenchEditState) {
    let hint = name.to_lowercase();
    match crate::ui::layout::docks::template_editor_dock::infer_module_slot(
        &hint,
        (layout.active_workbench_module)(),
    ) {
        Some(slot) => {
            layout.active_workbench_module.set(Some(slot));
            edit_state.load_module_request.set(Some(xml));
            edit_state.workbench_status.set(format!("Loaded {name} → {slot}."));
        }
        None => edit_state
            .workbench_status
            .set(format!("Couldn't detect module type for {name}; select a slot first.")),
    }
}

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

#[component]
pub fn ModuleWorkbench(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
) -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let mut edited_xml: Signal<String> = use_signal(String::new);
    let mut is_takeover_active = use_signal(|| false);
    let mut is_editor_takeover_active = use_signal(|| false);
    let mut workbench_status: Signal<String> = use_signal(String::new);
    let vfs = use_context::<VfsDictionary>().0;

    // Widget blueprint library (workspace/widgets/<group>/*.xml). Loaded once; reloaded
    // after saving a blueprint. When `editing_blueprint` is Some((group, name)) the editor
    // buffer targets that blueprint file instead of the active module.
    let mut blueprints: Signal<Vec<fs_bridge::WidgetBlueprint>> =
        use_signal(fs_bridge::load_widget_blueprints);
    let mut editing_blueprint: Signal<Option<(String, String)>> = use_signal(|| None);
    let mut show_widgets = use_signal(|| false);

    // Layout view (Blogger-style draggable widget cards) vs raw Code view. Drag state
    // tracks the card being dragged and the card it's hovering for the drop.
    let mut layout_view = use_signal(|| true);
    // "Add a Gadget" now lives in the Widgets dock. The "+ Add" buttons set the
    // target (socket key, or None to append) and open that dock; the dock posts the
    // chosen blueprint back via `add_widget_request`, consumed below.
    let edit_state = use_context::<WorkbenchEditState>();
    let open_gadget_picker = move |target: Option<String>| {
        let mut es = edit_state;
        es.add_target.set(target);
        let mut l = layout;
        l.request_dock("widgets", DockPosition::mor_panel_right);
    };

    // Reactively resolve the active module's XML fragment from the live config.
    let module_xml = use_memo(move || match (layout.active_workbench_module)() {
        Some(key) => {
            let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
            resolve_module_xml(&render, key, &config)
        }
        None => String::new(),
    });

    // Show user's scratchpad edits; fall back to the config-derived XML when pristine.
    let display_xml = use_memo(move || {
        let edited = edited_xml();
        if edited.is_empty() {
            module_xml()
        } else {
            edited
        }
    });

    // Reset the scratchpad when the active module changes, so switching modules
    // (now driven from the TemplateModulesDock) falls back to the new module's
    // config-derived XML. Keyed on the module, so it never clobbers live typing.
    let module_key = (layout.active_workbench_module)();
    use_effect(use_reactive!(|module_key| {
        let _ = module_key;
        edited_xml.set(String::new());
        editing_blueprint.set(None);
    }));

    // Honor "edit this widget" requests from the Widgets dock: load the blueprint
    // into the editor buffer and switch to Code view. Consumed once, then cleared.
    {
        let mut req = edit_state.edit_widget_request;
        use_effect(move || {
            if let Some(bp) = req() {
                editing_blueprint.set(Some((bp.group.clone(), bp.name.clone())));
                edited_xml.set(bp.xml.clone());
                layout_view.set(false);
                req.set(None);
            }
        });
    }

    // Honor "load this module file" requests (Template Modules dock picker or a
    // file dropped on the workspace): the sender already set active_workbench_module,
    // so we just push the XML into the editor buffer. Declared AFTER the module-key
    // reset effect so, when the slot also changes, this write wins over the reset.
    {
        let mut req = edit_state.load_module_request;
        use_effect(move || {
            if let Some(xml) = req() {
                editing_blueprint.set(None);
                edited_xml.set(xml);
                req.set(None);
            }
        });
    }

    // Generate the isolated module preview from the live config + the editable
    // buffer's widget visibility, so hiding/removing a widget updates the preview.
    let module_preview_html = use_memo(move || match (layout.active_workbench_module)() {
        Some(key) => {
            let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
            let slots = widget_layout::parse_slots(&display_xml());
            render_module_preview(key, &config, &*vfs.read(), &slots)
        }
        None => String::new(),
    });

    let mut preview_viewport: Signal<PreviewViewport> = use_signal(|| PreviewViewport::Fit);
    let mut preview_width: Signal<u32> = use_signal(|| 1200u32);

    // Save buffer to this module's canonical custom override (custom_<key>).
    // Copy closure (captures only Copy signals/memos) so it's reusable in both the
    // pane header and the editor-takeover bar.
    let save_module = move |_: Event<MouseData>| {
        let content = display_xml();
        let mut status = workbench_status;
        // Editing a widget blueprint → write back to its file, not the module override.
        if let Some((group, name)) = editing_blueprint() {
            match fs_bridge::save_widget_blueprint(&group, &name, &content) {
                Ok(path) => {
                    status.set(format!("Saved widget → {}", path.display()));
                    blueprints.set(fs_bridge::load_widget_blueprints());
                }
                Err(e) => status.set(format!("Save failed: {}", e)),
            }
            return;
        }
        let Some(key) = (layout.active_workbench_module)() else { return; };
        let category = module_key_to_category(key);
        let name = format!("custom_{}", key);
        match fs_bridge::save_custom_module(category, &name, &content) {
            Ok(path) => status.set(format!("Saved → {}", path.display())),
            Err(e) => status.set(format!("Save failed: {}", e)),
        }
    };

    // Append a blueprint's `<b:widget>` XML into the active module buffer (= import).
    // ponytail: appends at the end and keeps blueprint ids verbatim — if the module
    // already holds that widget id, dedupe/rename it by hand in the editor.
    let insert_blueprint = move |bp: fs_bridge::WidgetBlueprint| {
        let current = display_xml();
        let merged = if current.trim().is_empty() {
            bp.xml
        } else {
            format!("{current}\n{}", bp.xml)
        };
        editing_blueprint.set(None);
        edited_xml.set(merged);
        on_load_theme.call(config_toml());
    };

    // Load a blueprint into the editor so Save writes back to its file.
    let edit_blueprint = move |bp: fs_bridge::WidgetBlueprint| {
        editing_blueprint.set(Some((bp.group.clone(), bp.name.clone())));
        edited_xml.set(bp.xml.clone());
    };

    // Commit a rewritten buffer (from a Layout-view card action) and re-render.
    let apply_buffer = move |new_xml: String| {
        edited_xml.set(new_xml);
        on_load_theme.call(config_toml());
    };

    // Revert this module/blueprint to its compiled default (drop session edits).
    let revert_default = move |_: Event<MouseData>| {
        // Drop any saved override so the slot reverts to its compiled default.
        if let Some(key) = (layout.active_workbench_module)() {
            let category = module_key_to_category(key);
            let _ = fs_bridge::delete_custom_module(category, &format!("custom_{key}"));
        }
        editing_blueprint.set(None);
        edited_xml.set(String::new());
        on_load_theme.call(config_toml());
    };

    // ── Socket editing ─ widgets in a `{{SOCKET_*}}` live in config.widget_map ──
    // Mutate the live config and re-apply it via on_load_theme (same path the
    // preview's drag-reorder uses through workspace_service::handle_widget_move).
    let mutate_pack = move |edit: Box<dyn FnOnce(&mut mor_blogger_core::config::TemplatePackConfig)>| {
        let mut cfg = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
        edit(&mut cfg.template_pack);
        // to_string_pretty: matches how the rest of the app serialises ThemeConfig
        // (plain to_string hits ValueAfterTable on this struct and silently no-ops).
        if let Ok(s) = toml::to_string_pretty(&cfg) {
            on_load_theme.call(s);
        }
    };
    // Remove a widget id from a socket.
    let socket_remove = move |(key, wid): (String, String)| {
        let f = mutate_pack;
        f(Box::new(move |pack| {
            if let Some(v) = pack.widget_map.get_mut(&key) {
                v.retain(|x| x != &wid);
            }
        }));
    };
    // Swap a socket widget with its neighbour (delta -1 up / +1 down).
    let socket_move = move |(key, idx, delta): (String, usize, i32)| {
        let f = mutate_pack;
        f(Box::new(move |pack| {
            if let Some(v) = pack.widget_map.get_mut(&key) {
                let j = idx as i32 + delta;
                if j >= 0 && (j as usize) < v.len() {
                    v.swap(idx, j as usize);
                }
            }
        }));
    };
    // Move a socket widget from `from` to `to` (drag-to-reorder).
    let socket_reorder = move |(key, from, to): (String, usize, usize)| {
        let f = mutate_pack;
        f(Box::new(move |pack| {
            if let Some(v) = pack.widget_map.get_mut(&key) {
                if from < v.len() && to < v.len() && from != to {
                    let item = v.remove(from);
                    v.insert(to, item);
                }
            }
        }));
    };

    // ── Pointer drag-to-reorder state (mouse events only; HTML5 DnD crashes here) ──
    // Picked-up source + hovered drop target; the move is committed on mouse-up so the
    // list doesn't re-render on every hover.
    let mut drag_widget: Signal<Option<usize>> = use_signal(|| None);
    let mut over_widget: Signal<Option<usize>> = use_signal(|| None);
    let mut drag_socket: Signal<Option<(String, usize)>> = use_signal(|| None);
    let mut over_socket: Signal<Option<usize>> = use_signal(|| None);

    // Consume "add this gadget" requests from the Widgets dock: route into the
    // current target socket (config widget_map) or append to the module buffer.
    {
        let mut req = edit_state.add_widget_request;
        let mut target = edit_state.add_target;
        use_effect(move || {
            if let Some(bp) = req() {
                match target() {
                    Some(key) => {
                        if let Some(s) = widget_layout::parse_slots(&bp.xml).first() {
                            let id = if s.id.is_empty() { bp.name.clone() } else { s.id.clone() };
                            let title = s.title.clone();
                            let f = mutate_pack;
                            f(Box::new(move |pack| {
                                pack.move_widget(&id, &key);
                                if !title.trim().is_empty() {
                                    pack.widget_titles.insert(id.clone(), title);
                                }
                            }));
                        }
                    }
                    // No explicit socket: append to the active module's buffer. Refuse
                    // (with a hint) when nothing is selected, so the gadget can't vanish
                    // into the orphan module_fragment buffer that has no module to
                    // preview or save into — the silent no-op that read as "Add broken".
                    None => {
                        if (layout.active_workbench_module)().is_none()
                            && editing_blueprint().is_none()
                        {
                            workbench_status.set(
                                "Select a template module first (Template Editor dock), then Add."
                                    .to_string(),
                            );
                        } else {
                            let mut g = insert_blueprint;
                            g(bp.clone());
                        }
                    }
                }
                req.set(None);
                target.set(None); // one-shot: next Add defaults back to the active module
            }
        });
    }

    // Save buffer to a user-named new module file (lets you spin off variants).
    let save_as_new = move |_: Event<MouseData>| {
        let Some(key) = (layout.active_workbench_module)() else { return; };
        let content = display_xml();
        let category = module_key_to_category(key);
        let default_name = format!("{}_custom.xml", key);
        let start_dir = fs_bridge::category_dir(category).or_else(fs_bridge::templates_root);
        let mut status = workbench_status;
        spawn(async move {
            let mut dlg = rfd::AsyncFileDialog::new()
                .add_filter("Blogger module XML", &["xml"])
                .set_file_name(default_name);
            if let Some(dir) = start_dir {
                dlg = dlg.set_directory(dir);
            }
            if let Some(handle) = dlg.save_file().await {
                let path = handle.path().to_path_buf();
                match std::fs::write(&path, &content) {
                    Ok(_) => status.set(format!("Saved new module → {}", path.display())),
                    Err(e) => status.set(format!("Save failed: {}", e)),
                }
            }
        });
    };

    rsx! {
        if is_takeover_active() {
            // ── Takeover Mode ─ Full-viewport isolated canvas stage ───────
            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",
                ondragover: move |evt| { evt.prevent_default(); },
                ondrop: move |evt| {
                    async move {
                        evt.prevent_default();
                        if let Some(file) = evt.files().first() {
                            if let Ok(bytes) = file.read_bytes().await {
                                apply_loaded_module(&file.name(), String::from_utf8_lossy(&bytes).into_owned(), layout, edit_state);
                            }
                        }
                    }
                },

                // Top navigation bar
                div {
                    style: "flex-shrink: 0; display: flex; align-items: center; gap: 6px; padding: 6px 12px; background: var(--bg-elevated); border-bottom: 1px solid var(--editor-border);",

                    button {
                        class: if preview_viewport() == PreviewViewport::Desktop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Desktop, preview_width); preview_viewport.set(PreviewViewport::Desktop); },
                        "Desktop"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Laptop, preview_width); preview_viewport.set(PreviewViewport::Laptop); },
                        "Laptop"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Tablet, preview_width); preview_viewport.set(PreviewViewport::Tablet); },
                        "Tablet"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Phone, preview_width); preview_viewport.set(PreviewViewport::Phone); },
                        "Phone"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Fit, preview_width); preview_viewport.set(PreviewViewport::Fit); },
                        "Fit"
                    }

                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 2px;" }

                    label {
                        class: "preview-width-control",
                        span { class: "preview-width-label", "Width" }
                        input {
                            class: "preview-width-input",
                            r#type: "number", min: "240", max: "2400", step: "10",
                            value: "{preview_width()}",
                            oninput: move |evt| {
                                if let Ok(w) = evt.value().parse::<u32>() {
                                    preview_width.set(clamp_preview_width(w));
                                    preview_viewport.set(PreviewViewport::Custom);
                                }
                            }
                        }
                    }

                    // Spacer — pushes the exit button to the far right
                    div { style: "flex: 1;" }

                    button {
                        class: "editor-mini-button",
                        onclick: move |_| is_takeover_active.set(false),
                        "Editor ×"
                    }
                }

                // Canvas stage — transparent tiled grid background
                div {
                    style: "
                        flex: 1; min-height: 0; overflow: auto;
                        display: flex; align-items: stretch; justify-content: center;
                        padding: 32px;
                        background-color: var(--bg-base);
                        background-image:
                            linear-gradient(45deg, rgba(128,128,128,0.07) 25%, transparent 25%),
                            linear-gradient(-45deg, rgba(128,128,128,0.07) 25%, transparent 25%),
                            linear-gradient(45deg, transparent 75%, rgba(128,128,128,0.07) 75%),
                            linear-gradient(-45deg, transparent 75%, rgba(128,128,128,0.07) 75%);
                        background-size: 16px 16px;
                        background-position: 0 0, 0 8px, 8px -8px, -8px 0px;
                    ",

                    if module_preview_html().is_empty() {
                        div {
                            style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: var(--fg-muted); font-family: var(--font-mono); font-size: 0.9rem;",
                            "← select a module to preview"
                        }
                    } else {
                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column;",
                            PreviewCanvas {
                                preview_viewport,
                                preview_width,
                                preview_html: module_preview_html(),
                            }
                        }
                    }
                }
            }
        } else {
            // ── 3-Pane Editor Mode ────────────────────────────────────────
            div {
                class: "export-viewport",
                style: "display: flex; flex-direction: row; flex: 1; min-height: 0; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel);",
                ondragover: move |evt| { evt.prevent_default(); },
                ondrop: move |evt| {
                    async move {
                        evt.prevent_default();
                        if let Some(file) = evt.files().first() {
                            if let Ok(bytes) = file.read_bytes().await {
                                apply_loaded_module(&file.name(), String::from_utf8_lossy(&bytes).into_owned(), layout, edit_state);
                            }
                        }
                    }
                },

                // ── Left Pane ─ XML Fragment Editor ──────────────────────
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0; border-right: 1px solid var(--editor-border);",
                    div {
                        class: "editor-pane",
                        style: "display: flex; flex-direction: column; width: 100%; height: 100%; background: var(--bg-base); border-radius: 6px; overflow: hidden; border: 1px solid var(--border-color);",
                        
                        div {
                            class: "editor-pane-header",
                            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: rgba(0,0,0,0.2); border-bottom: 1px solid var(--border-color); flex-shrink: 0;",
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span {
                                    style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);",
                                    {editing_blueprint().map(|(g, n)| format!("{g}/{n}.xml")).or_else(|| (layout.active_workbench_module)().map(|k| format!("{k}.xml"))).unwrap_or_else(|| "module_fragment.xml".to_string())}
                                }
                                span {
                                    style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                                    "Blogger XML · Live"
                                }
                            }
                            div {
                                style: "display: flex; align-items: center; gap: 6px;",
                                button {
                                    class: if layout_view() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                                    title: "Visual widget layout (drag to reorder)",
                                    onclick: move |_| layout_view.set(true),
                                    "Layout"
                                }
                                button {
                                    class: if !layout_view() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                                    title: "Raw Blogger XML",
                                    onclick: move |_| layout_view.set(false),
                                    "Code"
                                }
                                button {
                                    class: "editor-mini-button",
                                    title: "Revert to compiled default (drops session edits)",
                                    onclick: revert_default,
                                    "Revert"
                                }
                                button {
                                    class: if is_editor_takeover_active() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                                    title: "Expand the editor (Layout or Code) to fill the workbench",
                                    onclick: move |_| { let v = is_editor_takeover_active(); is_editor_takeover_active.set(!v); },
                                    {if is_editor_takeover_active() { "Exit Takeover" } else { "Takeover" }}
                                }
                                button {
                                    class: if (layout.active_workbench_module)().is_some() || editing_blueprint().is_some() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                    title: "Save current XML buffer to this module's custom override",
                                    onclick: save_module,
                                    {if editing_blueprint().is_some() { "Save Widget" } else { "Save Module" }}
                                }
                                button {
                                    class: if (layout.active_workbench_module)().is_some() || editing_blueprint().is_some() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                    title: "Save the buffer as a new named template module",
                                    onclick: save_as_new,
                                    "Save as New"
                                }
                            }
                        }

                        // ── Widget Blueprints palette ─ Code-view insert / edit blueprint ──
                        // In Layout view, the inline "Add a Gadget" picker handles insertion.
                        if !layout_view() {
                        div {
                            style: "flex-shrink: 0; border-bottom: 1px solid var(--editor-border-soft);",
                            button {
                                style: "width: 100%; display: flex; align-items: center; justify-content: space-between; padding: 6px 12px; background: rgba(0,0,0,0.18); border: none; color: var(--fg-base); cursor: pointer; font-family: var(--font-mono); font-size: 0.78rem;",
                                onclick: move |_| show_widgets.set(!show_widgets()),
                                span { "Widget Blueprints ({blueprints().len()})" }
                                span { if show_widgets() { "▾" } else { "▸" } }
                            }
                            if show_widgets() {
                                div {
                                    style: "max-height: 220px; overflow-y: auto; padding: 4px 0; background: rgba(0,0,0,0.1);",
                                    for bp in blueprints() {
                                        div {
                                            style: "display: flex; align-items: center; gap: 6px; padding: 3px 12px; font-size: 0.75rem;",
                                            span {
                                                style: "flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-family: var(--font-mono); color: var(--fg-base);",
                                                title: "{bp.group}/{bp.name}",
                                                span { style: "color: var(--fg-muted);", "{bp.group}/" }
                                                "{bp.name}"
                                            }
                                            button {
                                                class: "editor-mini-button",
                                                title: "Append this widget into the active module",
                                                onclick: {
                                                    let bp = bp.clone();
                                                    move |_| { let mut f = insert_blueprint; f(bp.clone()); }
                                                },
                                                "Insert"
                                            }
                                            button {
                                                class: "editor-mini-button",
                                                title: "Open this blueprint in the editor (Save writes back to its file)",
                                                onclick: {
                                                    let bp = bp.clone();
                                                    move |_| { let mut f = edit_blueprint; f(bp.clone()); }
                                                },
                                                "Edit"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        }

                        if !workbench_status().is_empty() {
                            div {
                                style: "padding: 4px 12px; font-size: 0.75rem; color: var(--editor-accent-warm); background: rgba(0,0,0,0.15); font-family: var(--font-mono); border-bottom: 1px solid var(--editor-border-soft); display: flex; justify-content: space-between; align-items: center; flex-shrink: 0;",
                                span { "{workbench_status()}" }
                                button {
                                    style: "background: none; border: none; color: var(--fg-muted); cursor: pointer; font-size: 0.9rem; padding: 0 2px;",
                                    onclick: move |_| workbench_status.set(String::new()),
                                    "×"
                                }
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",
                            if layout_view() {
                                // ── Layout view ─ draggable Blogger-style widget cards ──
                                div {
                                    style: "flex: 1; min-height: 0; overflow-y: auto; padding: 10px; display: flex; flex-direction: column; gap: 8px;",
                                    // Commit a drag on release; cancel if the pointer leaves the list.
                                    onmouseup: move |_| {
                                        if let (Some(from), Some(to)) = (drag_widget(), over_widget()) {
                                            if from != to { let mut f = apply_buffer; f(widget_layout::reorder(&display_xml(), from, to)); }
                                        }
                                        if let (Some((key, from)), Some(to)) = (drag_socket(), over_socket()) {
                                            if from != to { let f = socket_reorder; f((key, from, to)); }
                                        }
                                        drag_widget.set(None); over_widget.set(None);
                                        drag_socket.set(None); over_socket.set(None);
                                    },
                                    onmouseleave: move |_| {
                                        drag_widget.set(None); over_widget.set(None);
                                        drag_socket.set(None); over_socket.set(None);
                                    },
                                    {
                                        let xml = display_xml();
                                        let parts = widget_layout::parse_parts(&xml);
                                        let pack = toml::from_str::<ThemeConfig>(&config_toml())
                                            .unwrap_or_default()
                                            .template_pack;
                                        // Fields grouped into a tree by their data-field-path
                                        // (footer columns, link rows, …) instead of a flat dump.
                                        let field_groups = widget_layout::group_fields(&parts);
                                        let structural: Vec<widget_layout::Part> = parts.into_iter()
                                            .filter(|p| !matches!(p, widget_layout::Part::Field { .. }))
                                            .collect();
                                        let has_socket = structural.iter().any(|p| matches!(p, widget_layout::Part::Socket { .. }));
                                        let widget_count = structural.iter().filter(|p| matches!(p, widget_layout::Part::Widget { .. })).count();

                                        if structural.is_empty() && field_groups.is_empty() {
                                            rsx! {
                                                div {
                                                    style: "margin: auto; text-align: center; color: var(--fg-muted); font-size: 0.85rem; font-family: var(--font-mono); line-height: 1.8; display: flex; flex-direction: column; align-items: center; gap: 12px;",
                                                    div { "This module is empty." }
                                                    button {
                                                        class: "editor-button",
                                                        onclick: move |_| { let mut f = open_gadget_picker; f(None); },
                                                        "+ Add a Gadget"
                                                    }
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                for (idx, part) in structural.into_iter().enumerate() {
                                                    { match part {
                                                        widget_layout::Part::Widget { slot, id, w_type, title, visible } => rsx! {
                                                            div {
                                                                key: "w{slot}-{id}",
                                                                class: "layout-card",
                                                                onmouseenter: move |_| { if drag_widget().is_some() { over_widget.set(Some(slot)); } },
                                                                style: format!(
                                                                    "display: flex; align-items: stretch; gap: 8px; padding: 8px 10px; border-radius: 6px; background: var(--bg-elevated); opacity: {}; border: 1px solid {};",
                                                                    if visible { "1" } else { "0.55" },
                                                                    if drag_widget() == Some(slot) { "var(--editor-accent)" }
                                                                    else if drag_widget().is_some() && over_widget() == Some(slot) { "var(--editor-accent-warm)" }
                                                                    else { "var(--editor-border-soft)" }
                                                                ),
                                                                // Drag grip (mouse-based reorder); ↑/↓ remain as a fallback.
                                                                div {
                                                                    style: "display: flex; align-items: center; color: var(--fg-muted); cursor: grab; padding: 0 2px;",
                                                                    title: "Drag to reorder",
                                                                    onmousedown: move |e| { e.prevent_default(); drag_widget.set(Some(slot)); over_widget.set(Some(slot)); },
                                                                    IconGrip {}
                                                                }
                                                                // Move column — stacked up/down (drag isn't supported on these cards).
                                                                div {
                                                                    style: "display: flex; flex-direction: column; justify-content: center; gap: 1px;",
                                                                    button {
                                                                        class: if slot > 0 { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                                                        style: "padding: 1px 5px; line-height: 0;",
                                                                        title: "Move up",
                                                                        onclick: move |_| { if slot > 0 { let mut f = apply_buffer; f(widget_layout::reorder(&display_xml(), slot, slot - 1)); } },
                                                                        IconChevronUp { size: "14".to_string() }
                                                                    }
                                                                    button {
                                                                        class: if slot + 1 < widget_count { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                                                        style: "padding: 1px 5px; line-height: 0;",
                                                                        title: "Move down",
                                                                        onclick: move |_| { if slot + 1 < widget_count { let mut f = apply_buffer; f(widget_layout::reorder(&display_xml(), slot, slot + 1)); } },
                                                                        IconChevronDown { size: "14".to_string() }
                                                                    }
                                                                }
                                                                button {
                                                                    class: "editor-mini-button",
                                                                    style: "padding: 4px 6px; line-height: 0; align-self: center;",
                                                                    title: if visible { "Hide widget" } else { "Show widget" },
                                                                    onclick: move |_| {
                                                                        let xml = display_xml();
                                                                        let cur = widget_layout::parse_slots(&xml).get(slot).map(|s| s.visible).unwrap_or(true);
                                                                        let mut f = apply_buffer;
                                                                        f(widget_layout::set_visible(&xml, slot, !cur));
                                                                    },
                                                                    if visible { IconEye {} } else { IconEyeOff {} }
                                                                }
                                                                div {
                                                                    style: "flex: 1; min-width: 0; display: flex; flex-direction: column; justify-content: center;",
                                                                    div {
                                                                        style: "font-size: 0.82rem; font-weight: 600; color: var(--fg-base); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                                                        { if title.trim().is_empty() { id.clone() } else { title.clone() } }
                                                                    }
                                                                    div { style: "font-size: 0.68rem; color: var(--fg-muted); font-family: var(--font-mono);", "{w_type} · {id}" }
                                                                }
                                                                button {
                                                                    class: "editor-mini-button",
                                                                    style: "padding: 4px 6px; line-height: 0; align-self: center;",
                                                                    title: "Edit this widget's XML",
                                                                    onclick: move |_| layout_view.set(false),
                                                                    IconPencil {}
                                                                }
                                                                button {
                                                                    class: "editor-mini-button",
                                                                    style: "padding: 4px 6px; line-height: 0; align-self: center;",
                                                                    title: "Remove widget from module",
                                                                    onclick: move |_| {
                                                                        let mut f = apply_buffer;
                                                                        f(widget_layout::remove(&display_xml(), slot));
                                                                    },
                                                                    IconTrash {}
                                                                }
                                                            }
                                                        },
                                                        widget_layout::Part::Section { id } => rsx! {
                                                            div {
                                                                key: "s{idx}-{id}",
                                                                style: "display: flex; align-items: center; gap: 8px; padding: 5px 10px; font-family: var(--font-mono); font-size: 0.72rem; color: var(--fg-muted);",
                                                                span { style: "padding: 1px 6px; border-radius: 3px; border: 1px solid var(--editor-border-soft);", "SECTION" }
                                                                span { "{id}" }
                                                            }
                                                        },
                                                        widget_layout::Part::ModuleRef { name } => rsx! {
                                                            div {
                                                                key: "m{idx}-{name}",
                                                                style: "display: flex; align-items: center; gap: 8px; padding: 8px 10px; border-radius: 6px; background: rgba(99,102,241,0.08); border: 1px dashed var(--editor-border-soft); font-size: 0.78rem;",
                                                                span { style: "padding: 1px 6px; border-radius: 3px; background: rgba(99,102,241,0.25); font-family: var(--font-mono); font-size: 0.68rem;", "MODULE" }
                                                                span { style: "flex: 1; color: var(--fg-base);", "{name}" }
                                                                span { style: "color: var(--fg-muted); font-size: 0.68rem;", "rendered from its own module" }
                                                            }
                                                        },
                                                        widget_layout::Part::Socket { name, key } => {
                                                            let widgets = pack.widget_map.get(&key).cloned().unwrap_or_default();
                                                            let n = widgets.len();
                                                            let key_add = key.clone();
                                                            rsx! {
                                                                div {
                                                                    key: "k{idx}-{key}",
                                                                    style: "border: 1px solid var(--editor-border-soft); border-radius: 6px; background: var(--bg-elevated); overflow: hidden;",
                                                                    div {
                                                                        style: "display: flex; align-items: center; gap: 8px; padding: 6px 10px; background: rgba(0,0,0,0.18);",
                                                                        span { style: "padding: 1px 6px; border-radius: 3px; background: rgba(34,197,94,0.25); font-family: var(--font-mono); font-size: 0.68rem;", "SLOT" }
                                                                        span { style: "flex: 1; font-size: 0.78rem; color: var(--fg-base);", "{name}" }
                                                                        button {
                                                                            class: "editor-mini-button",
                                                                            title: "Add a widget into this slot",
                                                                            onclick: move |_| { let mut f = open_gadget_picker; f(Some(key_add.clone())); },
                                                                            "+ Add"
                                                                        }
                                                                    }
                                                                    if widgets.is_empty() {
                                                                        div { style: "padding: 8px 12px; color: var(--fg-muted); font-size: 0.72rem; font-family: var(--font-mono);", "empty slot" }
                                                                    }
                                                                    for (wi, wid) in widgets.into_iter().enumerate() {
                                                                        {
                                                                            let title = pack.widget_titles.get(&wid).cloned().unwrap_or_else(|| wid.clone());
                                                                            let k_up = key.clone(); let k_dn = key.clone(); let k_rm = key.clone();
                                                                            let k_drag = key.clone();
                                                                            let dragging_here = drag_socket().as_ref().map(|(k, _)| k == &key).unwrap_or(false);
                                                                            let is_src = drag_socket() == Some((key.clone(), wi));
                                                                            let wid_rm = wid.clone();
                                                                            rsx! {
                                                                                div {
                                                                                    key: "{key}-{wi}-{wid}",
                                                                                    onmouseenter: move |_| { if dragging_here { over_socket.set(Some(wi)); } },
                                                                                    style: format!(
                                                                                        "display: flex; align-items: stretch; gap: 8px; padding: 6px 12px; font-size: 0.76rem; border-top: 1px solid {};",
                                                                                        if is_src { "var(--editor-accent)" }
                                                                                        else if dragging_here && over_socket() == Some(wi) { "var(--editor-accent-warm)" }
                                                                                        else { "var(--editor-border-soft)" }
                                                                                    ),
                                                                                    div {
                                                                                        style: "display: flex; align-items: center; color: var(--fg-muted); cursor: grab; padding: 0 2px;",
                                                                                        title: "Drag to reorder",
                                                                                        onmousedown: move |e| { e.prevent_default(); drag_socket.set(Some((k_drag.clone(), wi))); over_socket.set(Some(wi)); },
                                                                                        IconGrip {}
                                                                                    }
                                                                                    div {
                                                                                        style: "display: flex; flex-direction: column; justify-content: center; gap: 1px;",
                                                                                        button {
                                                                                            class: if wi > 0 { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                                                                            style: "padding: 1px 5px; line-height: 0;",
                                                                                            title: "Move up",
                                                                                            onclick: move |_| { if wi > 0 { let f = socket_move; f((k_up.clone(), wi, -1)); } },
                                                                                            IconChevronUp { size: "14".to_string() }
                                                                                        }
                                                                                        button {
                                                                                            class: if wi + 1 < n { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                                                                            style: "padding: 1px 5px; line-height: 0;",
                                                                                            title: "Move down",
                                                                                            onclick: move |_| { if wi + 1 < n { let f = socket_move; f((k_dn.clone(), wi, 1)); } },
                                                                                            IconChevronDown { size: "14".to_string() }
                                                                                        }
                                                                                    }
                                                                                    div {
                                                                                        style: "flex: 1; min-width: 0; display: flex; flex-direction: column; justify-content: center;",
                                                                                        div { style: "color: var(--fg-base); font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{title}" }
                                                                                        div { style: "font-size: 0.66rem; color: var(--fg-muted); font-family: var(--font-mono);", "{wid}" }
                                                                                    }
                                                                                    button {
                                                                                        class: "editor-mini-button",
                                                                                        style: "padding: 4px 6px; line-height: 0; align-self: center;",
                                                                                        title: "Remove from slot",
                                                                                        onclick: move |_| { let f = socket_remove; f((k_rm.clone(), wid_rm.clone())); },
                                                                                        IconTrash {}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        widget_layout::Part::Field { .. } => rsx! {},
                                                    } }
                                                }
                                                if !has_socket {
                                                    button {
                                                        class: "editor-button",
                                                        style: "align-self: flex-start; margin-top: 4px;",
                                                        onclick: move |_| { let mut f = open_gadget_picker; f(None); },
                                                        "+ Add a Gadget"
                                                    }
                                                }
                                                if !field_groups.is_empty() {
                                                    div {
                                                        style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--editor-border-soft);",
                                                        span { style: "display: block; color: var(--fg-muted); font-size: 0.68rem; font-family: var(--font-mono); margin-bottom: 4px;", "Fields" }
                                                        // Indented tree: each row is one node of the data-field-path
                                                        // structure (e.g. Footer ▸ Columns ▸ 1 ▸ Links ▸ 2) with its
                                                        // token chips. Keys are positional, so repeated tokens are safe.
                                                        for (gi, g) in field_groups.iter().enumerate() {
                                                            div {
                                                                key: "fg-{gi}",
                                                                style: format!("display: flex; flex-wrap: wrap; gap: 4px; align-items: center; margin: 2px 0; padding-left: {}px; {}", 8 + g.depth * 16, if g.depth > 0 { "border-left: 1px solid var(--editor-border-soft);" } else { "" }),
                                                                if !g.label.is_empty() {
                                                                    span { style: "color: var(--fg-base); font-size: 0.7rem; font-family: var(--font-mono); margin-right: 4px;", "{g.label}" }
                                                                }
                                                                for (ti, t) in g.tokens.iter().enumerate() {
                                                                    span {
                                                                        key: "fg-{gi}-{ti}",
                                                                        style: "padding: 1px 6px; border-radius: 3px; border: 1px solid var(--editor-border-soft); color: var(--fg-muted); font-size: 0.66rem; font-family: var(--font-mono);",
                                                                        "{t}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                CodeEditor {
                                    value: display_xml(),
                                    mode: "xml".to_string(),
                                    minimap: Some(false),
                                    minimap_key: Some("module_workbench".to_string()),
                                    on_change: move |new_val| {
                                        edited_xml.set(new_val);
                                        on_load_theme.call(config_toml());
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Right Pane ─ Module Preview (hidden during editor takeover) ──
                if !is_editor_takeover_active() {
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",

                    div {
                        style: "padding: 8px 12px; border-bottom: 1px solid var(--editor-border-soft); background: rgba(0,0,0,0.2); display: flex; align-items: center; justify-content: space-between;",
                        span {
                            style: "font-size: 0.8rem; color: var(--accent); font-family: var(--font-mono);",
                            "Module Preview"
                        }
                        button {
                            class: "editor-mini-button",
                            onclick: move |_| is_takeover_active.set(true),
                            "Takeover"
                        }
                    }

                    if module_preview_html().is_empty() {
                        div {
                            style: "flex: 1; display: flex; align-items: center; justify-content: center; color: var(--fg-muted); font-size: 0.9rem; font-family: var(--font-mono);",
                            "← select a module to preview"
                        }
                    } else {
                        PreviewCanvas {
                            preview_viewport,
                            preview_width,
                            preview_html: module_preview_html(),
                        }
                    }
                }
                }
            }
        }
    }
}
