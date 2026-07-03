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
                    style: "padding: 8px 12px; background: color-mix(in srgb, var(--editor-warning) 12%, transparent); color: var(--editor-warning); border-bottom: 1px solid var(--editor-border-soft); font-size: 0.78rem;",
                    "Restart required to apply changes."
                }
            }

            match &*install_status.read() {
                Some(Ok(msg)) => rsx! {
                    div {
                        style: "padding: 8px 12px; background: color-mix(in srgb, var(--editor-good) 12%, transparent); color: var(--editor-good); border-bottom: 1px solid var(--editor-border-soft); font-size: 0.78rem;",
                        "{msg}"
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        style: "padding: 8px 12px; background: color-mix(in srgb, var(--editor-danger) 12%, transparent); color: var(--editor-danger); border-bottom: 1px solid var(--editor-border-soft); font-size: 0.78rem;",
                        "{err}"
                    }
                },
                None => rsx! {}
            }

            // Tab bar — shared .mor-tab styling (underline active state).
            div { class: "mor-tabs",
                style: "margin: 0; padding: 0 8px; flex-shrink: 0;",
                button {
                    class: if active_tab() == ManagerTab::Installed { "mor-tab active" } else { "mor-tab" },
                    onclick: move |_| active_tab.set(ManagerTab::Installed),
                    "Installed ({installed_plugins.read().len()})"
                }
                button {
                    class: if active_tab() == ManagerTab::Discover { "mor-tab active" } else { "mor-tab" },
                    onclick: move |_| active_tab.set(ManagerTab::Discover),
                    "Discover"
                }
                button {
                    class: if active_tab() == ManagerTab::Updates { "mor-tab active" } else { "mor-tab" },
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
                                style: "display: flex; flex-direction: column; padding: 10px 12px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 6px; gap: 6px;",
                                div { style: "display: flex; align-items: center; gap: 8px;",
                                    span {
                                        style: "font-size: 0.65rem; font-weight: 700; letter-spacing: 0.05em; padding: 2px 6px; border-radius: 4px; background: color-mix(in srgb, var(--editor-good) 15%, transparent); color: var(--editor-good);",
                                        "MCP"
                                    }
                                    span { style: "font-weight: 600; color: var(--fg-base);", "{display_name}" }
                                }
                                div { style: "font-family: monospace; font-size: 0.7rem; color: var(--fg-muted);", "{server_key}" }
                                if !prompt.is_empty() {
                                    div {
                                        title: "{prompt}",
                                        style: "font-size: 0.75rem; color: var(--fg-muted); line-height: 1.4; display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;",
                                        "{prompt}"
                                    }
                                }
                            }
                        }

                        for local_plugin in current_read.iter() {
                            div {
                                key: "{local_plugin.id}",
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 12px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 6px; gap: 8px;",
                                div { style: "display: flex; gap: 8px; align-items: center; min-width: 0;",
                                    input {
                                        r#type: "checkbox",
                                        style: "cursor: pointer; flex-shrink: 0;",
                                        checked: local_plugin.enabled,
                                        onchange: {
                                            let id = local_plugin.id.to_string();
                                            move |evt: FormEvent| {
                                                let on = evt.checked();
                                                current_state.with_mut(|s| { if let Some(p) = s.iter_mut().find(|p| p.id == id) { p.enabled = on; } });
                                            }
                                        }
                                    }
                                    span { style: "font-weight: 600; color: var(--fg-base); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{local_plugin.id}" }
                                    span { style: "font-family: monospace; font-size: 0.7rem; color: var(--fg-muted); flex-shrink: 0;", "v{local_plugin.version}" }
                                }
                                button {
                                    style: "color: var(--editor-danger); background: transparent; border: 1px solid var(--editor-danger); border-radius: 4px; padding: 2px 8px; cursor: pointer; font-size: 0.75rem; flex-shrink: 0;",
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
                        if !compendium_read.iter().any(|r| !current_read.iter().any(|l| l.id == r.id)) {
                            p { style: "margin: 8px 0; text-align: center; font-size: 0.8rem; color: var(--fg-muted);",
                                "Nothing new to discover."
                            }
                        }
                        for remote in compendium_read.iter().filter(|r| !current_read.iter().any(|l| l.id == r.id)) {
                            div {
                                key: "discover-{remote.id}",
                                style: "display: flex; flex-direction: column; padding: 10px 12px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 6px; gap: 6px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center; gap: 8px;",
                                    div { style: "display: flex; align-items: center; gap: 8px; min-width: 0;",
                                        span { style: "font-weight: 600; color: var(--fg-base); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{remote.display_name}" }
                                        span { style: "font-family: monospace; font-size: 0.7rem; color: var(--fg-muted); flex-shrink: 0;", "v{remote.version}" }
                                    }
                                    button {
                                        class: "mor-btn-primary",
                                        style: "padding: 3px 10px; font-size: 0.75rem; flex-shrink: 0;",
                                        onclick: {
                                            let new_plugin = PluginState { id: remote.id.clone(), enabled: true, version: remote.version.clone() };
                                            move |_| { current_state.with_mut(|s| s.push(new_plugin.clone())); }
                                        },
                                        "Install"
                                    }
                                }
                                p { style: "margin: 0; font-size: 0.78rem; line-height: 1.4; color: var(--fg-muted);", "{remote.description}" }
                            }
                        }
                    },
                    ManagerTab::Updates => rsx! {
                        if updates_count == 0 {
                            p { style: "margin: 8px 0; text-align: center; font-size: 0.8rem; color: var(--fg-muted);",
                                "Everything is up to date."
                            }
                        }
                        for (local, remote) in updates_available.into_iter() {
                            div {
                                key: "update-{local.id}",
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 12px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 6px; gap: 8px;",
                                div { style: "display: flex; align-items: center; gap: 8px; min-width: 0;",
                                    span { style: "font-weight: 600; color: var(--fg-base); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{remote.display_name}" }
                                    span { style: "font-family: monospace; font-size: 0.75rem; color: var(--fg-muted); flex-shrink: 0;",
                                        "v{local.version} → v{remote.version}"
                                    }
                                }
                                button {
                                    class: "mor-btn-primary",
                                    style: "padding: 3px 10px; font-size: 0.75rem; flex-shrink: 0;",
                                    onclick: {
                                        let id = local.id.clone();
                                        let target_version = remote.version.clone();
                                        move |_| { current_state.with_mut(|s| { if let Some(p) = s.iter_mut().find(|p| p.id == id) { p.version = target_version.clone(); } }); }
                                    },
                                    "Update"
                                }
                            }
                        }
                    }
                }

                // Actions section
                div {
                    style: "margin-top: 10px; border-top: 1px solid var(--border); padding-top: 12px; display: flex; flex-direction: column; gap: 8px;",
                    button {
                        class: "mor-btn",
                        style: "width: 100%; border-style: dashed; font-size: 0.8rem;",
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
                            class: "mor-input",
                            style: "width: 100%; font-size: 0.8rem; box-sizing: border-box;",
                            placeholder: "Author/Repo (e.g. MoribundInstitute/mcp)",
                            value: "{repo_input}",
                            oninput: move |evt| repo_input.set(evt.value())
                        }
                        button {
                            class: "mor-btn-primary",
                            style: "width: 100%; font-size: 0.8rem;",
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
            floating_class: "floating-landscape",
            // Self-sufficient when floating: don't depend on another dock
            // having mounted the shared pane CSS/drag scripts (install-once guards).
            style { {crate::ui::layout::docks::shared::PANE_CSS} }
            script { dangerous_inner_html: crate::ui::layout::docks::shared::PANE_DRAG_JS }
            script { dangerous_inner_html: crate::ui::layout::docks::shared::PANE_RESIZE_JS }
            {inner_content}
        }
    }
}
