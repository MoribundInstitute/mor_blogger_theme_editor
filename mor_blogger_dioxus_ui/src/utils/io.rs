use rfd::FileDialog;
use std::fs::File;
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;

pub fn save_xml(xml: &str) {
    if let Some(path) = FileDialog::new()
        .set_title("Export Blogger XML")
        .set_file_name("mor_theme.xml")
        .add_filter("XML Document", &["xml"])
        .save_file()
    {
        let _ = std::fs::write(path, xml);
    }
}

pub fn save_toml(toml: &str) {
    if let Some(path) = FileDialog::new()
        .set_title("Save Workspace")
        .set_file_name("workspace.toml")
        .add_filter("TOML Config", &["toml"])
        .save_file()
    {
        let _ = std::fs::write(path, toml);
    }
}

pub fn load_toml() -> Option<String> {
    if let Some(path) = FileDialog::new()
        .set_title("Load Workspace")
        .add_filter("TOML Config", &["toml"])
        .pick_file()
    {
        std::fs::read_to_string(path).ok()
    } else {
        None
    }
}

pub fn export_bundle(xml: &str, toml: &str) {
    if let Some(path) = FileDialog::new()
        .set_title("Export Theme Bundle")
        .set_file_name("mor_theme_bundle.zip")
        .add_filter("ZIP Archive", &["zip"])
        .save_file()
    {
        if let Ok(file) = File::create(path) {
            let mut zip = zip::ZipWriter::new(file);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

            // Add the compiled XML
            let _ = zip.start_file("theme.xml", options);
            let _ = zip.write_all(xml.as_bytes());

            // Add the TOML workspace backup
            let _ = zip.start_file("workspace-backup.toml", options);
            let _ = zip.write_all(toml.as_bytes());

            // Add the deployment instructions
            let readme = "Moribund Theme Architect Bundle\n\n1. Go to Blogger.com -> Theme.\n2. Click the dropdown arrow next to 'Customize' and select 'Restore'.\n3. Upload the 'theme.xml' file.\n\nTo continue editing this theme later, load the 'workspace-backup.toml' file into the Moribund Architect desktop app.";
            let _ = zip.start_file("README.txt", options);
            let _ = zip.write_all(readme.as_bytes());

            let _ = zip.finish();
        }
    }
}
