use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalMcpManifest {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub system_prompt: String,
    pub entrypoint: String,
    #[serde(default = "default_mcp_server_key")]
    pub mcp_server_key: String,
}

fn default_mcp_server_key() -> String {
    "mor_blogger_engine".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginPrefEntry {
    pub id: String,
    pub enabled: bool,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpInstallReport {
    pub plugin_id: String,
    pub binary_path: PathBuf,
    pub plugin_dir: PathBuf,
    pub daemon_registry: PathBuf,
    pub editor_prefs: PathBuf,
    pub claude_config: Option<PathBuf>,
}

pub fn mcp_plugins_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("morblogger/plugins")
}

pub fn mcp_daemon_registry_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("mor_blogger/mcp_servers/registry.json")
}

pub fn load_local_mcp_manifest(plugin_dir: &Path) -> Result<LocalMcpManifest, String> {
    let manifest_path = plugin_dir.join("manifest.toml");
    if !manifest_path.exists() {
        return Err(format!(
            "Missing manifest.toml in plugin directory '{}'",
            plugin_dir.display()
        ));
    }

    let raw = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read {}: {e}", manifest_path.display()))?;
    toml::from_str(&raw).map_err(|e| format!("Invalid manifest.toml: {e}"))
}

pub fn install_local_mcp_plugin(plugin_dir: &Path) -> Result<McpInstallReport, String> {
    let manifest = load_local_mcp_manifest(plugin_dir)?;
    let entry_src = plugin_dir.join(&manifest.entrypoint);
    if !entry_src.exists() {
        return Err(format!(
            "Entrypoint '{}' not found in '{}'",
            manifest.entrypoint,
            plugin_dir.display()
        ));
    }

    let plugins_dir = mcp_plugins_dir();
    fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;

    let file_name = entry_src
        .file_name()
        .ok_or_else(|| "Plugin entrypoint has no file name".to_string())?;
    let installed_binary = plugins_dir.join(file_name);
    fs::copy(&entry_src, &installed_binary).map_err(|e| {
        format!(
            "Failed to copy '{}' -> '{}': {e}",
            entry_src.display(),
            installed_binary.display()
        )
    })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&installed_binary)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&installed_binary, perms).map_err(|e| e.to_string())?;
    }

    register_plugin_in_editor_prefs(&manifest.id, &manifest.version)?;
    register_mcp_daemon_entry(&manifest, &installed_binary)?;
    let claude_config = install_mcp_to_claude(&installed_binary, &manifest.mcp_server_key).ok();

    Ok(McpInstallReport {
        plugin_id: manifest.id.clone(),
        binary_path: installed_binary,
        plugin_dir: plugins_dir,
        daemon_registry: mcp_daemon_registry_path(),
        editor_prefs: crate::config::prefs::editor_prefs_path(),
        claude_config,
    })
}

