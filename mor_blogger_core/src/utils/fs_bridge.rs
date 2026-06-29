//! Filesystem bridge — user-space template directory management.
//!
//! On first launch this module creates the directory tree that mirrors the
//! built-in `template_parts/` layout and seeds it with the compiled-in
//! defaults so users can browse them with any native file manager. Subsequent
//! launches skip files that already exist, preserving any user edits.

use directories::ProjectDirs;
use std::path::PathBuf;

const APP_QUALIFIER: &str = "io.moribund";
const APP_ORG: &str = "Moribund Institute";
const APP_NAME: &str = "MorBloggerThemeEditor";

/// Category subdirectories that mirror `template_parts/`.
pub const TEMPLATE_CATEGORIES: &[&str] = &["headers", "footers", "sidebars", "layouts", "content"];

/// Widget blueprint groups (logical classification) seeded under `workspace/widgets/`.
pub const WIDGET_GROUPS: &[&str] = &["content", "navigation", "archive", "gadgets", "custom"];

/// Default widget blueprints seeded on first run. Tuple: (group, filename, content).
/// Reuses the existing `<b:widget>` templates plus a couple of new blueprints; the
/// `mor-blogger-widget-blueprints` repo can grow this library by dropping more files
/// into the seeded `workspace/widgets/<group>/` tree.
const WIDGET_BLUEPRINTS: &[(&str, &str, &str)] = &[
    (
        "content",
        "blog.xml",
        include_str!("../template_parts/widgets/blog1.xml"),
    ),
    (
        "content",
        "featured-post.xml",
        include_str!("../template_parts/widget_blueprints/content/featured-post.xml"),
    ),
    (
        "navigation",
        "labels.xml",
        include_str!("../template_parts/widgets/label1.xml"),
    ),
    (
        "navigation",
        "page-list.xml",
        include_str!("../template_parts/widget_blueprints/navigation/page-list.xml"),
    ),
    (
        "archive",
        "blog-archive.xml",
        include_str!("../template_parts/widgets/blogarchive1.xml"),
    ),
    (
        "custom",
        "html.xml",
        include_str!("../template_parts/widgets/html1.xml"),
    ),
    // Blogger built-in gadgets. Title is tokenized (data-field-path); other knobs
    // are edited in the Code tab — the app has no per-field override store yet.
    (
        "gadgets",
        "wikipedia.xml",
        include_str!("../template_parts/widget_blueprints/gadgets/wikipedia.xml"),
    ),
    (
        "gadgets",
        "translate.xml",
        include_str!("../template_parts/widget_blueprints/gadgets/translate.xml"),
    ),
];

/// Default modules seeded on first run. Tuple: (category, filename, content).
const DEFAULT_MODULES: &[(&str, &str, &str)] = &[
    // Headers
    (
        "headers",
        "mor_header_baseline.xml",
        include_str!("../template_parts/headers/mor_header_baseline.xml"),
    ),
    (
        "headers",
        "gtk_headerbar.xml",
        include_str!("../template_parts/headers/gtk_headerbar.xml"),
    ),
    (
        "headers",
        "mor_header_minimal.xml",
        include_str!("../template_parts/headers/mor_header_minimal.xml"),
    ),
    // Footers
    (
        "footers",
        "MorFooterBasic.xml",
        include_str!("../template_parts/footers/MorFooterBasic.xml"),
    ),
    (
        "footers",
        "MorFooterCompact.xml",
        include_str!("../template_parts/footers/MorFooterCompact.xml"),
    ),
    (
        "footers",
        "MorFooterMega.xml",
        include_str!("../template_parts/footers/MorFooterMega.xml"),
    ),
    (
        "footers",
        "MorFooterSocial.xml",
        include_str!("../template_parts/footers/MorFooterSocial.xml"),
    ),
    // Sidebars
    (
        "sidebars",
        "blogger_left.xml",
        include_str!("../template_parts/sidebars/blogger_left.xml"),
    ),
    (
        "sidebars",
        "gtk_dock_left.xml",
        include_str!("../template_parts/sidebars/gtk_dock_left.xml"),
    ),
    (
        "sidebars",
        "toc_right.xml",
        include_str!("../template_parts/sidebars/toc_right.xml"),
    ),
    // Layouts
    (
        "layouts",
        "sidebars.xml",
        include_str!("../template_parts/layouts/sidebars.xml"),
    ),
    (
        "layouts",
        "single_column.xml",
        include_str!("../template_parts/layouts/single_column.xml"),
    ),
    (
        "layouts",
        "two_column_right.xml",
        include_str!("../template_parts/layouts/two_column_right.xml"),
    ),
    // Content
    (
        "content",
        "blog_standard.xml",
        include_str!("../template_parts/content/blog_standard.xml"),
    ),
    (
        "content",
        "mor_magazine.xml",
        include_str!("../template_parts/content/mor_magazine.xml"),
    ),
    (
        "content",
        "mor_masonry.xml",
        include_str!("../template_parts/content/mor_masonry.xml"),
    ),
    (
        "content",
        "mor_minimal.xml",
        include_str!("../template_parts/content/mor_minimal.xml"),
    ),
];

