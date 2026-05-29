//! Application root.
//!
//! The app module is split into focused submodules under `src/app/`.
//! `src/lib.rs` declares `pub mod app;`, so this file is the module root
//! when the old flat `src/app.rs` file is removed.

use dioxus::prelude::*;

mod config_bridge;
mod hotswap;
mod keyboard;
mod layout_state;
mod render_state;
mod restore_drop;
mod shell;
mod state;

use keyboard::use_keyboard_shortcuts;
use layout_state::use_app_layout_state;
use render_state::use_app_render_state;
use restore_drop::use_restore_drop_bridge;
use shell::render_app_shell;
use state::use_theme_app_state;

pub fn App() -> Element {
    let theme = use_theme_app_state();
    let layout = use_app_layout_state();

    use_keyboard_shortcuts(layout);
    use_restore_drop_bridge(theme.signals, theme.active_preset);

    let render = use_app_render_state(theme, layout);

    render_app_shell(theme, layout, render)
}
