//! Application root.
//!
//! The app module is split into focused submodules under `src/app/`.
//! `src/lib.rs` declares `pub mod app;`, so this file is the module root
//! when the old flat `src/app.rs` file is removed.

use dioxus::prelude::*;

pub mod config_bridge;
pub mod hotswap;
mod keyboard;
mod restore_drop;
pub mod services;
pub mod shell;
pub mod shell_dialogs;
pub mod shell_file_actions;
pub mod shell_layout;
pub mod state;
#[cfg(not(target_arch = "wasm32"))]
mod theme_hot_reload;
pub mod theme_signals;
pub mod vfs;

use crate::app::vfs::VfsDictionary;
use keyboard::use_keyboard_shortcuts;
use restore_drop::use_restore_drop_bridge;
use shell::render_app_shell;
use state::{LayoutState, RenderState, SiteData, ThemeState, WindowManager};
#[cfg(not(target_arch = "wasm32"))]
use theme_hot_reload::use_theme_config_hot_reload;

#[allow(non_snake_case)]
pub fn App() -> Element {
    use_context_provider(|| {
        Signal::new(WindowManager {
            show_template_modules: false, // Hidden by default until called
        })
    });

    // VFS must be provided before any hook that calls use_context::<VfsDictionary>()
    let vfs_map = use_signal(|| {
        let mut map = mor_blogger_core::utils::fs_bridge::load_all_custom_css().unwrap_or_default();
        if let Ok(js_map) = mor_blogger_core::utils::fs_bridge::load_all_custom_js() {
            map.extend(js_map);
        }
        map
    });
    provide_context(VfsDictionary(vfs_map));

    let theme = ThemeState::new();
    let layout = LayoutState::new();
    let site_data = use_signal(|| SiteData::default());
    let render = RenderState::new(theme, layout, site_data);

    provide_context(theme);
    provide_context(layout);
    provide_context(site_data);
    provide_context(render);

    use_keyboard_shortcuts(layout);
    use_restore_drop_bridge(theme);
    #[cfg(not(target_arch = "wasm32"))]
    use_theme_config_hot_reload(theme);

    render_app_shell(theme, layout, render)
}
