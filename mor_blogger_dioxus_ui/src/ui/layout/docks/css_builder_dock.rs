use std::collections::HashMap;

use crate::app::state::{DockPosition, LayoutState, ThemeState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::dock_chrome::DockChrome;
use dioxus::prelude::*;

/// (key, tab label, CSS selector the exported theme really uses). `:root` is the
/// base palette; the toggler (07-Theme-Toggler.js) stamps `data-theme` on <html>.
const SCOPES: &[(&str, &str, &str)] = &[
    ("shared", "Shared", ":root"),
    ("light", "Light", "html[data-theme=\"light\"]"),
    ("dark", "Dark", "html[data-theme=\"dark\"]"),
];

/// Color tokens the exported skin actually consumes — the same vocabulary the
/// Theme Palette writes. Keep in sync with the `:root` block css_generator emits.
const COLOR_TOKENS: &[(&str, &str)] = &[
    ("--accent", "Accent"),
    ("--bg-base", "Page background"),
    ("--bg-panel", "Panel background"),
    ("--bg-elevated", "Elevated background"),
    ("--fg-base", "Text"),
    ("--fg-muted", "Muted text"),
    ("--border-color", "Border"),
];

/// Non-color tokens; only offered on the Shared tab.
const SIZE_TOKENS: &[(&str, &str)] = &[
    ("--panel-border-width", "Panel border width"),
    ("--glow-spread", "Glow spread"),
];

fn scope_tokens(scope: &str) -> Vec<(&'static str, &'static str, bool)> {
    let mut t: Vec<_> = COLOR_TOKENS.iter().map(|&(v, l)| (v, l, true)).collect();
    if scope == "shared" {
        t.extend(SIZE_TOKENS.iter().map(|&(v, l)| (v, l, false)));
    }
    t
}

/// Every scope with at least one filled-in token, as one paste-ready snippet.
fn build_css(values: &HashMap<String, String>) -> String {
    let mut blocks = Vec::new();
    for (scope, _, selector) in SCOPES {
        let lines: Vec<String> = scope_tokens(scope)
            .iter()
            .filter_map(|(var, _, _)| {
                let v = values.get(&format!("{scope}:{var}"))?.trim();
                (!v.is_empty()).then(|| format!("  {var}: {v};"))
            })
            .collect();
        if !lines.is_empty() {
            blocks.push(format!("{selector} {{\n{}\n}}", lines.join("\n")));
        }
    }
    blocks.join("\n\n")
}

#[component]
pub fn CssBuilderDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let theme_state = use_context::<ThemeState>();
    let pos = (layout.css_builder_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let mut active_scope = use_signal(|| "shared");
    let mut values = use_signal(HashMap::<String, String>::new);

    let css = build_css(&values());
    let css_empty = css.is_empty();
    let css_for_apply = css.clone();
    let mut preset_css = theme_state.signals.preset_css;

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "CSS Token Builder".to_string(),
                dock_id: "css_builder".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.css_builder_pos.set(DockPosition::Hidden);
                },
                div {
                    style: "display: flex; flex-direction: column; height: calc(100% - 45px); overflow: hidden; background: var(--bg-panel); color: var(--fg-base); font-size: 0.85rem;",

                    // Scope tabs
                    div {
                        style: "padding: 8px 12px; border-bottom: 1px solid var(--border);",
                        div {
                            class: "editor-segmented",
                            for (scope, label, _) in SCOPES.iter().copied() {
                                button {
                                    class: if active_scope() == scope { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                                    onclick: move |_| active_scope.set(scope),
                                    "{label}"
                                }
                            }
                        }
                    }
                    div {
                        class: "editor-mini-label",
                        style: "padding: 6px 12px 0;",
                        "Overrides for the exported theme's real tokens. Blank = untouched. Shared is the base palette; Light/Dark apply after the visitor toggles."
                    }

                    // Token rows for the active scope
                    div {
                        style: "padding: 10px 12px; display: flex; flex-direction: column; gap: 8px; border-bottom: 1px solid var(--border); overflow-y: auto;",
                        for (var, label, is_color) in scope_tokens(active_scope()) {
                            {
                                let key = format!("{}:{var}", active_scope());
                                let val = values().get(&key).cloned().unwrap_or_default();
                                let swatch = if val.len() == 7 && val.starts_with('#') { val.clone() } else { "#000000".to_string() };
                                let key_text = key.clone();
                                let key_color = key.clone();
                                rsx! {
                                    div {
                                        style: "display: flex; align-items: center; gap: 8px;",
                                        label {
                                            class: "editor-field-label",
                                            style: "flex: 1;",
                                            title: "{var}",
                                            "{label}"
                                        }
                                        if is_color {
                                            crate::ui::components::inputs::ColorInput {
                                                style: "width: 36px; flex: none;",
                                                value: swatch.clone(),
                                                swatches: false,
                                                oninput: move |v: String| { values.with_mut(|m| { m.insert(key_color.clone(), v); }); },
                                            }
                                        }
                                        input {
                                            class: "editor-field",
                                            style: "width: 120px; flex: none; font-family: monospace; font-size: 0.8rem;",
                                            placeholder: "{var}",
                                            value: "{val}",
                                            oninput: move |e| { values.with_mut(|m| { m.insert(key_text.clone(), e.value()); }); },
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Apply / reset
                    div {
                        style: "display: flex; gap: 6px; padding: 8px 12px; border-bottom: 1px solid var(--border);",
                        button {
                            class: if css_empty { "editor-button" } else { "editor-button editor-button-active" },
                            style: format!(
                                "flex: 1; font-size: 0.8rem; {}",
                                if css_empty { "opacity: 0.55; cursor: default;" } else { "" },
                            ),
                            disabled: css_empty,
                            onclick: move |_| {
                                let snippet = css_for_apply.clone();
                                preset_css.with_mut(|s| {
                                    if !s.is_empty() && !s.ends_with('\n') { s.push('\n'); }
                                    s.push_str("\n/* Token overrides — CSS Token Builder */\n");
                                    s.push_str(&snippet);
                                    s.push('\n');
                                });
                                values.set(HashMap::new());
                            },
                            "Append to Preset CSS"
                        }
                        button {
                            class: "editor-button",
                            style: "font-size: 0.8rem;",
                            onclick: move |_| values.set(HashMap::new()),
                            "Clear"
                        }
                    }

                    // Live snippet preview (all scopes with values, not just the active tab)
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; min-height: 120px;",
                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",
                            CodeEditor {
                                value: if css_empty { "/* Fill in a token to build a snippet */".to_string() } else { css },
                                mode: "css".to_string(),
                                minimap_key: Some("css_builder".to_string()),
                                read_only: true,
                                on_change: |_| {},
                            }
                        }
                    }
                }
            }
        }
    }
}