// ─── Path helpers ────────────────────────────────────────────────────────────

/// Returns the app-scoped workspace root, e.g.
/// `~/.local/share/morbloggerthemeeditor/` on Linux.
pub fn workspace_root() -> Option<PathBuf> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME).map(|d| d.data_dir().to_path_buf())
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

/// Returns the app-scoped css root, e.g.
/// `~/.local/share/morbloggerthemeeditor/css/` on Linux.
pub fn css_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("css"))
}

/// Returns the app-scoped js root, e.g.
/// `~/.local/share/morbloggerthemeeditor/js/` on Linux.
pub fn js_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("js"))
}

/// Returns the app-scoped plugins root, e.g.
/// `~/.local/share/morbloggerthemeeditor/plugins/` on Linux.
pub fn plugins_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("plugins"))
}

/// Returns the app-scoped pages root, e.g.
/// `~/.local/share/morbloggerthemeeditor/pages/` on Linux.
pub fn pages_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("pages"))
}

/// Returns the directory for a specific category inside the templates root.
pub fn category_dir(category: &str) -> Option<PathBuf> {
    templates_root().map(|r| r.join(category))
}

/// Returns the app-scoped widget-blueprints root, e.g.
/// `~/.local/share/morbloggerthemeeditor/widgets/` on Linux.
pub fn widgets_root() -> Option<PathBuf> {
    workspace_root().map(|r| r.join("widgets"))
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

    let css = css_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&css)?;
    log::info!("[fs_bridge] CSS dir ready at: {}", css.display());

    let js = js_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&js)?;
    log::info!("[fs_bridge] JS dir ready at: {}", js.display());

    let plugins = plugins_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&plugins)?;
    log::info!("[fs_bridge] Plugins dir ready at: {}", plugins.display());

    for (category, filename, content) in DEFAULT_MODULES {
        let dest = root.join(category).join(filename);
        if !dest.exists() {
            std::fs::write(&dest, content)?;
            log::info!(
                "[fs_bridge] Seeded default template: {}/{}",
                category,
                filename
            );
        }
    }

    let widgets = widgets_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    for group in WIDGET_GROUPS {
        std::fs::create_dir_all(widgets.join(group))?;
    }
    for (group, filename, content) in WIDGET_BLUEPRINTS {
        let dest = widgets.join(group).join(filename);
        if !dest.exists() {
            std::fs::write(&dest, content)?;
            log::info!("[fs_bridge] Seeded widget blueprint: {}/{}", group, filename);
        }
    }
    log::info!("[fs_bridge] Widget blueprints ready at: {}", widgets.display());

    log::info!("[fs_bridge] Template dirs ready at: {}", root.display());
    Ok(())
}

// ─── Save action ─────────────────────────────────────────────────────────────

/// Write a module XML buffer (e.g. from the ModuleWorkbench editor) into the
/// matching category folder. Creates the folder if absent.
/// Returns the path the file was written to.
pub fn save_custom_module(category: &str, name: &str, content: &str) -> std::io::Result<PathBuf> {
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
    log::info!(
        "[fs_bridge] Saved custom module: {}/{}",
        category,
        safe_name
    );
    Ok(dest)
}

// ─── Widget blueprints ───────────────────────────────────────────────────────

/// One widget blueprint: its group folder, file stem, and `<b:widget>` XML body.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetBlueprint {
    pub group: String,
    pub name: String,
    pub xml: String,
}

/// Read every widget blueprint under `workspace/widgets/<group>/*.xml`, sorted by
/// (group, name). Returns empty if the tree hasn't been seeded yet.
pub fn load_widget_blueprints() -> Vec<WidgetBlueprint> {
    let mut out = Vec::new();
    let Some(root) = widgets_root() else {
        return out;
    };
    let Ok(groups) = std::fs::read_dir(&root) else {
        return out;
    };
    for g in groups.flatten() {
        if !g.path().is_dir() {
            continue;
        }
        let group = g.file_name().to_string_lossy().to_string();
        let Ok(files) = std::fs::read_dir(g.path()) else {
            continue;
        };
        for f in files.flatten() {
            let p = f.path();
            if p.extension().and_then(|e| e.to_str()) != Some("xml") {
                continue;
            }
            let Ok(xml) = std::fs::read_to_string(&p) else {
                continue;
            };
            let name = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("widget")
                .to_string();
            out.push(WidgetBlueprint {
                group: group.clone(),
                name,
                xml,
            });
        }
    }
    out.sort_by(|a, b| (&a.group, &a.name).cmp(&(&b.group, &b.name)));
    out
}

