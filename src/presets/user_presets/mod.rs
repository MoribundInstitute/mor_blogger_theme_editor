//! User preset bundle support.
//!
//! User/imported presets live outside the Rust source tree under:
//! `~/.config/mor_blogger_theme_editor/presets/<id>/`.

mod bundle;
mod icons;
mod io;
mod paths;
mod runtime;

pub use bundle::{PresetSourceInfo, UserPresetBundle, USER_PRESET_BUNDLE_VERSION};
pub use icons::{IconAssetNames, UserPresetIconAssets};
pub use io::{
    load_user_preset_bundle, load_user_preset_icon_assets, save_user_preset_bundle,
    UserPresetDiskReport,
};
pub use paths::{default_user_presets_dir, sanitize_preset_id, user_preset_bundle_dir};
pub use runtime::load_user_presets_as_presets;