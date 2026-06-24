use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use mor_blogger_core::config::ThemeConfig;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use super::state::ThemeState;

enum ThemeReloadMsg {
    EditorPrefs(String),
    PresetFile(PathBuf),
}

static THEME_RELOAD_TX: OnceLock<UnboundedSender<ThemeReloadMsg>> = OnceLock::new();

pub fn register_theme_reload_sender(tx: UnboundedSender<ThemeReloadMsg>) {
    let _ = THEME_RELOAD_TX.set(tx);
}

fn send_reload(msg: ThemeReloadMsg) {
    if let Some(tx) = THEME_RELOAD_TX.get() {
        if tx.unbounded_send(msg).is_err() {
            log::warn!("Theme hot-reload channel closed");
        }
    }
}

pub fn spawn_editor_prefs_watcher() {
    static WATCHER_STARTED: OnceLock<()> = OnceLock::new();
    if WATCHER_STARTED.set(()).is_err() {
        return;
    }

    thread::spawn(|| {
        let prefs_path = mor_blogger_core::config::prefs::editor_prefs_path();
        let prefs_file_name = prefs_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "editor_prefs.toml".to_string());
        let watch_dir = prefs_path
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        if let Some(parent) = prefs_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let target_path = prefs_path.clone();
        let watched_file_name = prefs_file_name.clone();
        let mut watcher = match recommended_watcher(move |result: Result<Event, notify::Error>| {
            let Ok(event) = result else {
                return;
            };

            if !matches!(event.kind, EventKind::Modify(_)) {
                return;
            }

            let is_target = event.paths.iter().any(|path| {
                path.file_name()
                    .map(|name| name == watched_file_name.as_str())
                    .unwrap_or(false)
            });
            if !is_target {
                return;
            }

            thread::sleep(Duration::from_millis(50));

            let Ok(toml_str) = std::fs::read_to_string(&target_path) else {
                log::warn!("editor_prefs.toml changed but could not be read");
                return;
            };

            // EditorPrefs (pin state, UI chrome, plugins) shares this path but is not ThemeConfig.
            if toml::from_str::<ThemeConfig>(&toml_str).is_err() {
                return;
            }

            send_reload(ThemeReloadMsg::EditorPrefs(toml_str));
        }) {
            Ok(watcher) => watcher,
            Err(err) => {
                log::error!("Failed to create editor_prefs.toml watcher: {}", err);
                return;
            }
        };

        if let Err(err) = watcher.watch(&watch_dir, RecursiveMode::NonRecursive) {
            log::error!(
                "Failed to watch {:?} for editor_prefs.toml changes: {}",
                watch_dir,
                err
            );
            return;
        }

        log::info!(
            "Watching {:?} for external edits to {}",
            watch_dir,
            prefs_file_name
        );

        loop {
            thread::sleep(Duration::from_secs(3600));
        }
    });
}

pub fn spawn_theme_presets_watcher() {
    static WATCHER_STARTED: OnceLock<()> = OnceLock::new();
    if WATCHER_STARTED.set(()).is_err() {
        return;
    }

    thread::spawn(|| {
        let preset_dir = mor_blogger_core::presets::get_canonical_presets_dir();
        if let Err(err) = std::fs::create_dir_all(&preset_dir) {
            log::error!("Failed to ensure theme_presets directory exists: {}", err);
            return;
        }

        let watch_dir = preset_dir.clone();
        let mut watcher = match recommended_watcher(move |result: Result<Event, notify::Error>| {
            let Ok(event) = result else {
                return;
            };

            if !matches!(event.kind, EventKind::Modify(_)) {
                return;
            }

            let changed_preset = event.paths.iter().find(|path| {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext == "toml")
            });
            let Some(path) = changed_preset.cloned() else {
                return;
            };

            thread::sleep(Duration::from_millis(50));
            send_reload(ThemeReloadMsg::PresetFile(path));
        }) {
            Ok(watcher) => watcher,
            Err(err) => {
                log::error!("Failed to create theme_presets watcher: {}", err);
                return;
            }
        };

        if let Err(err) = watcher.watch(&watch_dir, RecursiveMode::NonRecursive) {
            log::error!(
                "Failed to watch {:?} for theme preset changes: {}",
                watch_dir,
                err
            );
            return;
        }

        log::info!("Watching {:?} for external theme preset edits", watch_dir);

        loop {
            thread::sleep(Duration::from_secs(3600));
        }
    });
}

pub fn use_theme_config_hot_reload(theme: ThemeState) {
    let signals = theme.signals;
    let mut active_preset = theme.active_preset;

    let reload = use_coroutine(move |mut rx: UnboundedReceiver<ThemeReloadMsg>| async move {
        while let Some(msg) = rx.next().await {
            match msg {
                ThemeReloadMsg::EditorPrefs(toml_str) => {
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
                ThemeReloadMsg::PresetFile(path) => {
                    match mor_blogger_core::presets::theme_config_from_path(&path) {
                        Ok(config) => {
                            let preset_stem = path
                                .file_stem()
                                .and_then(|stem| stem.to_str())
                                .unwrap_or("");
                            let current_active = active_preset();
                            let should_apply = match current_active {
                                None => true,
                                Some(active_id) => active_id == preset_stem,
                            };

                            if !should_apply {
                                log::debug!(
                                    "Ignored preset hot-reload for {:?} (active preset: {:?})",
                                    path,
                                    current_active
                                );
                                continue;
                            }

                            signals.apply_config(&config);
                            if let Some(preset) = mor_blogger_core::presets::all_presets()
                                .into_iter()
                                .find(|preset| preset.id == preset_stem)
                            {
                                active_preset.set(Some(preset.id));
                            }
                            log::info!("Hot-reloaded theme preset from {:?}", path);
                        }
                        Err(err) => {
                            log::error!(
                                "Hot-reload failed: {:?} is not a valid preset TOML: {}",
                                path,
                                err
                            );
                        }
                    }
                }
            }
        }
    });

    use_effect(move || {
        register_theme_reload_sender(reload.tx());
        spawn_editor_prefs_watcher();
        spawn_theme_presets_watcher();
    });
}