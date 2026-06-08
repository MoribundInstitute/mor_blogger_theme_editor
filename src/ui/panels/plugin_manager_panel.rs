//! Floating Plugin Manager window for the Moribund Theme Architect.
//!
//! Follows the suckless line of the project: no extra dependencies, standard
//! HTML form controls, and a single owned-scroll workbench layout (no rigid
//! split-pane). The visual register is deliberately Old-World / Dark Academia
//! — a high-contrast ink-and-ivory monochrome with serif chrome and hairline
//! rules. There are no purple accents anywhere; the native checkbox tint is
//! pinned to ivory via `accent-color` so the browser default cannot leak in.

use dioxus::prelude::*;
use crate::app::config_bridge::PluginState;

/// Basic information needed to display a plugin on the screen.
#[derive(Clone, PartialEq, Props)]
pub struct PluginDisplayInfo {
    pub id: &'static str,
    pub display_name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

#[component]
pub fn PluginManagerPanel(
    mut show_panel: Signal<bool>,
    launch_state: ReadSignal<Vec<PluginState>>,
    mut current_state: Signal<Vec<PluginState>>,
    available_plugins: ReadSignal<Vec<PluginDisplayInfo>>,
) -> Element {
    // Restart is required whenever the working set the user is editing has
    // drifted from the set the application booted with. Comparison is done by
    // value through the read guards — no Vec is cloned to compute it.
    let needs_restart = use_memo(move || *launch_state.read() != *current_state.read());

    // Hooks above this line always run; the visibility gate is safe here.
    if !show_panel() {
        return rsx! {};
    }

    // Cheap scalar summaries (usize is Copy — the underlying Vecs are not cloned).
    let enabled_count = current_state.read().iter().filter(|s| s.enabled).count();
    let total_count = available_plugins.read().len();

    rsx! {
        section {
            class: "plugin-manager-modal floating-window",
            role: "dialog",
            "aria-label": "Plugin Manager",
            style: "position: fixed; top: 88px; left: 50%; transform: translateX(-50%); \
                    z-index: 3200; width: min(560px, calc(100vw - 48px)); \
                    max-height: min(680px, calc(100vh - 120px)); \
                    display: flex; flex-direction: column; overflow: hidden; \
                    background: #11100e; color: #ece7da; \
                    border: 1px solid rgba(236, 231, 218, 0.22); border-radius: 4px; \
                    box-shadow: 0 24px 60px rgba(0, 0, 0, 0.7), 0 0 0 1px rgba(0, 0, 0, 0.45); \
                    font-family: 'Iowan Old Style', 'Palatino Linotype', 'Book Antiqua', Palatino, Georgia, 'Times New Roman', serif;",

            // ---- Title bar ------------------------------------------------
            header {
                class: "plugin-manager-bar",
                style: "flex: 0 0 auto; display: flex; align-items: center; \
                        justify-content: space-between; gap: 12px; padding: 14px 18px; \
                        background: #0b0a09; border-bottom: 1px solid rgba(236, 231, 218, 0.18);",

                div { class: "plugin-manager-titles", style: "min-width: 0;",
                    p {
                        class: "plugin-manager-eyebrow",
                        style: "margin: 0 0 2px; font-family: ui-monospace, 'SFMono-Regular', Menlo, monospace; \
                                font-size: 0.62rem; letter-spacing: 0.22em; text-transform: uppercase; color: #8c8678;",
                        "Moribund · Module Registry"
                    }
                    h2 {
                        class: "plugin-manager-title",
                        style: "margin: 0; font-size: 1.12rem; font-weight: 600; letter-spacing: 0.01em; color: #f4f1ea;",
                        "Plugin Manager"
                    }
                }

                button {
                    class: "plugin-manager-close",
                    r#type: "button",
                    "aria-label": "Close",
                    style: "flex: 0 0 auto; width: 28px; height: 28px; line-height: 1; cursor: pointer; \
                            background: transparent; color: #b8b2a4; border: 1px solid rgba(236, 231, 218, 0.22); \
                            border-radius: 3px; font-size: 0.85rem;",
                    onclick: move |_| show_panel.set(false),
                    "✕"
                }
            }

            // ---- Restart warning banner -----------------------------------
            if needs_restart() {
                div {
                    class: "plugin-restart-banner",
                    role: "status",
                    style: "flex: 0 0 auto; margin: 12px 18px 0; padding: 10px 14px; \
                            background: #1c1a16; color: #f1ede2; \
                            border: 1px solid rgba(236, 231, 218, 0.28); border-left: 3px solid #ece7da; \
                            border-radius: 3px; font-size: 0.82rem; line-height: 1.5; letter-spacing: 0.01em;",
                    "Changes require an application restart to take effect."
                }
            }

            // ---- Owned-scroll body (the workbench) ------------------------
            div {
                class: "plugin-manager-body",
                style: "flex: 1 1 auto; min-height: 0; overflow-y: auto; padding: 14px 18px 18px;",

                p {
                    class: "plugin-manager-summary",
                    style: "margin: 0 0 12px; font-size: 0.78rem; letter-spacing: 0.02em; color: #948e80;",
                    "{enabled_count} of {total_count} modules enabled."
                }

                div {
                    class: "plugin-roster",
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    if available_plugins.read().is_empty() {
                        p {
                            class: "plugin-empty",
                            style: "margin: 0; padding: 24px 12px; text-align: center; \
                                    font-style: italic; color: #8c8678;",
                            "No modules are registered in this build."
                        }
                    }

                    // Iterate directly over the read signals. Display fields are
                    // interpolated by reference; only the per-row id is cloned,
                    // and only because the event handler must own it.
                    for plugin in available_plugins.read().iter() {
                        label {
                            key: "{plugin.id}",
                            class: "plugin-entry",
                            style: "display: flex; align-items: flex-start; gap: 12px; padding: 12px 14px; \
                                    cursor: pointer; background: #16140f; \
                                    border: 1px solid rgba(236, 231, 218, 0.12); border-radius: 3px;",

                            input {
                                r#type: "checkbox",
                                class: "plugin-entry-toggle",
                                style: "flex: 0 0 auto; margin-top: 2px; width: 15px; height: 15px; \
                                        accent-color: #ece7da; cursor: pointer;",
                                // Reading current_state here keeps the box reactive to state changes.
                                checked: current_state.read().iter().any(|s| s.id == plugin.id && s.enabled),
                                onchange: {
                                    let id = plugin.id.to_string();
                                    move |evt: FormEvent| {
                                        let on = evt.checked();
                                        let id = id.clone();
                                        current_state.with_mut(|states| {
                                            if let Some(existing) = states.iter_mut().find(|s| s.id == id) {
                                                existing.enabled = on;
                                            } else {
                                                states.push(PluginState { id, enabled: on });
                                            }
                                        });
                                    }
                                },
                            }

                            div {
                                class: "plugin-entry-meta",
                                style: "min-width: 0; display: flex; flex-direction: column; gap: 3px;",

                                div {
                                    class: "plugin-entry-headline",
                                    style: "display: flex; align-items: baseline; gap: 8px; flex-wrap: wrap;",
                                    span {
                                        class: "plugin-entry-name",
                                        style: "font-size: 0.92rem; font-weight: 600; color: #f1ede2;",
                                        "{plugin.display_name}"
                                    }
                                    span {
                                        class: "plugin-entry-version",
                                        style: "font-family: ui-monospace, 'SFMono-Regular', Menlo, monospace; \
                                                font-size: 0.68rem; letter-spacing: 0.06em; color: #8c8678;",
                                        "v{plugin.version}"
                                    }
                                }
                                p {
                                    class: "plugin-entry-desc",
                                    style: "margin: 0; font-size: 0.8rem; line-height: 1.5; color: #a8a294;",
                                    "{plugin.description}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}