use crate::app::state::{DockPosition, LayoutState};
use crate::ui::components::dock_chrome::DockChrome;
use dioxus::prelude::*;

// The TOML and XML buffers share one CodeEditor instance (Dioxus reuses it across
// the toggle, so its host id is fixed at first mount). Both the editor and this
// nav dock reference it; the live doc is whichever buffer is showing.
pub const CODE_EDITOR_ID: &str = "code-editor-buffer";

/// Scroll the shared Code Editor buffer to the first occurrence of `target`.
/// No-op if the editor isn't mounted (e.g. not on the Code Editor view).
pub fn reveal_code_target(target: String) {
    spawn(async move {
        let eval = dioxus::document::eval(
            r#"
            let m = await dioxus.recv();
            if (window.morCM) window.morCM.reveal(m.id, m.t);
            "#,
        );
        let _ = eval.send(serde_json::json!({ "id": CODE_EDITOR_ID, "t": target }));
    });
}

// (label, toml_anchor, xml_anchor). The dock reveals the matching anchor in
// whichever buffer is showing, so it navigates both the TOML and compiled XML.
const TEMPLATE_LAYOUTS: &[(&str, &str, &str)] = &[
    ("Header Variant", "header_variant", "<header"),
    ("Main Canvas Variant", "main_variant", "class='canvas-core'"),
    ("Content Feed", "content_variant", "id='Blog1'"),
    ("Left Sidebar", "left_sidebar_variant", "id='panel-left'"),
    ("Right Sidebar", "right_sidebar_variant", "id='panel-right'"),
    ("Footer Grid", "footer_variant", "<footer"),
];

const CORE_IDENTITY: &[(&str, &str, &str)] = &[
    ("Site Information", "[site]", "name='description'"),
    ("Color Palette", "[colors]", "name='theme-color'"),
    ("Typography", "[typography]", "--font-body"),
];

#[component]
pub fn CodeNavDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let pos = (layout.code_nav_pos)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let show_xml = layout.code_show_xml;
    let mut active_target = use_signal(|| None::<String>);

    // Reveal this item's anchor in whichever buffer the editor is currently showing.
    let mut on_nav = move |toml_anchor: &str, xml_anchor: &str| {
        active_target.set(Some(toml_anchor.to_string()));
        let target = if show_xml() { xml_anchor } else { toml_anchor };
        reveal_code_target(target.to_string());
    };

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "Code Nav".to_string(),
                dock_id: "code_nav".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.code_nav_pos.set(DockPosition::Hidden);
                },
                div {
                    class: "code-nav-panel",
                    style: "height: calc(100% - 45px); overflow-y: auto; background: var(--bg-panel);",

                    div {
                        style: "padding: 12px; border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                        span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Template Layouts" }
                    }
                    div {
                        style: "padding: 12px; display: flex; flex-direction: column; gap: 8px;",
                        for (label, toml_anchor, xml_anchor) in TEMPLATE_LAYOUTS {
                            button {
                                class: if active_target() == Some(toml_anchor.to_string()) { "editor-button editor-button-active" } else { "editor-button" },
                                style: "text-align: left; font-size: 0.85rem;",
                                onclick: move |_| on_nav(toml_anchor, xml_anchor),
                                "{label}"
                            }
                        }
                    }

                    div {
                        style: "padding: 12px; border-top: 1px solid var(--editor-border-soft); border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                        span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Core Identity" }
                    }
                    div {
                        style: "padding: 12px; display: flex; flex-direction: column; gap: 8px;",
                        for (label, toml_anchor, xml_anchor) in CORE_IDENTITY {
                            button {
                                class: if active_target() == Some(toml_anchor.to_string()) { "editor-button editor-button-active" } else { "editor-button" },
                                style: "text-align: left; font-size: 0.85rem;",
                                onclick: move |_| on_nav(toml_anchor, xml_anchor),
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
