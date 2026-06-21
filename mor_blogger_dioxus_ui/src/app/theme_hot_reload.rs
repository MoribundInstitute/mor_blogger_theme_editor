use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use mor_blogger_core::config::ThemeConfig;

use super::config_bridge::spawn_editor_prefs_watcher;
use super::state::ThemeState;

pub fn use_theme_config_hot_reload(theme: ThemeState) {
    let signals = theme.signals;
    let mut active_preset = theme.active_preset;

    let reload = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        while let Some(toml_str) = rx.next().await {
            match toml::from_str::<ThemeConfig>(&toml_str) {
                Ok(config) => {
                    signals.apply_config(&config);
                    active_preset.set(None);
                    log::info!("Hot-reloaded theme config from editor_prefs.toml");
                }
                Err(err) => {
                    log::error!(
                        "Hot-reload failed: editor_prefs.toml is not a valid ThemeConfig: {}",
                        err
                    );
                }
            }
        }
    });

    use_effect(move || {
        super::config_bridge::register_theme_reload_sender(reload.tx());
        spawn_editor_prefs_watcher();
    });
}
