//! Floating Plugin Manager window for the Moribund Theme Architect.
use crate::app::config_bridge::{CompendiumManifest, PluginState};
use crate::ui::components::modal::Modal;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum ManagerTab {
    Installed,
    Discover,
    Updates,
}

#[component]
pub fn PluginManagerDialog(
    mut show_panel: Signal<bool>,
    launch_state: ReadSignal<Vec<PluginState>>,
    mut current_state: Signal<Vec<PluginState>>,
    compendium_registry: ReadSignal<Vec<CompendiumManifest>>,
) -> Element {
    let mut active_tab = use_signal(|| ManagerTab::Installed);
    let needs_restart = use_memo(move || *launch_state.read() != *current_state.read());

    if !show_panel() {
        return rsx! {};
    }

    let current_read = current_state.read();
    let compendium_read = compendium_registry.read();

    let installed_count = current_read.len();

    let updates_available: Vec<_> = current_read
        .iter()
        .filter_map(|local| {
            compendium_read
                .iter()
                .find(|remote| remote.id == local.id)
                .and_then(|remote| {
                    if remote.version != local.version {
                        Some((local.clone(), remote.clone()))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let updates_count = updates_available.len();

    rsx! {
        Modal {
            open: show_panel,
            title: "Plugin Manager",
            style: "width: 720px; height: 600px;", // Pass dimensions to the modal
            on_close: move |_| show_panel.set(false),

            section {
                class: "plugin-manager-modal floating-window",
                role: "dialog",

                if needs_restart() {
                    div {
                        style: "margin: 12px 18px 0; padding: 10px 14px; background: #1c1a16; color: #f1ede2; \
                                border: 1px solid rgba(236, 231, 218, 0.28); border-left: 3px solid #ece7da; font-size: 0.82rem;",
                        "Changes require an application restart to take effect."
                    }
                }

                div {
                    style: "flex: 1 1 auto; display: flex; min-height: 0; margin-top: 12px;",
                    nav {
                        style: "flex: 0 0 160px; display: flex; flex-direction: column; gap: 4px; border-right: 1px solid rgba(236, 231, 218, 0.18); padding: 0 12px 18px 18px;",
                        button {
                            style: format!("text-align: left; padding: 8px 12px; border: none; cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.95rem; background: {}; color: {};", if active_tab() == ManagerTab::Installed { "rgba(236, 231, 218, 0.08)" } else { "transparent" }, if active_tab() == ManagerTab::Installed { "#f4f1ea" } else { "#a8a294" }),
                            onclick: move |_| active_tab.set(ManagerTab::Installed),
                            "Installed ({installed_count})"
                        }
                        button {
                            style: format!("text-align: left; padding: 8px 12px; border: none; cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.95rem; background: {}; color: {};", if active_tab() == ManagerTab::Discover { "rgba(236, 231, 218, 0.08)" } else { "transparent" }, if active_tab() == ManagerTab::Discover { "#f4f1ea" } else { "#a8a294" }),
                            onclick: move |_| active_tab.set(ManagerTab::Discover),
                            "Discover"
                        }
                        button {
                            style: format!("text-align: left; padding: 8px 12px; border: none; cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.95rem; background: {}; color: {};", if active_tab() == ManagerTab::Updates { "rgba(236, 231, 218, 0.08)" } else { "transparent" }, if active_tab() == ManagerTab::Updates { "#f4f1ea" } else { "#a8a294" }),
                            onclick: move |_| active_tab.set(ManagerTab::Updates),
                            "Updates ({updates_count})"
                        }
                        div { style: "flex-grow: 1;" }
                        button {
                            style: "text-align: left; padding: 8px 12px; border: 1px dashed rgba(236, 231, 218, 0.3); cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.85rem; background: transparent; color: #a8a294; margin-top: auto;",
                            onclick: move |_| {
                                // In a real app, this would use the `rfd` crate to open a file picker.
                                // For now, we log the intent.
                                log::info!("Action triggered: Open OS File Picker to sideload plugin");
                            },
                            "+ Install from Disk"
                        }
                    }

                    div {
                        style: "flex: 1 1 auto; overflow-y: auto; padding: 0 18px 18px 18px; display: flex; flex-direction: column; gap: 8px;",
                        match active_tab() {
                            ManagerTab::Installed => rsx! {
                                for local_plugin in current_read.iter() {
                                    div {
                                        key: "{local_plugin.id}",
                                        style: "display: flex; justify-content: space-between; padding: 12px 14px; background: #16140f; border: 1px solid rgba(236, 231, 218, 0.12); border-radius: 3px;",
                                        div { style: "display: flex; gap: 12px;",
                                            input {
                                                r#type: "checkbox",
                                                style: "margin-top: 4px; accent-color: #ece7da; cursor: pointer;",
                                                checked: local_plugin.enabled,
                                                onchange: {
                                                    let id = local_plugin.id.to_string();
                                                    move |evt: FormEvent| {
                                                        let on = evt.checked();
                                                        current_state.with_mut(|s| { if let Some(p) = s.iter_mut().find(|p| p.id == id) { p.enabled = on; } });
                                                    }
                                                }
                                            }
                                            div {
                                                span { style: "font-size: 0.95rem; font-weight: 600; color: #f1ede2;", "{local_plugin.id} " }
                                                span { style: "font-family: monospace; font-size: 0.7rem; color: #8c8678;", "v{local_plugin.version}" }
                                                p { style: "margin: 4px 0 0; font-size: 0.8rem; color: #a8a294;", "Stored locally in configuration." }
                                            }
                                        }
                                        button {
                                            style: "color: #d29922; background: transparent; border: 1px solid #d29922; border-radius: 3px; padding: 4px 10px; cursor: pointer;",
                                            onclick: {
                                                let id = local_plugin.id.to_string();
                                                move |_| { current_state.with_mut(|s| s.retain(|p| p.id != id)); }
                                            },
                                            "Remove"
                                        }
                                    }
                                }
                            },
                            ManagerTab::Discover => rsx! {
                                for remote in compendium_read.iter().filter(|r| !current_read.iter().any(|l| l.id == r.id)) {
                                    div {
                                        key: "discover-{remote.id}",
                                        style: "display: flex; justify-content: space-between; padding: 12px 14px; background: #16140f; border: 1px solid rgba(236, 231, 218, 0.12); border-radius: 3px;",
                                        div {
                                            span { style: "font-size: 0.95rem; font-weight: 600; color: #f1ede2;", "{remote.display_name} " }
                                            span { style: "font-family: monospace; font-size: 0.7rem; color: #8c8678;", "v{remote.version}" }
                                            p { style: "margin: 4px 0 0; font-size: 0.82rem; color: #a8a294;", "{remote.description}" }
                                        }
                                        button {
                                            style: "color: #11100e; background: #ece7da; border: none; border-radius: 3px; padding: 6px 14px; font-weight: 600; cursor: pointer;",
                                            onclick: {
                                                let new_plugin = PluginState { id: remote.id.clone(), enabled: true, version: remote.version.clone() };
                                                move |_| { current_state.with_mut(|s| s.push(new_plugin.clone())); }
                                            },
                                            "Fetch"
                                        }
                                    }
                                }
                            },
                            ManagerTab::Updates => rsx! {
                                for (local, remote) in updates_available.into_iter() {
                                    div {
                                        key: "update-{local.id}",
                                        style: "display: flex; justify-content: space-between; padding: 12px 14px; background: #16140f; border: 1px solid rgba(236, 231, 218, 0.12); border-radius: 3px;",
                                        div {
                                            span { style: "font-size: 0.95rem; font-weight: 600; color: #f1ede2; display: block;", "{remote.display_name}" }
                                            span { style: "font-family: monospace; font-size: 0.8rem; color: #a8a294;", "v{local.version} → " }
                                            span { style: "font-family: monospace; font-size: 0.8rem; color: #73c991;", "v{remote.version}" }
                                        }
                                        button {
                                            style: "color: #11100e; background: #73c991; border: none; border-radius: 3px; padding: 6px 14px; font-weight: 600; cursor: pointer;",
                                            onclick: {
                                                let id = local.id.clone();
                                                let target_version = remote.version.clone();
                                                move |_| { current_state.with_mut(|s| { if let Some(p) = s.iter_mut().find(|p| p.id == id) { p.version = target_version.clone(); } }); }
                                            },
                                            "Sync Update"
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
}



