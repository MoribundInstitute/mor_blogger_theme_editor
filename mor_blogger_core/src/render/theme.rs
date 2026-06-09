//! Public Blogger theme export API.
//!
//! Headless: this module performs no UI work. It takes a destination
//! `&Path` (the UI crate is responsible for opening the file dialog and
//! passing the chosen path in) and writes the rendered theme or bundle.
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

use super::xml_generator;
use crate::config::{StaticPagesConfig, ThemeConfig};
use crate::presets::PresetPalette;
use crate::render::pages::{
    generate_about_html, generate_archive_html, generate_categories_html,
    generate_course_catalog_html, generate_portfolio_html, generate_syllabus_html,
};

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

/// Write the rendered Blogger XML to `file_path`.
///
/// The caller decides the destination (e.g. via a UI file dialog) and passes
/// the resolved path here.
pub fn save_xml_to_disk(xml_content: &str, file_path: &Path) -> Result<String, String> {
    match fs::write(file_path, xml_content) {
        Ok(_) => Ok(format!("System success: Theme exported to {:?}", file_path)),
        Err(e) => Err(format!("I/O Error: {}", e)),
    }
}

/// Write a `.zip` bundle (theme XML + any requested static-page stencils) to
/// `file_path`. `site_title` is only used to name the XML entry *inside* the
/// archive, not to choose the destination.
pub fn save_bundle_to_disk(
    xml_content: &str,
    site_title: &str,
    pages_config: &StaticPagesConfig,
    file_path: &Path,
) -> Result<String, String> {
    let file = File::create(file_path).map_err(|e| format!("Failed to create zip file: {}", e))?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // 1. Pack the XML Theme
    let safe_title = site_title.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    let xml_filename = format!("{}_theme.xml", safe_title.to_lowercase());
    zip.start_file(&xml_filename, options)
        .map_err(|e| format!("Zip error: {}", e))?;
    zip.write_all(xml_content.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;

    // 2. Build the stencils list based on user inclusion flags
    let mut stencils = Vec::new();

    if pages_config.archive.include_in_bundle {
        stencils.push(("archive.html", generate_archive_html(&pages_config.archive)));
    }
    if pages_config.categories.include_in_bundle {
        stencils.push((
            "directory.html",
            generate_categories_html(&pages_config.categories),
        ));
    }
    if pages_config.portfolio.include_in_bundle {
        stencils.push((
            "portfolio.html",
            generate_portfolio_html(&pages_config.portfolio),
        ));
    }
    if pages_config.about.include_in_bundle {
        stencils.push(("about.html", generate_about_html(&pages_config.about)));
    }
    if pages_config.lms.include_catalog_in_bundle {
        stencils.push((
            "course_catalog.html",
            generate_course_catalog_html(&pages_config.lms),
        ));
    }
    if pages_config.lms.include_syllabus_in_bundle {
        stencils.push(("syllabus.html", generate_syllabus_html(&pages_config.lms)));
    }

    // 3. Create directory and pack requested stencils
    if !stencils.is_empty() {
        zip.add_directory("html_pages", options)
            .map_err(|e| format!("Zip error: {}", e))?;

        for (filename, html) in stencils {
            zip.start_file(format!("html_pages/{}", filename), options)
                .map_err(|e| format!("Zip error: {}", e))?;
            zip.write_all(html.as_bytes())
                .map_err(|e| format!("Write error: {}", e))?;
        }
    }

    zip.finish()
        .map_err(|e| format!("Failed to finalize zip: {}", e))?;

    Ok(format!("System success: Bundle exported to {:?}", file_path))
}