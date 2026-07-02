//! Typography panel — LibreOffice-style.
//!
//! Font pickers are custom dropdowns that preview each face in its own
//! typeface (the panel loads all registry Google Fonts via @import so previews
//! are accurate). Rich controls (size, weight, line-height) sit inline, a live
//! preview pane reflects the current settings, and a per-element section lets
//! each of H1/H2/H3/Body/Quote/Code override size, weight, italic and color.
//! Per-element overrides resolve to CSS in `build_element_typography_css`.

use dioxus::document;
use dioxus::prelude::*;

use crate::app::state::ThemeState;
use crate::ui::components::inputs::EditorInput;
use mor_blogger_core::config::fonts::{
    all_registry_google_fonts_url, resolve_font_stack_with_fallback, FontPreset, FONT_REGISTRY,
    MONO_FONT_REGISTRY,
};
use mor_blogger_core::config::ElementStyle;

/// Fixed elements offered in the per-element section:
/// (css key, tab label, full label, default font-size used by the live preview).
const ELEMENTS: &[(&str, &str, &str, &str)] = &[
    ("h1", "H1", "Heading 1", "1.9em"),
    ("h2", "H2", "Heading 2", "1.45em"),
    ("h3", "H3", "Heading 3", "1.15em"),
    ("p", "Body", "Body text", "1em"),
    ("blockquote", "Quote", "Blockquote", "1em"),
    ("code", "Code", "Inline / code", "0.9em"),
];

const SIZE_PRESETS: &[&str] = &[
    "12px", "13px", "14px", "16px", "18px", "20px", "24px", "1rem", "1.125rem", "1.25rem",
];

const WEIGHTS: &[(&str, &str)] = &[
    ("Regular", "400"),
    ("Medium", "500"),
    ("Semibold", "600"),
    ("Bold", "700"),
];

const SECTION_H: &str = "color: var(--editor-accent, #a9aae2); margin: 4px 0 2px 0; font-size: 13px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px;";

/// Dropdown/option/weight-button styling. Kept free of `&` so it survives as a
/// plain `<style>` text node (Dioxus escapes `&` in text — see the font link,
/// which is injected as a real <link> attribute instead).
const PANEL_CSS: &str = "\
.mor-font-option { padding: 7px 10px; cursor: pointer; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; font-size: 15px; }\
.mor-font-option:hover { background: color-mix(in srgb, var(--accent, #5a5a5c) 35%, transparent); }\
.mor-font-option[data-selected=\"true\"] { background: color-mix(in srgb, var(--accent, #5a5a5c) 25%, transparent); }\
.mor-weight-btn { flex: 1; padding: 6px 4px; font-size: 12px; cursor: pointer; background: var(--bg-elevated, #2c2c2e); color: var(--fg-base, #ddd); border: 1px solid var(--editor-border-soft, #3a3a3c); }\
.mor-weight-btn[data-active=\"true\"] { background: var(--accent, #5a5a5c); color: #fff; }";

