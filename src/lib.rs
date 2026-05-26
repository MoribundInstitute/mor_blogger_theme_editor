#![allow(non_snake_case)]

pub mod app;
pub mod clipboard;
pub mod config;
pub mod defaults;
pub mod diagnostics;
pub mod presets;
pub mod rehydration;
pub mod render;
pub mod ui;

// Export the App component so binaries can launch it
pub use app::App;
