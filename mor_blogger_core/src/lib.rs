#![allow(non_snake_case)]

pub mod config;
pub mod diagnostics;
pub mod presets;
pub mod render;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::config::defaults::default_theme_config;
    use super::diagnostics::check_integrity;
    use super::render::theme::render_theme;

    #[test]
    fn test_default_theme_integrity() {
        let config = default_theme_config();
        let rendered_xml = render_theme(&config);
        
        let result = check_integrity(&rendered_xml, &config.template_pack);
        
        // Assert that the generated XML is valid and has no errors
        assert!(
            result.is_valid,
            "Integrity check failed: {:?}",
            result.errors
        );
    }
}