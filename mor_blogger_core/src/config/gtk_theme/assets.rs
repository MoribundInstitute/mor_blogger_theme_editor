use std::fs;
use std::path::{Path, PathBuf};

use crate::presets::user_presets::UserPresetIconAssets;

use super::super::styling::IconConfig;
use super::super::ThemeConfig;
use super::GtkImportReport;

pub(crate) fn apply_icon_assets(
    theme_root: &Path,
    config: &mut ThemeConfig,
    report: &mut GtkImportReport,
) -> UserPresetIconAssets {
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
        return UserPresetIconAssets::default();
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
            "search-symbolic.svg",
            "edit-find-symbolic.svg",
            "more-results.svg",
        ],
        report,
    );

    let menu_svg = load_svg(
        &existing_dirs,
        &[
            "open-menu-symbolic.svg",
            "view-more-symbolic.svg",
            "menu-symbolic.svg",
            "application-menu-symbolic.svg",
        ],
        report,
    );

    let sidebar_left_svg = load_svg(
        &existing_dirs,
        &[
            "sidebar-show-symbolic.svg",
            "view-sidebar-symbolic.svg",
            "view-left-pane-symbolic.svg",
            "view-dual-symbolic.svg",
        ],
        report,
    );

    let sidebar_right_svg = load_svg(
        &existing_dirs,
        &[
            "sidebar-show-right-symbolic.svg",
            "view-sidebar-right-symbolic.svg",
            "view-right-pane-symbolic.svg",
            "sidebar-hide-symbolic.svg",
        ],
        report,
    );

    config.icons = IconConfig {
        panel_close: panel_close_svg
            .as_deref()
            .map(svg_to_mask_uri)
            .unwrap_or(before.panel_close),
        search: search_svg
            .as_deref()
            .map(svg_to_mask_uri)
            .unwrap_or(before.search),
        menu: menu_svg
            .as_deref()
            .map(svg_to_mask_uri)
            .unwrap_or(before.menu),
        sidebar_left: sidebar_left_svg
            .as_deref()
            .map(svg_to_mask_uri)
            .unwrap_or(before.sidebar_left),
        sidebar_right: sidebar_right_svg
            .as_deref()
            .map(svg_to_mask_uri)
            .unwrap_or(before.sidebar_right),
        custom_icons: std::collections::HashMap::new(),
    };

    UserPresetIconAssets {
        sidebar_left_svg,
        sidebar_right_svg,
        panel_close_svg,
        search_svg,
        menu_svg,
    }
}

fn candidate_asset_dirs(theme_root: &Path) -> Vec<PathBuf> {
    [
        "gnome-shell/assets",
        "gtk-4.0/assets",
        "gtk-3.0/assets",
        "assets",
        "icons",
        "scalable/actions",
        "symbolic/actions",
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

    format!("url(\"data:image/svg+xml,{}\")", encoded)
}