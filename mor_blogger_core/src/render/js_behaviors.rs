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

/// The `_MOR_CONFIG` keys `build_master_js` emits, with editor-facing details.
/// Kept next to the catalog so the editor hints below can't drift far; a test
/// asserts these match the real preamble.
const MOR_CONFIG_KEYS: &[(&str, &str)] = &[
    ("MOBILE_BREAKPOINT", "number — panels auto-collapse below this width (px)"),
    ("PANELS_COLLAPSED_MOBILE", "bool — sidebars start collapsed on mobile"),
    ("THEME_TOGGLE", "bool — light/dark toggle behavior enabled"),
    ("SHARE_ACTIONS", "bool — share menu behavior enabled"),
];

/// Completion hints for the in-app JS editor: the `_MOR_CONFIG` runtime object
/// and the DOM hooks each behavior queries. Serialized as the JSON array CM6
/// autocompletion consumes (`[{label, type, detail}]`), injected once by the
/// shell as `window.MOR_JS_HINTS`.
pub fn editor_hints_json() -> String {
    let mut hints = vec![serde_json::json!({
        "label": "_MOR_CONFIG",
        "type": "constant",
        "detail": "theme runtime settings object",
    })];
    for (key, detail) in MOR_CONFIG_KEYS {
        hints.push(serde_json::json!({
            "label": format!("_MOR_CONFIG.{key}"),
            "type": "property",
            "detail": detail,
        }));
    }
    for b in JS_BEHAVIORS {
        for hook in b.requires {
            hints.push(serde_json::json!({
                "label": hook,
                "type": "class",
                "detail": format!("DOM hook — used by {}", b.label),
            }));
        }
    }
    serde_json::to_string(&hints).expect("static hint data serializes")
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BehaviorState {
    /// Shipped and its hooks are present — needed.
    Active,
    /// Shipped, but none of its hooks are in the markup — dead weight to trim.
    Wasted,
    /// Not shipped (a setting disabled it, or the current behavior set excludes it).
    Off,
}

/// One DOM hook a behavior queries, resolved against the active modules.
pub struct HookStatus {
    pub hook: &'static str,
    /// Which template module renders it (e.g. "header · mor_centered_search"),
    /// or None when nothing in the current permutation does.
    pub found_in: Option<String>,
}

pub struct BehaviorStatus {
    pub file: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub hooks: Vec<HookStatus>,
    pub bytes: usize,
    pub state: BehaviorState,
    /// The codeless setting that gates this behavior, if any — lets the UI
    /// offer the fix (ship / stop shipping) right on the card.
    pub setting: Option<JsSetting>,
}

impl BehaviorStatus {
    pub fn hooks_present(&self) -> bool {
        self.hooks.iter().all(|h| h.found_in.is_some())
    }
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
    let pack = &config.template_pack;
    // Per-part markup, labeled with the module that produced it, so a hook can
    // be attributed to a concrete template module rather than "somewhere".
    let parts_named: [(&str, &String, &String); 6] = [
        ("header", &pack.header_variant, &parts.header),
        ("main", &pack.main_variant, &parts.main),
        ("content", &pack.content_variant, &parts.content),
        ("left sidebar", &pack.left_sidebar_variant, &parts.sidebar_left),
        ("right sidebar", &pack.right_sidebar_variant, &parts.sidebar_right),
        ("footer", &pack.footer_variant, &parts.footer),
    ];

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
            let hooks: Vec<HookStatus> = b
                .requires
                .iter()
                .map(|sel| HookStatus {
                    hook: sel,
                    found_in: parts_named
                        .iter()
                        .find(|(_, _, markup)| markup.contains(sel))
                        .map(|(part, variant, _)| format!("{part} · {variant}")),
                })
                .collect();
            let hooks_present = hooks.iter().all(|h| h.found_in.is_some());
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
                hooks,
                bytes,
                state,
                setting: b.setting,
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

        // Hook attribution: a shipped-and-Active behavior has every hook traced
        // to a "part · variant" source; Wasted means at least one is unattributed.
        for s in &statuses {
            match s.state {
                BehaviorState::Active => assert!(s.hooks_present(), "{} active but hook missing", s.file),
                BehaviorState::Wasted => assert!(!s.hooks_present(), "{} wasted but all hooks found", s.file),
                BehaviorState::Off => {}
            }
            for h in &s.hooks {
                if let Some(src) = &h.found_in {
                    assert!(src.contains(" · "), "unlabeled hook source: {src}");
                }
            }
        }
    }

    #[test]
    fn editor_hint_config_keys_match_master_js_preamble() {
        // The hint list is hand-maintained; make sure every advertised
        // _MOR_CONFIG key actually exists in the emitted preamble.
        let preamble =
            crate::render::xml_parts::javascript_generator::build_master_js("", &Default::default());
        for (key, _) in MOR_CONFIG_KEYS {
            assert!(preamble.contains(&format!("{key}:")), "stale hint key {key}");
        }
        assert!(editor_hints_json().contains("mor-theme-toggle"));
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
