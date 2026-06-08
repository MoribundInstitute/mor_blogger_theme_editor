pub mod about;
pub mod analytics;
pub mod archive;
pub mod categories;
pub mod lms;
pub mod portfolio;

pub use about::generate_about_html;
pub use analytics::{generate_analytics_dashboard_html, generate_analytics_html};
pub use archive::generate_archive_html;
pub use categories::generate_categories_html;
pub use lms::course_catalog::generate_course_catalog_html;
pub use lms::syllabus::generate_syllabus_html;
pub use portfolio::generate_portfolio_html;

use crate::config::styling::ColorConfig;

// Central HTML escaper. Replaces duplicates.
pub(crate) fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// Wraps any stencil output in custom CSS variables if needed.
pub fn apply_stencil_colors(
    raw_html: String,
    sync_global: bool,
    custom_colors: Option<&ColorConfig>,
) -> String {
    if sync_global || custom_colors.is_none() {
        return raw_html;
    }

    let colors = custom_colors.unwrap();
    let style_block = format!(
        "<style>\n.mor-stencil-scope {{\n  --bg-panel: {};\n  --fg-base: {};\n  --fg-dim: {};\n  --fg-muted: {};\n  --border-color: {};\n  --accent: {};\n}}\n</style>\n",
        colors.bg_panel.to_css(),
        colors.fg_base,
        colors.fg_muted,
        colors.fg_muted,
        colors.border,
        colors.accent
    );

    format!(
        "{}<div class=\"mor-stencil-scope\">\n{}\n</div>",
        style_block, raw_html
    )
}
