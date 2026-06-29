use crate::app::state::ThemeState;
use crate::ui::dialogs::modal::Modal;
use crate::ui::panels::theme_palette::cursor_panel::get_cursor_css;
use dioxus::prelude::*;

// cursor_style is a raw CSS `cursor` value injected straight into the stylesheet
// ({{CURSOR_DEFAULT}}), so strip the few chars that would let a URL break out of
// the `url('…')` wrapper or the declaration. The user is editing their own theme,
// so this is breakage-prevention, not untrusted-input defense.
fn sanitize_url(u: &str) -> String {
    u.chars()
        .filter(|c| !matches!(c, '\'' | '"' | ';' | '{' | '}' | '(' | ')' | '<' | '>' | '\n' | '\r'))
        .collect()
}

fn compose_cursor(url: &str, x: &str, y: &str) -> String {
    let u = sanitize_url(url.trim());
    if u.is_empty() {
        return "auto".to_string();
    }
    let xi: i32 = x.trim().parse().unwrap_or(0);
    let yi: i32 = y.trim().parse().unwrap_or(0);
    format!("url('{u}') {xi} {yi}, auto")
}

// Pull a non-data URL back out of an existing `url('…') …, auto` value so reopening
// the dialog keeps the field populated. ponytail: data: URIs (the built-in presets)
// are skipped — they're huge and not meant to be hand-edited — and the hotspot isn't
// parsed back, it just defaults to 0,0. Add round-tripping if users ask.
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

// Signals are Copy, so take them by value and recompose from a free fn — avoids a
// shared closure that several move-handlers would each need to own.
fn recompose(
    mut cursor_style: Signal<String>,
    url: Signal<String>,
    hx: Signal<String>,
    hy: Signal<String>,
) {
    cursor_style.set(compose_cursor(&url.read(), &hx.read(), &hy.read()));
}

const FIELD_STYLE: &str =
    "width: 100%; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; padding: 6px; border-radius: 4px; font-size: 12px;";
const LABEL_STYLE: &str = "font-size: 11px; color: #8e8e93;";

#[component]
pub fn AdvancedCursorsDialog(mut open_signal: Signal<bool>) -> Element {
    let theme = use_context::<ThemeState>();
    let mut signals = theme.signals;

    let mut url = use_signal(|| extract_url(&signals.cursor_style.read()));
    let mut hx = use_signal(|| "0".to_string());
    let mut hy = use_signal(|| "0".to_string());

    let preview_css = compose_cursor(&url.read(), &hx.read(), &hy.read());

    rsx! {
        Modal {
            open: open_signal,
            title: "Advanced Cursor",
            style: "width: 480px; max-height: 80vh; overflow-y: auto;".to_string(),
            on_close: move |_| open_signal.set(false),

            div { style: "padding: 20px; display: flex; flex-direction: column; gap: 20px;",

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 0; font-size: 16px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Built-in Presets" }
                div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    for name in ["Default", "Bibata Amber", "Bibata Ice"] {
                        button {
                            class: "mor-btn-secondary",
                            onclick: move |_| {
                                url.set(String::new());
                                hx.set("0".to_string());
                                hy.set("0".to_string());
                                signals.cursor_style.set(get_cursor_css(name).to_string());
                            },
                            "{name}"
                        }
                    }
                }

                h3 { style: "color: var(--editor-accent, #a9aae2); margin: 10px 0 0 0; font-size: 16px; font-weight: 600; border-bottom: 1px solid #333; padding-bottom: 8px;", "Custom Cursor" }
                p { style: "margin: 0; font-size: 12px; color: var(--fg-muted);",
                    "Paste a direct image URL (PNG, SVG, or .cur — 32×32 or smaller works best). The hotspot is the pixel inside the image that acts as the click point."
                }

                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "{LABEL_STYLE}", "Image URL" }
                    input {
                        r#type: "text",
                        placeholder: "https://example.com/cursor.png",
                        value: "{url}",
                        style: "{FIELD_STYLE}",
                        oninput: move |e| { url.set(e.value()); recompose(signals.cursor_style, url, hx, hy); },
                    }
                }

                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "{LABEL_STYLE}", "Hotspot X" }
                        input {
                            r#type: "number",
                            min: "0",
                            value: "{hx}",
                            style: "{FIELD_STYLE}",
                            oninput: move |e| { hx.set(e.value()); recompose(signals.cursor_style, url, hx, hy); },
                        }
                    }
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "{LABEL_STYLE}", "Hotspot Y" }
                        input {
                            r#type: "number",
                            min: "0",
                            value: "{hy}",
                            style: "{FIELD_STYLE}",
                            oninput: move |e| { hy.set(e.value()); recompose(signals.cursor_style, url, hx, hy); },
                        }
                    }
                }

                div {
                    style: "margin-top: 4px; height: 90px; display: flex; align-items: center; justify-content: center; background: #1C1C1E; border: 1px dashed #3A3A3C; border-radius: 6px; color: #8e8e93; font-size: 12px; cursor: {preview_css};",
                    "Hover here to preview the cursor"
                }

                p { style: "margin: 0; font-size: 11px; color: #6e6e73; font-family: monospace; word-break: break-all;",
                    "cursor: {preview_css};"
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
        assert_eq!(compose_cursor("", "0", "0"), "auto");
        assert_eq!(
            compose_cursor("https://x/c.png", "4", "2"),
            "url('https://x/c.png') 4 2, auto"
        );
        // Non-numeric hotspot falls back to 0.
        assert_eq!(
            compose_cursor("/c.svg", "", "bad"),
            "url('/c.svg') 0 0, auto"
        );
        // CSS-breaking chars are stripped from the URL.
        assert_eq!(
            compose_cursor("a';}body{x", "0", "0"),
            "url('abodyx') 0 0, auto"
        );
        // extract pulls a plain url back out, but skips data: presets.
        assert_eq!(extract_url("url('/c.svg') 4 2, auto"), "/c.svg");
        assert_eq!(extract_url("url('data:image/svg+xml;base64,AAA'), auto"), "");
        assert_eq!(extract_url("auto"), "");
    }
}
