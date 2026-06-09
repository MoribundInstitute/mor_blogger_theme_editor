mod ads;
mod preview;
mod util;
mod xml_generator;

pub mod theme;
pub mod css_builder;
pub mod pages;
pub mod plugins;
pub mod template_resolver;
pub mod xml_parts;

// Re-export PreviewTemplateMode so the UI crate can reach it without the
// `preview` module being public. The type is the single canonical definition
// the headless renderer (render_preview_html) expects.
pub use preview::{render_preview_html, PreviewTemplateMode};
// Added save_bundle_to_disk to the public exports
pub use theme::{render_theme, save_bundle_to_disk, save_xml_to_disk};