use rfd::FileDialog;
use serde::Deserialize;

use crate::config::gtk_theme::{import_gtk4_preset, ImportedGtkPreset};
use crate::config::ThemeConfig;
use crate::presets::user_presets::{save_user_preset_bundle, UserPresetDiskReport};

/// Pick a GTK theme folder and convert it into a reusable imported preset.
///
/// This does not save anything to disk. The caller can apply
/// `imported.config` immediately, then optionally call
/// `save_imported_gtk_preset` if the user chooses to keep it.
pub(crate) fn choose_gtk_theme(
    current_config: &ThemeConfig,
) -> Result<Option<ImportedGtkPreset>, String> {
    let Some(dir) = FileDialog::new()
        .set_title("Select GTK4 Theme Folder")
        .pick_folder()
    else {
        return Ok(None);
    };

    import_gtk4_preset(&dir, current_config).map(Some)
}

/// Save the most recently imported GTK preset as a reusable user preset bundle.
pub(crate) fn save_imported_gtk_preset(
    imported: &ImportedGtkPreset,
) -> Result<UserPresetDiskReport, String> {
    let bundle = imported.to_user_preset_bundle();
    save_user_preset_bundle(&bundle, Some(&imported.icon_assets))
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

    url.to_string()
}
