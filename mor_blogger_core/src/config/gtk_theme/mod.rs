mod assets;
mod generator;
mod parser;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::styling::{BackgroundMode, ColorConfig, SurfaceFill};
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
    pub icon_assets: assets::GtkIconAssets,
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

pub fn import_gtk_theme(theme_root: &Path) -> Result<ImportedGtkPreset, String> {
    let mut report = GtkImportReport::default();
    let mut config = crate::config::defaults::default_theme_config();

    let (vars, files) = parser::load_theme_color_vars(theme_root)?;
    report.files_read = files;
    report.colors_found = vars.len();

    if vars.is_empty() {
        return Err(format!(
            "No valid GTK color variables found in {}",
            theme_root.display()
        ));
    }

    apply_gtk_colors(&vars, &mut config);

    let icon_assets = assets::apply_icon_assets(theme_root, &mut config, &mut report);

    let source_name = source_name_from_path(theme_root);
    let title = title_from_source_name(&source_name);
    let preset_css = generator::generate_gtk_preset_css(&config, &source_name);

    config.site.site_title = title.clone();
    config.site.site_subtitle = "Imported Linux Environment".to_string();

    Ok(ImportedGtkPreset {
        id: source_name,
        name: title,
        description: "Generated via automated parsing of GTK+ CSS variables.".to_string(),
        config,
        preset_css,
        source_dir: theme_root.to_path_buf(),
        icon_assets,
        report,
    })
}

fn apply_gtk_colors(vars: &HashMap<String, String>, config: &mut ThemeConfig) {
    let mut colors = ColorConfig::default();

    if let Some(bg) = resolve_color(vars, "theme_bg_color") {
        colors.bg_base = bg.clone();
        config.background.mode = BackgroundMode::Solid { color: bg };
    }

    if let Some(panel) = resolve_color(vars, "theme_base_color") {
        colors.bg_panel = SurfaceFill::solid(&panel);
        colors.bg_elevated = SurfaceFill::solid(&panel);
    }

    if let Some(fg) = resolve_color(vars, "theme_fg_color") {
        colors.fg_base = fg;
    }

    if let Some(text) = resolve_color(vars, "theme_text_color") {
        if colors.fg_base.is_empty() {
            colors.fg_base = text;
        } else {
            colors.fg_muted = text;
        }
    }

    if let Some(sel) = resolve_color(vars, "theme_selected_bg_color") {
        colors.accent = sel;
    }

    if let Some(border) = resolve_color(vars, "borders") {
        colors.border = border;
    }

    config.colors = colors;
}

fn resolve_color(vars: &HashMap<String, String>, key: &str) -> Option<String> {
    let raw = vars.get(&normalize_lookup_key(key))?;

    if raw.starts_with('@') {
        let alias = raw.trim_start_matches('@').trim();
        if !alias.is_empty() {
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
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
