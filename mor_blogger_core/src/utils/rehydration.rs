use std::collections::HashMap;
use std::path::Path;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::config::ThemeConfig;

/// Current marker for the embedded rehydration payload.
///
/// Keep these markers non-empty. Empty markers make every XML string look like it
/// already contains state and cause extraction/replacement to search from byte 0.
const MARKER_START: &str = "<!-- MORIBUND_THEME_STATE:";
const MARKER_END: &str = ":MORIBUND_THEME_STATE_END -->";

/// Legacy marker used by earlier exports.
///
/// Keep this so old Blogger XML exports can still restore their workspace state.
const LEGACY_MARKER_START: &str = "<!-- MOR_BLOGGER_THEME_STATE:";
const LEGACY_MARKER_END: &str = ":MOR_BLOGGER_THEME_STATE -->";

/// Full editor workspace state returned by the rehydration engine.
#[derive(Clone, Debug, PartialEq)]
pub struct RehydrationPayload {
    pub config: ThemeConfig,
    pub vfs: HashMap<String, String>,
    /// Original TOML source bytes preserved verbatim when available.
    pub config_toml: String,
}

/// Wire format embedded in exported XML. TOML bytes + VFS only — avoids postcard
/// schema drift from `ThemeConfig`'s `skip_serializing_if` attributes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct RehydrationBlob {
    pub config_toml: String,
    #[serde(default)]
    pub vfs: HashMap<String, String>,
}

/// Legacy V3 on-disk layout kept for backward compatibility with earlier exports.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct LegacyRehydrationPayloadV3 {
    pub config: ThemeConfig,
    #[serde(default)]
    pub vfs: HashMap<String, String>,
    #[serde(default)]
    pub config_toml: String,
}

impl RehydrationPayload {
    pub fn from_config(config: ThemeConfig) -> Self {
        Self {
            config,
            vfs: HashMap::new(),
            config_toml: String::new(),
        }
    }

    pub fn with_vfs(mut self, vfs: HashMap<String, String>) -> Self {
        self.vfs = vfs;
        self
    }

    pub fn with_config_toml(mut self, config_toml: impl Into<String>) -> Self {
        self.config_toml = config_toml.into();
        self
    }
}

/// Injects the compressed, base64-encoded workspace state directly into the Blogger XML skeleton.
pub fn inject_workspace_state(
    xml: &str,
    payload: &RehydrationPayload,
) -> Result<String, String> {
    let injection = encode_state_marker(payload)?;
    inject_marker(xml, &injection)
}

/// Extracts the embedded workspace payload from exported Blogger XML.
pub fn extract_workspace_state(pasted_xml: &str) -> Result<RehydrationPayload, String> {
    let payload = find_state_payload(pasted_xml)?;

    // Blogger sometimes injects linebreaks or spaces into long HTML comments.
    // Clean the string before passing it to the strict Base64 decoder.
    let base64_str = payload.replace(&['\n', '\r', ' ', '\t'][..], "");

    if base64_str.is_empty() {
        return Err("Theme data marker was found, but it was empty.".to_string());
    }

    // 1. Decode Base64 string back into bytes.
    let decoded_bytes = STANDARD
        .decode(&base64_str)
        .map_err(|err| format!("Failed to decode base64 theme data: {}", err))?;

    // 2. Try to decompress zstd (current V2/V3 format).
    match zstd::decode_all(decoded_bytes.as_slice()) {
        Ok(binary_data) => decode_binary_payload(&binary_data),
        Err(zstd_err) => {
            // If decompression fails, this may be a V1 XML file from before the
            // zstd upgrade, where the payload was raw TOML -> Base64.
            if let Ok(toml_str) = String::from_utf8(decoded_bytes.clone()) {
                if let Ok(v1_config) = toml::from_str::<ThemeConfig>(&toml_str) {
                    return Ok(RehydrationPayload::from_config(v1_config));
                }
            }

            Err(format!("Decompression failed: {}", zstd_err))
        }
    }
}

