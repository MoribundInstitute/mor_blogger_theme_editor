//! App-level shell toolbar.
//!
//! Export actions live in `crate::ui::workspace::master_canvas` and are
//! wired through `CenterWorkspacePanel`. If a top-level menu bar ever moves
//! export logic here, pass `active_preset: Signal<Option<&'static str>>` down
//! from `app.rs` and call:
//!
//! ```rust
//! let (light, dark) = crate::presets::resolve_palette_pair(active_preset(), &config);
//! let xml = crate::render::render_theme(&config, &light, &dark);
//! ```