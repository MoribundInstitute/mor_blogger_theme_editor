//! Dedicated Widget Workbench.
//! A focused editing stage for ONE Blogger widget blueprint (its own preview and
//! Save, no module chrome). Symmetric with the Module Workbench, but the unit of
//! work is a single `workspace/widgets/<group>/<name>.xml` blueprint, opened from
//! the Widgets dock. Reuses the same blueprint CRUD, XML parsing, code editor and
//! theme CSS the module workbench uses, so the two never drift.

use std::collections::HashMap;

use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::render::preview;
use mor_blogger_core::render::template_resolver;
use mor_blogger_core::render::xml_parts::css_generator::render_css_sockets;
use mor_blogger_core::utils::fs_bridge;

use crate::app::shell::WorkbenchEditState;
use crate::app::vfs::VfsDictionary;
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::icons::{
    IconChevronDown, IconChevronUp, IconEye, IconEyeOff, IconTrash,
};
use crate::ui::workspace::layout::{apply_preview_viewport, clamp_preview_width, PreviewViewport};
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use crate::ui::workspace::widget_layout;

/// Pull the previewable HTML out of a widget blueprint.
/// HTML/custom gadgets store literal markup in `<![CDATA[ … ]]>`, which renders
/// statically. Dynamic widgets (Blog, Label, Archive) only hold Blogger
/// templating (`b:loop` / `data:`), which can't render outside Blogger — those
/// return an empty body and `dynamic = true`.
fn extract_widget_body(xml: &str) -> (String, bool) {
    let mut cdata = String::new();
    let mut rest = xml;
    while let Some(s) = rest.find("<![CDATA[") {
        let after = &rest[s + 9..];
        match after.find("]]>") {
            Some(e) => {
                cdata.push_str(&after[..e]);
                cdata.push('\n');
                rest = &after[e + 3..];
            }
            None => break,
        }
    }
    if cdata.trim().is_empty() {
        // No literal HTML → a dynamic Blogger widget; nothing to render statically.
        return (String::new(), true);
    }
    let dynamic =
        cdata.contains("<data:") || cdata.contains("<b:") || cdata.contains("expr:");
    (cdata, dynamic)
}

/// The notification-bell gadget is a FeaturedPost whose markup lives in
/// `b:includable`s (not CDATA) and is fed by `data:` post fields, so neither the
/// CDATA path nor the generic by-type card can show it. Recognize it by its
/// `.mor-bell` class and render the blueprint's own `<style>` plus an *opened*
/// dropdown with a sample post, so the workbench shows the real thing.
/// ponytail: reuses the blueprint's CSS verbatim (no drift); sample post stands
/// in for Blogger's data:post, which only exists on the live blog.
fn bell_preview_html(widget_xml: &str) -> Option<String> {
    if !widget_xml.contains("mor-bell") {
        return None;
    }
    let mut styles = String::new();
    let mut rest = widget_xml;
    while let Some(s) = rest.find("<style>") {
        let after = &rest[s + "<style>".len()..];
        let Some(e) = after.find("</style>") else { break };
        styles.push_str(&after[..e]);
        styles.push('\n');
        rest = &after[e + "</style>".len()..];
    }
    Some(format!(
        r##"<style>{styles}</style>
<style>
/* preview-only: the bell sits at the top-left of this isolated canvas, so anchor
   the floating panel leftward — the blueprint's right:0 (correct on the blog,
   where the bell is right-aligned) would push the 280px panel off-screen here. */
.mor-bell-panel {{ right: auto; left: 0; }}
body {{ min-height: 240px; }}
</style>
<div class="mor-bell open">
  <button class="mor-bell-btn" type="button" aria-label="Recent post">
    <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M12 2a6 6 0 0 0-6 6c0 5-2 7-2 7h16s-2-2-2-7a6 6 0 0 0-6-6zm0 20a3 3 0 0 0 3-3H9a3 3 0 0 0 3 3z"/></svg>
  </button>
  <div class="mor-bell-panel">
    <h3 class="mor-bell-heading">Newest Post</h3>
    <a class="mor-bell-post" href="#">
      <span class="mor-bell-title">Sample: The Newest Post Title</span>
      <span class="mor-bell-snippet">A preview of the most recent post summary. On the live blog, Blogger fills this with your newest entry.</span>
      <span class="mor-bell-thumb" style="display:block;height:120px;margin-top:10px;border-radius:8px;background:linear-gradient(135deg,var(--bg-base,#222),var(--accent,#555));opacity:.55;"></span>
    </a>
  </div>
</div>"##
    ))
}

