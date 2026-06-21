use std::fs;
use std::path::{Path, PathBuf};
use serde_json::{Value, json};
use reqwest::header::USER_AGENT;

pub async fn install_plugin_from_github(repo_path: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo_path);

    // 1. Ping GitHub for the latest release data
    let res = client.get(&api_url)
        .header(USER_AGENT, "MorBlogger-Plugin-Manager")
        .send().await.map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Repository not found or no releases exist. (Status: {})", res.status()));
    }

    let release_data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let assets = release_data["assets"].as_array().ok_or("No compiled assets found in this release.")?;

    // 2. Identify the user's OS to grab the correct binary
    let os_target = if cfg!(target_os = "linux") { "linux" }
                    else if cfg!(target_os = "windows") { "windows" }
                    else { "macos" };

    let mut download_url = String::new();
    let mut file_name = String::new();

    // 3. Scan the release assets for a matching OS string
    for asset in assets {
        let name = asset["name"].as_str().unwrap_or("").to_lowercase();
        if name.contains(os_target) || (os_target == "windows" && name.contains(".exe")) {
            download_url = asset["browser_download_url"].as_str().unwrap_or("").to_string();
            file_name = asset["name"].as_str().unwrap_or("plugin.bin").to_string();
            break;
        }
    }

    if download_url.is_empty() {
        return Err(format!("Found the release, but no binary matched your OS ({}).", os_target));
    }

    // 4. Download and save the binary
    println!("Downloading {}...", file_name);
    let plugin_dir = dirs::data_local_dir().unwrap().join("morblogger/plugins");
    fs::create_dir_all(&plugin_dir).map_err(|e| e.to_string())?;
    let out_path = plugin_dir.join(&file_name);

    let plugin_res = client.get(&download_url)
        .header(USER_AGENT, "MorBlogger-Plugin-Manager")
        .send().await.map_err(|e| e.to_string())?;
    
    let bytes = plugin_res.bytes().await.map_err(|e| e.to_string())?;
    fs::write(&out_path, bytes).map_err(|e| e.to_string())?;

    // 5. Ensure the file is executable on Unix systems
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&out_path).map_err(|e| e.to_string())?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&out_path, perms).map_err(|e| e.to_string())?;
    }

    // Optional: If the plugin name contains "mcp", automatically wire it to the AI clients here
    if file_name.contains("mcp") {
        install_mcp_to_claude(&out_path)?;
    }

    Ok(file_name)
}

pub async fn fetch_and_install_from_github() -> Result<(), String> {
    // 1. Define where the binary should live (e.g., ~/.local/share/morblogger/plugins)
    let data_dir = dirs::data_local_dir().ok_or("Could not find local data directory")?;
    let plugin_dir = data_dir.join("morblogger/plugins");
    fs::create_dir_all(&plugin_dir).map_err(|e| e.to_string())?;

    // 2. Define the download URL based on the OS (Assuming you upload these to GitHub Releases)
    // Note: You'll need to update this URL once you actually publish a GitHub Release!
    let download_url = if cfg!(target_os = "linux") {
        "https://github.com/MoribundInstitute/mor-blogger-theme-editor-mcp/releases/latest/download/mor-blogger-mcp-linux"
    } else if cfg!(target_os = "windows") {
        "https://github.com/MoribundInstitute/mor-blogger-theme-editor-mcp/releases/latest/download/mor-blogger-mcp.exe"
    } else {
        return Err("OS not supported for auto-install yet.".to_string());
    };

    let binary_name = if cfg!(target_os = "windows") { "mor-blogger-mcp.exe" } else { "mor-blogger-mcp" };
    let binary_path = plugin_dir.join(binary_name);

    // 3. Download the binary
    println!("Downloading AI Bridge from GitHub...");
    let response = reqwest::get(download_url).await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    
    // 4. Save the binary to the plugin folder
    fs::write(&binary_path, bytes).map_err(|e| e.to_string())?;

    // Make it executable on Linux/macOS
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary_path).map_err(|e| e.to_string())?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary_path, perms).map_err(|e| e.to_string())?;
    }

    // 5. Wire it up to the AI clients! (Reusing the function we discussed earlier)
    install_mcp_to_claude(&binary_path)?;

    Ok(())
}


pub fn install_mcp_to_claude(binary_path: &PathBuf) -> Result<(), String> {
    // Locate the standard Claude Desktop config path for Linux
    let config_dir = dirs::config_dir().ok_or("Could not find OS config directory")?;
    let claude_config_path = config_dir.join("Claude/claude_desktop_config.json");
    install_mcp_to_path(&claude_config_path, binary_path)
}

pub fn install_mcp_to_path(claude_config_path: &Path, binary_path: &Path) -> Result<(), String> {
    // Read existing config or create a fresh one
    let mut config: Value = if claude_config_path.exists() {
        let data = fs::read_to_string(claude_config_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or(json!({ "mcpServers": {} }))
    } else {
        // Ensure directory exists
        if let Some(parent) = claude_config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        json!({ "mcpServers": {} })
    };

    // Ensure root config is an object
    if !config.is_object() {
        config = json!({ "mcpServers": {} });
    } else if config.get("mcpServers").is_none() || !config["mcpServers"].is_object() {
        if let Some(obj) = config.as_object_mut() {
            obj.insert("mcpServers".to_string(), json!({}));
        }
    }

    // Inject the MorBlogger Engine into the mcpServers object
    if let Some(servers) = config.get_mut("mcpServers").and_then(|s| s.as_object_mut()) {
        servers.insert(
            "mor_blogger_engine".to_string(),
            json!({
                "command": binary_path.to_str().unwrap(),
                "args": []
            })
        );
    }

    // Write the updated config back to disk
    let pretty_json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(claude_config_path, pretty_json).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_temp_file() -> PathBuf {
        let mut path = std::env::temp_dir();
        let name = format!("claude_config_test_{}.json", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos().to_string())
            .unwrap_or_else(|_| "fallback".to_string()));
        path.push(name);
        path
    }

    #[test]
    fn test_install_mcp_creates_new_config() {
        let config_file = get_temp_file();
        let binary_path = PathBuf::from("/usr/bin/mor_blogger_engine");

        let res = install_mcp_to_path(&config_file, &binary_path);
        assert!(res.is_ok());

        let content = fs::read_to_string(&config_file).unwrap();
        let parsed: Value = serde_json::from_str(&content).unwrap();

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

        let initial_json = json!({
            "existingKey": "existingValue",
            "mcpServers": {
                "other_server": {
                    "command": "other_binary"
                }
            }
        });
        fs::write(&config_file, initial_json.to_string()).unwrap();

        let res = install_mcp_to_path(&config_file, &binary_path);
        assert!(res.is_ok());

        let content = fs::read_to_string(&config_file).unwrap();
        let parsed: Value = serde_json::from_str(&content).unwrap();

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

