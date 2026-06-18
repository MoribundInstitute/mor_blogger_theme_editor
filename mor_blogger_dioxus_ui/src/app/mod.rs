//! Application root.
//!
//! The app module is split into focused submodules under `src/app/`.
//! `src/lib.rs` declares `pub mod app;`, so this file is the module root
//! when the old flat `src/app.rs` file is removed.

use dioxus::prelude::*;

pub mod config_bridge;
pub mod hotswap;
mod keyboard;
pub mod layout_state;
mod render_state;
mod restore_drop;
mod shell;
pub mod state;

use keyboard::use_keyboard_shortcuts;
use layout_state::use_app_layout_state;
use render_state::use_app_render_state;
use restore_drop::use_restore_drop_bridge;
use shell::render_app_shell;
use state::use_theme_app_state;

pub fn App() -> Element {
    let theme = use_theme_app_state();
    let layout = use_app_layout_state();

    provide_context(theme);
    provide_context(layout);

    use_keyboard_shortcuts(layout);
    use_restore_drop_bridge(theme.signals, theme.active_preset);

    let render = use_app_render_state(theme, layout);

    render_app_shell(theme, layout, render)
}
