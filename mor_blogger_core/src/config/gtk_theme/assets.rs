use std::fs;
use std::path::{Path, PathBuf};

use super::super::ThemeConfig;
use super::GtkImportReport;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GtkIconAssets {
    pub sidebar_left_svg: Option<String>,
    pub sidebar_right_svg: Option<String>,
    pub panel_close_svg: Option<String>,
    pub search_svg: Option<String>,
    pub menu_svg: Option<String>,
}

pub(crate) fn apply_icon_assets(
    theme_root: &Path,
    config: &mut ThemeConfig,
    report: &mut GtkImportReport,
) -> GtkIconAssets {
    let asset_dirs = candidate_asset_dirs(theme_root);

    let existing_dirs: Vec<PathBuf> = asset_dirs
        .into_iter()
        .filter(|path| path.is_dir())
        .collect();

    if existing_dirs.is_empty() {
        report.warnings.push(format!(
            "No GTK SVG asset directories found under {}",
            theme_root.display()
        ));
        eprintln!(
            "[gtk_theme] no SVG asset directories found under {}",
            theme_root.display()
        );
        return GtkIconAssets::default();
    }

    let before = config.icons.clone();

    let panel_close_svg = load_svg(
        &existing_dirs,
        &[
            "window-close-symbolic.svg",
            "close-symbolic.svg",
            "window-close.svg",
            "action-unavailable-symbolic.svg",
            "cross-large-symbolic.svg",
        ],
        report,
    );

    let search_svg = load_svg(
        &existing_dirs,
        &[
            "system-search-symbolic.svg",
            "edit-find-symbolic.svg",
            "search-symbolic.svg",
        ],
        report,
    );

    let menu_svg = load_svg(
        &existing_dirs,
        &[
            "open-menu-symbolic.svg",
            "view-more-symbolic.svg",
            "menu-symbolic.svg",
            "format-justify-fill-symbolic.svg",
        ],
        report,
    );

    let sidebar_left_svg = load_svg(
        &existing_dirs,
        &[
            "sidebar-show-symbolic.svg",
            "view-sidebar-symbolic.svg",
            "layout-sidebar-left-symbolic.svg",
        ],
        report,
    );

    let sidebar_right_svg = load_svg(
        &existing_dirs,
        &[
            "sidebar-show-right-symbolic.svg",
            "layout-sidebar-right-symbolic.svg",
        ],
        report,
    );

    if let Some(ref svg) = panel_close_svg {
        config.icons.panel_close = svg_to_mask_uri(svg);
    }
    if let Some(ref svg) = search_svg {
        config.icons.search = svg_to_mask_uri(svg);
    }
    if let Some(ref svg) = menu_svg {
        config.icons.menu = svg_to_mask_uri(svg);
    }
    if let Some(ref svg) = sidebar_left_svg {
        config.icons.sidebar_left = svg_to_mask_uri(svg);
    }
    if let Some(ref svg) = sidebar_right_svg {
        config.icons.sidebar_right = svg_to_mask_uri(svg);
    }

    if config.icons == before {
        report
            .warnings
            .push("Found SVG directories, but no matching icon names were found.".to_string());
    }

    GtkIconAssets {
        sidebar_left_svg,
        sidebar_right_svg,
        panel_close_svg,
        search_svg,
        menu_svg,
    }
}

fn candidate_asset_dirs(theme_root: &Path) -> Vec<PathBuf> {
    vec![
        "gtk-4.0/assets",
        "gtk-3.0/assets",
        "assets",
        "cinnamon/assets",
        "gnome-shell/assets",
        "Adwaita/scalable/ui",
        "Adwaita/symbolic/ui",
        "Adwaita/scalable/actions",
        "Adwaita/symbolic/actions",
    ]
    .into_iter()
    .map(|rel| theme_root.join(rel))
    .collect()
}

fn load_svg(dirs: &[PathBuf], filenames: &[&str], report: &mut GtkImportReport) -> Option<String> {
    for dir in dirs {
        for filename in filenames {
            let path = dir.join(filename);

            if !path.exists() {
                continue;
            }

            match fs::read_to_string(&path) {
                Ok(svg) => {
                    report.icons_found += 1;
                    eprintln!("[gtk_theme] loaded icon {}", path.display());
                    return Some(svg);
                }
                Err(err) => {
                    report.warnings.push(format!(
                        "Could not read icon {}: {}",
                        path.display(),
                        err
                    ));
                    eprintln!(
                        "[gtk_theme] could not read icon {}: {}",
                        path.display(),
                        err
                    );
                }
            }
        }
    }

    None
}

pub fn svg_to_mask_uri(svg: &str) -> String {
    let encoded = svg
        .replace('"', "%22")
        .replace('#', "%23")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('\n', "%0A")
        .replace('\r', "")
        .replace(' ', "%20");

    format!("url('data:image/svg+xml,{}')", encoded)
}
