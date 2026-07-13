//! JavaScript workspace.
//!
//! Surfaces which JS behaviors actually ship for the user's current template-module
//! permutation (so they don't pay for JS the page can't use), plus codeless
//! behavior settings. There is intentionally no raw-JS-token editor here: arbitrary
//! JS rewriting is fragile; the right "codeless" knobs are the config-driven
//! behavior flags (`_MOR_CONFIG`), exposed below.
//!
//! [`JsWorkspaceBody`] is the shared view. The center view ([`JsWorkbench`]) and the
//! dockable "JS Behavior Builder" both render it, so the two surfaces can never
//! drift — the dock just wires it to different read/write plumbing.

use dioxus::prelude::*;
use mor_blogger_core::config::{ScriptBehaviorConfig, ThemeConfig};
use mor_blogger_core::render::js_behaviors::{analyze_js_usage, BehaviorState, JsSetting};
use mor_blogger_core::render::template_resolver::fetch_js;

use crate::app::state::{DockPosition, LayoutState};
use crate::app::vfs::VfsDictionary;
use crate::ui::components::inputs::EditorCheckbox;

fn fmt_bytes(b: usize) -> String {
    if b >= 1024 {
        format!("{:.1} KB", b as f64 / 1024.0)
    } else {
        format!("{b} B")
    }
}

