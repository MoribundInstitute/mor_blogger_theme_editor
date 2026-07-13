pub mod about;
pub mod analytics;
pub mod archive;
pub mod categories;
pub mod lms;
pub mod portfolio;

pub use about::generate_about_html;
pub use analytics::generate_analytics_dashboard_html;
pub use archive::generate_archive_html;
pub use categories::generate_categories_html;
pub use lms::course_catalog::generate_course_catalog_html;
pub use lms::my_courses::generate_my_courses_html;
pub use lms::syllabus::generate_syllabus_html;
pub use portfolio::generate_portfolio_html;

use crate::config::pages::PageLayout;

/// Scoped chrome overrides for a static page. Returns a `<style>` block that
/// hides/adjusts the theme shell (sidebars, header, footer, search, content
/// width) ONLY when this page is viewed — because Blogger loads a page's pasted
/// `<style>` only on that page. Empty string when nothing is overridden.
pub fn page_chrome_overrides(layout: &PageLayout) -> String {
    let mut rules = String::new();
    // Hiding a sidebar also hides its header toggle button. Cover both the
    // exported Blogger header (.left-panel-toggle) and the in-app preview header
    // (.header-panel-toggle-left / #mor-dock-left-toggle).
    if layout.hide_left_sidebar {
        rules.push_str("#panel-left, .panel-left { display: none !important; }\n");
        rules.push_str(".left-panel-toggle, .header-panel-toggle-left, #mor-dock-left-toggle { display: none !important; }\n");
    }
    if layout.hide_right_sidebar {
        rules.push_str("#panel-right, .panel-right { display: none !important; }\n");
        rules.push_str(".right-panel-toggle, .header-panel-toggle-right, #mor-dock-right-toggle { display: none !important; }\n");
    }
    if layout.hide_header {
        rules.push_str(".main-header { display: none !important; }\n");
    }
    if layout.hide_footer {
        rules.push_str(".mor-footer { display: none !important; }\n");
    }
    if layout.hide_search {
        rules.push_str(".mor-search { display: none !important; }\n");
    }
    match layout.width.as_str() {
        "full" => rules.push_str(
            ".canvas-core { max-width: 100% !important; width: 100% !important; }\n",
        ),
        "centered" => rules.push_str(
            ".canvas-core { max-width: 880px !important; margin-left: auto !important; margin-right: auto !important; }\n",
        ),
        _ => {}
    }
    if rules.is_empty() {
        String::new()
    } else {
        format!("<style>\n/* Per-page chrome overrides (MorBlogger) */\n{rules}</style>\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_overrides_emit_scoped_css() {
        // Default = no overrides = empty (page keeps the full theme chrome).
        assert_eq!(page_chrome_overrides(&PageLayout::default()), "");

        let layout = PageLayout {
            hide_left_sidebar: true,
            hide_right_sidebar: false,
            hide_header: true,
            hide_footer: false,
            hide_search: false,
            width: "centered".to_string(),
        };
        let css = page_chrome_overrides(&layout);
        assert!(css.contains("#panel-left, .panel-left { display: none"));
        assert!(!css.contains("#panel-right")); // right sidebar kept
        assert!(css.contains(".left-panel-toggle")); // left toggle auto-hidden with sidebar
        assert!(!css.contains(".right-panel-toggle")); // right toggle kept
        assert!(css.contains(".main-header { display: none"));
        assert!(!css.contains(".mor-footer { display: none")); // not hidden
        assert!(css.contains(".canvas-core { max-width: 880px"));
    }

    #[test]
    fn generated_about_page_includes_chrome_when_set() {
        let mut about = crate::config::pages::AboutPageConfig::default();
        about.layout.hide_left_sidebar = true;
        about.layout.hide_right_sidebar = true;
        let html = generate_about_html(&about);
        assert!(html.contains("#panel-left, .panel-left { display: none"));
    }
}