#[component]
pub fn TypographyPanel() -> Element {
    let mut app_state = use_context::<ThemeState>();
    let signals = app_state.signals;

    // Editor-side font loading (opt-in: only fetched while this panel is mounted).
    // Injected as a real <link> so the many `&family=` separators in the URL are
    // NOT HTML-escaped (an inline `@import` text node would become `&amp;` and 404).
    let fonts_url = all_registry_google_fonts_url();

    rsx! {
        if !fonts_url.is_empty() {
            document::Stylesheet { href: fonts_url }
        }
        style { "{PANEL_CSS}" }

        section { class: "editor-card",

            p {
                class: "editor-mini-label",
                title: "This panel fetches Google Fonts so previews are accurate. Exported Blogger XML loads its own font links.",
                "Font previews use live Google Fonts. ⓘ"
            }

            // --- Fonts -------------------------------------------------------
            h4 { style: "{SECTION_H}", "Fonts" }
            FontPreviewSelect {
                label: "Body Font".to_string(),
                value: signals.body_font_stack,
                options: FONT_REGISTRY,
                include_match_body: false,
            }
            FontPreviewSelect {
                label: "Heading Font".to_string(),
                value: signals.heading_font_stack,
                options: FONT_REGISTRY,
                include_match_body: true,
            }
            FontPreviewSelect {
                label: "Monospace Font".to_string(),
                value: signals.mono_font_stack,
                options: MONO_FONT_REGISTRY,
                include_match_body: false,
            }

            // --- Base controls ----------------------------------------------
            h4 { style: "{SECTION_H} margin-top: 12px;", "Base" }
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div { class: "editor-field-group",
                    label { class: "editor-field-label", "Base Size" }
                    input {
                        r#type: "text",
                        class: "editor-field",
                        list: "mor-type-sizes",
                        value: "{signals.base_size}",
                        placeholder: "16px",
                        oninput: move |e| signals.base_size.clone().set(e.value()),
                    }
                    datalist { id: "mor-type-sizes",
                        for s in SIZE_PRESETS.iter() {
                            option { value: "{s}" }
                        }
                    }
                }
                EditorInput {
                    label: "Line Height".to_string(),
                    value: signals.line_height,
                    input_type: "text".to_string(),
                    placeholder: "1.6".to_string(),
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Heading Weight" }
                div { style: "display: flex; gap: 4px;",
                    for (name, w) in WEIGHTS.iter() {
                        button {
                            class: "mor-weight-btn",
                            "data-active": "{signals.heading_weight.read().as_str() == *w}",
                            onclick: move |_| signals.heading_weight.clone().set(w.to_string()),
                            "{name}"
                        }
                    }
                }
            }

            // --- Live preview ------------------------------------------------
            h4 { style: "{SECTION_H} margin-top: 12px;", "Preview" }
            TypographyPreview { signals }

            // --- Per-element overrides --------------------------------------
            h4 { style: "{SECTION_H} margin-top: 12px;", "Per-Element Styles" }
            p { class: "editor-mini-label", style: "margin-top: 0;", "Pick an element, then override only what you need. Blank = inherit. Applies to preview and export." }
            ElementStyleEditor { elements: signals.type_elements }

            button {
                class: "mor-btn-secondary",
                style: "width: 100%; margin-top: 14px;",
                onclick: move |_| app_state.show_advanced_typography.set(true),
                "⚙ Font Files & Scaling"
            }
        }
    }
}

/// Live sample, reflecting both the global signals AND per-element overrides
/// (so the preview is true WYSIWYG for the per-element editor below).
#[component]
fn TypographyPreview(signals: crate::app::theme_signals::ThemeSignals) -> Element {
    let body_ff = resolve_font_stack_with_fallback(&signals.body_font_stack.read(), false);
    let heading_raw = signals.heading_font_stack.read().clone();
    let heading_ff = if heading_raw.trim().is_empty() {
        body_ff.clone()
    } else {
        resolve_font_stack_with_fallback(&heading_raw, false)
    };
    let mono_ff = resolve_font_stack_with_fallback(&signals.mono_font_stack.read(), true);
    let base = signals.base_size.read().clone();
    let lh = signals.line_height.read().clone();
    let hweight = signals.heading_weight.read().clone();

    let els = signals.type_elements.read().clone();
    let ov = |sel: &str| els.iter().find(|e| e.selector == sel).cloned();

    let s_h1 = sample_style(&heading_ff, "1.9em", &hweight, ov("h1").as_ref(), "");
    let s_h2 = sample_style(&heading_ff, "1.45em", &hweight, ov("h2").as_ref(), "");
    let s_h3 = sample_style(&heading_ff, "1.15em", &hweight, ov("h3").as_ref(), "");
    let s_p = sample_style(&body_ff, "1em", "", ov("p").as_ref(), "");
    let s_quote = sample_style(
        &body_ff,
        "1em",
        "",
        ov("blockquote").as_ref(),
        "padding-left: 12px; border-left: 3px solid #ccc; color: #555; font-style: italic;",
    );
    let s_code = sample_style(
        &mono_ff,
        "0.9em",
        "",
        ov("code").as_ref(),
        "background: #f0f0f0; padding: 1px 5px; border-radius: 3px;",
    );

    rsx! {
        div {
            style: "background: #fff; color: #111; border: 1px solid var(--editor-border-soft, #3a3a3c); border-radius: 6px; padding: 14px; font-family: {body_ff}; font-size: {base}; line-height: {lh};",
            h1 { style: "{s_h1} margin: 0 0 6px 0;", "Heading One" }
            h2 { style: "{s_h2} margin: 0 0 6px 0;", "Heading Two" }
            h3 { style: "{s_h3} margin: 0 0 8px 0;", "Heading Three" }
            p { style: "{s_p} margin: 0 0 8px 0;",
                "The quick brown fox jumps over the lazy dog — typography sets the tone of the whole page."
            }
            blockquote { style: "{s_quote} margin: 0 0 8px 0;",
                "A blockquote shows how quoted passages will look."
            }
            p { style: "{s_p} margin: 0;",
                "Inline "
                code { style: "{s_code}", "monospace code" }
                " sample."
            }
        }
    }
}

