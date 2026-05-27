mod ads;
mod preview;
mod theme;
mod util;
mod xml_generator;

pub mod pages;
pub mod xml_parts;

pub use preview::render_preview_html;
pub use theme::{render_theme, save_xml_to_disk};