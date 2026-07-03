use crate::app::state::ThemeState;
use crate::ui::dialogs::modal::Modal;
use crate::ui::panels::theme_palette::cursor_panel::{apply_cursor_pack, CURSOR_PACKS};
use dioxus::prelude::*;
use mor_blogger_core::config::CursorSetConfig;

// cursor values are raw CSS `cursor` values injected straight into the stylesheet,
// so strip the few chars that would let a URL break out of the `url('…')` wrapper
// or the declaration. The user is editing their own theme, so this is
// breakage-prevention, not untrusted-input defense.
fn sanitize_url(u: &str) -> String {
    u.chars()
        .filter(|c| !matches!(c, '\'' | '"' | ';' | '{' | '}' | '(' | ')' | '<' | '>' | '\n' | '\r'))
        .collect()
}

fn compose_cursor(url: &str, x: &str, y: &str, fallback: &str) -> String {
    let u = sanitize_url(url.trim());
    if u.is_empty() {
        return fallback.to_string();
    }
    let xi: i32 = x.trim().parse().unwrap_or(0);
    let yi: i32 = y.trim().parse().unwrap_or(0);
    format!("url('{u}') {xi} {yi}, {fallback}")
}

// Pull a non-data URL back out of an existing `url('…') …, kw` value so switching
// slots keeps the field populated. data: URIs (built-in packs) are skipped —
// they're generated, not hand-edited.
fn extract_url(s: &str) -> String {
    if let Some(start) = s.find("url('") {
        let rest = &s[start + 5..];
        if let Some(end) = rest.find("')") {
            let u = &rest[..end];
            if !u.starts_with("data:") {
                return u.to_string();
            }
        }
    }
    String::new()
}

/// Every cursor slot a Blogger reader can hit: (label, keyword fallback,
/// what it applies to). Index 0 is the default arrow (ThemeConfig.cursor_style);
/// the rest live in CursorSetConfig.
const SLOTS: &[(&str, &str, &str)] = &[
    ("Default", "auto", "the page at rest"),
    ("Link / Button", "pointer", "links, buttons, nav"),
    ("Text", "text", "inputs, textareas, editable text"),
    ("Help", "help", "abbr titles, tooltip triggers"),
    ("Busy", "wait", "loading states (.mor-busy)"),
    ("Blocked", "not-allowed", "disabled buttons and fields"),
    ("Crosshair", "crosshair", "precision targets"),
    ("Move", "move", "drag handles, active drags"),
    ("Grab", "grab", "draggable elements"),
    ("Zoom", "zoom-in", "post images and thumbnails"),
];

const KEYWORDS: &[&str] = &[
    "auto", "default", "pointer", "text", "help", "wait", "progress", "crosshair",
    "not-allowed", "move", "grab", "grabbing", "zoom-in", "zoom-out", "cell", "copy",
    "alias", "col-resize", "row-resize", "ew-resize", "ns-resize", "nesw-resize",
    "nwse-resize", "none",
];

fn read_slot(state: &ThemeState, idx: usize) -> String {
    let signals = state.signals;
    if idx == 0 {
        return signals.cursor_style.read().clone();
    }
    let set = signals.cursor_set.read();
    match idx {
        1 => set.pointer.clone(),
        2 => set.text.clone(),
        3 => set.help.clone(),
        4 => set.wait.clone(),
        5 => set.not_allowed.clone(),
        6 => set.crosshair.clone(),
        7 => set.move_.clone(),
        8 => set.grab.clone(),
        _ => set.zoom_in.clone(),
    }
}

fn write_slot(state: &ThemeState, idx: usize, value: String) {
    let mut signals = state.signals;
    if idx == 0 {
        signals.cursor_style.set(value);
        return;
    }
    let mut set: CursorSetConfig = signals.cursor_set.read().clone();
    match idx {
        1 => set.pointer = value,
        2 => set.text = value,
        3 => set.help = value,
        4 => set.wait = value,
        5 => set.not_allowed = value,
        6 => set.crosshair = value,
        7 => set.move_ = value,
        8 => set.grab = value,
        _ => set.zoom_in = value,
    }
    signals.cursor_set.set(set);
}

const FIELD_STYLE: &str =
    "width: 100%; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; padding: 6px; border-radius: 4px; font-size: 12px;";
const LABEL_STYLE: &str = "font-size: 11px; color: #8e8e93;";