/// Build an inline style for one preview sample: base defaults merged with the
/// element's override (override wins per-property), plus any static `extra`.
fn sample_style(
    family: &str,
    default_size: &str,
    default_weight: &str,
    ov: Option<&ElementStyle>,
    extra: &str,
) -> String {
    let nz = |s: &str| {
        let t = s.trim();
        (!t.is_empty()).then(|| t.to_string())
    };
    let size = ov
        .and_then(|o| nz(&o.font_size))
        .unwrap_or_else(|| default_size.to_string());
    let mut s = format!("font-family: {}; font-size: {};", family, size);

    let weight = ov
        .and_then(|o| nz(&o.font_weight))
        .unwrap_or_else(|| default_weight.to_string());
    if !weight.is_empty() {
        s.push_str(&format!(" font-weight: {};", weight));
    }
    if let Some(o) = ov {
        if let Some(v) = nz(&o.line_height) {
            s.push_str(&format!(" line-height: {};", v));
        }
        if let Some(v) = nz(&o.letter_spacing) {
            s.push_str(&format!(" letter-spacing: {};", v));
        }
        if let Some(v) = nz(&o.color) {
            s.push_str(&format!(" color: {};", v));
        }
        if o.italic {
            s.push_str(" font-style: italic;");
        }
    }
    if !extra.is_empty() {
        s.push(' ');
        s.push_str(extra);
    }
    s
}