/// One template-module file: its category, file stem, and XML.
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleFile {
    pub category: String,
    pub name: String,
    pub xml: String,
}

/// Read every module XML under `<templates_root>/<category>/*.xml` (headers,
/// footers, sidebars, layouts, content), sorted by (category, name). Mirrors
/// `load_widget_blueprints` for the "Share Creations" export.
pub fn load_modules() -> Vec<ModuleFile> {
    let mut out = Vec::new();
    for category in TEMPLATE_CATEGORIES {
        let Some(dir) = category_dir(category) else {
            continue;
        };
        let Ok(files) = std::fs::read_dir(&dir) else {
            continue;
        };
        for f in files.flatten() {
            let p = f.path();
            if p.extension().and_then(|e| e.to_str()) != Some("xml") {
                continue;
            }
            let Ok(xml) = std::fs::read_to_string(&p) else {
                continue;
            };
            let name = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("module")
                .to_string();
            out.push(ModuleFile {
                category: category.to_string(),
                name,
                xml,
            });
        }
    }
    out.sort_by(|a, b| (&a.category, &a.name).cmp(&(&b.category, &b.name)));
    out
}

/// Write a widget blueprint into `workspace/widgets/<group>/<name>.xml`.
/// Creates the group folder if absent. Returns the path written.
pub fn save_widget_blueprint(group: &str, name: &str, content: &str) -> std::io::Result<PathBuf> {
    let safe_group: String = group
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    let safe_group = if safe_group.is_empty() {
        "custom".to_string()
    } else {
        safe_group
    };
    let dir = widgets_root()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cannot determine system data directory",
            )
        })?
        .join(safe_group);
    std::fs::create_dir_all(&dir)?;

    let safe_name = sanitize_filename(name);
    let dest = dir.join(&safe_name);
    std::fs::write(&dest, content)?;
    log::info!("[fs_bridge] Saved widget blueprint: {}", dest.display());
    Ok(dest)
}

/// Delete a saved module override (`templates/<category>/<name>.xml`), e.g. on
/// Revert. Missing file is treated as success.
pub fn delete_custom_module(category: &str, name: &str) -> std::io::Result<()> {
    let Some(path) = category_dir(category).map(|d| d.join(sanitize_filename(name))) else {
        return Ok(());
    };
    match std::fs::remove_file(&path) {
        Ok(()) => {
            log::info!("[fs_bridge] Deleted custom module: {}", path.display());
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

/// Delete a widget blueprint file. Missing file is treated as success.
pub fn delete_widget_blueprint(group: &str, name: &str) -> std::io::Result<()> {
    let safe_group: String = group
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    let safe_group = if safe_group.is_empty() {
        "custom".to_string()
    } else {
        safe_group
    };
    let path = widgets_root()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cannot determine system data directory",
            )
        })?
        .join(safe_group)
        .join(sanitize_filename(name));
    match std::fs::remove_file(&path) {
        Ok(()) => {
            log::info!("[fs_bridge] Deleted widget blueprint: {}", path.display());
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

// ─── Open folder ─────────────────────────────────────────────────────────────

/// Spawn the platform-native file manager focused on the templates root.
/// Creates the directory first if it does not yet exist.
/// Open a directory in the platform-native file manager, creating it first.
fn open_dir(path: PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(&path)?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open").arg(&path).spawn().map(|_| ())?;

    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(&path).spawn().map(|_| ())?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer").arg(&path).spawn().map(|_| ())?;

    Ok(())
}

/// Spawn the file manager focused on the static-pages root (`workspace/pages/`).
pub fn open_pages_folder() -> std::io::Result<()> {
    let path = pages_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    open_dir(path)
}

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

/// Spawn the platform-native file manager focused on the widgets-blueprint root.
/// Creates the directory first if it does not yet exist.
pub fn open_widgets_folder() -> std::io::Result<()> {
    let path = widgets_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;

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
    let path = icons_root().ok_or_else(|| "Cannot determine system data directory".to_string())?;

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
    let cleaned = if cleaned.is_empty() {
        "custom_module".to_string()
    } else {
        cleaned
    };

    if cleaned.to_lowercase().ends_with(".xml") {
        cleaned
    } else {
        format!("{}.xml", cleaned)
    }
}

/// Strip path-traversal characters and ensure a `.css` extension.
fn sanitize_css_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c => c,
        })
        .collect();

    let cleaned = cleaned.trim_start_matches('.').to_string();
    let cleaned = if cleaned.is_empty() {
        "custom_css".to_string()
    } else {
        cleaned
    };

    if cleaned.to_lowercase().ends_with(".css") {
        cleaned
    } else {
        format!("{}.css", cleaned)
    }
}