/// The shared JS workspace UI: bundle budget, per-behavior tree-shaking, and the
/// codeless behavior settings. Reads the full config for analysis; every settings
/// change is handed back as a fresh [`ScriptBehaviorConfig`] via `on_scripts_change`
/// so each host can persist it however it likes.
#[component]
pub fn JsWorkspaceBody(
    config: ThemeConfig,
    on_scripts_change: EventHandler<ScriptBehaviorConfig>,
) -> Element {
    let vfs = use_context::<VfsDictionary>().0;
    let mut layout = use_context::<LayoutState>();

    let statuses = analyze_js_usage(&config, &vfs.read());
    // Behavior files whose VFS override actually differs from the bundled source
    // (an edit-then-undo leaves an identical entry behind — not "edited").
    let edited_files: Vec<&'static str> = {
        let map = vfs.read();
        statuses
            .iter()
            .map(|s| s.file)
            .filter(|f| map.get(*f).is_some_and(|c| c != fetch_js(f)))
            .collect()
    };
    let custom_js_bytes = config.plugins.custom_js.trim().len();
    let shipped: usize = statuses.iter().filter(|s| s.state != BehaviorState::Off).map(|s| s.bytes).sum::<usize>()
        + custom_js_bytes;
    let wasted: usize = statuses.iter().filter(|s| s.state == BehaviorState::Wasted).map(|s| s.bytes).sum();
    let scripts = config.scripts.clone();

    rsx! {
        div {
            h2 { style: "margin: 0 0 4px 0; font-size: 1.05rem; color: var(--fg-base);", "JavaScript" }
            p { style: "margin: 0; font-size: 0.82rem; color: var(--fg-muted); line-height: 1.5;",
                "One question: is your exported theme shipping JavaScript the page can't use? Each card below is one script in the export, checked against the markup your current template modules actually render."
            }
            p { style: "margin: 6px 0 0 0; font-size: 0.75rem; color: var(--fg-muted); line-height: 1.6;",
                span { style: "color: #3fb950; font-weight: 600;", "Active" }
                " — a module renders the markup it hooks into.  "
                span { style: "color: #eab308; font-weight: 600;", "Wasted" }
                " — ships, but nothing on the page uses it.  "
                span { style: "font-weight: 600;", "Off" }
                " — not shipped.  Click any card to edit its source."
            }
        }

        // ── Bundle summary ───────────────────────────────────
        div {
            class: "editor-card",
            style: "display: flex; gap: 24px; align-items: baseline; padding: 12px 14px;",
            div {
                span { style: "font-size: 1.4rem; font-weight: 700; color: var(--fg-base);", "{fmt_bytes(shipped)}" }
                span { style: "font-size: 0.75rem; color: var(--fg-muted); margin-left: 4px;", "shipped" }
            }
            if wasted > 0 {
                div {
                    span { style: "font-size: 1.4rem; font-weight: 700; color: #eab308;", "{fmt_bytes(wasted)}" }
                    span { style: "font-size: 0.75rem; color: var(--fg-muted); margin-left: 4px;", "wasted — trimmable" }
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
                    let edited = edited_files.contains(&s.file);
                    let file = s.file;
                    let setting = s.setting;
                    let hooks_present = s.hooks_present();
                    // One-click fix: flip the codeless setting that gates this behavior.
                    let flip_setting = {
                        let scripts = scripts.clone();
                        move |enable: bool| {
                            let Some(setting) = setting else { return };
                            let mut s2 = scripts.clone();
                            match setting {
                                JsSetting::ThemeToggle => s2.enable_theme_toggle = enable,
                                JsSetting::ShareActions => s2.enable_share_actions = enable,
                            }
                            on_scripts_change.call(s2);
                        }
                    };
                    rsx! {
                        div {
                            key: "{s.file}",
                            class: "editor-card",
                            style: "padding: 10px 12px; display: flex; flex-direction: column; gap: 4px; opacity: {card_opacity}; cursor: pointer;",
                            title: "Open {s.file} in the JS Editor",
                            onclick: move |_| {
                                if (layout.js_editor_pos)() == DockPosition::Hidden {
                                    layout.request_dock("js_editor", DockPosition::mor_panel_left);
                                }
                                layout.js_editor_open_file.set(Some(file.to_string()));
                            },
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "font-size: 0.85rem; font-weight: 600; color: var(--fg-base); flex: 1;", "{s.label}" }
                                if edited {
                                    span {
                                        style: "font-size: 0.66rem; font-weight: 600; padding: 2px 8px; border-radius: 10px; background: rgba(194,98,42,0.18); color: var(--accent, #c2622a);",
                                        title: "This file's source has been customized in the JS Editor",
                                        "Edited"
                                    }
                                }
                                span { style: "font-size: 0.66rem; color: var(--fg-muted); font-family: var(--font-mono);", "{fmt_bytes(s.bytes)}" }
                                span {
                                    style: "font-size: 0.66rem; font-weight: 600; padding: 2px 8px; border-radius: 10px; background: {badge_bg}; color: {badge_fg};",
                                    "{badge_text}"
                                }
                            }
                            div { style: "font-size: 0.72rem; color: var(--fg-muted); line-height: 1.4;", "{s.description}" }

                            // Each hook, attributed to the template module that renders
                            // it — this is the concrete module ↔ JS tie.
                            if !s.hooks.is_empty() {
                                div { style: "display: flex; flex-direction: column; gap: 2px;",
                                    for h in s.hooks.iter() {
                                        {
                                            let (text, color) = match &h.found_in {
                                                Some(src) => (format!("hooks into .{} — rendered by your {}", h.hook, src), "#3fb950"),
                                                None => (format!("hooks into .{} — no active template module renders it", h.hook), "#eab308"),
                                            };
                                            rsx! {
                                                span {
                                                    key: "{h.hook}",
                                                    style: "font-size: 0.66rem; font-family: var(--font-mono); color: {color};",
                                                    "{text}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if s.state == BehaviorState::Wasted {
                                div {
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    span { style: "font-size: 0.7rem; color: #eab308; flex: 1;",
                                        "Dead weight: this ships in your export, but nothing on the page uses it."
                                    }
                                    if setting.is_some() {
                                        button {
                                            class: "editor-mini-button",
                                            style: "padding: 3px 10px; font-size: 0.7rem; border-radius: 4px; cursor: pointer;",
                                            onclick: {
                                                let flip = flip_setting.clone();
                                                move |e: Event<MouseData>| { e.stop_propagation(); flip(false); }
                                            },
                                            "Stop shipping it"
                                        }
                                    }
                                }
                            }
                            if s.state == BehaviorState::Off && hooks_present && setting.is_some() && !s.hooks.is_empty() {
                                div {
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    span { style: "font-size: 0.7rem; color: #3fb950; flex: 1;",
                                        "Your modules render its markup, but the script is switched off."
                                    }
                                    button {
                                        class: "editor-mini-button",
                                        style: "padding: 3px 10px; font-size: 0.7rem; border-radius: 4px; cursor: pointer;",
                                        onclick: {
                                            let flip = flip_setting.clone();
                                            move |e: Event<MouseData>| { e.stop_propagation(); flip(true); }
                                        },
                                        "Ship it"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Custom JS: always appended to the export bundle when non-empty,
            // so it belongs in the same shipped-bytes story as the behaviors.
            {
                let (badge_text, badge_bg, badge_fg) = if custom_js_bytes > 0 {
                    ("Active", "rgba(34,197,94,0.18)", "#3fb950")
                } else {
                    ("Empty", "rgba(128,128,128,0.15)", "var(--fg-muted)")
                };
                let card_opacity = if custom_js_bytes == 0 { "0.6" } else { "1" };
                rsx! {
                    div {
                        class: "editor-card",
                        style: "padding: 10px 12px; display: flex; flex-direction: column; gap: 4px; opacity: {card_opacity}; cursor: pointer;",
                        title: "Open custom_js.js in the JS Editor",
                        onclick: move |_| {
                            if (layout.js_editor_pos)() == DockPosition::Hidden {
                                layout.request_dock("js_editor", DockPosition::mor_panel_left);
                            }
                            layout.js_editor_open_file.set(Some("custom_js.js".to_string()));
                        },
                        div {
                            style: "display: flex; align-items: center; gap: 8px;",
                            span { style: "font-size: 0.85rem; font-weight: 600; color: var(--fg-base); flex: 1;", "Custom JS" }
                            span { style: "font-size: 0.66rem; color: var(--fg-muted); font-family: var(--font-mono);", "{fmt_bytes(custom_js_bytes)}" }
                            span {
                                style: "font-size: 0.66rem; font-weight: 600; padding: 2px 8px; border-radius: 10px; background: {badge_bg}; color: {badge_fg};",
                                "{badge_text}"
                            }
                        }
                        div { style: "font-size: 0.72rem; color: var(--fg-muted); line-height: 1.4;",
                            "Your hand-written JavaScript (custom_js.js), appended to the exported bundle."
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
                    onchange: {
                        let scripts = scripts.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                let mut s = scripts.clone();
                                s.mobile_breakpoint = v;
                                on_scripts_change.call(s);
                            }
                        }
                    }
                }
            }

            EditorCheckbox {
                label: "Collapse sidebars on mobile by default".to_string(),
                checked: scripts.panels_collapsed_mobile,
                onchange: {
                    let scripts = scripts.clone();
                    move |v| { let mut s = scripts.clone(); s.panels_collapsed_mobile = v; on_scripts_change.call(s); }
                }
            }
            EditorCheckbox {
                label: "Light / dark toggle  (ships 07-Theme-Toggler.js)".to_string(),
                checked: scripts.enable_theme_toggle,
                onchange: {
                    let scripts = scripts.clone();
                    move |v| { let mut s = scripts.clone(); s.enable_theme_toggle = v; on_scripts_change.call(s); }
                }
            }
            EditorCheckbox {
                label: "Share menu  (ships 08-Share-Actions.js)".to_string(),
                checked: scripts.enable_share_actions,
                onchange: {
                    let scripts = scripts.clone();
                    move |v| { let mut s = scripts.clone(); s.enable_share_actions = v; on_scripts_change.call(s); }
                }
            }
        }
    }
}

/// Center-view host: reads the workspace TOML, renders the shared body, and persists
/// script changes back through the standard load-theme path.
#[component]
pub fn JsWorkbench(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
) -> Element {
    let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();

    rsx! {
        div {
            style: "flex: 1; min-height: 0; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 16px;",
            JsWorkspaceBody {
                config,
                on_scripts_change: move |scripts: ScriptBehaviorConfig| {
                    let mut cfg = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
                    cfg.scripts = scripts;
                    if let Ok(s) = toml::to_string_pretty(&cfg) {
                        on_load_theme.call(s);
                    }
                }
            }
        }
    }
}