pub fn register_plugin_in_editor_prefs(plugin_id: &str, version: &str) -> Result<(), String> {
    let prefs_path = crate::config::prefs::editor_prefs_path();
    if let Some(parent) = prefs_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut doc = if prefs_path.exists() {
        let raw = fs::read_to_string(&prefs_path).unwrap_or_default();
        raw.parse::<toml_edit::DocumentMut>()
            .map_err(|e| format!("Failed to parse editor_prefs.toml: {e}"))?
    } else {
        toml_edit::DocumentMut::new()
    };

    let plugins_key = "plugins";
    if doc.get(plugins_key).is_none() || !doc[plugins_key].is_array_of_tables() {
        doc[plugins_key] = toml_edit::Item::ArrayOfTables(toml_edit::ArrayOfTables::new());
    }

    let plugins = doc[plugins_key]
        .as_array_of_tables_mut()
        .expect("plugins key was just normalized to array-of-tables");

    let already_registered = plugins.iter().any(|table| {
        table
            .get("id")
            .and_then(|item| item.as_str())
            .is_some_and(|id| id == plugin_id)
    });

    if !already_registered {
        let mut entry = toml_edit::Table::new();
        entry["id"] = toml_edit::value(plugin_id);
        entry["enabled"] = toml_edit::value(true);
        entry["version"] = toml_edit::value(version);
        plugins.push(entry);
    } else {
        for table in plugins.iter_mut() {
            if table
                .get("id")
                .and_then(|item| item.as_str())
                .is_some_and(|id| id == plugin_id)
            {
                table["enabled"] = toml_edit::value(true);
                table["version"] = toml_edit::value(version);
            }
        }
    }

    fs::write(&prefs_path, doc.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn register_mcp_daemon_entry(
    manifest: &LocalMcpManifest,
    binary_path: &Path,
) -> Result<(), String> {
    let registry_path = mcp_daemon_registry_path();
    if let Some(parent) = registry_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut registry: Value = if registry_path.exists() {
        let raw = fs::read_to_string(&registry_path).unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or_else(|_| json!({ "servers": {} }))
    } else {
        json!({ "servers": {} })
    };

    if !registry.is_object() {
        registry = json!({ "servers": {} });
    }
    if registry.get("servers").is_none() || !registry["servers"].is_object() {
        registry["servers"] = json!({});
    }

    if let Some(servers) = registry.get_mut("servers").and_then(|s| s.as_object_mut()) {
        servers.insert(
            manifest.mcp_server_key.clone(),
            json!({
                "id": manifest.id,
                "display_name": manifest.display_name,
                "version": manifest.version,
                "description": manifest.description,
                "system_prompt": manifest.system_prompt,
                "command": binary_path.to_string_lossy(),
                "args": [],
                "enabled": true
            }),
        );
    }

    let pretty = serde_json::to_string_pretty(&registry).map_err(|e| e.to_string())?;
    fs::write(&registry_path, pretty).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn install_mcp_to_claude(binary_path: &Path, server_key: &str) -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir().ok_or("Could not find OS config directory")?;
    let claude_config_path = config_dir.join("Claude/claude_desktop_config.json");
    install_mcp_to_path(&claude_config_path, binary_path, server_key)?;
    Ok(claude_config_path)
}

pub fn install_mcp_to_path(
    claude_config_path: &Path,
    binary_path: &Path,
    server_key: &str,
) -> Result<(), String> {
    let mut config: Value = if claude_config_path.exists() {
        let data = fs::read_to_string(claude_config_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or(json!({ "mcpServers": {} }))
    } else {
        if let Some(parent) = claude_config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        json!({ "mcpServers": {} })
    };

    if !config.is_object() {
        config = json!({ "mcpServers": {} });
    } else if config.get("mcpServers").is_none() || !config["mcpServers"].is_object() {
        if let Some(obj) = config.as_object_mut() {
            obj.insert("mcpServers".to_string(), json!({}));
        }
    }

    if let Some(servers) = config.get_mut("mcpServers").and_then(|s| s.as_object_mut()) {
        servers.insert(
            server_key.to_string(),
            json!({
                "command": binary_path.to_string_lossy(),
                "args": []
            }),
        );
    }

    let pretty_json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(claude_config_path, pretty_json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_installed_mcp_binaries() -> Vec<String> {
    let plugin_dir = mcp_plugins_dir();
    let Ok(entries) = fs::read_dir(plugin_dir) else {
        return Vec::new();
    };

    entries
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}

pub fn list_registered_plugin_ids() -> Result<Vec<String>, String> {
    let prefs_path = crate::config::prefs::editor_prefs_path();
    if !prefs_path.exists() {
        return Ok(Vec::new());
    }

    let raw = fs::read_to_string(&prefs_path).map_err(|e| e.to_string())?;
    let doc = raw
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| format!("Failed to parse editor_prefs.toml: {e}"))?;

    let Some(item) = doc.get("plugins") else {
        return Ok(Vec::new());
    };

    if let Some(plugins) = item.as_array_of_tables() {
        return Ok(plugins
            .iter()
            .filter_map(|table| {
                table
                    .get("id")
                    .and_then(|item| item.as_str().map(str::to_string))
            })
            .collect());
    }

    Ok(Vec::new())
}

pub fn read_daemon_registry() -> Result<Value, String> {
    let path = mcp_daemon_registry_path();
    if !path.exists() {
        return Ok(json!({ "servers": {} }));
    }
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| format!("Invalid MCP daemon registry: {e}"))
}

pub fn moribund_config_dir() -> Option<PathBuf> {
    ProjectDirs::from("io", "Moribund", "MorBloggerThemeEditor").map(|d| d.config_dir().to_path_buf())
}