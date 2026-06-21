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
        let rendered_xml = render_theme(&config, &std::collections::HashMap::new());

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
            rendered_xml.contains("#1e1a4d") && rendered_xml.contains("#5b2c8a"),
            "Workspace gradient colors from background struct not propagated"
        );
    }


#[test]
fn test_mor_magazine_integrity() {
    use crate::config::defaults::default_theme_config;
    use crate::diagnostics::check_integrity;
    use crate::render::theme::render_theme;
    let mut config = default_theme_config();
    config.template_pack.content_variant = "mor_magazine".to_string();
    let rendered_xml = render_theme(&config, &std::collections::HashMap::new());
    let result = check_integrity(&rendered_xml, &config.template_pack);
    assert!(result.is_valid, "Integrity check failed: {:?}", result.errors);
    for cls in ["mor-magazine-feed", "hero-post", "grid-post", "mor-pager", "pager-btn"] {
        assert!(rendered_xml.contains(cls), "missing {}", cls);
    }
}

    #[test]
    fn test_script_behavior_config_propagation() {
        let mut config = default_theme_config();
        config.scripts.mobile_breakpoint = 1234.0;
        config.scripts.panels_collapsed_mobile = false;
        config.scripts.enable_theme_toggle = false;
        config.scripts.enable_share_actions = false;

        let rendered_xml = render_theme(&config, &std::collections::HashMap::new());

        assert!(
            rendered_xml.contains("MOBILE_BREAKPOINT: 1234"),
            "Mobile breakpoint config not propagated to scripts"
        );
        assert!(
            rendered_xml.contains("PANELS_COLLAPSED_MOBILE: false"),
            "Panels collapsed config not propagated to scripts"
        );
        assert!(
            rendered_xml.contains("THEME_TOGGLE: false"),
            "Theme toggle config not propagated to scripts"
        );
        assert!(
            rendered_xml.contains("SHARE_ACTIONS: false"),
            "Share actions config not propagated to scripts"
        );
    }
}
