pub mod about;
pub mod archive;
pub mod categories;
pub mod lms;
pub mod portfolio;

// Re-export standard pages
pub use about::generate_about_html;
pub use archive::generate_archive_html;
pub use categories::generate_categories_html;
pub use portfolio::generate_portfolio_html;

// Re-export LMS pages so the UI panel can find them
pub use lms::course_catalog::generate_course_catalog_html;
pub use lms::syllabus::generate_syllabus_html;
