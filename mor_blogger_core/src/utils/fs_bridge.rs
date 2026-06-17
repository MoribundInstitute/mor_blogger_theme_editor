//! Filesystem bridge — user-space template directory management.
//!
//! On first launch this module creates the directory tree that mirrors the
//! built-in `template_parts/` layout and seeds it with the compiled-in
//! defaults so users can browse them with any native file manager. Subsequent
//! launches skip files that already exist, preserving any user edits.

use std::path::PathBuf;
use directories::ProjectDirs;

const APP_QUALIFIER: &str = "io.moribund";
const APP_ORG: &str = "Moribund Institute";
const APP_NAME: &str = "MorBloggerThemeEditor";

/// Category subdirectories that mirror `template_parts/`.
pub const TEMPLATE_CATEGORIES: &[&str] = &[
    "headers",
    "footers",
    "sidebars",
    "layouts",
    "content",
];

/// Default modules seeded on first run. Tuple: (category, filename, content).
const DEFAULT_MODULES: &[(&str, &str, &str)] = &[
    // Headers
    ("headers", "mor_header_baseline.xml",
        include_str!("../template_parts/headers/mor_header_baseline.xml")),
    ("headers", "gtk_headerbar.xml",
        include_str!("../template_parts/headers/gtk_headerbar.xml")),
    // Footers
    ("footers", "MorFooterBasic.xml",
        include_str!("../template_parts/footers/MorFooterBasic.xml")),
    ("footers", "MorFooterCompact.xml",
        include_str!("../template_parts/footers/MorFooterCompact.xml")),
    ("footers", "MorFooterMega.xml",
        include_str!("../template_parts/footers/MorFooterMega.xml")),
    // Sidebars
    ("sidebars", "blogger_left.xml",
        include_str!("../template_parts/sidebars/blogger_left.xml")),
    ("sidebars", "gtk_dock_left.xml",
        include_str!("../template_parts/sidebars/gtk_dock_left.xml")),
    ("sidebars", "toc_right.xml",
        include_str!("../template_parts/sidebars/toc_right.xml")),
    // Layouts
    ("layouts", "sidebars.xml",
        include_str!("../template_parts/layouts/sidebars.xml")),
    ("layouts", "single_column.xml",
        include_str!("../template_parts/layouts/single_column.xml")),
    // Content
    ("content", "blog_standard.xml",
        include_str!("../template_parts/content/blog_standard.xml")),
    ("content", "mor_magazine.xml",
        include_str!("../template_parts/content/mor_magazine.xml")),
    ("content", "mor_masonry.xml",
        include_str!("../template_parts/content/mor_masonry.xml")),
    ("content", "mor_minimal.xml",
        include_str!("../template_parts/content/mor_minimal.xml")),
];

// ─── Path helpers ────────────────────────────────────────────────────────────

/// Returns the app-scoped workspace root, e.g.
/// `~/.local/share/morbloggerthemeeditor/` on Linux.
pub fn workspace_root() -> Option<PathBuf> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
        .map(|d| d.data_dir().to_path_buf())
}

/// Returns the app-scoped templates root, e.g.
/// `~/.local/share/morbloggerthemeeditor/templates/` on Linux.
pub fn templates_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("templates"))
}

/// Returns the app-scoped icons root, e.g.
/// `~/.local/share/morbloggerthemeeditor/icons/` on Linux.
pub fn icons_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("icons"))
}

/// Returns the directory for a specific category inside the templates root.
pub fn category_dir(category: &str) -> Option<PathBuf> {
    templates_root().map(|r| r.join(category))
}

// ─── Startup init ────────────────────────────────────────────────────────────

/// Create the templates directory tree and seed default modules on first launch.
/// Safe to call on every startup — existing files are never overwritten.
pub fn init_template_dirs() -> std::io::Result<()> {
    let root = templates_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;

    for category in TEMPLATE_CATEGORIES {
        std::fs::create_dir_all(root.join(category))?;
    }

    let icons = icons_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&icons)?;
    log::info!("[fs_bridge] Icons dir ready at: {}", icons.display());

    for (category, filename, content) in DEFAULT_MODULES {
        let dest = root.join(category).join(filename);
        if !dest.exists() {
            std::fs::write(&dest, content)?;
            log::info!("[fs_bridge] Seeded default template: {}/{}", category, filename);
        }
    }

    log::info!("[fs_bridge] Template dirs ready at: {}", root.display());
    Ok(())
}

// ─── Save action ─────────────────────────────────────────────────────────────

/// Write a module XML buffer (e.g. from the ModuleWorkbench editor) into the
/// matching category folder. Creates the folder if absent.
/// Returns the path the file was written to.
pub fn save_custom_module(
    category: &str,
    name: &str,
    content: &str,
) -> std::io::Result<PathBuf> {
    let dir = category_dir(category).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&dir)?;

    let safe_name = sanitize_filename(name);
    let dest = dir.join(&safe_name);
    std::fs::write(&dest, content)?;
    log::info!("[fs_bridge] Saved custom module: {}/{}", category, safe_name);
    Ok(dest)
}

// ─── Open folder ─────────────────────────────────────────────────────────────

/// Spawn the platform-native file manager focused on the templates root.
/// Creates the directory first if it does not yet exist.
pub fn open_templates_folder() -> std::io::Result<()> {
    let path = templates_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;

    // Ensure the directory exists before asking the OS to open it.
    std::fs::create_dir_all(&path)?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map(|_| ())?;

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&path)
        .spawn()
        .map(|_| ())?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&path)
        .spawn()
        .map(|_| ())?;

    Ok(())
}

/// Spawn the platform-native file manager focused on the icons root.
/// Creates the directory first if it does not yet exist.
pub fn open_icons_folder() -> Result<(), String> {
    let path = icons_root().ok_or_else(|| {
        "Cannot determine system data directory".to_string()
    })?;

    // Ensure the directory exists before asking the OS to open it.
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Strip path-traversal characters and ensure an `.xml` extension.
fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c => c,
        })
        .collect();

    let cleaned = cleaned.trim_start_matches('.').to_string();
    let cleaned = if cleaned.is_empty() { "custom_module".to_string() } else { cleaned };

    if cleaned.to_lowercase().ends_with(".xml") {
        cleaned
    } else {
        format!("{}.xml", cleaned)
    }
}
