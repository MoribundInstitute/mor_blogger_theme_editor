use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

const MAX_IMPORT_DEPTH: usize = 8;

pub(crate) fn load_theme_color_vars(theme_root: &Path) -> Result<(HashMap<String, String>, Vec<PathBuf>), String> {
    let mut vars = HashMap::new();
    let mut read_files = Vec::new();
    let mut seen = HashSet::new();

    for path in candidate_css_files(theme_root) {
        if !path.exists() {
            continue;
        }

        let content = read_css_with_imports(&path, 0, &mut seen, &mut read_files)?;

        if path.file_name().and_then(|n| n.to_str()) == Some("gtkrc") {
            parse_gtk2_color_scheme(&content, &mut vars);
        } else {
            parse_gtk_color_vars_into(&content, &mut vars);
            parse_literal_css_palette(&content, &mut vars);
        }
    }

    Ok((vars, read_files))
}

fn candidate_css_files(theme_root: &Path) -> Vec<PathBuf> {
    [
        "gtk-4.0/gtk-dark.css",
        "gtk-4.0/gtk.css",
        "gtk-3.0/gtk-dark.css",
        "gtk-3.0/gtk.css",
        "gnome-shell/gnome-shell.css",
        "cinnamon/cinnamon.css",
        "gtk-2.0/gtkrc",
    ]
    .into_iter()
    .map(|rel| theme_root.join(rel))
    .collect()
}

fn read_css_with_imports(
    path: &Path,
    depth: usize,
    seen: &mut HashSet<PathBuf>,
    read_files: &mut Vec<PathBuf>,
) -> Result<String, String> {
    if depth > MAX_IMPORT_DEPTH {
        return Ok(String::new());
    }

    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    if !seen.insert(canonical) {
        return Ok(String::new());
    }

    let css = fs::read_to_string(path)
        .map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;

    read_files.push(path.to_path_buf());

    let mut bundled = String::new();

    for import in find_local_imports(&css) {
        let import_path = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(import);

        if import_path.exists() {
            bundled.push_str(&read_css_with_imports(
                &import_path,
                depth + 1,
                seen,
                read_files,
            )?);
            bundled.push('\n');
        }
    }

    bundled.push_str(&css);
    bundled.push('\n');

    Ok(bundled)
}

fn find_local_imports(css: &str) -> Vec<String> {
    let mut imports = Vec::new();

    for line in css.lines() {
        let line = line.trim();

        if !line.starts_with("@import") {
            continue;
        }

        let Some(import) = extract_import_target(line) else {
            continue;
        };

        if import.starts_with("resource:")
            || import.starts_with("http:")
            || import.starts_with("https:")
            || import.starts_with('/')
        {
            continue;
        }

        imports.push(import);
    }

    imports
}

fn extract_import_target(line: &str) -> Option<String> {
    let raw = line
        .trim()
        .trim_start_matches("@import")
        .trim()
        .trim_end_matches(';')
        .trim();

    let raw = if raw.starts_with("url(") {
        raw.trim_start_matches("url(").trim_end_matches(')').trim()
    } else {
        raw
    };

    let raw = raw.trim_matches('"').trim_matches('\'');

    if raw.is_empty() {
        None
    } else {
        Some(raw.to_string())
    }
}

fn parse_gtk2_color_scheme(content: &str, map: &mut HashMap<String, String>) {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("gtk-color-scheme") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].rfind('"') {
                    let scheme = &line[start + 1..start + 1 + end];
                    let pairs = scheme.replace("\\n", "\n");
                    for pair in pairs.split('\n') {
                        if let Some((name, value)) = pair.split_once(':') {
                            insert_var(map, name.trim(), value.trim());
                        }
                    }
                }
            }
        }
    }
}

fn parse_literal_css_palette(css: &str, map: &mut HashMap<String, String>) {
    let mut color_counts: HashMap<String, usize> = HashMap::new();
    let mut chars = css.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut hex = String::new();
            hex.push('#');
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_hexdigit() {
                    hex.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if hex.len() == 4 || hex.len() == 7 {
                *color_counts.entry(hex.to_lowercase()).or_insert(0) += 1;
            }
        }
    }

    let mut sorted: Vec<_> = color_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    if let Some((color, _)) = sorted.get(0) {
        map.entry("literal_fallback_1".to_string()).or_insert(color.clone());
    }
    if let Some((color, _)) = sorted.get(1) {
        map.entry("literal_fallback_2".to_string()).or_insert(color.clone());
    }
    if let Some((color, _)) = sorted.get(2) {
        map.entry("literal_fallback_3".to_string()).or_insert(color.clone());
    }
}

fn parse_gtk_color_vars_into(css: &str, map: &mut HashMap<String, String>) {
    let css = strip_css_comments(css);

    for part in css.split(';') {
        let part = part.trim();

        if let Some(rest) = part.strip_prefix("@define-color") {
            let rest = rest.trim();
            if let Some((name, value)) = rest.split_once(char::is_whitespace) {
                insert_var(map, name, value);
            }
        }
    }

    for part in css.split(';') {
        let Some(start) = part.find("--") else { continue; };
        let candidate = &part[start..];
        let Some((name, value)) = candidate.split_once(':') else { continue; };
        insert_var(map, name, value);
    }
}

fn insert_var(map: &mut HashMap<String, String>, name: &str, value: &str) {
    let key = normalize_key(name);
    let value = clean_value(value);

    if key.is_empty() || value.is_empty() {
        return;
    }
    map.entry(key).or_insert(value);
}

pub(super) fn normalize_key(name: &str) -> String {
    name.trim()
        .trim_start_matches("--")
        .trim_start_matches('@')
        .replace('-', "_")
}

fn clean_value(value: &str) -> String {
    value
        .trim()
        .trim_end_matches(';')
        .trim()
        .trim_end_matches("!important")
        .trim()
        .to_string()
}

fn strip_css_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    let mut in_comment = false;

    while let Some(ch) = chars.next() {
        if in_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_comment = false;
            }
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
            continue;
        }
        out.push(ch);
    }
    out
}