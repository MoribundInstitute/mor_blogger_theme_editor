use std::fs;
use std::path::Path;

use crate::presets::user_presets::UserPresetIconAssets;
use super::super::styling::IconConfig;
use super::super::ThemeConfig;
use super::GtkImportReport;

pub(crate) fn apply_icon_assets(
    theme_root: &Path,
    config: &mut ThemeConfig,
    report: &mut GtkImportReport,
) -> UserPresetIconAssets {
    let assets_dir = theme_root.join("gnome-shell/assets");

    if !assets_dir.is_dir() {
        report
            .warnings
            .push(format!("No GTK icon asset directory at {}", assets_dir.display()));
        eprintln!("[gtk_theme] no icon asset directory at {}", assets_dir.display());
        return UserPresetIconAssets::default();
    }

    let before = config.icons.clone();

    let panel_close_svg = load_svg(&assets_dir, &["window-close-symbolic.svg", "close-symbolic.svg", "window-close.svg"], report);
    let search_svg = load_svg(&assets_dir, &["more-results.svg", "search-symbolic.svg", "edit-find-symbolic.svg"], report);
    let menu_svg = load_svg(&assets_dir, &["open-menu-symbolic.svg", "view-more-symbolic.svg", "menu-symbolic.svg"], report);

    // NEW: Search the GNOME directory for sidebar/pane SVGs
    let sidebar_left_svg = load_svg(&assets_dir, &["sidebar-show-symbolic.svg", "view-sidebar-symbolic.svg", "view-sidebar.svg"], report);
    let sidebar_right_svg = load_svg(&assets_dir, &["sidebar-show-right-symbolic.svg", "view-sidebar-right-symbolic.svg", "view-right-pane-symbolic.svg"], report);

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
    };

    UserPresetIconAssets {
        sidebar_left_svg,
        sidebar_right_svg,
        panel_close_svg,
        search_svg,
        menu_svg,
    }
}

fn load_svg(dir: &Path, filenames: &[&str], report: &mut GtkImportReport) -> Option<String> {
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
                report
                    .warnings
                    .push(format!("Could not read icon {}: {}", path.display(), err));
                eprintln!("[gtk_theme] could not read icon {}: {}", path.display(), err);
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