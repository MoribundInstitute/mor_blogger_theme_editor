//! JavaScript behavior catalog + usage analysis.
//!
//! The exported theme only ships JS that the active template-module permutation
//! actually needs. Each behavior declares the DOM hooks it queries; we resolve the
//! real markup (not the JS, which mentions the same selectors) and report, per
//! behavior, whether it is Active, Wasted (shipped but its hooks aren't present),
//! or Off (not shipped — a setting disabled it or the current behavior set excludes
//! it). This powers the JS workspace's "use no more JS than you need" view.

use std::collections::HashMap;

use crate::config::ThemeConfig;
use crate::render::template_resolver::{fetch_js, resolve_template_parts};

/// One shippable JS behavior.
pub struct JsBehavior {
    pub file: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    /// Substrings (class / id fragments) that must appear in the rendered markup
    /// for this behavior to do anything. Empty = always relevant when shipped.
    pub requires: &'static [&'static str],
    /// The `ScriptBehaviorConfig` flag that gates shipping it, if any.
    pub setting: Option<JsSetting>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum JsSetting {
    ThemeToggle,
    ShareActions,
}

/// The catalog. Order = display order.
pub const JS_BEHAVIORS: &[JsBehavior] = &[
    JsBehavior {
        file: "01-Core-Helpers.js",
        label: "Sidebars · Catalog · Back-to-Top",
        description: "Collapsible BROWSE/CONTENTS panels, the mega-menu catalog, and scroll-to-top. The base behavior bundle.",
        requires: &[],
        setting: None,
    },
    JsBehavior {
        file: "07-Theme-Toggler.js",
        label: "Light / Dark Toggle",
        description: "Remembers the visitor's light/dark choice across pages.",
        requires: &["mor-theme-toggle"],
        setting: Some(JsSetting::ThemeToggle),
    },
    JsBehavior {
        file: "08-Share-Actions.js",
        label: "Share Menu",
        description: "Opens the post share dropdown and the copy-link action.",
        requires: &["sharing-button"],
        setting: Some(JsSetting::ShareActions),
    },
    JsBehavior {
        file: "09-Magazine-Grid-Logic.js",
        label: "Magazine Grid Reveal",
        description: "Scroll-reveal animation for the magazine content layout.",
        requires: &["mor-magazine-feed"],
        setting: None,
    },
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BehaviorState {
    /// Shipped and its hooks are present — needed.
    Active,
    /// Shipped, but none of its hooks are in the markup — dead weight to trim.
    Wasted,
    /// Not shipped (a setting disabled it, or the current behavior set excludes it).
    Off,
}

pub struct BehaviorStatus {
    pub file: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub requires: &'static [&'static str],
    pub bytes: usize,
    pub state: BehaviorState,
}

/// Whether the core bundle (01/07/08) ships for this script variant.
fn ships_core(variant: &str) -> bool {
    matches!(
        variant,
        "mor_panels" | "mor_collapsible_sidebars" | "magazine_grid_logic"
    )
}

/// Resolve which behaviors ship for `config`, and classify each. Markup is scanned
/// without the `javascript` part so a behavior isn't called "used" just because its
/// own handler names the selector.
pub fn analyze_js_usage(config: &ThemeConfig, vfs: &HashMap<String, String>) -> Vec<BehaviorStatus> {
    let parts = resolve_template_parts(config, vfs);
    let markup = format!(
        "{}{}{}{}{}{}",
        parts.header, parts.main, parts.content, parts.sidebar_left, parts.sidebar_right, parts.footer
    );

    let pack = &config.template_pack;
    let core = ships_core(&pack.script_variant);
    let scripts = &config.scripts;

    JS_BEHAVIORS
        .iter()
        .map(|b| {
            let setting_on = match b.setting {
                Some(JsSetting::ThemeToggle) => scripts.enable_theme_toggle,
                Some(JsSetting::ShareActions) => scripts.enable_share_actions,
                None => true,
            };
            let shipped = match b.file {
                "09-Magazine-Grid-Logic.js" => pack.script_variant == "magazine_grid_logic",
                _ => core && setting_on,
            };
            let hooks_present = b.requires.iter().all(|sel| markup.contains(sel));
            let state = if !shipped {
                BehaviorState::Off
            } else if hooks_present {
                BehaviorState::Active
            } else {
                BehaviorState::Wasted
            };
            let bytes = vfs
                .get(b.file)
                .map(|s| s.len())
                .unwrap_or_else(|| fetch_js(b.file).len());
            BehaviorStatus {
                file: b.file,
                label: b.label,
                description: b.description,
                requires: b.requires,
                bytes,
                state,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magazine_grid_wasted_without_feed() {
        // Default config (collapsible sidebars, standard feed) ships core JS but no
        // magazine feed → the magazine behavior is Off (not shipped).
        let cfg = ThemeConfig::default();
        let vfs = HashMap::new();
        let statuses = analyze_js_usage(&cfg, &vfs);
        let mag = statuses.iter().find(|s| s.file == "09-Magazine-Grid-Logic.js").unwrap();
        assert_eq!(mag.state, BehaviorState::Off);
        // Core helpers ship by default.
        let core = statuses.iter().find(|s| s.file == "01-Core-Helpers.js").unwrap();
        assert_eq!(core.state, BehaviorState::Active);
        assert!(core.bytes > 0);
    }

    #[test]
    fn share_off_when_setting_disabled() {
        let mut cfg = ThemeConfig::default();
        cfg.scripts.enable_share_actions = false;
        let statuses = analyze_js_usage(&cfg, &HashMap::new());
        let share = statuses.iter().find(|s| s.file == "08-Share-Actions.js").unwrap();
        assert_eq!(share.state, BehaviorState::Off);
    }
}
