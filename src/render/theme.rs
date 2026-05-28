//! Public Blogger theme export API.

use rfd::FileDialog;
use std::fs;

use super::xml_generator;
use crate::config::ThemeConfig;
use crate::presets::PresetPalette;

pub fn render_theme(
    config: &ThemeConfig,
    light_palette: &PresetPalette,
    dark_palette: &PresetPalette,
) -> String {
    eprintln!(
        "[render_theme] preset_css bytes = {}",
        config.preset_css.len()
    );
    xml_generator::render_template(config, light_palette, dark_palette)
}

pub fn save_xml_to_disk(xml_content: &str, site_title: &str) -> Result<String, String> {
    let safe_title = site_title.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    let default_filename = format!("{}_theme.xml", safe_title.to_lowercase());

    let file_path = FileDialog::new()
        .add_filter("Blogger Theme XML", &["xml"])
        .set_file_name(&default_filename)
        .save_file();

    match file_path {
        Some(path) => match fs::write(&path, xml_content) {
            Ok(_) => Ok(format!("System success: Theme exported to {:?}", path)),
            Err(e) => Err(format!("I/O Error: {}", e)),
        },
        None => Err("Export aborted.".to_string()),
    }
}