mod assets;
mod generator;
mod parser;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::presets::user_presets::{sanitize_preset_id, UserPresetIconAssets};

use super::styling::{BackgroundConfig, BackgroundMode, ColorConfig, SurfaceFill};
use super::ThemeConfig;

pub use assets::svg_to_mask_uri;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GtkImportReport {
    pub files_read: Vec<PathBuf>,
    pub colors_found: usize,
    pub icons_found: usize,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportedGtkPreset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: ThemeConfig,
    pub preset_css: String,
    pub source_dir: PathBuf,
    pub icon_assets: UserPresetIconAssets,
    pub report: GtkImportReport,
}

impl GtkImportReport {
    pub fn short_status(&self) -> String {
        format!(
            "{} color var(s), {} icon(s), {} CSS file(s), {} warning(s)",
            self.colors_found,
            self.icons_found,
            self.files_read.len(),
            self.warnings.len()
        )
    }
}

impl ImportedGtkPreset {
    pub fn to_user_preset_bundle(&self) -> crate::presets::user_presets::UserPresetBundle {
        crate::presets::user_presets::UserPresetBundle {
            version: crate::presets::user_presets::USER_PRESET_BUNDLE_VERSION,
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            preset_css: self.preset_css.clone(),
            config: self.config.clone(),
            source: crate::presets::user_presets::PresetSourceInfo {
                source_kind: "gtk4".to_string(),
                source_path: Some(self.source_dir.display().to_string()),
                imported_at: Some(current_unix_timestamp_string()),
                importer_version: "gtk-importer-v1".to_string(),
                warnings: self.report.warnings.clone(),
            },
        }
    }
}

fn current_unix_timestamp_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

/// Compatibility API for older callers that only want to mutate the active
/// ThemeConfig immediately.
pub fn apply_gtk4_theme(theme_root: &Path, config: &mut ThemeConfig) -> Result<(), String> {
    let imported = import_gtk4_preset(theme_root, config)?;
    *config = imported.config;
    Ok(())
}

/// Full GTK importer API used by the preset UI.
///
/// This produces a Blogger-shaped imported preset:
/// - ThemeConfig colors/background/icons
/// - generated Blogger preset CSS
/// - raw SVG icon assets for saving into user preset bundles
/// - import report/warnings for UI/debug output
pub fn import_gtk4_preset(
    theme_root: &Path,
    current_config: &ThemeConfig,
) -> Result<ImportedGtkPreset, String> {
    let mut report = GtkImportReport::default();
    let mut config = current_config.clone();

    let (vars, files_read) = parser::load_theme_color_vars(theme_root)?;
    report.files_read = files_read;
    report.colors_found = vars.len();

    if vars.is_empty() {
        report
            .warnings
            .push("No GTK color variables found; colors were left unchanged.".to_string());
    } else {
        apply_color_vars_to_config(&vars, &mut config);
    }

    let source_name = source_name_from_path(theme_root);
    let id = sanitize_preset_id(&source_name);
    let name = title_from_source_name(&source_name);
    let description = format!("GTK-derived Blogger preset generated from {}.", name);

    let icon_assets = assets::apply_icon_assets(theme_root, &mut config, &mut report);

    let preset_css = generator::generate_gtk_preset_css(&config, &name);
    config.preset_css = preset_css.clone();

    eprintln!(
        "[gtk_theme] imported {}: {} color vars, {} icon(s), {} CSS file(s), {} warning(s)",
        name,
        report.colors_found,
        report.icons_found,
        report.files_read.len(),
        report.warnings.len()
    );

    Ok(ImportedGtkPreset {
        id,
        name,
        description,
        config,
        preset_css,
        source_dir: theme_root.to_path_buf(),
        icon_assets,
        report,
    })
}

