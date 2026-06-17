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

        // Verify new background struct logic: sidebars inherit main workspace gradient via --bg-workspace
        // (default now shares the gradient in bg_panel/elevated and CSS var)
        assert!(
            rendered_xml.contains("--bg-workspace") || rendered_xml.contains("linear-gradient"),
            "New background inheritance logic not reflected in output CSS"
        );
        // Ensure the workspace gradient values are present (from default_theme_config)
        assert!(
            rendered_xml.contains("#0a0c18") && rendered_xml.contains("#151b2c"),
            "Workspace gradient colors from background struct not propagated"
        );
    }
}