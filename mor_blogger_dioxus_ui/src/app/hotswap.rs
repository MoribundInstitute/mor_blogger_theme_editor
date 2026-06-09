use mor_blogger_core::config::ThemeConfig;
use crate::ui::panels::presets::ThemeSignals;

pub fn apply_hotswap_json(signals: ThemeSignals, json_text: String) {
    // 1. Instantly parse the JSON straight into your master config struct
    match serde_json::from_str::<ThemeConfig>(&json_text) {
        Ok(config) => {
            // 2. Push the loaded config into the Dioxus UI signals using your original method!
            signals.apply_config(&config);
            log::info!("Hotswap successful: Project loaded and UI updated.");
        }
        Err(e) => {
            log::error!("Load Project failed: invalid JSON payload. Error: {}", e);
        }
    }
}
