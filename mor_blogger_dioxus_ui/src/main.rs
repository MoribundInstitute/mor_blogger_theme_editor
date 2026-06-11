use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

mod app;
mod ui;
pub mod utils;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    let mut mode = "frameless".to_string();

    // 1. Read persistent preferences from disk
    if let Ok(prefs_str) = std::fs::read_to_string("editor_prefs.json") {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&prefs_str) {
            if let Some(m) = parsed.get("ui_mode").and_then(|v| v.as_str()) {
                mode = m.to_string();
            }
        }
    }

    // 2. Allow environment variables to override
    if let Ok(env_mode) = std::env::var("MOR_UI_MODE") {
        mode = env_mode;
    }

    let is_native = mode == "native";

    let cfg = Config::new()
        .with_menu(None::<dioxus::desktop::muda::Menu>)
        .with_window(
            WindowBuilder::new()
                .with_title("MorBlogger Theme Editor")
                .with_inner_size(LogicalSize::new(1280.0, 800.0))
                .with_decorations(is_native)
                .with_transparent(!is_native),
        );

    std::env::set_var("MOR_ACTIVE_UI_MODE", mode);
    
    // THE FIX: Modern Dioxus 0.7 LaunchBuilder
    LaunchBuilder::new().with_cfg(cfg).launch(app::App);
}