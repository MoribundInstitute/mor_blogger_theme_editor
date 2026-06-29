//! JavaScript workspace.
//!
//! Surfaces which JS behaviors actually ship for the user's current template-module
//! permutation (so they don't pay for JS the page can't use), plus codeless
//! behavior settings. There is intentionally no raw-JS-token editor here: arbitrary
//! JS rewriting is fragile; the right "codeless" knobs are the config-driven
//! behavior flags (`_MOR_CONFIG`), exposed below.

use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::render::js_behaviors::{analyze_js_usage, BehaviorState};

use crate::app::vfs::VfsDictionary;

#[component]
pub fn JsWorkbench(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
) -> Element {
    let vfs = use_context::<VfsDictionary>().0;

    // Apply a config edit through the same path the rest of the app uses.
    let apply = move |edit: Box<dyn FnOnce(&mut ThemeConfig)>| {
        let mut cfg = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
        edit(&mut cfg);
        if let Ok(s) = toml::to_string_pretty(&cfg) {
            on_load_theme.call(s);
        }
    };

    rsx! {
        div {
            style: "flex: 1; min-height: 0; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 16px;",
            {
                let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
                let statuses = analyze_js_usage(&config, &vfs.read());
                let shipped: usize = statuses.iter().filter(|s| s.state != BehaviorState::Off).map(|s| s.bytes).sum();
                let wasted: usize = statuses.iter().filter(|s| s.state == BehaviorState::Wasted).map(|s| s.bytes).sum();
                let scripts = config.scripts.clone();

                rsx! {
                    div {
                        h2 { style: "margin: 0 0 4px 0; font-size: 1.05rem; color: var(--fg-base);", "JavaScript" }
                        p { style: "margin: 0; font-size: 0.82rem; color: var(--fg-muted); line-height: 1.5;",
                            "Only behaviors your current template modules can actually use are shipped. Trim the rest, or tune behavior with the codeless settings below."
                        }
                    }

                    // ── Bundle summary ───────────────────────────────────
                    div {
                        class: "editor-card",
                        style: "display: flex; gap: 24px; align-items: baseline; padding: 12px 14px;",
                        div {
                            span { style: "font-size: 1.4rem; font-weight: 700; color: var(--fg-base);", "{shipped}" }
                            span { style: "font-size: 0.75rem; color: var(--fg-muted); margin-left: 4px;", "bytes shipped" }
                        }
                        if wasted > 0 {
                            div {
                                span { style: "font-size: 1.4rem; font-weight: 700; color: #eab308;", "{wasted}" }
                                span { style: "font-size: 0.75rem; color: var(--fg-muted); margin-left: 4px;", "bytes wasted — trimmable" }
                            }
                        }
                    }

                    // ── Behaviors ────────────────────────────────────────
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "color: var(--fg-muted); font-size: 0.68rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;", "Behaviors" }
                        for s in statuses.iter() {
                            {
                                let (badge_text, badge_bg, badge_fg) = match s.state {
                                    BehaviorState::Active => ("Active", "rgba(34,197,94,0.18)", "#3fb950"),
                                    BehaviorState::Wasted => ("Wasted", "rgba(234,179,8,0.18)", "#eab308"),
                                    BehaviorState::Off => ("Off", "rgba(128,128,128,0.15)", "var(--fg-muted)"),
                                };
                                let card_opacity = if s.state == BehaviorState::Off { "0.6" } else { "1" };
                                rsx! {
                                    div {
                                        key: "{s.file}",
                                        class: "editor-card",
                                        style: "padding: 10px 12px; display: flex; flex-direction: column; gap: 4px; opacity: {card_opacity};",
                                        div {
                                            style: "display: flex; align-items: center; gap: 8px;",
                                            span { style: "font-size: 0.85rem; font-weight: 600; color: var(--fg-base); flex: 1;", "{s.label}" }
                                            span { style: "font-size: 0.66rem; color: var(--fg-muted); font-family: var(--font-mono);", "{s.bytes} B" }
                                            span {
                                                style: "font-size: 0.66rem; font-weight: 600; padding: 2px 8px; border-radius: 10px; background: {badge_bg}; color: {badge_fg};",
                                                "{badge_text}"
                                            }
                                        }
                                        div { style: "font-size: 0.72rem; color: var(--fg-muted); line-height: 1.4;", "{s.description}" }
                                        if s.state == BehaviorState::Wasted {
                                            div { style: "font-size: 0.7rem; color: #eab308; font-family: var(--font-mono);",
                                                "Shipped, but this layout has no matching markup — switch the content/JS behavior to drop it."
                                            }
                                        }
                                        if !s.requires.is_empty() {
                                            div { style: "display: flex; flex-wrap: wrap; gap: 4px;",
                                                for hook in s.requires.iter() {
                                                    span {
                                                        key: "{hook}",
                                                        style: "padding: 1px 6px; border-radius: 3px; border: 1px solid var(--editor-border-soft); color: var(--fg-muted); font-size: 0.64rem; font-family: var(--font-mono);",
                                                        "needs .{hook}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Codeless behavior settings ───────────────────────
                    div { class: "editor-card", style: "display: flex; flex-direction: column; gap: 10px; padding: 12px 14px;",
                        span { style: "color: var(--fg-muted); font-size: 0.68rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;", "Behavior Settings (no code)" }

                        label {
                            style: "display: flex; flex-direction: column; gap: 2px; font-size: 0.78rem; color: var(--fg-muted);",
                            "Mobile breakpoint (px) — panels auto-collapse below this width"
                            input {
                                class: "editor-input",
                                style: "max-width: 140px; font-size: 0.8rem; padding: 4px 6px;",
                                r#type: "number", min: "320", max: "1600", step: "10",
                                value: "{scripts.mobile_breakpoint}",
                                onchange: move |e| {
                                    if let Ok(v) = e.value().parse::<f64>() {
                                        let mut f = apply; f(Box::new(move |c| c.scripts.mobile_breakpoint = v));
                                    }
                                }
                            }
                        }

                        Toggle {
                            label: "Collapse sidebars on mobile by default".to_string(),
                            checked: scripts.panels_collapsed_mobile,
                            on_toggle: move |v| { let mut f = apply; f(Box::new(move |c| c.scripts.panels_collapsed_mobile = v)); }
                        }
                        Toggle {
                            label: "Light / dark toggle  (ships 07-Theme-Toggler.js)".to_string(),
                            checked: scripts.enable_theme_toggle,
                            on_toggle: move |v| { let mut f = apply; f(Box::new(move |c| c.scripts.enable_theme_toggle = v)); }
                        }
                        Toggle {
                            label: "Share menu  (ships 08-Share-Actions.js)".to_string(),
                            checked: scripts.enable_share_actions,
                            on_toggle: move |v| { let mut f = apply; f(Box::new(move |c| c.scripts.enable_share_actions = v)); }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Toggle(label: String, checked: bool, on_toggle: EventHandler<bool>) -> Element {
    rsx! {
        label {
            style: "display: flex; align-items: center; gap: 8px; font-size: 0.8rem; color: var(--fg-base); cursor: pointer;",
            input {
                r#type: "checkbox",
                checked: checked,
                onchange: move |e| on_toggle.call(e.checked()),
            }
            "{label}"
        }
    }
}
