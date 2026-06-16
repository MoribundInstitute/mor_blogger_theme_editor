use rfd::FileDialog;
use serde::{Deserialize, Serialize};

use mor_blogger_core::config::gtk_theme::{import_gtk_theme, ImportedGtkPreset};
use mor_blogger_core::config::ThemeConfig;

pub(crate) fn choose_gtk_theme(
    _current_config: &ThemeConfig,
) -> Result<Option<ImportedGtkPreset>, String> {
    let Some(dir) = FileDialog::new()
        .set_title("Select extracted theme root folder (e.g. 'Mojave-Dark-alt')")
        .pick_folder()
    else {
        return Ok(None);
    };

    import_gtk_theme(&dir).map(Some)
}

// Emulates the shape of our new single-file TOML presets
#[derive(Serialize)]
struct TomlExport<'a> {
    name: &'a str,
    description: &'a str,
    colors: &'a mor_blogger_core::config::ColorConfig,
    background: &'a mor_blogger_core::config::BackgroundConfig,
    typography: &'a mor_blogger_core::config::TypographyConfig,
    buttons: &'a mor_blogger_core::config::ButtonConfig,
    preset_css: &'a str,
}

pub(crate) fn save_imported_gtk_preset(
    imported: &ImportedGtkPreset,
) -> Result<String, String> {
    let export = TomlExport {
        name: &imported.name,
        description: &imported.description,
        colors: &imported.config.colors,
        background: &imported.config.background,
        typography: &imported.config.typography,
        buttons: &imported.config.buttons,
        preset_css: &imported.preset_css,
    };

    let toml_str = toml::to_string_pretty(&export).map_err(|e| e.to_string())?;

    let preset_dir = std::path::PathBuf::from("theme_presets");
    if !preset_dir.exists() {
        std::fs::create_dir_all(&preset_dir).map_err(|e| e.to_string())?;
    }

    let file_path = preset_dir.join(format!("{}.toml", imported.id));
    std::fs::write(&file_path, toml_str).map_err(|e| e.to_string())?;

    Ok(format!("GTK Theme successfully compiled to {}", file_path.display()))
}

pub(crate) async fn fetch_remote_theme(url: &str) -> Result<ThemeConfig, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("request failed: {}", err))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let text = response
        .text()
        .await
        .map_err(|err| format!("could not read response: {}", err))?;

    parse_theme_text(&text)
}

pub(crate) fn parse_theme_text(text: &str) -> Result<ThemeConfig, String> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return Err("theme text is empty".to_string());
    }

    if let Ok(config) = serde_json::from_str::<ThemeConfig>(trimmed) {
        return Ok(config);
    }

    if let Ok(config) = parse_json_wrapped_theme(trimmed) {
        return Ok(config);
    }

    if let Ok(config) = toml::from_str::<ThemeConfig>(trimmed) {
        return Ok(config);
    }

    Err(
        "expected a ThemeConfig JSON/TOML document, or a JSON object with base_config/config/theme"
            .to_string(),
    )
}

#[derive(Deserialize)]
struct WrappedTheme {
    #[serde(default)]
    base_config: Option<ThemeConfig>,

    #[serde(default)]
    config: Option<ThemeConfig>,

    #[serde(default)]
    theme: Option<ThemeConfig>,

    #[serde(default)]
    preset_css: Option<String>,
}

fn parse_json_wrapped_theme(text: &str) -> Result<ThemeConfig, String> {
    let wrapped = serde_json::from_str::<WrappedTheme>(text).map_err(|err| err.to_string())?;

    let mut config = wrapped
        .base_config
        .or(wrapped.config)
                .or(wrapped.theme)
        .ok_or_else(|| "no base_config/config/theme object found".to_string())?;

    if config.preset_css.trim().is_empty() {
        if let Some(css) = wrapped.preset_css {
            config.preset_css = css;
        }
    }

    Ok(config)
}

pub(crate) fn normalize_preset_url(input: &str) -> String {
    let url = input.trim();

    if url.contains("github.com/") && url.contains("/blob/") {
        let without_scheme = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        let parts: Vec<&str> = without_scheme.split('/').collect();

        if parts.len() >= 6 && parts[0] == "github.com" && parts[3] == "blob" {
            let owner = parts[1];
            let repo = parts[2];
            let branch = parts[4];
            let path = parts[5..].join("/");

            return format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, branch, path
            );
        }
    }

    if url.contains("cdn.jsdelivr.net/gh/") {
        let without_scheme = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        let parts: Vec<&str> = without_scheme.splitn(4, '/').collect();

        if parts.len() >= 4 && parts[0] == "cdn.jsdelivr.net" && parts[1] == "gh" {
            let owner = parts[2];
            let rest = parts[3];

            if let Some((repo_branch, path)) = rest.split_once('/') {
                let (repo, branch) = if let Some((r, b)) = repo_branch.split_once('@') {
                    (r, b)
                } else {
                    (repo_branch, "main")
                };

                return format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}",
                    owner, repo, branch, path
                );
            }
        }
    }

    url.to_string()
}