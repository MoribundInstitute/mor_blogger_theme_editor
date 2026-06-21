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

    xml_generator::render_template(config, vfs)
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
