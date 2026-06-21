//! Floating Plugin Manager window for the Moribund Theme Architect.
use crate::app::config_bridge::{CompendiumManifest, PluginState};
use crate::ui::components::modal::Modal;
use dioxus::prelude::*;
use rfd::FileDialog;

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
    let mut install_status = use_signal(|| Option::<Result<String, String>>::None);
    let mut repo_input = use_signal(|| String::new());
    let mut installed_plugins = use_signal(|| Vec::<String>::new());

    use_effect(move || {
        use std::fs;
        let mut loaded = Vec::new();
        if let Some(data_dir) = dirs::data_local_dir() {
            let plugin_dir = data_dir.join("morblogger/plugins");
            if let Ok(entries) = fs::read_dir(plugin_dir) {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        loaded.push(name);
                    }
                }
            }
        }
        installed_plugins.set(loaded);
    });

    if !show_panel() {
        return rsx! {};
    }

    let current_read = current_state.read();
    let compendium_read = compendium_registry.read();

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

                match &*install_status.read() {
                    Some(Ok(msg)) => rsx! {
                        div {
                            style: "margin: 12px 18px 0; padding: 10px 14px; background: #132317; color: #73c991; \
                                    border: 1px solid rgba(115, 201, 145, 0.28); border-left: 3px solid #73c991; font-size: 0.82rem;",
                            "{msg}"
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div {
                            style: "margin: 12px 18px 0; padding: 10px 14px; background: #2a1415; color: #ea8285; \
                                    border: 1px solid rgba(234, 130, 133, 0.28); border-left: 3px solid #ea8285; font-size: 0.82rem;",
                            "{err}"
                        }
                    },
                    None => rsx! {}
                }

                div {
                    style: "flex: 1 1 auto; display: flex; min-height: 0; margin-top: 12px;",
                    nav {
                        style: "flex: 0 0 160px; display: flex; flex-direction: column; gap: 4px; border-right: 1px solid rgba(236, 231, 218, 0.18); padding: 0 12px 18px 18px;",
                        button {
                            style: format!("text-align: left; padding: 8px 12px; border: none; cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.95rem; background: {}; color: {};", if active_tab() == ManagerTab::Installed { "rgba(236, 231, 218, 0.08)" } else { "transparent" }, if active_tab() == ManagerTab::Installed { "#f4f1ea" } else { "#a8a294" }),
                            onclick: move |_| active_tab.set(ManagerTab::Installed),
                            "Installed ({installed_plugins.read().len()})"
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
                        div {
                            class: "sidebar-actions",
                            style: "display: flex; flex-direction: column; gap: 8px;",
                            
                            // The standard, offline Disk Installer
                            button {
                                class: "mor-btn-secondary",
                                style: "text-align: left; padding: 8px 12px; border: 1px dashed rgba(236, 231, 218, 0.3); cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.85rem; background: transparent; color: #a8a294; width: 100%;",
                                onclick: move |_| {
                                    if let Some(file_path) = FileDialog::new()
                                        .set_title("Select MorBlogger MCP Binary")
                                        .pick_file() 
                                    {
                                        // 1. Copy the binary to ~/.config/mor_blogger/mcp_servers/ so it isn't accidentally deleted from the user's Downloads folder.
                                        let mut final_path = file_path.clone();
                                        let mut copy_success = false;
                                        if let Some(config_dir) = dirs::config_dir() {
                                            let mcp_dir = config_dir.join("mor_blogger/mcp_servers");
                                            if let Ok(_) = std::fs::create_dir_all(&mcp_dir) {
                                                if let Some(file_name) = file_path.file_name() {
                                                    let dest_path = mcp_dir.join(file_name);
                                                    if let Ok(_) = std::fs::copy(&file_path, &dest_path) {
                                                        final_path = dest_path;
                                                        copy_success = true;
                                                    }
                                                }
                                            }
                                        }

                                        // 2. Run the auto-installer
                                        match crate::utils::mcp_installer::install_mcp_to_claude(&final_path) {
                                            Ok(_) => {
                                                let msg = if copy_success {
                                                    "Successfully installed MCP plugin to AI clients (copied to internal directory)!"
                                                } else {
                                                    "Successfully installed MCP plugin to AI clients directly!"
                                                };
                                                install_status.set(Some(Ok(msg.to_string())));
                                                log::info!("Successfully installed MCP plugin to AI clients!");
                                                if let Some(file_name) = final_path.file_name().and_then(|n| n.to_str()) {
                                                    installed_plugins.write().push(file_name.to_string());
                                                }
                                            },
                                            Err(e) => {
                                                install_status.set(Some(Err(format!("Failed to install MCP: {}", e))));
                                                log::error!("Failed to install MCP: {}", e);
                                            }
                                        }
                                    }
                                },
                                "+ Install from Disk"
                            }

                            // The new Agnostic GitHub Fetcher
                            div {
                                style: "margin-top: 16px; border-top: 1px solid rgba(236, 231, 218, 0.18); padding-top: 16px; display: flex; flex-direction: column; gap: 8px;",
                                input {
                                    class: "editor-field",
                                    style: "width: 100%; border: 1px solid rgba(236, 231, 218, 0.18); border-radius: 3px; font-family: inherit; font-size: 0.85rem; background: #16140f; color: #f1ede2; padding: 8px 12px; box-sizing: border-box;",
                                    placeholder: "Author/Repo (e.g., MoribundInstitute/mcp)",
                                    value: "{repo_input}",
                                    oninput: move |evt| repo_input.set(evt.value())
                                }
                                button {
                                    class: "mor-btn-primary",
                                    style: "text-align: center; padding: 8px 12px; cursor: pointer; border-radius: 3px; font-family: inherit; font-size: 0.85rem; width: 100%;",
                                    onclick: move |_| {
                                        let repo = repo_input.read().clone();
                                        if !repo.is_empty() {
                                            spawn(async move {
                                                match crate::utils::mcp_installer::install_plugin_from_github(&repo).await {
                                                    Ok(file) => {
                                                        let msg = format!("Successfully installed plugin: {}", file);
                                                        install_status.set(Some(Ok(msg.clone())));
                                                        println!("{}", msg);
                                                        installed_plugins.write().push(file);
                                                    }
                                                    Err(e) => {
                                                        let msg = format!("GitHub Install Failed: {}", e);
                                                        install_status.set(Some(Err(msg.clone())));
                                                        eprintln!("{}", msg);
                                                    }
                                                }
                                            });
                                        }
                                    },
                                    "Fetch from GitHub"
                                }
                            }
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

                                div { class: "installed-list", style: "margin-top: 16px;",
                                    h4 { "Local Plugins" }
                                    for plugin in installed_plugins.read().iter() {
                                        div { 
                                            class: "plugin-card", 
                                            style: "padding: 8px; border: 1px solid var(--border-color); margin-bottom: 8px; border-radius: 4px;",
                                            "{plugin}" 
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