#[component]
pub fn AdvancedCursorsDialog(mut open_signal: Signal<bool>) -> Element {
    let theme = use_context::<ThemeState>();

    let mut slot = use_signal(|| 0usize);
    let mut url = use_signal(String::new);
    let mut hx = use_signal(|| "0".to_string());
    let mut hy = use_signal(|| "0".to_string());

    let (slot_label, slot_kw, slot_desc) = SLOTS[slot()];
    let current_value = read_slot(&theme, slot());
    let preview_css = if url.read().trim().is_empty() {
        current_value.clone()
    } else {
        compose_cursor(&url.read(), &hx.read(), &hy.read(), slot_kw)
    };

    rsx! {
        Modal {
            open: open_signal,
            title: "Advanced Cursors",
            style: "width: 560px; max-height: 85vh; overflow-y: auto;".to_string(),
            on_close: move |_| open_signal.set(false),

            div { style: "padding: 20px; display: flex; flex-direction: column; gap: 18px;",

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Cursor Packs" }
                div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    for name in CURSOR_PACKS {
                        button {
                            class: "mor-btn-secondary",
                            onclick: move |_| {
                                apply_cursor_pack(&theme, name);
                                url.set(String::new());
                            },
                            "{name}"
                        }
                    }
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 6px 0 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Edit a Slot" }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "{LABEL_STYLE}", "Cursor slot" }
                        select {
                            class: "mor-select",
                            style: "{FIELD_STYLE}",
                            onchange: move |e| {
                                let idx: usize = e.value().parse().unwrap_or(0);
                                slot.set(idx);
                                url.set(extract_url(&read_slot(&theme, idx)));
                                hx.set("0".to_string());
                                hy.set("0".to_string());
                            },
                            for (i, (label, _, _)) in SLOTS.iter().enumerate() {
                                option { value: "{i}", selected: slot() == i, "{label}" }
                            }
                        }
                        span { style: "font-size: 10px; color: #6e6e73;", "{slot_desc}" }
                    }
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "{LABEL_STYLE}", "Keyword (no custom image)" }
                        select {
                            class: "mor-select",
                            style: "{FIELD_STYLE}",
                            onchange: move |e| {
                                url.set(String::new());
                                write_slot(&theme, slot(), e.value());
                            },
                            for kw in KEYWORDS {
                                option { value: "{kw}", selected: current_value == *kw, "{kw}" }
                            }
                        }
                    }
                }

                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "{LABEL_STYLE}", "Custom image URL (PNG, SVG, or .cur — keep it ≤ 32×32; the keyword above becomes the mandatory fallback)" }
                    input {
                        r#type: "text",
                        placeholder: "https://example.com/cursor.png",
                        value: "{url}",
                        style: "{FIELD_STYLE}",
                        oninput: move |e| {
                            url.set(e.value());
                            write_slot(&theme, slot(), compose_cursor(&url.read(), &hx.read(), &hy.read(), SLOTS[slot()].1));
                        },
                    }
                }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "{LABEL_STYLE}", "Hotspot X" }
                        input {
                            r#type: "number", min: "0", value: "{hx}", style: "{FIELD_STYLE}",
                            oninput: move |e| {
                                hx.set(e.value());
                                if !url.read().trim().is_empty() {
                                    write_slot(&theme, slot(), compose_cursor(&url.read(), &hx.read(), &hy.read(), SLOTS[slot()].1));
                                }
                            },
                        }
                    }
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "{LABEL_STYLE}", "Hotspot Y" }
                        input {
                            r#type: "number", min: "0", value: "{hy}", style: "{FIELD_STYLE}",
                            oninput: move |e| {
                                hy.set(e.value());
                                if !url.read().trim().is_empty() {
                                    write_slot(&theme, slot(), compose_cursor(&url.read(), &hx.read(), &hy.read(), SLOTS[slot()].1));
                                }
                            },
                        }
                    }
                }

                div {
                    style: "height: 70px; display: flex; align-items: center; justify-content: center; background: #1C1C1E; border: 1px dashed #3A3A3C; border-radius: 6px; color: #8e8e93; font-size: 12px; cursor: {preview_css};",
                    "Hover to preview: {slot_label}"
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 6px 0 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Full Set Preview" }
                div { style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 6px;",
                    for (i, (label, _, _)) in SLOTS.iter().enumerate() {
                        {
                            let cell_cursor = read_slot(&theme, i);
                            let cell_border = if slot() == i { "var(--editor-accent, #a9aae2)" } else { "#3A3A3C" };
                            rsx! {
                                div {
                                    key: "{label}",
                                    style: "height: 48px; display: flex; align-items: center; justify-content: center; text-align: center; background: #1C1C1E; border: 1px solid {cell_border}; border-radius: 6px; color: #8e8e93; font-size: 10px; cursor: {cell_cursor};",
                                    onclick: move |_| {
                                        slot.set(i);
                                        url.set(extract_url(&read_slot(&theme, i)));
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_and_extract_roundtrip() {
        assert_eq!(compose_cursor("", "0", "0", "auto"), "auto");
        assert_eq!(compose_cursor("", "0", "0", "not-allowed"), "not-allowed");
        assert_eq!(
            compose_cursor("https://x/c.png", "4", "2", "pointer"),
            "url('https://x/c.png') 4 2, pointer"
        );
        // Non-numeric hotspot falls back to 0.
        assert_eq!(
            compose_cursor("/c.svg", "", "bad", "auto"),
            "url('/c.svg') 0 0, auto"
        );
        // CSS-breaking chars are stripped from the URL.
        assert_eq!(
            compose_cursor("a';}body{x", "0", "0", "auto"),
            "url('abodyx') 0 0, auto"
        );
        // extract pulls a plain url back out, but skips data: packs.
        assert_eq!(extract_url("url('/c.svg') 4 2, auto"), "/c.svg");
        assert_eq!(extract_url("url('data:image/svg+xml;base64,AAA'), auto"), "");
        assert_eq!(extract_url("auto"), "");
    }
}
