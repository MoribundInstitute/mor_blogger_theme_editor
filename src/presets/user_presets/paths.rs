use std::path::PathBuf;

/// Returns the app's user preset directory without requiring an extra crate.
///
/// Linux target:
/// - `$XDG_CONFIG_HOME/mor_blogger_theme_editor/presets`
/// - fallback: `$HOME/.config/mor_blogger_theme_editor/presets`
/// - last resort: `./user_presets`
pub fn default_user_presets_dir() -> PathBuf {
    if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home)
            .join("mor_blogger_theme_editor")
            .join("presets");
    }

    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".config")
            .join("mor_blogger_theme_editor")
            .join("presets");
    }

    PathBuf::from("user_presets")
}

pub fn user_preset_bundle_dir(id: &str) -> PathBuf {
    default_user_presets_dir().join(sanitize_preset_id(id))
}

/// Make a stable folder/file id from a user-facing preset name.
///
/// Keeps lowercase ASCII letters and digits, converts separators to a single
/// dash, and trims leading/trailing dashes.
pub fn sanitize_preset_id(input: &str) -> String {
    let mut out = String::new();
    let mut last_was_dash = false;

    for ch in input.chars() {
        let ch = ch.to_ascii_lowercase();

        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_was_dash = false;
        } else if !last_was_dash {
            out.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = out.trim_matches('-').to_string();

    if trimmed.is_empty() {
        "imported-preset".to_string()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_preset_id;

    #[test]
    fn sanitizes_preset_ids() {
        assert_eq!(sanitize_preset_id("WhiteSur Dark Solid Nord"), "whitesur-dark-solid-nord");
        assert_eq!(sanitize_preset_id("  Catppuccin--Mocha!! "), "catppuccin-mocha");
        assert_eq!(sanitize_preset_id("!!!"), "imported-preset");
    }
}
