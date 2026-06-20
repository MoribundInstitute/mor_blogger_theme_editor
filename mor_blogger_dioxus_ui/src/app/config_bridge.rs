use crate::ui::shell::theme::MorTheme;
use crate::ui::workspace::layout::PanelLayout;
use mor_blogger_core::config::ThemeConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginState {
    pub id: String,
    pub enabled: bool,
    #[serde(default = "default_plugin_version")]
    pub version: String,
}

fn default_plugin_version() -> String {
    "1.0.0".to_string()
}

/// The blueprint for the data fetched from your decentralized Blogger/GitHub compendium.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompendiumManifest {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub payload_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CustomEditorColors {
    pub bg: Option<String>,
    pub panel: Option<String>,
    pub header: Option<String>,
    pub text: Option<String>,
    pub text_muted: Option<String>,
    pub border: Option<String>,
    pub border_light: Option<String>,
    pub accent: Option<String>,
    pub accent_hover: Option<String>,
    pub btn: Option<String>,
    pub btn_hover: Option<String>,
    pub font_family: Option<String>,
    pub font_size_base: Option<String>,
    pub font_size_h1: Option<String>,
    pub padding_base: Option<String>,
    pub border_radius: Option<String>,
    pub destructive: Option<String>,
    pub success: Option<String>,
    pub warning: Option<String>,
    #[serde(default)]
    pub panel_title_color: Option<String>,
}

pub fn resolve_effective_theme(base: MorTheme, overrides: &CustomEditorColors) -> MorTheme {
    macro_rules! apply {
        ($field:ident) => {
            overrides.$field.clone().unwrap_or(base.$field)
        };
    }
    MorTheme {
        bg: apply!(bg),
        panel: apply!(panel),
        header: apply!(header),
        text: apply!(text),
        text_muted: apply!(text_muted),
        border: apply!(border),
        border_light: apply!(border_light),
        accent: apply!(accent),
        accent_hover: apply!(accent_hover),
        btn: apply!(btn),
        btn_hover: apply!(btn_hover),
        font_family: apply!(font_family),
        font_size_base: apply!(font_size_base),
        font_size_h1: apply!(font_size_h1),
        padding_base: apply!(padding_base),
        border_radius: apply!(border_radius),
        destructive: apply!(destructive),
        success: apply!(success),
        warning: apply!(warning),
        enable_image_borders: base.enable_image_borders,
        custom_border_url: base.custom_border_url.clone(),
        svg_border_slice: base.svg_border_slice.clone(),
        image_border_width: base.image_border_width.clone(),
        target_sidebars: base.target_sidebars,
        target_canvas: base.target_canvas,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutPrefs {
    pub undo: Option<String>,
    pub redo: Option<String>,
    pub copy_raw_xml: Option<String>,
    pub toggle_left_dock: Option<String>,
    pub toggle_right_dock: Option<String>,
    #[serde(default)]
    pub user_prefs: Option<String>,
    #[serde(default)]
    pub theme_diagnostics: Option<String>,
    #[serde(default)]
    pub toggle_preview: Option<String>,
    #[serde(default)]
    pub exit_architect: Option<String>,
    #[serde(default)]
    pub open_project: Option<String>,
    #[serde(default)]
    pub save_project: Option<String>,
    #[serde(default)]
    pub export_xml: Option<String>,
    #[serde(default)]
    pub reset_zoom: Option<String>,
}

impl Default for ShortcutPrefs {
    fn default() -> Self {
        Self {
            undo: Some("Ctrl+Z".to_string()),
            redo: Some("Ctrl+Y".to_string()),
            copy_raw_xml: Some("Ctrl+C".to_string()),
            toggle_left_dock: Some("Ctrl+B".to_string()),
            toggle_right_dock: Some("Ctrl+E".to_string()),
            user_prefs: Some("Ctrl+P".to_string()),
            theme_diagnostics: Some("Ctrl+D".to_string()),
            toggle_preview: Some("F9".to_string()),
            exit_architect: Some("Ctrl+Q".to_string()),
            open_project: Some("Ctrl+O".to_string()),
            save_project: Some("Ctrl+S".to_string()),
            export_xml: Some("Shift+Ctrl+E".to_string()),
            reset_zoom: Some("Ctrl+0".to_string()),
        }
    }
}

impl ShortcutPrefs {
    pub fn load() -> Self {
        let path = mor_blogger_core::config::prefs::shortcuts_path();
        let Ok(toml_str) = std::fs::read_to_string(&path) else {
            let default_prefs = Self::default();
            let _ = default_prefs.save();
            return default_prefs;
        };
        toml::from_str(&toml_str).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = mor_blogger_core::config::prefs::shortcuts_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        std::fs::write(&path, toml_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorPrefs {
    #[serde(default)]
    pub plugins: Vec<PluginState>,
    #[serde(default)]
    pub ui_mode: Option<String>,
    #[serde(default)]
    pub workspace_theme: Option<String>,
    #[serde(default)]
    pub workspace_theme_preset: Option<String>,
    #[serde(default)]
    pub custom_editor_colors: CustomEditorColors,
    #[serde(default)]
    pub default_template_pack: Option<mor_blogger_core::config::TemplatePackConfig>,
}

impl EditorPrefs {
    pub fn load() -> Self {
        let path = mor_blogger_core::config::prefs::editor_prefs_path();
        if !path.exists() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        let Ok(toml_str) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str(&toml_str).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = mor_blogger_core::config::prefs::editor_prefs_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        std::fs::write(&path, toml_str)
    }

    pub fn update_ui_mode(mode: String) {
        let mut prefs = Self::load();
        prefs.ui_mode = Some(mode);
        let _ = prefs.save();
    }

    pub fn update_workspace_theme(theme: String) {
        let mut prefs = Self::load();
        prefs.workspace_theme = Some(theme);
        let _ = prefs.save();
    }

    #[allow(dead_code)] // reserved for CSS token builder button-color hook
    pub fn update_custom_btn(btn: String) {
        let mut prefs = Self::load();
        prefs.custom_editor_colors.btn = Some(btn);
        let _ = prefs.save();
    }

    pub fn update_default_template_pack(pack: mor_blogger_core::config::TemplatePackConfig) {
        let mut prefs = Self::load();
        prefs.default_template_pack = Some(pack);
        let _ = prefs.save();
    }

    pub fn clear_default_template_pack() {
        let mut prefs = Self::load();
        prefs.default_template_pack = None;
        let _ = prefs.save();
    }
}

pub fn menu_label(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|link| link.label.clone())
        .unwrap_or_default()
}

pub fn menu_url(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|link| link.url.clone())
        .unwrap_or_default()
}

pub fn panel_layout_class(layout: PanelLayout) -> &'static str {
    match layout {
        PanelLayout::Split => "split",
        PanelLayout::Wide => "wide",
        PanelLayout::Floating => "floating",
        PanelLayout::Hidden => "hidden",
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod theme_reload_watcher {
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use std::thread;
    use std::time::Duration;

    use dioxus::prelude::UnboundedSender;
    use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};

    static THEME_RELOAD_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();

    pub fn register_theme_reload_sender(tx: UnboundedSender<String>) {
        let _ = THEME_RELOAD_TX.set(tx);
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

                if let Some(tx) = THEME_RELOAD_TX.get() {
                    if tx.unbounded_send(toml_str).is_err() {
                        log::warn!("Theme hot-reload channel closed");
                    }
                }
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
}

#[cfg(not(target_arch = "wasm32"))]
pub use theme_reload_watcher::{register_theme_reload_sender, spawn_editor_prefs_watcher};