/// LibreOffice-style per-element editor: pick an element via the tab row, then
/// edit its full style with room to breathe (replaces the cramped grid rows).
#[component]
fn ElementStyleEditor(elements: Signal<Vec<ElementStyle>>) -> Element {
    let mut active = use_signal(|| 0usize);
    let idx = active().min(ELEMENTS.len() - 1);
    let (selector, _tab, full, _def) = ELEMENTS[idx];

    let cur = elements
        .read()
        .iter()
        .find(|e| e.selector == selector)
        .cloned()
        .unwrap_or(ElementStyle {
            selector: selector.to_string(),
            ..Default::default()
        });
    let italic_now = cur.italic;
    let has_any = elements.read().iter().any(|e| e.selector == selector);

    rsx! {
        // Element tabs (• marks elements that carry an override).
        div { style: "display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 10px;",
            for (i, (sel, tab, _f, _d)) in ELEMENTS.iter().enumerate() {
                button {
                    class: "mor-weight-btn",
                    style: "flex: 0 0 auto; min-width: 42px;",
                    "data-active": "{i == idx}",
                    onclick: move |_| active.set(i),
                    "{tab}"
                    if elements.read().iter().any(|e| e.selector == *sel) {
                        span { style: "color: var(--accent, #8ab4f8); margin-left: 3px;", "•" }
                    }
                }
            }
        }

        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 10px;",
            div { class: "editor-field-group", style: "margin: 0;",
                label { class: "editor-field-label", "Size" }
                input {
                    r#type: "text", class: "editor-field", list: "mor-type-sizes",
                    value: "{cur.font_size}", placeholder: "inherit",
                    oninput: move |e| update_element(elements, selector, |s| s.font_size = e.value()),
                }
            }
            div { class: "editor-field-group", style: "margin: 0;",
                label { class: "editor-field-label", "Weight" }
                select {
                    class: "editor-select",
                    value: "{cur.font_weight}",
                    onchange: move |e| update_element(elements, selector, |s| s.font_weight = e.value()),
                    option { value: "", selected: cur.font_weight.is_empty(), "Inherit" }
                    for (name, w) in WEIGHTS.iter() {
                        option { value: "{w}", selected: cur.font_weight == *w, "{name} ({w})" }
                    }
                }
            }
            div { class: "editor-field-group", style: "margin: 0;",
                label { class: "editor-field-label", "Line Height" }
                input {
                    r#type: "text", class: "editor-field",
                    value: "{cur.line_height}", placeholder: "inherit",
                    oninput: move |e| update_element(elements, selector, |s| s.line_height = e.value()),
                }
            }
            div { class: "editor-field-group", style: "margin: 0;",
                label { class: "editor-field-label", "Letter Spacing" }
                input {
                    r#type: "text", class: "editor-field",
                    value: "{cur.letter_spacing}", placeholder: "e.g. 0.5px",
                    oninput: move |e| update_element(elements, selector, |s| s.letter_spacing = e.value()),
                }
            }
        }

        div { style: "display: flex; gap: 8px; align-items: center; margin-top: 8px;",
            button {
                class: "mor-weight-btn",
                style: "flex: 0 0 auto; min-width: 64px; font-style: italic;",
                "data-active": "{italic_now}",
                onclick: move |_| update_element(elements, selector, move |s| s.italic = !italic_now),
                "Italic"
            }
            input {
                r#type: "text", class: "editor-field", style: "flex: 1; margin: 0;",
                value: "{cur.color}", placeholder: "color (hex, blank = inherit)",
                oninput: move |e| update_element(elements, selector, |s| s.color = e.value()),
            }
            if has_any {
                button {
                    class: "mor-btn-secondary",
                    style: "flex: 0 0 auto;",
                    title: "Clear {full} overrides",
                    onclick: move |_| { elements.write().retain(|e| e.selector != selector); },
                    "Clear"
                }
            }
        }
    }
}

/// Find-or-create the override for `selector`, apply `f`, then drop it again if
/// it ended up all-default (keeps saved configs clean).
fn update_element(
    mut elements: Signal<Vec<ElementStyle>>,
    selector: &str,
    f: impl FnOnce(&mut ElementStyle),
) {
    let mut v = elements.write();
    let idx = v.iter().position(|e| e.selector == selector);
    let mut entry = match idx {
        Some(i) => v[i].clone(),
        None => ElementStyle {
            selector: selector.to_string(),
            ..Default::default()
        },
    };
    f(&mut entry);

    let is_default = entry.font_size.trim().is_empty()
        && entry.font_weight.trim().is_empty()
        && entry.line_height.trim().is_empty()
        && entry.letter_spacing.trim().is_empty()
        && entry.color.trim().is_empty()
        && !entry.italic;

    match idx {
        Some(i) if is_default => {
            v.remove(i);
        }
        Some(i) => v[i] = entry,
        None if is_default => {}
        None => v.push(entry),
    }
}