fn apply_color_vars_to_config(vars: &HashMap<String, String>, config: &mut ThemeConfig) {
    let bg_base = get_color(
        vars,
        &[
            "base_color",
            "window_bg_color",
            "view_bg_color",
            "theme_base_color",
            "literal_fallback_1",
        ],
        &config.colors.bg_base,
    );

    let bg_panel = get_color(
        vars,
        &[
            "bg_color",
            "menu_bg",
            "headerbar_bg_color",
            "sidebar_bg_color",
            "dark_sidebar_bg",
            "theme_bg_color",
            "literal_fallback_2",
        ],
        &config.colors.bg_panel.color,
    );

    let bg_elevated = get_color(
        vars,
        &[
            "dark_sidebar_bg",
            "popover_bg_color",
            "card_bg_color",
            "tooltip_bg_color",
            "menu_bg",
            "literal_fallback_3",
        ],
        &config.colors.bg_elevated.color,
    );

    let fg_base = get_color(
        vars,
        &[
            "text_color",
            "fg_color",
            "window_fg_color",
            "view_fg_color",
            "theme_text_color",
            "theme_fg_color",
            "menu_fg",
        ],
        &config.colors.fg_base,
    );

    let fg_muted = get_color(
        vars,
        &[
            "insensitive_fg_color",
            "insensitive_button_fg_color",
            "dim_label_color",
            "inactive_text_color",
            "tooltip_fg_color",
        ],
        &config.colors.fg_muted,
    );

    let accent = get_color(
        vars,
        &[
            "selected_bg_color",
            "accent_color",
            "link_color",
            "theme_selected_bg_color",
            "selection_background_color",
        ],
        &config.colors.accent,
    );

    let border = get_color(
        vars,
        &[
            "borders",
            "border_color",
            "active_window_border",
            "inactive_window_border",
            "literal_fallback_3",
        ],
        &config.colors.border,
    );

    config.colors = ColorConfig {
        bg_base: bg_base.clone(),
        bg_panel: SurfaceFill::solid(&bg_panel),
        bg_elevated: SurfaceFill::solid(&bg_elevated),
        fg_base,
        fg_muted,
        accent,
        border,
    };

    config.background = BackgroundConfig {
        mode: BackgroundMode::Solid { color: bg_base },
    };

    // GTK-like defaults. These are intentionally conservative and can be made
    // more advanced later when the importer reads border-radius/font hints.
    config.buttons.radius = "8px".to_string();
    config.buttons.border_width = "1px".to_string();
    config.buttons.text_transform = "none".to_string();

    // ==========================================
    // NEW: FORCE GTK TEMPLATE LAYOUTS ON IMPORT
    // ==========================================
    config.template_pack.header_variant = "gtk_headerbar".to_string();
    config.template_pack.main_variant = "sidebars".to_string();
    config.template_pack.content_variant = "blog_standard".to_string();
    config.template_pack.left_sidebar_variant = "gtk_dock_left".to_string();
    config.template_pack.script_variant = "minimal".to_string();
}

fn get_color(vars: &HashMap<String, String>, keys: &[&str], fallback: &str) -> String {
    for key in keys {
        if let Some(value) = resolve_color(vars, key) {
            if is_supported_css_color(&value) {
                return value;
            }
        }
    }

    fallback.to_string()
}

fn resolve_color(vars: &HashMap<String, String>, key: &str) -> Option<String> {
    let raw = vars.get(key)?.trim();

    if let Some(alias) = raw.strip_prefix('@') {
        let alias = normalize_lookup_key(alias);
        return resolve_color(vars, &alias);
    }

    if let Some(alias) = raw.strip_prefix("var(--") {
        if let Some(alias) = alias.strip_suffix(')') {
            let alias = normalize_lookup_key(alias);
            return resolve_color(vars, &alias);
        }
    }

    Some(clean_color_value(raw))
}

fn clean_color_value(value: &str) -> String {
    value
        .trim()
        .trim_end_matches(';')
        .trim_end_matches("!important")
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn normalize_lookup_key(key: &str) -> String {
    key.trim()
        .trim_start_matches("--")
        .trim_start_matches('@')
        .replace('-', "_")
}

fn is_supported_css_color(value: &str) -> bool {
    let value = value.trim();

    value.starts_with('#')
        || value.starts_with("rgb(")
        || value.starts_with("rgba(")
        || value.starts_with("hsl(")
        || value.starts_with("hsla(")
}

fn source_name_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("imported-gtk-theme")
        .to_string()
}

fn title_from_source_name(source_name: &str) -> String {
    source_name
        .replace(['_', '-'], " ")
        .split_whitespace()
        .map(title_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn title_word(word: &str) -> String {
    let mut chars = word.chars();

    let Some(first) = chars.next() else {
        return String::new();
    };

    let mut out = String::new();
    out.extend(first.to_uppercase());
    out.push_str(chars.as_str());

    out
}