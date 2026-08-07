use crate::app::state::{
    CenterView, ContextMenuPayload, LayoutState, PreviewContextInfo, ThemeState,
};
use crate::ui::workspace::layout::{apply_preview_viewport, PreviewViewport};
use dioxus::prelude::*;

const ITEM_STYLE: &str = "text-align: left; padding: 7px 10px; font-size: 12px; cursor: pointer; width: 100%; display: flex; align-items: center; gap: 8px; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5; transition: background 0.15s ease;";
const SECTION_STYLE: &str = "font-size: 9px; font-weight: bold; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em; margin-top: 4px;";

/// Tags whose right-click should offer the typography editor.
const TEXT_TAGS: &[&str] = &[
    "h1", "h2", "h3", "h4", "h5", "h6", "p", "span", "a", "li", "blockquote", "time", "small",
];

#[derive(Props, Clone, PartialEq)]
pub struct IconContextMenuProps {
    pub payload: ContextMenuPayload,
}

#[component]
fn MenuItem(icon: String, label: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "editor-mini-button",
            style: ITEM_STYLE,
            onclick: move |_| onclick.call(()),
            span { style: "font-size: 12px; width: 16px; text-align: center;", "{icon}" }
            span { "{label}" }
        }
    }
}

/// Color row: swatch + hex; click copies the hex and remembers it as a recent color.
#[component]
fn ColorItem(label: String, hex: String, onpick: EventHandler<String>) -> Element {
    let hex_for_click = hex.clone();
    rsx! {
        button {
            class: "editor-mini-button",
            style: ITEM_STYLE,
            title: "Copy {hex} and add to recent colors",
            onclick: move |_| onpick.call(hex_for_click.clone()),
            span { style: "width: 12px; height: 12px; border-radius: 3px; border: 1px solid #3d372c; background: {hex}; flex-shrink: 0;" }
            span { "{label} · {hex}" }
        }
    }
}

#[component]
pub fn IconContextMenu(props: IconContextMenuProps) -> Element {
    let mut layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();
    let target_id = props.payload.target_id.clone();

    let friendly_name = target_id
        .strip_prefix("icons.")
        .unwrap_or(&target_id)
        .replace('_', " ");

    let mut active_context_menu = layout.active_context_menu;

    rsx! {
        // Overlay to close when clicking outside
        div {
            style: "position: fixed; inset: 0; z-index: 9998; background: transparent;",
            onclick: move |_| { active_context_menu.set(None); },
            oncontextmenu: move |e| {
                e.prevent_default();
                active_context_menu.set(None);
            }
        }

        // Custom context menu container
        div {
            style: "position: fixed; left: {props.payload.x}px; top: {props.payload.y}px; z-index: 9999; background: #16140f; border: 1px solid #3c382f; border-radius: 8px; box-shadow: 0 10px 30px rgba(0,0,0,0.6); padding: 12px; width: 230px; display: flex; flex-direction: column; gap: 6px; user-select: none; max-height: calc(100vh - {props.payload.y}px - 12px); overflow-y: auto;",
            onclick: move |e| { e.stop_propagation(); },
            oncontextmenu: move |e| { e.prevent_default(); e.stop_propagation(); },

            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                span {
                    style: "font-size: 10px; font-weight: bold; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;",
                    match props.payload.kind.as_str() {
                        "preview" => {
                            let region = props.payload.preview.as_ref().map(|p| p.region.as_str()).unwrap_or("");
                            rsx! { "Preview · {region}" }
                        }
                        "svg" => rsx! { "Icon Context Menu" },
                        "ui_typography" => rsx! { "UI Context Menu" },
                        _ => rsx! { "Context Menu" }
                    }
                }
                span {
                    style: "font-size: 12px; font-weight: 600; color: var(--accent); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{friendly_name}"
                }
            }

            hr { style: "border: 0; border-top: 1px solid var(--border-color); margin: 2px 0 2px 0;" }

            match props.payload.kind.as_str() {
                "preview" => {
                    let info = props.payload.preview.clone().unwrap_or_default();
                    rsx! { PreviewMenu { info } }
                }
                "svg" => {
                    let is_dock = target_id == "Theme Palette"
                        || target_id == "CSS Editor"
                        || target_id == "JS Editor"
                        || target_id == "XML Editor"
                        || target_id == "Site Data";
                    let is_pinned = layout.is_dock_pinned(&target_id);
                    rsx! {
                        if is_dock {
                            MenuItem {
                                icon: if is_pinned { "📌" } else { "📍" },
                                label: if is_pinned { "Unpin" } else { "Pin to Activity Bar" },
                                onclick: {
                                    let target_id = target_id.clone();
                                    move |_| {
                                        layout.toggle_pinned_dock(&target_id);
                                        active_context_menu.set(None);
                                    }
                                },
                            }
                        }
                        MenuItem {
                            icon: "✨",
                            label: "Swap Icon...",
                            onclick: move |_| {
                                layout.active_icon_picker.set(Some(target_id.clone()));
                                active_context_menu.set(None);
                            },
                        }
                    }
                },
                "ui_typography" => rsx! {
                    MenuItem {
                        icon: "✍️",
                        label: "Edit Theme Typography",
                        onclick: move |_| {
                            theme.show_advanced_typography.clone().set(true);
                            active_context_menu.set(None);
                        },
                    }
                },
                "dock" => {
                    let is_pinned = layout.is_dock_pinned(&target_id);
                    let icon_target = target_id.clone();
                    rsx! {
                        MenuItem {
                            icon: if is_pinned { "📌" } else { "📍" },
                            label: if is_pinned { "Unpin from Activity Bar" } else { "Pin to Activity Bar" },
                            onclick: move |_| {
                                layout.toggle_pinned_dock(&target_id);
                                active_context_menu.set(None);
                            },
                        }
                        MenuItem {
                            icon: "✨",
                            label: "Change Icon...",
                            onclick: move |_| {
                                layout.active_activity_icon_picker.set(Some(icon_target.clone()));
                                active_context_menu.set(None);
                            },
                        }
                    }
                },
                _ => rsx! {
                    div {
                        style: "font-size: 11px; color: var(--fg-muted); padding: 4px;",
                        "Unknown Context"
                    }
                }
            }
        }
    }
}

