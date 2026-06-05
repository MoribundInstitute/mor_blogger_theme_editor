#![allow(non_snake_case)]

pub mod app;
pub mod config;
pub mod diagnostics;
pub mod presets;
pub mod render;
pub mod ui;
pub mod utils;

// Export the App component so binaries can launch it
pub use app::App;