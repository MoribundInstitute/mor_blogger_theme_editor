pub use mor_blogger_core::utils::mcp_install::{
    install_local_mcp_plugin, install_mcp_to_path, list_installed_mcp_binaries,
    list_registered_plugin_ids, load_local_mcp_manifest, mcp_daemon_registry_path, mcp_plugins_dir,
    read_daemon_registry, LocalMcpManifest, McpInstallReport,
};

use mor_blogger_core::utils::mcp_install::{
    install_mcp_to_claude as core_install_mcp_to_claude, mcp_plugins_dir as core_plugins_dir,
};
use reqwest::header::USER_AGENT;
use std::fs;
use std::path::PathBuf;

pub async fn install_plugin_from_github(repo_path: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo_path);

    let res = client
        .get(&api_url)
        .header(USER_AGENT, "MorBlogger-Plugin-Manager")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!(
            "Repository not found or no releases exist. (Status: {})",
            res.status()
        ));
    }

    let release_data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let assets = release_data["assets"]
        .as_array()
        .ok_or("No compiled assets found in this release.")?;

    let os_target = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "macos"
    };

    let mut download_url = String::new();
    let mut file_name = String::new();

    for asset in assets {
        let name = asset["name"].as_str().unwrap_or("").to_lowercase();
        if name.contains(os_target) || (os_target == "windows" && name.contains(".exe")) {
            download_url = asset["browser_download_url"]
                .as_str()
                .unwrap_or("")
                .to_string();
            file_name = asset["name"].as_str().unwrap_or("plugin.bin").to_string();
            break;
        }
    }

    if download_url.is_empty() {
        return Err(format!(
            "Found the release, but no binary matched your OS ({}).",
            os_target
        ));
    }

    println!("Downloading {}...", file_name);
    let plugin_dir = core_plugins_dir();
    fs::create_dir_all(&plugin_dir).map_err(|e| e.to_string())?;
    let out_path = plugin_dir.join(&file_name);

    let plugin_res = client
        .get(&download_url)
        .header(USER_AGENT, "MorBlogger-Plugin-Manager")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let bytes = plugin_res.bytes().await.map_err(|e| e.to_string())?;
    fs::write(&out_path, bytes).map_err(|e| e.to_string())?;

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&out_path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&out_path, perms).map_err(|e| e.to_string())?;
    }

    if file_name.contains("mcp") {
        core_install_mcp_to_claude(&out_path, "mor_blogger_engine")?;
    }

    Ok(file_name)
}

pub fn install_mcp_to_claude(binary_path: &PathBuf) -> Result<(), String> {
    core_install_mcp_to_claude(binary_path, "mor_blogger_engine").map(|_| ())
}

/// Open a terminal running an AI CLI (Claude Code) with this MCP server
/// attached, so the user can drive it in plain English instead of JSON-RPC.
pub fn spawn_chat_in_terminal(
    server_key: &str,
    command: &str,
    display_name: &str,
    system_prompt: &str,
) -> Result<(), String> {
    // Write the MCP config to a file instead of inlining JSON — sidesteps
    // shell/cmd quoting entirely.
    let cfg_dir = mcp_daemon_registry_path()
        .parent()
        .map(std::path::Path::to_path_buf)
        .ok_or("Could not resolve MCP config directory")?;
    fs::create_dir_all(&cfg_dir).map_err(|e| e.to_string())?;
    let cfg_path = cfg_dir.join(format!("{server_key}.claude_mcp.json"));
    let cfg_json = serde_json::json!({
        "mcpServers": { server_key: { "command": command, "args": [] } }
    });
    fs::write(&cfg_path, cfg_json.to_string()).map_err(|e| e.to_string())?;
    let cfg = cfg_path.to_string_lossy();

    #[cfg(target_os = "windows")]
    {
        return std::process::Command::new("cmd")
            .args(["/C", "start", "cmd", "/K"])
            .arg(format!("claude --mcp-config \"{cfg}\""))
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Failed to open terminal: {e}"));
    }

    #[cfg(not(target_os = "windows"))]
    {
        let name = display_name.replace('\'', "");
        let prompt_arg = if system_prompt.is_empty() {
            String::new()
        } else {
            format!(
                " --append-system-prompt '{}'",
                system_prompt.replace('\'', "'\\''")
            )
        };
        // ponytail: claude only; add grok/gemini branches when their MCP flags are known.
        let wrapped = format!(
            "echo '=== {name} — plain-English chat ==='; \
             if command -v claude >/dev/null 2>&1; then \
               claude --mcp-config '{cfg}'{prompt_arg}; \
             else \
               echo 'No AI CLI found. Install Claude Code:'; \
               echo '  npm install -g @anthropic-ai/claude-code'; \
             fi; echo; echo '[chat ended]'; read -r _"
        );
        spawn_shell_in_terminal(&wrapped)
    }
}

#[cfg(not(target_os = "windows"))]
fn spawn_shell_in_terminal(wrapped: &str) -> Result<(), String> {
    // ponytail: fixed candidate list; $TERMINAL env var is the escape hatch.
    let mut candidates: Vec<(String, Vec<&str>)> = Vec::new();
    if let Ok(term) = std::env::var("TERMINAL") {
        candidates.push((term, vec![]));
    }
    for (bin, pre) in [
        ("gnome-terminal", vec!["--"]),
        ("konsole", vec!["-e"]),
        ("alacritty", vec!["-e"]),
        ("kitty", vec![]),
        ("foot", vec![]),
        ("xterm", vec!["-e"]),
    ] {
        candidates.push((bin.to_string(), pre));
    }

    for (bin, pre) in candidates {
        if std::process::Command::new(&bin)
            .args(pre)
            .args(["sh", "-c", wrapped])
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
    }
    Err("No terminal emulator found. Set the $TERMINAL environment variable.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_temp_file() -> PathBuf {
        let mut path = std::env::temp_dir();
        let name = format!(
            "claude_config_test_{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos().to_string())
                .unwrap_or_else(|_| "fallback".to_string())
        );
        path.push(name);
        path
    }

    #[test]
    fn test_install_mcp_creates_new_config() {
        let config_file = get_temp_file();
        let binary_path = PathBuf::from("/usr/bin/mor_blogger_engine");

        let res = install_mcp_to_path(&config_file, &binary_path, "mor_blogger_engine");
        assert!(res.is_ok());

        let content = fs::read_to_string(&config_file).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(
            parsed["mcpServers"]["mor_blogger_engine"]["command"],
            "/usr/bin/mor_blogger_engine"
        );

        let _ = fs::remove_file(config_file);
    }

    #[test]
    fn test_install_mcp_updates_existing_config() {
        let config_file = get_temp_file();
        let binary_path = PathBuf::from("/usr/bin/mor_blogger_engine");

        let initial_json = serde_json::json!({
            "existingKey": "existingValue",
            "mcpServers": {
                "other_server": {
                    "command": "other_binary"
                }
            }
        });
        fs::write(&config_file, initial_json.to_string()).unwrap();

        let res = install_mcp_to_path(&config_file, &binary_path, "mor_blogger_engine");
        assert!(res.is_ok());

        let content = fs::read_to_string(&config_file).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(parsed["existingKey"], "existingValue");
        assert_eq!(
            parsed["mcpServers"]["other_server"]["command"],
            "other_binary"
        );
        assert_eq!(
            parsed["mcpServers"]["mor_blogger_engine"]["command"],
            "/usr/bin/mor_blogger_engine"
        );

        let _ = fs::remove_file(config_file);
    }
}