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

pub async fn fetch_and_install_from_github() -> Result<(), String> {
    let plugin_dir = core_plugins_dir();
    fs::create_dir_all(&plugin_dir).map_err(|e| e.to_string())?;

    let download_url = if cfg!(target_os = "linux") {
        "https://github.com/MoribundInstitute/mor-blogger-theme-editor-mcp/releases/latest/download/mor-blogger-mcp-linux"
    } else if cfg!(target_os = "windows") {
        "https://github.com/MoribundInstitute/mor-blogger-theme-editor-mcp/releases/latest/download/mor-blogger-mcp.exe"
    } else {
        return Err("OS not supported for auto-install yet.".to_string());
    };

    let binary_name = if cfg!(target_os = "windows") {
        "mor-blogger-mcp.exe"
    } else {
        "mor-blogger-mcp"
    };
    let binary_path = plugin_dir.join(binary_name);

    println!("Downloading AI Bridge from GitHub...");
    let response = reqwest::get(download_url)
        .await
        .map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!(
            "Download failed with status: {}",
            response.status()
        ));
    }
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    fs::write(&binary_path, bytes).map_err(|e| e.to_string())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary_path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary_path, perms).map_err(|e| e.to_string())?;
    }

    core_install_mcp_to_claude(&binary_path, "mor_blogger_engine")?;
    Ok(())
}

pub fn install_mcp_to_claude(binary_path: &PathBuf) -> Result<(), String> {
    core_install_mcp_to_claude(binary_path, "mor_blogger_engine").map(|_| ())
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