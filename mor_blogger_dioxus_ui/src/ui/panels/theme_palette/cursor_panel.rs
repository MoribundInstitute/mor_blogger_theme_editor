use crate::app::state::ThemeState;
use dioxus::prelude::*;
use mor_blogger_core::config::CursorSetConfig;

/// Build a `cursor: url(...)` value from an inline SVG (URL-encoded data URI).
/// Keep SVGs 24px and single-path simple — browsers cap cursor images ~32px.
fn svg_cursor(svg_body: &str, hx: u8, hy: u8, fallback: &str) -> String {
    let svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24'>{svg_body}</svg>"
    );
    let encoded = svg.replace('#', "%23").replace('"', "'");
    format!("url(\"data:image/svg+xml,{encoded}\") {hx} {hy}, {fallback}")
}

/// A full themed cursor set: arrow, hand, I-beam, help, hourglass, no-entry.
/// Positional cursors (crosshair/move/grab/zoom) stay native keywords — the
/// browser's are already correct and DPI-aware.
fn build_pack(fill: &str, stroke: &str) -> (String, CursorSetConfig) {
    let arrow = svg_cursor(
        &format!("<path d='M1 1L1 17L5 13L8 19L11 18L8 12L14 12Z' fill='{fill}' stroke='{stroke}' stroke-width='1'/>"),
        1, 1, "auto",
    );
    let hand = svg_cursor(
        &format!("<path d='M9 2.2c-.7 0-1.3.6-1.3 1.3v6.5H6.9c-.6 0-1.1.3-1.4.8-.3.5-.2 1.1.1 1.6l2.2 3.1c.5.7 1.3 1.1 2.1 1.1h3.4c1 0 1.9-.7 2.1-1.7l.7-3.2c.2-.9-.3-1.8-1.2-2-.2 0-.3-.1-.5-.1h-2.8V3.5c0-.7-.6-1.3-1.3-1.3z' fill='{fill}' stroke='{stroke}' stroke-width='1'/>"),
        9, 2, "pointer",
    );
    let ibeam = svg_cursor(
        &format!("<path d='M9 3h6M12 3v18M9 21h6' fill='none' stroke='{fill}' stroke-width='1.8'/><path d='M12 5v14' stroke='{stroke}' stroke-width='0.6'/>"),
        12, 12, "text",
    );
    let help = svg_cursor(
        &format!("<path d='M1 1L1 15L4.5 11.5L7 16.5L9.5 15.5L7 10.5L12 10.5Z' fill='{fill}' stroke='{stroke}' stroke-width='1'/><text x='13' y='14' font-size='13' font-weight='bold' fill='{fill}' stroke='{stroke}' stroke-width='0.5'>?</text>"),
        1, 1, "help",
    );
    let hourglass = svg_cursor(
        &format!("<path d='M6 2h12M6 22h12M7 3c0 6 4 5.5 5 8-1 2.5-5 2-5 8m10-16c0 6-4 5.5-5 8 1 2.5 5 2 5 8' fill='none' stroke='{fill}' stroke-width='2'/><path d='M9 20c1-2 2-2.5 3-2.5s2 .5 3 2.5z' fill='{fill}'/>"),
        12, 12, "wait",
    );
    let no_entry = svg_cursor(
        &format!("<circle cx='12' cy='12' r='8.5' fill='none' stroke='{fill}' stroke-width='2.5'/><path d='M6 6L18 18' stroke='{fill}' stroke-width='2.5'/><circle cx='12' cy='12' r='10' fill='none' stroke='{stroke}' stroke-width='0.8'/>"),
        12, 12, "not-allowed",
    );

    let set = CursorSetConfig {
        pointer: hand,
        text: ibeam,
        help,
        wait: hourglass,
        not_allowed: no_entry,
        ..CursorSetConfig::default()
    };
    (arrow, set)
}

/// Named packs. Returns (default-arrow css, full slot set); None = system.
pub fn cursor_pack(name: &str) -> Option<(String, CursorSetConfig)> {
    match name {
        "Bibata Amber" => Some(build_pack("#ffb300", "#2a2012")),
        "Bibata Ice" => Some(build_pack("#eceff4", "#4c566a")),
        _ => None,
    }
}

pub const CURSOR_PACKS: &[&str] = &["System Default", "Bibata Amber", "Bibata Ice"];

/// Apply a pack to the live theme (System Default resets every slot).
pub fn apply_cursor_pack(state: &ThemeState, name: &str) {
    let mut signals = state.signals;
    match cursor_pack(name) {
        Some((default_css, set)) => {
            signals.cursor_style.set(default_css);
            signals.cursor_set.set(set);
        }
        None => {
            signals.cursor_style.set("auto".to_string());
            signals.cursor_set.set(CursorSetConfig::default());
        }
    }
}

#[component]
pub fn CursorPanel() -> Element {
    let mut state = consume_context::<ThemeState>();

    rsx! {
        div { class: "panel-section",
            button {
                class: "mor-btn-secondary",
                style: "width: 100%; margin-bottom: 12px;",
                onclick: move |_| state.show_advanced_cursors.set(true),
                "⚙ Advanced Cursors (all slots)"
            }
            div { class: "setting-row",
                label { "Cursor Pack" }
                select {
                    class: "mor-select",
                    onchange: move |evt| apply_cursor_pack(&state, &evt.value()),
                    for name in CURSOR_PACKS {
                        option { value: "{name}", "{name}" }
                    }
                }
            }
            p { style: "margin: 8px 0 0; font-size: 0.72rem; color: var(--editor-muted); line-height: 1.4;",
                "A pack themes the arrow, hand, I-beam, help, busy, and blocked cursors together. Fine-tune any single slot in Advanced."
            }
        }
    }
}
