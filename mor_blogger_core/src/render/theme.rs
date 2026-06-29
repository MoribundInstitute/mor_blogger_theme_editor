//! Public Blogger theme export API.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use super::xml_generator;
use crate::config::ThemeConfig;
use crate::utils::fs_bridge;

pub fn render_theme(config: &ThemeConfig, vfs: &HashMap<String, String>) -> String {
    eprintln!(
        "[render_theme] preset_css bytes = {}",
        config.preset_css.len()
    );

    strip_editor_attrs(xml_generator::render_template(config, vfs))
}

/// Remove editor-only widget markers (`data-mor-field`) from the final theme XML so
/// they never ship in the exported Blogger theme. They drive the Widget Workbench's
/// field form but carry no meaning on the live blog.
fn strip_editor_attrs(mut xml: String) -> String {
    for needle in [" data-mor-field='", " data-mor-field=\""] {
        let q = needle.as_bytes()[needle.len() - 1] as char;
        while let Some(p) = xml.find(needle) {
            let after = &xml[p + needle.len()..];
            let Some(e) = after.find(q) else { break };
            let end = p + needle.len() + e + q.len_utf8();
            xml.replace_range(p..end, "");
        }
    }
    xml
}

#[cfg(test)]
mod tests {
    use super::strip_editor_attrs;

    #[test]
    fn strips_editor_only_markers() {
        let xml = "<a href='x' data-mor-field='href|Wiki base URL' target='_blank'>".to_string();
        let out = strip_editor_attrs(xml);
        assert_eq!(out, "<a href='x' target='_blank'>");
        assert!(!out.contains("data-mor-field"));
    }
}

/// Write arbitrary `(filename, contents)` pairs into a single `.zip` at `dest`.
/// Used to package a shareable custom creation (widget/module + manifest + README).
pub fn write_files_zip(dest: &Path, files: &[(String, String)]) -> std::io::Result<()> {
    let file = File::create(dest)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, content) in files {
        zip.start_file(name.as_str(), options)?;
        zip.write_all(content.as_bytes())?;
    }
    zip.finish()?;
    Ok(())
}

pub fn save_xml_to_disk(xml_content: &str, file_path: &Path) -> Result<String, String> {
    match fs::write(file_path, xml_content) {
        Ok(_) => Ok(format!("System success: Theme exported to {:?}", file_path)),
        Err(e) => Err(format!("I/O Error: {}", e)),
    }
}

/// Packages the master XML, the raw TOML config, and the user's custom CSS
/// into a standard deployment .zip file.
pub fn save_bundle_to_disk(
    xml_content: &str,
    toml_content: &str,
    dest_path: &Path,
) -> std::io::Result<()> {
    let file = File::create(dest_path)?;
    let mut zip = ZipWriter::new(file);

    // Use SimpleFileOptions for newer `zip` crate versions, or FileOptions for older ones.
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 1. Write the compiled Blogger XML payload
    zip.start_file("theme.xml", options)?;
    zip.write_all(xml_content.as_bytes())?;

    // 2. Write the raw TOML configuration backup
    zip.start_file("theme_config.toml", options)?;
    zip.write_all(toml_content.as_bytes())?;

    // 3. Scrape the local workspace and bundle all CSS overrides
    if let Some(css_dir) = fs_bridge::css_root() {
        if css_dir.exists() {
            zip.add_directory("css/", options)?;

            for entry in std::fs::read_dir(css_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("css") {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        zip.start_file(format!("css/{}", filename), options)?;
                        let mut f = File::open(&path)?;
                        let mut buffer = Vec::new();
                        f.read_to_end(&mut buffer)?;
                        zip.write_all(&buffer)?;
                    }
                }
            }
        }
    }

    zip.finish()?;
    Ok(())
}
