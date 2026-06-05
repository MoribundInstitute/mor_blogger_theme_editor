use base64::{engine::general_purpose::STANDARD, Engine as _};
use postcard::{from_bytes, to_allocvec};

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

/// Injects the compressed, base64-encoded state directly into the Blogger XML skeleton.
pub fn inject_state(xml: &str, config: &ThemeConfig) -> Result<String, String> {
    // 1. Serialize to a highly compact binary format (Postcard).
    let binary_data = to_allocvec(config)
        .map_err(|e| format!("Serialization failed: {}", e))?;

    // 2. Compress the binary payload to eliminate redundant CSS/text space.
    // Level 3 is a standard, efficient default for zstd.
    let compressed_data = zstd::encode_all(binary_data.as_slice(), 3)
        .map_err(|e| format!("Compression failed: {}", e))?;

    // 3. Encode into safe characters to prevent generating invalid XML sequences.
    let encoded = STANDARD.encode(&compressed_data);
    let injection = format!("{}{}{}\n", MARKER_START, encoded, MARKER_END);

    // Replace current embedded state if it is already present.
    if xml.contains(MARKER_START) && xml.contains(MARKER_END) {
        return Ok(replace_existing_state(
            xml,
            MARKER_START,
            MARKER_END,
            &injection,
        ));
    }

    // Replace legacy embedded state too. This prevents duplicated state comments
    // when an old export is loaded and then exported again.
    if xml.contains(LEGACY_MARKER_START) && xml.contains(LEGACY_MARKER_END) {
        return Ok(replace_existing_state(
            xml,
            LEGACY_MARKER_START,
            LEGACY_MARKER_END,
            &injection,
        ));
    }

    // Safely inject the state marker inside the head tag so Blogger is less likely to strip it.
    if let Some(idx) = xml.find("<head>") {
        let split_point = idx + "<head>".len();
        let (first, second) = xml.split_at(split_point);
        Ok(format!("{}\n{}{}", first, injection, second))
    } else {
        Ok(format!("{}{}", injection, xml))
    }
}

/// Extracts the base64 string from pasted XML, decompresses it, and returns the ThemeConfig.
pub fn extract_and_decode(pasted_xml: &str) -> Result<ThemeConfig, String> {
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

    // 2. Try to decompress zstd (current V2 format).
    match zstd::decode_all(decoded_bytes.as_slice()) {
        Ok(binary_data) => {
            // 3. Deserialize the binary data directly back into the Rust struct.
            from_bytes(&binary_data)
                .map_err(|e| format!("Failed to deserialize V2 theme data: {}", e))
        }
        Err(zstd_err) => {
            // If decompression fails, this may be a V1 XML file from before the
            // zstd upgrade, where the payload was raw TOML -> Base64.
            if let Ok(toml_str) = String::from_utf8(decoded_bytes.clone()) {
                if let Ok(v1_config) = toml::from_str::<ThemeConfig>(&toml_str) {
                    return Ok(v1_config);
                }
            }

            // If it was not V1 TOML either, return the original zstd error.
            Err(format!("Decompression failed: {}", zstd_err))
        }
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