/// Write a CSS buffer (e.g. from the VFS) into the css folder.
/// Creates the folder if absent.
/// Returns the path the file was written to.
pub fn save_custom_css(filename: &str, content: &str) -> std::io::Result<PathBuf> {
    let dir = css_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&dir)?;

    let safe_name = sanitize_css_filename(filename);
    let dest = dir.join(&safe_name);
    std::fs::write(&dest, content)?;
    log::info!("[fs_bridge] Saved custom CSS: {}", safe_name);
    Ok(dest)
}

/// Load all custom CSS files from the css directory.
/// If the directory doesn't exist, returns an empty HashMap.
pub fn load_all_custom_css() -> std::io::Result<std::collections::HashMap<String, String>> {
    let mut css_files = std::collections::HashMap::new();
    let dir = match css_root() {
        Some(d) => d,
        None => return Ok(css_files),
    };
    if !dir.exists() {
        return Ok(css_files);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "css" {
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                        let content = std::fs::read_to_string(&path)?;
                        css_files.insert(filename.to_string(), content);
                    }
                }
            }
        }
    }
    Ok(css_files)
}

/// Strip path-traversal characters and ensure a `.js` extension.
fn sanitize_js_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c => c,
        })
        .collect();

    let cleaned = cleaned.trim_start_matches('.').to_string();
    let cleaned = if cleaned.is_empty() {
        "custom_js".to_string()
    } else {
        cleaned
    };

    if cleaned.to_lowercase().ends_with(".js") {
        cleaned
    } else {
        format!("{}.js", cleaned)
    }
}

/// Write a JS buffer (e.g. from the VFS) into the js folder.
/// Creates the folder if absent.
/// Returns the path the file was written to.
pub fn save_custom_js(filename: &str, content: &str) -> std::io::Result<PathBuf> {
    let dir = js_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&dir)?;

    let safe_name = sanitize_js_filename(filename);
    let dest = dir.join(&safe_name);
    std::fs::write(&dest, content)?;
    log::info!("[fs_bridge] Saved custom JS: {}", safe_name);
    Ok(dest)
}

/// Load all custom JS files from the js directory.
/// If the directory doesn't exist, returns an empty HashMap.
pub fn load_all_custom_js() -> std::io::Result<std::collections::HashMap<String, String>> {
    let mut js_files = std::collections::HashMap::new();
    let dir = match js_root() {
        Some(d) => d,
        None => return Ok(js_files),
    };
    if !dir.exists() {
        return Ok(js_files);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "js" {
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                        let content = std::fs::read_to_string(&path)?;
                        js_files.insert(filename.to_string(), content);
                    }
                }
            }
        }
    }
    Ok(js_files)
}

/// Copies the given plugin file into the plugins directory.
pub fn install_local_plugin(file_path: &std::path::Path) -> std::io::Result<PathBuf> {
    let plugins_dir = plugins_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;

    // Ensure the folder exists (in case it was deleted or not created)
    std::fs::create_dir_all(&plugins_dir)?;

    let file_name = file_path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File path does not have a valid filename",
        )
    })?;

    let dest = plugins_dir.join(file_name);
    std::fs::copy(file_path, &dest)?;
    log::info!("[fs_bridge] Installed local plugin to: {}", dest.display());
    Ok(dest)
}

/// Write a static page HTML buffer into the pages folder.
/// Creates the folder if absent. Returns the path the file was written to.
pub fn save_custom_page(name: &str, content: &str) -> std::io::Result<PathBuf> {
    let dir = pages_root().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine system data directory",
        )
    })?;
    std::fs::create_dir_all(&dir)?;

    let safe_name = format!("{}.html", name.to_lowercase());
    let dest = dir.join(&safe_name);
    std::fs::write(&dest, content)?;
    log::info!("[fs_bridge] Saved custom static page HTML: {}", safe_name);
    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Round-trips a blueprint through save -> load: verifies group sanitization,
    // .xml enforcement, and that the dir-walk reads it back. Self-cleaning;
    // skips when no system data dir is available (e.g. headless CI).
    #[test]
    fn widget_blueprint_round_trip() {
        let Some(root) = widgets_root() else { return };
        let group = "zz_test_group";
        let dir = root.join(group);
        let _ = std::fs::remove_dir_all(&dir); // start clean

        let path = save_widget_blueprint(group, "round/trip", "<b:widget id='X'/>").unwrap();
        // Path-traversal stripped from the name, .xml appended.
        assert!(path.ends_with("zz_test_group/round_trip.xml"));

        let found = load_widget_blueprints()
            .into_iter()
            .find(|b| b.group == group && b.name == "round_trip")
            .expect("saved blueprint should load back");
        assert_eq!(found.xml, "<b:widget id='X'/>");

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