/// Render a single widget blueprint to an isolated HTML document, wrapped in the
/// live theme CSS so it looks the way it will on the blog.
fn widget_preview_html(
    widget_xml: &str,
    config: &ThemeConfig,
    vfs: &HashMap<String, String>,
) -> String {
    let (body, dynamic) = extract_widget_body(widget_xml);
    let parts = template_resolver::resolve_template_parts(config, vfs);
    // Decode XML entities: this CSS is Blogger-escaped but goes into a browser
    // <style>, where entities aren't decoded (see unescape_for_style).
    let true_css =
        mor_blogger_core::render::util::unescape_for_style(&render_css_sockets(parts.css, config));

    let inner = if let Some(bell) = bell_preview_html(widget_xml) {
        bell
    } else if body.trim().is_empty() {
        // No static HTML (dynamic Blogger widget). Master canvas: render the widget
        // by type with representative dummy data, inside the live theme CSS, so the
        // user sees an approximation of how it lands on the blog.
        let slots = widget_layout::parse_slots(widget_xml);
        let (w_type, title) = slots
            .first()
            .map(|s| (s.w_type.clone(), s.title.clone()))
            .unwrap_or_default();
        preview::preview_widget_html(config, &w_type, &title, &[])
    } else if dynamic {
        format!(
            r#"<div style="margin:0 0 16px;padding:8px 12px;border-radius:6px;background:#3a2f12;color:#eab308;font:0.8rem/1.4 monospace;">⚠ Uses Blogger templating (b:loop / data:). Dynamic parts render only on the live blog.</div>{body}"#
        )
    } else {
        body
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style id="mor-true-css">
{true_css}
html, body {{ margin: 0; padding: 16px; }}
</style>
</head>
<body>
{inner}
</body>
</html>"#
    )
}

