use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");

    // Reverting to Native OS Decorations.
    // Windows/macOS/Linux will now handle dragging, snapping, and maximizing perfectly.
    let cfg = Config::new().with_window(
        WindowBuilder::new()
            .with_title("Moribund Theme Architect")
            .with_decorations(true)
            .with_transparent(false)
    );

    LaunchBuilder::desktop().with_cfg(cfg).launch(mor_blogger_theme_editor::app::App);
}