/// Custom font dropdown with per-option previews. Keeps the "Match body" and
/// custom/Google-font escape hatches of the original plain `<select>`.
#[component]
fn FontPreviewSelect(
    label: String,
    value: Signal<String>,
    options: &'static [FontPreset],
    include_match_body: bool,
) -> Element {
    let mut value = value;
    let mut open = use_signal(|| false);
    let mut force_custom = use_signal(|| false);

    let current = value.read().clone();
    let current_trimmed = current.trim().to_string();

    let matched = options.iter().find(|f| {
        f.name.eq_ignore_ascii_case(&current_trimmed)
            || f.css_stack.eq_ignore_ascii_case(&current_trimmed)
    });
    let is_match_body =
        include_match_body && current_trimmed.is_empty() && !force_custom();
    let is_custom =
        force_custom() || (!is_match_body && matched.is_none() && !current_trimmed.is_empty());

    let (display, preview_ff) = if is_custom {
        let d = if current_trimmed.is_empty() {
            "Custom / Google Font…".to_string()
        } else {
            current_trimmed.clone()
        };
        let ff = if current_trimmed.is_empty() {
            "inherit".to_string()
        } else {
            resolve_font_stack_with_fallback(&current_trimmed, false)
        };
        (d, ff)
    } else if is_match_body {
        ("Match body".to_string(), "inherit".to_string())
    } else if let Some(f) = matched {
        (f.name.to_string(), f.css_stack.to_string())
    } else {
        ("Select…".to_string(), "inherit".to_string())
    };

    rsx! {
        div { class: "editor-field-group", style: "position: relative;",
            label { class: "editor-field-label", "{label}" }

            button {
                class: "editor-select",
                style: "width: 100%; text-align: left; display: flex; justify-content: space-between; align-items: center; cursor: pointer; font-family: {preview_ff};",
                onclick: move |_| { let o = *open.read(); open.set(!o); },
                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{display}" }
                span { style: "opacity: 0.6; font-family: initial; margin-left: 6px;", "▾" }
            }

            if open() {
                // Click-catcher closes the popup when clicking elsewhere.
                div {
                    style: "position: fixed; inset: 0; z-index: 40;",
                    onclick: move |_| open.set(false),
                }
                div {
                    style: "position: absolute; left: 0; right: 0; top: 100%; z-index: 50; max-height: 340px; overflow-y: auto; margin-top: 4px; background: var(--bg-elevated, #2c2c2e); border: 1px solid var(--editor-border-soft, #3a3a3c); border-radius: 6px; box-shadow: 0 8px 24px rgba(0,0,0,0.45);",

                    if include_match_body {
                        div {
                            class: "mor-font-option",
                            "data-selected": "{is_match_body}",
                            onclick: move |_| { force_custom.set(false); value.set(String::new()); open.set(false); },
                            "Match body"
                        }
                    }

                    for (i, font) in options.iter().enumerate() {
                        if i == 0 || options[i - 1].category != font.category {
                            div {
                                style: "padding: 6px 10px 2px; font-size: 10px; text-transform: uppercase; letter-spacing: 0.5px; opacity: 0.5; font-family: initial;",
                                "{font.category}"
                            }
                        }
                        div {
                            class: "mor-font-option",
                            style: "font-family: {font.css_stack};",
                            "data-selected": "{matched.map(|m| m.name == font.name).unwrap_or(false)}",
                            onclick: move |_| { force_custom.set(false); value.set(font.name.to_string()); open.set(false); },
                            "{font.name}"
                        }
                    }

                    div {
                        class: "mor-font-option",
                        style: "font-family: initial; border-top: 1px solid var(--editor-border-soft, #3a3a3c);",
                        "data-selected": "{is_custom}",
                        onclick: move |_| { force_custom.set(true); open.set(false); },
                        "Custom / Google Font…"
                    }
                }
            }

            if is_custom {
                input {
                    r#type: "text",
                    value: "{current}",
                    placeholder: "e.g. Fira Code, Noto Serif KR, or Georgia, serif",
                    class: "editor-field",
                    style: "width: 100%; margin-top: 8px;",
                    oninput: move |e| value.set(e.value()),
                }
            }
        }
    }
}
