use crate::app::config_bridge::EditorPrefs;
use crate::app::state::ThemeState;
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

pub fn new_workspace(mut theme: ThemeState, mut original_toml: Signal<String>) {
    let fresh_prefs = EditorPrefs::load();
    let mut config = mor_blogger_core::config::defaults::default_theme_config();
    if let Some(pack) = fresh_prefs.default_template_pack {
        config.template_pack = pack;
    }
    theme.signals.apply_config(&config);
    original_toml.set(toml::to_string_pretty(&config).unwrap_or_default());
    theme.active_preset.set(None);
    theme.commit();
}

pub fn load_theme(theme: ThemeState, mut original_toml: Signal<String>) {
    if let Some(content) = crate::utils::io::load_toml() {
        if let Ok(new_config) = toml::from_str::<ThemeConfig>(&content) {
            theme.signals.apply_config(&new_config);
            original_toml.set(content);
        }
    }
}

pub fn save_theme(config_toml: &str, mut original_toml: Signal<String>) {
    crate::utils::io::save_toml(config_toml);
    original_toml.set(config_toml.to_string());
}

pub fn import_xml(theme: ThemeState, mut original_toml: Signal<String>) {
    let sigs = theme.signals.clone();
    spawn(async move {
        let Some(file) = rfd::AsyncFileDialog::new()
            .set_title("Import Blogger XML")
            .add_filter("XML", &["xml"])
            .pick_file()
            .await
        else {
            return;
        };
        let Ok(xml_str) = std::fs::read_to_string(file.path()) else {
            return;
        };
        match mor_blogger_core::utils::rehydration::extract_workspace_state(&xml_str) {
            Ok(payload) => {
                sigs.apply_config(&payload.config);
                let restored_toml = if payload.config_toml.is_empty() {
                    toml::to_string_pretty(&payload.config).unwrap_or_default()
                } else {
                    payload.config_toml
                };
                original_toml.set(restored_toml);
            }
            Err(e) => log::error!("Failed to import XML: {}", e),
        }
    });
}

pub fn load_data(current_cfg: ThemeConfig, theme: ThemeState) {
    let sigs = theme.signals.clone();
    let mut current_cfg = current_cfg;
    spawn(async move {
        let Some(file) = rfd::AsyncFileDialog::new()
            .set_title("Load Site Data")
            .add_filter("JSON", &["json"])
            .pick_file()
            .await
        else {
            return;
        };
        let Ok(json) = std::fs::read_to_string(file.path()) else {
            return;
        };
        let Ok(loaded_data) = serde_json::from_str::<ThemeConfig>(&json) else {
            return;
        };
        current_cfg.apply_site_data(&loaded_data);
        sigs.apply_config(&current_cfg);
    });
}

pub fn save_data(current_cfg: ThemeConfig) {
    spawn(async move {
        let Some(file) = rfd::AsyncFileDialog::new()
            .set_file_name("my_site_data.json")
            .add_filter("JSON", &["json"])
            .save_file()
            .await
        else {
            return;
        };
        let Ok(json) = serde_json::to_string_pretty(&current_cfg) else {
            return;
        };
        let _ = std::fs::write(file.path(), json);
    });
}

pub fn export_xml(xml: &str) {
    crate::utils::io::save_xml(xml);
}

pub fn export_zip(xml: &str, config_toml: &str) {
    crate::utils::io::export_bundle(xml, config_toml);
}

pub fn copy_xml() {
    log::info!("Copy XML triggered (requires arboard binding)");
}
