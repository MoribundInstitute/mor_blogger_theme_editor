use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::config::ThemeConfig;

const MARKER_START: &str = "<!-- MOR_BLOGGER_THEME_STATE:";
const MARKER_END: &str = ":MOR_BLOGGER_THEME_STATE -->";

/// Injects the base64-encoded TOML state directly into the Blogger XML skeleton.
pub fn inject_state(xml: &str, toml_data: &str) -> String {
    let encoded = STANDARD.encode(toml_data);
    let injection = format!("{}{}{}\n", MARKER_START, encoded, MARKER_END);

    if xml.contains(MARKER_START) && xml.contains(MARKER_END) {
        return replace_existing_state(xml, &injection);
    }

    if let Some(idx) = xml.find("<!DOCTYPE html>") {
        let split_point = idx + "<!DOCTYPE html>".len();
        let (first, second) = xml.split_at(split_point);
        format!("{}\n{}{}", first, injection, second)
    } else {
        format!("{}{}", injection, xml)
    }
}

/// Extracts the base64 string from pasted XML, decodes it, and returns the ThemeConfig.
pub fn extract_and_decode(pasted_xml: &str) -> Result<ThemeConfig, String> {
    let start_idx = pasted_xml
        .find(MARKER_START)
        .ok_or_else(|| "No theme data found. Did you paste a valid exported template?".to_string())?;

    let content_start = start_idx + MARKER_START.len();

    let end_rel_idx = pasted_xml[content_start..]
        .find(MARKER_END)
        .ok_or_else(|| "Malformed theme data marker.".to_string())?;

    let content_end = content_start + end_rel_idx;
    let base64_str = pasted_xml[content_start..content_end].trim();

    if base64_str.is_empty() {
        return Err("Theme data marker was found, but it was empty.".to_string());
    }

    let decoded_bytes = STANDARD
        .decode(base64_str)
        .map_err(|err| format!("Failed to decode base64 theme data: {}", err))?;

    let decoded_str = String::from_utf8(decoded_bytes)
        .map_err(|_| "Theme data is not valid UTF-8.".to_string())?;

    println!("DEBUG: Attempting to parse TOML: {:?}", decoded_str);

    let config: ThemeConfig = toml::from_str(&decoded_str)
        .map_err(|e| format!("Failed to parse embedded TOML: {}", e))?;

    Ok(config)
}

fn replace_existing_state(xml: &str, new_injection: &str) -> String {
    let Some(start_idx) = xml.find(MARKER_START) else {
        return xml.to_string();
    };

    let content_start = start_idx + MARKER_START.len();

    let Some(end_rel_idx) = xml[content_start..].find(MARKER_END) else {
        return xml.to_string();
    };

    let end_idx = content_start + end_rel_idx + MARKER_END.len();

    format!("{}{}{}", &xml[..start_idx], new_injection, &xml[end_idx..])
}