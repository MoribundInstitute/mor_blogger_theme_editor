use crate::app::config_bridge::PluginState;
use crate::app::state::{DockPosition, LayoutState, PluginManagerContext};
use crate::ui::components::dock_chrome::DockChrome;
use dioxus::prelude::*;
use rfd::FileDialog;

#[derive(Clone, Copy, PartialEq)]
enum ManagerTab {
    Installed,
    Discover,
    Updates,
}

#[component]
pub fn PluginManagerDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let plugin_ctx = use_context::<PluginManagerContext>();
    let pos = (layout.plugin_manager_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let launch_state = plugin_ctx.launch_plugins;
    let mut current_state = plugin_ctx.current_plugins;
    let compendium_registry = plugin_ctx.compendium_registry;

    let mut active_tab = use_signal(|| ManagerTab::Installed);
    let needs_restart = use_memo(move || *launch_state.read() != *current_state.read());
    let mut install_status = use_signal(|| Option::<Result<String, String>>::None);
    let mut repo_input = use_signal(|| String::new());
    let mut installed_plugins = use_signal(|| Vec::<String>::new());

    use_effect(move || {
        installed_plugins.set(crate::utils::mcp_installer::list_installed_mcp_binaries());
    });

    let current_read = current_state.read();
    let compendium_read = compendium_registry.read();

    let mcp_daemon_cards = use_memo(move || {
        let Ok(registry) = crate::utils::mcp_installer::read_daemon_registry() else {
            return Vec::new();
        };
        let Some(servers) = registry.get("servers").and_then(|v| v.as_object()) else {
            return Vec::new();
        };

        servers
            .iter()
            .map(|(key, entry)| {
                let display_name = entry
                    .get("display_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(key)
                    .to_string();
                let prompt = entry
                    .get("system_prompt")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (key.clone(), display_name, prompt)
            })
            .collect::<Vec<_>>()
    });

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
    let inner_content = rsx! {
        DockChrome {
            title: "Plugin Manager".to_string(),
            dock_id: "plugin_manager".to_string(),
            position: pos,
            on_close: move |_| {
                layout.plugin_manager_pos.set(DockPosition::Hidden);
            },
            div {
                style: "display: flex; flex-direction: column; height: calc(100% - 45px); overflow: hidden; background: var(--bg-panel); color: var(--fg-base); font-size: 0.85rem;",
            
            if needs_restart() {
                div {
                    style: "padding: 8px 12px; background: #1c1a16; color: #f1ede2; border-bottom: 1px solid rgba(236, 231, 218, 0.2); font-size: 0.78rem;",
                    "Restart required to apply changes."
                }
            }

            match &*install_status.read() {
                Some(Ok(msg)) => rsx! {
                    div {
                        style: "padding: 8px 12px; background: #132317; color: #73c991; border-bottom: 1px solid rgba(115, 201, 145, 0.2); font-size: 0.78rem;",
                        "{msg}"
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        style: "padding: 8px 12px; background: #2a1415; color: #ea8285; border-bottom: 1px solid rgba(234, 130, 133, 0.2); font-size: 0.78rem;",
                        "{err}"
                    }
                },
                None => rsx! {}
            }

            // Tab bar
            div {
                style: "display: flex; border-bottom: 1px solid var(--border); background: var(--bg-elevated); padding: 4px 8px; gap: 4px;",
                button {
                    style: format!("padding: 4px 8px; border: none; background: {}; color: {}; font-size: 0.8rem; cursor: pointer; border-radius: 2px;", if active_tab() == ManagerTab::Installed { "var(--accent)" } else { "transparent" }, if active_tab() == ManagerTab::Installed { "#111" } else { "inherit" }),
                    onclick: move |_| active_tab.set(ManagerTab::Installed),
                    "Installed ({installed_plugins.read().len()})"
                }
                button {
                    style: format!("padding: 4px 8px; border: none; background: {}; color: {}; font-size: 0.8rem; cursor: pointer; border-radius: 2px;", if active_tab() == ManagerTab::Discover { "var(--accent)" } else { "transparent" }, if active_tab() == ManagerTab::Discover { "#111" } else { "inherit" }),
                    onclick: move |_| active_tab.set(ManagerTab::Discover),
                    "Discover"
                }
                button {
                    style: format!("padding: 4px 8px; border: none; background: {}; color: {}; font-size: 0.8rem; cursor: pointer; border-radius: 2px;", if active_tab() == ManagerTab::Updates { "var(--accent)" } else { "transparent" }, if active_tab() == ManagerTab::Updates { "#111" } else { "inherit" }),
                    onclick: move |_| active_tab.set(ManagerTab::Updates),
                    "Updates ({updates_count})"
                }
            }

            // Tab contents
            div {
                style: "flex: 1; overflow-y: auto; padding: 12px; display: flex; flex-direction: column; gap: 10px;",
                match active_tab() {
                    ManagerTab::Installed => rsx! {
                        for (server_key, display_name, prompt) in mcp_daemon_cards().into_iter() {
                            div {
                                key: "mcp-{server_key}",
                                style: "display: flex; flex-direction: column; padding: 8px 10px; background: rgba(5, 165, 129, 0.08); border: 1px solid rgba(5, 165, 129, 0.25); border-radius: 3px; gap: 4px;",
                                div { style: "font-weight: bold; color: var(--accent);", "MCP: {display_name}" }
                                div { style: "font-family: monospace; font-size: 0.7rem; color: var(--fg-muted);", "daemon key: {server_key}" }
                                if !prompt.is_empty() {
                                    div { style: "font-size: 0.75rem; color: var(--fg-muted); line-height: 1.35;", "prompt: {prompt}" }
                                }
                            }
                        }

                        for local_plugin in current_read.iter() {
                            div {
                                key: "{local_plugin.id}",
                                style: "display: flex; flex-direction: column; padding: 8px 10px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 3px; gap: 6px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    div { style: "display: flex; gap: 8px; align-items: center;",
                                        input {
                                            r#type: "checkbox",
                                            style: "cursor: pointer;",
                                            checked: local_plugin.enabled,
                                            onchange: {
                                                let id = local_plugin.id.to_string();
                                                move |evt: FormEvent| {
                                                    let on = evt.checked();
                                                    current_state.with_mut(|s| { if let Some(p) = s.iter_mut().find(|p| p.id == id) { p.enabled = on; } });
                                                }
                                            }
                                        }
                                        span { style: "font-weight: bold; color: var(--fg-base);", "{local_plugin.id}" }
                                    }
                                    button {
                                        style: "color: #ea8285; background: transparent; border: 1px solid #ea8285; border-radius: 3px; padding: 2px 6px; cursor: pointer; font-size: 0.75rem;",
                                        onclick: {
                                            let id = local_plugin.id.to_string();
                                            move |_| { current_state.with_mut(|s| s.retain(|p| p.id != id)); }
                                        },
                                        "Remove"
                                    }
                                }
                                div { style: "font-family: monospace; font-size: 0.7rem; color: var(--fg-muted);", "version: v{local_plugin.version}" }
                            }
                        }
                    },
                    ManagerTab::Discover => rsx! {
                        for remote in compendium_read.iter().filter(|r| !current_read.iter().any(|l| l.id == r.id)) {
                            div {
                                key: "discover-{remote.id}",
                                style: "display: flex; flex-direction: column; padding: 8px 10px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 3px; gap: 6px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    span { style: "font-weight: bold; color: var(--fg-base);", "{remote.display_name}" }
                                    button {
                                        style: "color: #111; background: var(--accent); border: none; border-radius: 3px; padding: 3px 8px; font-weight: bold; cursor: pointer; font-size: 0.75rem;",
                                        onclick: {
                                            let new_plugin = PluginState { id: remote.id.clone(), enabled: true, version: remote.version.clone() };
                                            move |_| { current_state.with_mut(|s| s.push(new_plugin.clone())); }
                                        },
                                        "Install"
                                    }
                                }
                                div { style: "font-family: monospace; font-size: 0.7rem; color: var(--fg-muted);", "version: v{remote.version}" }
                                p { style: "margin: 0; font-size: 0.78rem; color: var(--fg-muted);", "{remote.description}" }
                            }
                        }
                    },
                    ManagerTab::Updates => rsx! {
                        for (local, remote) in updates_available.into_iter() {
                            div {
                                key: "update-{local.id}",
                                style: "display: flex; flex-direction: column; padding: 8px 10px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 3px; gap: 6px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    span { style: "font-weight: bold; color: var(--fg-base);", "{remote.display_name}" }
                                    button {
                                        style: "color: #111; background: #73c991; border: none; border-radius: 3px; padding: 3px 8px; font-weight: bold; cursor: pointer; font-size: 0.75rem;",
                                        onclick: {
                                            let id = local.id.clone();
                                            let target_version = remote.version.clone();
                                            move |_| { current_state.with_mut(|s| { if let Some(p) = s.iter_mut().find(|p| p.id == id) { p.version = target_version.clone(); } }); }
                                        },
                                        "Update"
                                    }
                                }
                                div { style: "font-family: monospace; font-size: 0.75rem; color: var(--fg-muted);",
                                    "v{local.version} → v{remote.version}"
                                }
                            }
                        }
                    }
                }

                // Actions section
                div {
                    style: "margin-top: 10px; border-top: 1px solid var(--border); padding-top: 12px; display: flex; flex-direction: column; gap: 8px;",
                    button {
                        style: "padding: 6px; border: 1px dashed var(--border); background: transparent; color: var(--fg-base); cursor: pointer; border-radius: 3px; font-size: 0.8rem;",
                        onclick: move |_| {
                            if let Some(file_path) = FileDialog::new()
                                .set_title("Select MorBlogger MCP Binary")
                                .pick_file()
                            {
                                let mut final_path = file_path.clone();
                                let mut copy_success = false;
                                // Reuse core's canonical MCP location (parent of the daemon registry)
                                // instead of re-deriving the OS config dir.
                                if let Some(mcp_dir) = crate::utils::mcp_installer::mcp_daemon_registry_path()
                                    .parent()
                                    .map(std::path::Path::to_path_buf)
                                {
                                    if std::fs::create_dir_all(&mcp_dir).is_ok() {
                                        if let Some(file_name) = file_path.file_name() {
                                            let dest_path = mcp_dir.join(file_name);
                                            if std::fs::copy(&file_path, &dest_path).is_ok() {
                                                final_path = dest_path;
                                                copy_success = true;
                                            }
                                        }
                                    }
                                }

                                match crate::utils::mcp_installer::install_mcp_to_claude(&final_path.clone()) {
                                    Ok(_) => {
                                        let msg = if copy_success {
                                            "Successfully installed MCP plugin (copied to internal config dir)!"
                                        } else {
                                            "Successfully installed MCP plugin directly!"
                                        };
                                        install_status.set(Some(Ok(msg.to_string())));
                                        if let Some(file_name) = final_path.file_name().and_then(|n| n.to_str()) {
                                            installed_plugins.write().push(file_name.to_string());
                                        }
                                    },
                                    Err(e) => {
                                        install_status.set(Some(Err(format!("Failed to install MCP: {}", e))));
                                    }
                                }
                            }
                        },
                        "+ Install from Disk"
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        input {
                            style: "width: 100%; border: 1px solid var(--border); border-radius: 3px; background: var(--bg-elevated); color: var(--fg-base); padding: 6px 10px; font-size: 0.8rem; box-sizing: border-box;",
                            placeholder: "Author/Repo (e.g. MoribundInstitute/mcp)",
                            value: "{repo_input}",
                            oninput: move |evt| repo_input.set(evt.value())
                        }
                        button {
                            style: "padding: 6px; border: none; background: var(--accent); color: #111; font-weight: bold; cursor: pointer; border-radius: 3px; font-size: 0.8rem;",
                            onclick: move |_| {
                                let repo = repo_input.read().clone();
                                if !repo.is_empty() {
                                    spawn(async move {
                                        match crate::utils::mcp_installer::install_plugin_from_github(&repo).await {
                                            Ok(file) => {
                                                install_status.set(Some(Ok(format!("Successfully installed plugin: {}", file))));
                                                installed_plugins.write().push(file);
                                            }
                                            Err(e) => {
                                                install_status.set(Some(Err(format!("GitHub Install Failed: {}", e))));
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
