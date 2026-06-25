use dioxus::desktop::tao::window::Icon;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

mod app;
mod ui;
pub mod ui_kit;
pub mod utils;

/// Caveman pixel extraction.
/// OS wants raw bytes, not encoded files.
fn load_app_icon() -> Icon {
    let img = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .expect("Failed to load icon from memory")
        .into_rgba8();

    let (width, height) = img.dimensions();
    let raw_pixels = img.into_raw();

    Icon::from_rgba(raw_pixels, width, height)
        .expect("Failed to construct window icon from pixel matrix")
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");

    // Wayland ignores the WindowBuilder pixel icon for the taskbar/dock; it maps a
    // window to its icon by app_id -> a matching .desktop file. tao 0.34 has no
    // window-level app_id setter, but GTK derives the Wayland app_id from the glib
    // program name, so set it here (before launch/gtk_init). Must equal the
    // .desktop basename and its StartupWMClass (see
    // packaging/com.moribundinstitute.morblogger-theme-editor.desktop).
    #[cfg(target_os = "linux")]
    glib::set_prgname(Some("com.moribundinstitute.morblogger-theme-editor"));
    let mut mode = "frameless".to_string();

    // 1. Read persistent preferences from disk
    let prefs = app::config_bridge::EditorPrefs::load();
    if let Some(m) = prefs.ui_mode {
        mode = m;
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
                .with_transparent(!is_native)
                .with_window_icon(Some(load_app_icon())),
        )
        .with_disable_drag_drop_handler(true);

    std::env::set_var("MOR_ACTIVE_UI_MODE", mode);

    // THE FIX: Modern Dioxus 0.7 LaunchBuilder
    LaunchBuilder::new().with_cfg(cfg).launch(app::App);
}
// watcher probe 1782216799

