use crate::app::state::{DockPosition, LayoutState, RenderState};
use crate::ui::components::icons::{IconClose, IconDockLeft, IconDockRight, IconFloat, IconGrip};
use dioxus::prelude::*;
use mor_blogger_core::diagnostics::Severity;

#[component]
pub fn DiagnosticsDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let pos = (layout.diagnostics_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let is_floating = pos == DockPosition::Floating;

    let header_actions = rsx! {
        div { class: "floating-editor-actions",
            if pos != DockPosition::mor_panel_left {
                button {
                    title: "Dock Left",
                    onclick: move |_| {
                        layout.request_exclusive_dock("diagnostics", DockPosition::mor_panel_left);
                    },
                    IconDockLeft {}
                }
            }
            if pos != DockPosition::mor_panel_right {
                button {
                    title: "Dock Right",
                    onclick: move |_| {
                        layout.request_exclusive_dock("diagnostics", DockPosition::mor_panel_right);
                    },
                    IconDockRight {}
                }
            }
            if !is_floating {
                button {
                    title: "Float Window",
                    onclick: move |_| {
                        layout.request_exclusive_dock("diagnostics", DockPosition::Floating);
                    },
                    IconFloat {}
                }
            }
            button {
                title: "Close",
                onclick: move |_| {
                    layout.diagnostics_pos.set(DockPosition::Hidden);
                },
                IconClose {}
            }
        }
    };

    let result = render.diag.read();
    let error_count = result.errors.len();
    let warning_count = result
        .warnings
        .iter()
        .filter(|w| w.severity == Severity::Warning)
        .count();

    let inner_content = rsx! {
        if is_floating {
            div { class: "floating-editor-window-bar",
                div {
                    class: "floating-editor-grip-group",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", IconGrip {} }
                    span {
                        class: "floating-editor-title",
                        "Diagnostics"
                    }
                }
                {header_actions}
            }
        } else {
            div { class: "editor-panel-header",
                h2 { class: "editor-panel-title", "Diagnostics" }
                {header_actions}
            }
        }

        section {
            "aria-live": "polite",
            class: "diagnostics-panel",
            style: "padding: 16px; height: calc(100% - 45px); overflow-y: auto; background: var(--bg-panel); color: var(--fg-base); font-size: 0.85rem;",

            if result.is_valid && warning_count == 0 {
                div {
                    class: "diagnostics-ok",
                    style: "display: flex; align-items: center; gap: 8px; color: #73c991; padding: 12px; background: rgba(115, 201, 145, 0.1); border: 1px solid rgba(115, 201, 145, 0.2); border-radius: 4px;",
                    span {
                        class: "diagnostics-ok-icon",
                        style: "font-size: 1.25rem;",
                        "✓"
                    }
                    span { "sys.integrity — all checks passed" }
                }
            } else {
                div {
                    class: "diagnostics-problems",
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    div {
                        class: "diagnostics-badges",
                        style: "display: flex; gap: 8px;",

                        if error_count > 0 {
                            span {
                                class: "diagnostics-badge diagnostics-error",
                                style: "background: #ea8285; color: #111; padding: 2px 8px; border-radius: 3px; font-weight: bold; font-size: 0.75rem;",
                                "{error_count} error(s)"
                            }
                        }

                        if warning_count > 0 {
                            span {
                                class: "diagnostics-badge diagnostics-warning",
                                style: "background: #d29922; color: #111; padding: 2px 8px; border-radius: 3px; font-weight: bold; font-size: 0.75rem;",
                                "{warning_count} warning(s)"
                            }
                        }
                    }

                    for (i, err) in result.errors.iter().enumerate() {
                        div {
                            key: "e-{i}",
                            class: "diagnostics-error",
                            style: "display: flex; gap: 6px; padding: 8px; background: rgba(234, 130, 133, 0.1); border: 1px solid rgba(234, 130, 133, 0.2); border-radius: 3px;",
                            span {
                                class: "diagnostics-prefix",
                                style: "color: #ea8285; font-weight: bold; font-family: monospace;",
                                "ERR"
                            }
                            span { style: "color: #ea8285;", "{err}" }
                        }
                    }

                    for (i, w) in result.warnings.iter()
                        .filter(|w| w.severity == Severity::Warning)
                        .enumerate()
                    {
                        div {
                            key: "w-{i}",
                            class: "diagnostics-warning",
                            style: "display: flex; gap: 6px; padding: 8px; background: rgba(210, 153, 34, 0.1); border: 1px solid rgba(210, 153, 34, 0.2); border-radius: 3px;",
                            span {
                                class: "diagnostics-prefix",
                                style: "color: #d29922; font-weight: bold; font-family: monospace;",
                                "WRN"
                            }
                            span { style: "color: #d29922;", "[{w.code}] {w.message}" }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            {inner_content}
        }
    }
}