/// Extracts the base64 string from pasted XML, decompresses it, and returns the ThemeConfig.
pub fn extract_and_decode(pasted_xml: &str) -> Result<ThemeConfig, String> {
    extract_workspace_state(pasted_xml).map(|payload| payload.config)
}

/// Writes CSS/JS overrides from a restored VFS back to the user workspace directory.
pub fn persist_vfs_overrides(vfs: &HashMap<String, String>) -> Result<(), String> {
    for (filename, content) in vfs {
        if filename == "preset_css.css" || filename == "custom_js.js" {
            continue;
        }

        if filename.ends_with(".css") {
            crate::utils::fs_bridge::save_custom_css(filename, content)
                .map_err(|e| format!("Failed to persist CSS '{}': {}", filename, e))?;
        } else if filename.ends_with(".js") {
            crate::utils::fs_bridge::save_custom_js(filename, content)
                .map_err(|e| format!("Failed to persist JS '{}': {}", filename, e))?;
        }
    }

    Ok(())
}

/// Loads `.css` / `.js` override files from a directory into a VFS map keyed by filename.
pub fn load_vfs_from_dir(dir: &Path) -> Result<HashMap<String, String>, String> {
    let mut vfs = HashMap::new();

    if !dir.exists() {
        return Ok(vfs);
    }

    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read VFS directory '{}': {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read VFS entry: {}", e))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(filename) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if !(filename.ends_with(".css") || filename.ends_with(".js")) {
            continue;
        }

        let content = std::fs::read_to_string(&path).map_err(|e| {
            format!(
                "Failed to read VFS override '{}': {}",
                path.display(),
                e
            )
        })?;
        vfs.insert(filename.to_string(), content);
    }

    Ok(vfs)
}

/// Writes restored VFS overrides into a directory (used by headless restore flows).
pub fn write_vfs_to_dir(vfs: &HashMap<String, String>, dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir)
        .map_err(|e| format!("Failed to create VFS output directory '{}': {}", dir.display(), e))?;

    for (filename, content) in vfs {
        let dest = dir.join(filename);
        std::fs::write(&dest, content.as_bytes()).map_err(|e| {
            format!(
                "Failed to write VFS override '{}': {}",
                dest.display(),
                e
            )
        })?;
    }

    Ok(())
}

fn encode_state_marker(payload: &RehydrationPayload) -> Result<String, String> {
    let config_toml = if payload.config_toml.is_empty() {
        toml::to_string_pretty(&payload.config)
            .map_err(|e| format!("Failed to serialize theme config to TOML: {}", e))?
    } else {
        payload.config_toml.clone()
    };

    let blob = RehydrationBlob {
        config_toml,
        vfs: payload.vfs.clone(),
    };

    let binary_data =
        to_allocvec(&blob).map_err(|e| format!("Serialization failed: {}", e))?;

    // ponytail: level 19 — payload is ~1-2KB so encode cost is negligible; decode is level-agnostic
    let compressed_data = zstd::encode_all(binary_data.as_slice(), 19)
        .map_err(|e| format!("Compression failed: {}", e))?;

    let encoded = STANDARD.encode(&compressed_data);
    Ok(format!("{}{}{}\n", MARKER_START, encoded, MARKER_END))
}

fn decode_binary_payload(binary_data: &[u8]) -> Result<RehydrationPayload, String> {
    if let Ok(blob) = from_bytes::<RehydrationBlob>(binary_data) {
        return blob_into_payload(blob);
    }

    if let Ok(legacy) = from_bytes::<LegacyRehydrationPayloadV3>(binary_data) {
        let config_toml = if legacy.config_toml.is_empty() {
            toml::to_string_pretty(&legacy.config)
                .map_err(|e| format!("Failed to serialize legacy theme config to TOML: {}", e))?
        } else {
            legacy.config_toml
        };
        return Ok(RehydrationPayload {
            config: legacy.config,
            vfs: legacy.vfs,
            config_toml,
        });
    }

    if let Ok(config) = from_bytes::<ThemeConfig>(binary_data) {
        let config_toml = toml::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize legacy theme config to TOML: {}", e))?;
        return Ok(RehydrationPayload {
            config,
            vfs: HashMap::new(),
            config_toml,
        });
    }

    Err("Failed to deserialize embedded workspace state.".to_string())
}