/// The extensive right-click menu for the preview canvas: every action that
/// applies to what's under the cursor, grouped Edit / Colors / View / History.
#[component]
fn PreviewMenu(info: PreviewContextInfo) -> Element {
    let mut layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();
    let mut active_context_menu = layout.active_context_menu;

    let is_text = TEXT_TAGS.contains(&info.tag.as_str());
    // An `icons.*` edit target is a swappable icon.
    let icon_target = info
        .edit_target
        .clone()
        .filter(|t| t.starts_with("icons."));
    // Best token to reveal in the code editor, most specific first.
    let reveal_target = info
        .field_path
        .clone()
        .or_else(|| info.edit_target.clone())
        .or_else(|| info.block_id.clone());

    let pick_color = move |hex: String| {
        crate::utils::clipboard::copy_to_clipboard(hex.clone());
        theme.push_recent_color(&hex);
        active_context_menu.set(None);
    };

    let has_edit_section =
        icon_target.is_some() || info.field_path.is_some() || reveal_target.is_some() || is_text;

    rsx! {
        if has_edit_section {
            span { style: SECTION_STYLE, "Edit" }
        }

        if let Some(icon) = icon_target {
            MenuItem {
                icon: "✨",
                label: "Swap Icon...",
                onclick: move |_| {
                    layout.active_icon_picker.set(Some(icon.clone()));
                    active_context_menu.set(None);
                },
            }
        }

        if let Some(fp) = info.field_path.clone() {
            MenuItem {
                icon: "✏️",
                label: "Edit Text Inline",
                onclick: move |_| {
                    let js = format!(
                        r#"(function() {{
                            const frm = document.getElementById('mor-preview-frame');
                            const doc = frm && (frm.contentDocument || frm.contentWindow.document);
                            const el = doc && doc.querySelector('[data-field-path="{fp}"]');
                            if (el) {{ el.scrollIntoView({{block: 'center'}}); el.contentEditable = true; el.focus(); }}
                        }})();"#
                    );
                    let _ = dioxus::document::eval(&js);
                    active_context_menu.set(None);
                },
            }
        }

        if let Some(target) = reveal_target.clone() {
            MenuItem {
                icon: "📄",
                label: "Reveal in Code Editor",
                onclick: move |_| {
                    layout.active_xray_target.set(Some(target.clone()));
                    layout.center_view.set(CenterView::Split);
                    active_context_menu.set(None);
                },
            }
        }

        if is_text {
            MenuItem {
                icon: "✍️",
                label: "Edit Typography...",
                onclick: move |_| {
                    theme.show_advanced_typography.clone().set(true);
                    active_context_menu.set(None);
                },
            }
        }

        if info.color.is_some() || info.bg.is_some() {
            span { style: SECTION_STYLE, "Colors · click to copy" }
        }
        if let Some(c) = info.color.clone() {
            ColorItem { label: "Text", hex: c, onpick: pick_color }
        }
        if let Some(b) = info.bg.clone() {
            ColorItem { label: "Fill", hex: b, onpick: pick_color }
        }

        if let Some(link) = info.link.clone() {
            span { style: SECTION_STYLE, "Link" }
            MenuItem {
                icon: "🔗",
                label: "Copy Link Address",
                onclick: move |_| {
                    crate::utils::clipboard::copy_to_clipboard(link.clone());
                    active_context_menu.set(None);
                },
            }
        }

        span { style: SECTION_STYLE, "View" }
        MenuItem {
            icon: "🩻",
            label: if (layout.is_xray_active)() { "X-Ray Off" } else { "X-Ray Inspect" },
            onclick: move |_| {
                let now = !(layout.is_xray_active)();
                layout.is_xray_active.set(now);
                active_context_menu.set(None);
            },
        }
        MenuItem {
            icon: if (theme.signals.is_dark_mode)() { "☀" } else { "☾" },
            label: if (theme.signals.is_dark_mode)() { "Switch to Light Mode" } else { "Switch to Dark Mode" },
            onclick: move |_| {
                theme.perform_dark_mode_toggle();
                active_context_menu.set(None);
            },
        }
        div {
            style: "display: flex; gap: 4px;",
            for vp in [PreviewViewport::Fit, PreviewViewport::Desktop, PreviewViewport::Tablet, PreviewViewport::Phone] {
                button {
                    class: "editor-mini-button",
                    style: "flex: 1; padding: 5px 0; font-size: 10px; cursor: pointer; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5;",
                    title: "Preview as {vp.label()}",
                    onclick: move |_| {
                        apply_preview_viewport(vp, layout.preview_width);
                        layout.preview_viewport.set(vp);
                        active_context_menu.set(None);
                    },
                    "{vp.label()}"
                }
            }
        }

        span { style: SECTION_STYLE, "History" }
        {
            // Hint the real (possibly rebound) shortcuts, penpot-style.
            let sc = try_consume_context::<Signal<crate::app::config_bridge::ShortcutPrefs>>();
            let hint = |v: fn(&crate::app::config_bridge::ShortcutPrefs) -> Option<String>| {
                sc.and_then(|s| v(&s.read())).map(|c| format!(" ({c})")).unwrap_or_default()
            };
            let undo_hint = hint(|s| s.undo.clone());
            let redo_hint = hint(|s| s.redo.clone());
            rsx! {
                div {
                    style: "display: flex; gap: 4px;",
                    button {
                        class: "editor-mini-button",
                        style: "flex: 1; padding: 6px 0; font-size: 11px; cursor: pointer; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5;",
                        disabled: !theme.can_undo(),
                        title: "Undo{undo_hint}",
                        onclick: move |_| {
                            theme.undo();
                            active_context_menu.set(None);
                        },
                        "↶ Undo"
                    }
                    button {
                        class: "editor-mini-button",
                        style: "flex: 1; padding: 6px 0; font-size: 11px; cursor: pointer; border-radius: 4px; background: #232018; border: 1px solid #3d372c; color: #e6e1d5;",
                        disabled: !theme.can_redo(),
                        title: "Redo{redo_hint}",
                        onclick: move |_| {
                            theme.redo();
                            active_context_menu.set(None);
                        },
                        "↷ Redo"
                    }
                }
            }
        }
    }
}