#[component]
pub fn WidgetWorkbench(config_toml: ReadSignal<String>) -> Element {
    let edit_state = use_context::<WorkbenchEditState>();
    let vfs = use_context::<VfsDictionary>().0;

    // (group, name) of the blueprint under edit; None = nothing opened yet.
    let mut active_widget: Signal<Option<(String, String)>> = use_signal(|| None);
    let mut edited_xml: Signal<String> = use_signal(String::new);
    let mut status: Signal<String> = use_signal(String::new);
    let mut layout_view = use_signal(|| true);

    let mut preview_viewport: Signal<PreviewViewport> = use_signal(|| PreviewViewport::Fit);
    let mut preview_width: Signal<u32> = use_signal(|| 1200u32);

    // Honor "edit this widget" requests from the Widgets dock: load the blueprint
    // into the buffer. Consumed once, then cleared.
    {
        let mut req = edit_state.edit_widget_request;
        use_effect(move || {
            if let Some(bp) = req() {
                active_widget.set(Some((bp.group.clone(), bp.name.clone())));
                edited_xml.set(bp.xml.clone());
                status.set(String::new());
                req.set(None);
            }
        });
    }

    let preview_html = use_memo(move || {
        if active_widget().is_none() {
            return String::new();
        }
        let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
        widget_preview_html(&edited_xml(), &config, &*vfs.read())
    });

    // Save the buffer back to its blueprint file.
    let save_widget = move |_: Event<MouseData>| {
        let Some((g, n)) = active_widget() else { return; };
        match fs_bridge::save_widget_blueprint(&g, &n, &edited_xml()) {
            Ok(p) => status.set(format!("Saved → {}", p.display())),
            Err(e) => status.set(format!("Save failed: {e}")),
        }
    };

    // Drop session edits, reloading the on-disk version of this blueprint.
    let revert = move |_: Event<MouseData>| {
        let Some((g, n)) = active_widget() else { return; };
        if let Some(bp) = fs_bridge::load_widget_blueprints()
            .into_iter()
            .find(|b| b.group == g && b.name == n)
        {
            edited_xml.set(bp.xml);
            status.set("Reverted to saved.".to_string());
        }
    };

    // Commit a Layout-view edit (visibility/remove/reorder rewrites the buffer).
    let apply_buffer = move |new_xml: String| edited_xml.set(new_xml);

    rsx! {
        div {
            class: "export-viewport",
            style: "display: flex; flex-direction: row; flex: 1; min-height: 0; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel);",

            // ── Left Pane ─ Widget XML editor ─────────────────────────────
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
                                { active_widget().map(|(g, n)| format!("{g}/{n}.xml")).unwrap_or_else(|| "no widget selected".to_string()) }
                            }
                            span {
                                style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                                "Blogger Widget · Live"
                            }
                        }
                        div {
                            style: "display: flex; align-items: center; gap: 6px;",
                            button {
                                class: if layout_view() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                                title: "Visual widget layout",
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
                                class: if active_widget().is_some() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                title: "Reload the saved version (drops session edits)",
                                onclick: revert,
                                "Revert"
                            }
                            button {
                                class: if active_widget().is_some() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                title: "Save this widget blueprint",
                                onclick: save_widget,
                                "Save Widget"
                            }
                        }
                    }

                    if !status().is_empty() {
                        div {
                            style: "padding: 4px 12px; font-size: 0.75rem; color: var(--editor-accent-warm); background: rgba(0,0,0,0.15); font-family: var(--font-mono); border-bottom: 1px solid var(--editor-border-soft); display: flex; justify-content: space-between; align-items: center; flex-shrink: 0;",
                            span { "{status()}" }
                            button {
                                style: "background: none; border: none; color: var(--fg-muted); cursor: pointer; font-size: 0.9rem; padding: 0 2px;",
                                onclick: move |_| status.set(String::new()),
                                "×"
                            }
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",
                        if active_widget().is_none() {
                            div {
                                style: "margin: auto; text-align: center; color: var(--fg-muted); font-size: 0.85rem; font-family: var(--font-mono); line-height: 1.8; padding: 40px;",
                                "Pick a widget in the Widgets dock (Edit), or create one with + New Widget."
                            }
                        } else if layout_view() {
                            // ── Layout view ─ the blueprint's widget cards + fields ──
                            div {
                                style: "flex: 1; min-height: 0; overflow-y: auto; padding: 10px; display: flex; flex-direction: column; gap: 8px;",
                                // ── Settings ─ editable title + marked fields, written into the buffer ──
                                {
                                    let xml_now = edited_xml();
                                    let cur_title = widget_layout::parse_slots(&xml_now).first().map(|s| s.title.clone()).unwrap_or_default();
                                    let fields = widget_layout::parse_fields(&xml_now);
                                    rsx! {
                                        div {
                                            class: "editor-card",
                                            style: "display: flex; flex-direction: column; gap: 8px; padding: 10px 12px;",
                                            span { style: "color: var(--fg-muted); font-size: 0.68rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;", "Settings" }
                                            label {
                                                style: "display: flex; flex-direction: column; gap: 2px; font-size: 0.72rem; color: var(--fg-muted);",
                                                "Title"
                                                input {
                                                    class: "editor-input",
                                                    style: "width: 100%; font-size: 0.8rem; padding: 4px 6px;",
                                                    value: "{cur_title}",
                                                    onchange: move |e| { let mut f = apply_buffer; f(widget_layout::set_widget_title(&edited_xml(), &e.value())); }
                                                }
                                            }
                                            for (i, fld) in fields.into_iter().enumerate() {
                                                label {
                                                    key: "fld-{i}",
                                                    style: "display: flex; flex-direction: column; gap: 2px; font-size: 0.72rem; color: var(--fg-muted);",
                                                    "{fld.label}"
                                                    if fld.options.is_empty() {
                                                        input {
                                                            class: "editor-input",
                                                            style: "width: 100%; font-size: 0.8rem; padding: 4px 6px;",
                                                            value: "{fld.value}",
                                                            onchange: move |e| { let mut f = apply_buffer; f(widget_layout::set_field(&edited_xml(), i, &e.value())); }
                                                        }
                                                    } else {
                                                        select {
                                                            class: "editor-input",
                                                            style: "width: 100%; font-size: 0.8rem; padding: 4px 6px;",
                                                            value: "{fld.value}",
                                                            onchange: move |e| { let mut f = apply_buffer; f(widget_layout::set_field(&edited_xml(), i, &e.value())); },
                                                            for opt in fld.options.iter() {
                                                                option { key: "{opt}", value: "{opt}", "{opt}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    let xml = edited_xml();
                                    let slots = widget_layout::parse_slots(&xml);
                                    let n = slots.len();
                                    let parts = widget_layout::parse_parts(&xml);
                                    let field_groups = widget_layout::group_fields(&parts);
                                    if slots.is_empty() && field_groups.is_empty() {
                                        rsx! {
                                            div { style: "margin: auto; color: var(--fg-muted); font-size: 0.85rem; font-family: var(--font-mono);", "Empty blueprint — switch to Code to add widget XML." }
                                        }
                                    } else {
                                        rsx! {
                                            for (slot, w) in slots.into_iter().enumerate() {
                                                div {
                                                    key: "w{slot}-{w.id}",
                                                    class: "layout-card",
                                                    style: format!("display: flex; align-items: stretch; gap: 8px; padding: 8px 10px; border-radius: 6px; background: var(--bg-elevated); opacity: {}; border: 1px solid var(--editor-border-soft);", if w.visible { "1" } else { "0.55" }),
                                                    div {
                                                        style: "display: flex; flex-direction: column; justify-content: center; gap: 1px;",
                                                        button {
                                                            class: if slot > 0 { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                                            style: "padding: 1px 5px; line-height: 0;",
                                                            title: "Move up",
                                                            onclick: move |_| { if slot > 0 { let mut f = apply_buffer; f(widget_layout::reorder(&edited_xml(), slot, slot - 1)); } },
                                                            IconChevronUp { size: "14".to_string() }
                                                        }
                                                        button {
                                                            class: if slot + 1 < n { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                                            style: "padding: 1px 5px; line-height: 0;",
                                                            title: "Move down",
                                                            onclick: move |_| { if slot + 1 < n { let mut f = apply_buffer; f(widget_layout::reorder(&edited_xml(), slot, slot + 1)); } },
                                                            IconChevronDown { size: "14".to_string() }
                                                        }
                                                    }
                                                    button {
                                                        class: "editor-mini-button",
                                                        style: "padding: 4px 6px; line-height: 0; align-self: center;",
                                                        title: if w.visible { "Hide widget" } else { "Show widget" },
                                                        onclick: move |_| {
                                                            let xml = edited_xml();
                                                            let cur = widget_layout::parse_slots(&xml).get(slot).map(|s| s.visible).unwrap_or(true);
                                                            let mut f = apply_buffer;
                                                            f(widget_layout::set_visible(&xml, slot, !cur));
                                                        },
                                                        if w.visible { IconEye {} } else { IconEyeOff {} }
                                                    }
                                                    div {
                                                        style: "flex: 1; min-width: 0; display: flex; flex-direction: column; justify-content: center;",
                                                        div { style: "font-size: 0.82rem; font-weight: 600; color: var(--fg-base); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", { if w.title.trim().is_empty() { w.id.clone() } else { w.title.clone() } } }
                                                        div { style: "font-size: 0.68rem; color: var(--fg-muted); font-family: var(--font-mono);", "{w.w_type} · {w.id}" }
                                                    }
                                                    button {
                                                        class: "editor-mini-button",
                                                        style: "padding: 4px 6px; line-height: 0; align-self: center;",
                                                        title: "Remove this widget from the blueprint",
                                                        onclick: move |_| { let mut f = apply_buffer; f(widget_layout::remove(&edited_xml(), slot)); },
                                                        IconTrash {}
                                                    }
                                                }
                                            }
                                            if !field_groups.is_empty() {
                                                div {
                                                    style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--editor-border-soft);",
                                                    span { style: "display: block; color: var(--fg-muted); font-size: 0.68rem; font-family: var(--font-mono); margin-bottom: 4px;", "Fields" }
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
                                value: edited_xml(),
                                mode: "xml".to_string(),
                                minimap: Some(false),
                                minimap_key: Some("widget_workbench".to_string()),
                                on_change: move |new_val| edited_xml.set(new_val),
                            }
                        }
                    }
                }
            }

            // ── Right Pane ─ Widget preview ───────────────────────────────
            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",
                div {
                    style: "padding: 8px 12px; border-bottom: 1px solid var(--editor-border-soft); background: rgba(0,0,0,0.2); display: flex; align-items: center; gap: 6px;",
                    span { style: "font-size: 0.8rem; color: var(--accent); font-family: var(--font-mono); flex: 1;", "Widget Preview" }
                    for (vp, lbl) in [(PreviewViewport::Phone, "Phone"), (PreviewViewport::Tablet, "Tablet"), (PreviewViewport::Fit, "Fit")] {
                        button {
                            key: "{lbl}",
                            class: if preview_viewport() == vp { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                            onclick: move |_| { apply_preview_viewport(vp, preview_width); preview_viewport.set(vp); },
                            "{lbl}"
                        }
                    }
                    label {
                        class: "preview-width-control",
                        span { class: "preview-width-label", "W" }
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
                }

                if preview_html().is_empty() {
                    div {
                        style: "flex: 1; display: flex; align-items: center; justify-content: center; color: var(--fg-muted); font-size: 0.9rem; font-family: var(--font-mono);",
                        "no widget selected"
                    }
                } else {
                    PreviewCanvas {
                        preview_viewport,
                        preview_width,
                        preview_html: preview_html(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::extract_widget_body;

    #[test]
    fn html_widget_body_is_extracted_and_static() {
        let xml = "<b:widget id='HTML1' type='HTML'><b:includable id='main'><![CDATA[<div class='x'>hi</div>]]></b:includable></b:widget>";
        let (body, dynamic) = extract_widget_body(xml);
        assert!(body.contains("<div class='x'>hi</div>"));
        assert!(!dynamic);
    }

    #[test]
    fn dynamic_widget_has_no_static_body() {
        let xml = "<b:widget id='Blog1' type='Blog'><b:includable id='main'><b:loop values='data:posts'>x</b:loop></b:includable></b:widget>";
        let (body, dynamic) = extract_widget_body(xml);
        assert!(body.trim().is_empty());
        assert!(dynamic);
    }

    #[test]
    fn cdata_with_blogger_tags_flags_dynamic() {
        let xml = "<b:widget><b:includable id='main'><![CDATA[<data:title/>]]></b:includable></b:widget>";
        let (_b, dynamic) = extract_widget_body(xml);
        assert!(dynamic);
    }
}