fn blob_into_payload(blob: RehydrationBlob) -> Result<RehydrationPayload, String> {
    let config = toml::from_str(&blob.config_toml).map_err(|e| {
        format!(
            "Failed to parse embedded theme TOML ({} bytes): {}",
            blob.config_toml.len(),
            e
        )
    })?;

    Ok(RehydrationPayload {
        config,
        vfs: blob.vfs,
        config_toml: blob.config_toml,
    })
}

fn inject_marker(xml: &str, injection: &str) -> Result<String, String> {
    if xml.contains(MARKER_START) && xml.contains(MARKER_END) {
        return Ok(replace_existing_state(
            xml,
            MARKER_START,
            MARKER_END,
            injection,
        ));
    }

    if xml.contains(LEGACY_MARKER_START) && xml.contains(LEGACY_MARKER_END) {
        return Ok(replace_existing_state(
            xml,
            LEGACY_MARKER_START,
            LEGACY_MARKER_END,
            injection,
        ));
    }

    if let Some(idx) = xml.find("<head>") {
        let split_point = idx + "<head>".len();
        let (first, second) = xml.split_at(split_point);
        Ok(format!("{}\n{}{}", first, injection, second))
    } else {
        Ok(format!("{}{}", injection, xml))
    }
}

fn find_state_payload<'a>(xml: &'a str) -> Result<&'a str, String> {
    if let Some(payload) = extract_between(xml, MARKER_START, MARKER_END) {
        return Ok(payload);
    }

    if let Some(payload) = extract_between(xml, LEGACY_MARKER_START, LEGACY_MARKER_END) {
        return Ok(payload);
    }

    Err(format!(
        "No theme data found. Did you paste a valid exported template? Looked for both {} and {} markers.",
        "MORIBUND_THEME_STATE",
        "MOR_BLOGGER_THEME_STATE",
    ))
}

fn extract_between<'a>(xml: &'a str, marker_start: &str, marker_end: &str) -> Option<&'a str> {
    let start_idx = xml.find(marker_start)?;
    let content_start = start_idx + marker_start.len();
    let end_rel_idx = xml[content_start..].find(marker_end)?;
    let content_end = content_start + end_rel_idx;

    Some(&xml[content_start..content_end])
}

fn replace_existing_state(
    xml: &str,
    marker_start: &str,
    marker_end: &str,
    new_injection: &str,
) -> String {
    let Some(start_idx) = xml.find(marker_start) else {
        return xml.to_string();
    };

    let content_start = start_idx + marker_start.len();

    let Some(end_rel_idx) = xml[content_start..].find(marker_end) else {
        return xml.to_string();
    };

    let end_idx = content_start + end_rel_idx + marker_end.len();

    format!("{}{}{}", &xml[..start_idx], new_injection, &xml[end_idx..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::defaults::default_theme_config;

    fn complex_vfs() -> HashMap<String, String> {
        HashMap::from([
            (
                "99-RoundTrip-Probe.css".to_string(),
                "/* trailing newline preserved below */\n.quest-board::after { content: '⚔️'; }\n"
                    .to_string(),
            ),
            (
                "42-Custom-Behavior.js".to_string(),
                "window.__MOR_ROUNDTRIP__ = 'byte-perfect\\n';\n".to_string(),
            ),
        ])
    }

    fn complex_config() -> ThemeConfig {
        let mut config = default_theme_config();
        config.site.site_title = "Quest Board Ω".to_string();
        config.site.site_subtitle = "Guild notices • trading post 🛡️".to_string();
        config.site.header_logo_url = "https://example.test/logo.png".to_string();
        config.site.home_url = "https://example.test/".to_string();
        config.colors.accent = "#ffae42".to_string();
        config.preset_css = "/* preset bytes 🎮 */\nbody { letter-spacing: 0.5px; }\n".to_string();
        config.active_preset_id = Some("mor_retro_mmorpg".to_string());
        config.template_pack.header_variant = "gtk_headerbar".to_string();
        config.template_pack.widget_titles.insert(
            "sidebar-left".to_string(),
            "Guild Ledger".to_string(),
        );
        config
    }

    #[test]
    fn round_trip_preserves_config_vfs_and_toml_bytes() {
        let config = complex_config();
        let vfs = complex_vfs();
        let config_toml =
            toml::to_string_pretty(&config).expect("serialize complex config for blob");

        let payload = RehydrationPayload::from_config(config.clone())
            .with_vfs(vfs.clone())
            .with_config_toml(config_toml.clone());

        let rendered = crate::render::render_theme(&config, &vfs);
        let exported = inject_workspace_state(&rendered, &payload).expect("inject state");
        let restored = extract_workspace_state(&exported).expect("extract state");

        assert_eq!(restored.config, config);
        assert_eq!(restored.vfs, vfs);
        assert_eq!(restored.config_toml, config_toml);
    }

    fn fixture_paths() -> (std::path::PathBuf, std::path::PathBuf) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        (
            root.join("build/fixtures/complex_workspace.toml"),
            root.join("build/fixtures/vfs"),
        )
    }

    #[test]
    fn fixture_file_round_trip_is_byte_exact() {
        let (workspace, vfs_dir) = fixture_paths();
        if !workspace.exists() {
            return;
        }

        let config_toml = std::fs::read_to_string(&workspace).expect("read workspace fixture");
        let config = crate::presets::theme_config_from_path(&workspace).expect("parse workspace");
        let vfs = load_vfs_from_dir(&vfs_dir).expect("load vfs fixture");
        let payload = RehydrationPayload::from_config(config)
            .with_vfs(vfs.clone())
            .with_config_toml(config_toml.clone());

        let rendered = crate::render::render_theme(&payload.config, &payload.vfs);
        let exported = inject_workspace_state(&rendered, &payload).expect("inject");
        let restored = extract_workspace_state(&exported).expect("extract");

        assert_eq!(restored.config_toml, config_toml);
        assert_eq!(restored.vfs, vfs);
        assert_eq!(restored.config.site.site_title, payload.config.site.site_title);
        assert!(
            !restored.config.preset_css.is_empty(),
            "fixture preset_css must round-trip through embedded TOML bytes"
        );
    }

    #[test]
    fn fixture_xml_on_disk_round_trip() {
        let (workspace, vfs_dir) = fixture_paths();
        if !workspace.exists() {
            return;
        }

        let config_toml = std::fs::read_to_string(&workspace).expect("read workspace fixture");
        let config = crate::presets::theme_config_from_path(&workspace).expect("parse workspace");
        let vfs = load_vfs_from_dir(&vfs_dir).expect("load vfs fixture");
        let payload = RehydrationPayload::from_config(config)
            .with_vfs(vfs.clone())
            .with_config_toml(config_toml.clone());

        let rendered = crate::render::render_theme(&payload.config, &payload.vfs);
        let exported = inject_workspace_state(&rendered, &payload).expect("inject");

        let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../build/proof/test_disk_roundtrip.xml");
        if let Some(parent) = out.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&out, exported.as_bytes()).expect("write xml");
        let from_disk = std::fs::read_to_string(&out).expect("read xml");
        let restored = extract_workspace_state(&from_disk).expect("extract from disk");

        assert_eq!(restored.config_toml, config_toml);
        assert_eq!(restored.vfs, vfs);
    }
}