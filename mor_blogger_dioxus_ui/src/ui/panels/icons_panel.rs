use dioxus::prelude::*;

use mor_blogger_core::config::gtk_theme::svg_to_mask_uri;
use mor_blogger_core::config::ThemeConfig;

const ICON_REPO_URL: &str = "https://github.com/lucide-icons/lucide/tree/main/icons";
const ALT_ICON_REPO_URL: &str = "https://github.com/tabler/tabler-icons/tree/main/icons";

const CLOSE_X: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.4' stroke-linecap='round' stroke-linejoin='round'><path d='M18 6 6 18'/><path d='m6 6 12 12'/></svg>"#;
const SEARCH: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><circle cx='11' cy='11' r='7'/><path d='m20 20-3.5-3.5'/></svg>"#;
const MENU: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.4' stroke-linecap='round'><path d='M4 7h16'/><path d='M4 12h16'/><path d='M4 17h16'/></svg>"#;
const SIDEBAR_LEFT: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.1' stroke-linecap='round' stroke-linejoin='round'><rect x='3' y='4' width='18' height='16' rx='2'/><path d='M9 4v16'/><path d='M6 8h.01'/><path d='M6 12h.01'/></svg>"#;
const SIDEBAR_RIGHT: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.1' stroke-linecap='round' stroke-linejoin='round'><rect x='3' y='4' width='18' height='16' rx='2'/><path d='M15 4v16'/><path d='M18 8h.01'/><path d='M18 12h.01'/></svg>"#;
const CHEVRON_LEFT: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.6' stroke-linecap='round' stroke-linejoin='round'><path d='m15 18-6-6 6-6'/></svg>"#;
const CHEVRON_RIGHT: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.6' stroke-linecap='round' stroke-linejoin='round'><path d='m9 18 6-6-6-6'/></svg>"#;
const HOME: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><path d='m3 11 9-8 9 8'/><path d='M5 10v10h14V10'/><path d='M9 20v-6h6v6'/></svg>"#;
const ARCHIVE: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.1' stroke-linecap='round' stroke-linejoin='round'><rect x='3' y='4' width='18' height='4' rx='1'/><path d='M5 8v12h14V8'/><path d='M10 12h4'/></svg>"#;
const TAG: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2.1' stroke-linecap='round' stroke-linejoin='round'><path d='M20 13 12 21 3 12V3h9l8 8a2 2 0 0 1 0 2Z'/><circle cx='7.5' cy='7.5' r='1.5'/></svg>"#;

const ICON_PRESETS: [(&str, &str); 10] = [
    ("Close X", CLOSE_X),
    ("Search", SEARCH),
    ("Menu", MENU),
    ("Sidebar Left", SIDEBAR_LEFT),
    ("Sidebar Right", SIDEBAR_RIGHT),
    ("Chevron Left", CHEVRON_LEFT),
    ("Chevron Right", CHEVRON_RIGHT),
    ("Home", HOME),
    ("Archive", ARCHIVE),
    ("Tag", TAG),
];

#[component]
pub fn SvgIconsPanel(
    current_config: ThemeConfig,
    on_apply_theme: EventHandler<ThemeConfig>,
) -> Element {
    let mut selected_slot = use_signal(|| "panel_close".to_string());
    let mut custom_svg = use_signal(String::new);
    let mut icon_url = use_signal(String::new);
    let mut status = use_signal(String::new);

    let selected_slot_id = selected_slot();
    let selected_slot_label = slot_label(&selected_slot_id);
    let current_slot_value = icon_value_for_slot(&current_config, &selected_slot_id).to_string();

    rsx! {
        div { class: "editor-card",
            h3 { class: "editor-card-title", "SVG Icons" }

            p { class: "editor-help",
                "Each icon can be "
                strong { "embedded" }
                " (the SVG is encoded into the theme as a "
                code { "data:image/svg+xml" }
                " value, so the export stays a single self-contained file) or "
                strong { "linked" }
                " to an external "
                code { "https" }
                " URL. Embedded icons are tinted to match the theme; linked icons keep their own colors, so pick ones that already suit your palette."
            }

            div {
                style: "display: grid; grid-template-columns: 1fr; gap: 8px; margin: 12px 0;",
                IconPreview { label: "Panel Close", value: current_config.icons.panel_close.clone() }
                IconPreview { label: "Search", value: current_config.icons.search.clone() }
                IconPreview { label: "Menu", value: current_config.icons.menu.clone() }
                IconPreview { label: "Left Sidebar", value: current_config.icons.sidebar_left.clone() }
                IconPreview { label: "Right Sidebar", value: current_config.icons.sidebar_right.clone() }
            }

            div { class: "editor-button-row", style: "display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 12px;",
                button {
                    class: "editor-button",
                    onclick: {
                        let current_config = current_config.clone();
                        move |_| {
                            let mut next = current_config.clone();
                            next.icons.panel_close = svg_to_mask_uri(&sanitize_svg(CLOSE_X));
                            next.icons.search = svg_to_mask_uri(&sanitize_svg(SEARCH));
                            next.icons.menu = svg_to_mask_uri(&sanitize_svg(MENU));
                            next.icons.sidebar_left = svg_to_mask_uri(&sanitize_svg(SIDEBAR_LEFT));
                            next.icons.sidebar_right = svg_to_mask_uri(&sanitize_svg(SIDEBAR_RIGHT));
                            on_apply_theme.call(next);
                            status.set("Applied clean GTK-style icon set.".to_string());
                        }
                    },
                    "Apply Clean GTK Set"
                }

                button {
                    class: "editor-button",
                    onclick: {
                        let current_config = current_config.clone();
                        move |_| {
                            let mut next = current_config.clone();
                            next.icons.sidebar_left = svg_to_mask_uri(&sanitize_svg(CHEVRON_LEFT));
                            next.icons.sidebar_right = svg_to_mask_uri(&sanitize_svg(CHEVRON_RIGHT));
                            on_apply_theme.call(next);
                            status.set("Applied chevron sidebar icons.".to_string());
                        }
                    },
                    "Use Chevron Sidebars"
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Icon Slot" }
                select {
                    class: "editor-input",
                    style: "width: 100%;",
                    value: "{selected_slot_id}",
                    onchange: move |evt| {
                        selected_slot.set(evt.value().clone());
                        status.set(String::new());
                    },
                    option { value: "panel_close", "Panel Close" }
                    option { value: "search", "Search" }
                    option { value: "menu", "Menu" }
                    option { value: "sidebar_left", "Left Sidebar" }
                    option { value: "sidebar_right", "Right Sidebar" }
                }
            }

            div { class: "editor-field-group", style: "margin-top: 12px;",
                label { class: "editor-field-label", "Preset Icons" }
                div { style: "display: flex; flex-wrap: wrap; gap: 6px;",
                    for (preset_name, preset_svg) in ICON_PRESETS {
                        button {
                            key: "{preset_name}",
                            class: "editor-button editor-button-small",
                            onclick: {
                                let current_config = current_config.clone();
                                move |_| {
                                    let slot = selected_slot();
                                    let icon_value = svg_to_mask_uri(&sanitize_svg(preset_svg));
                                    let next = set_icon_by_slot(current_config.clone(), &slot, icon_value);
                                    on_apply_theme.call(next);
                                    status.set(format!("Embedded {} into {}.", preset_name, slot_label(&slot)));
                                }
                            },
                            "{preset_name}"
                        }
                    }
                }
            }

            div { class: "editor-field-group", style: "margin-top: 12px;",
                label { class: "editor-field-label", "Raw SVG Editor" }
                textarea {
                    class: "editor-input",
                    style: "width: 100%; min-height: 130px; resize: vertical; font-family: monospace;",
                    rows: "7",
                    value: "{custom_svg}",
                    placeholder: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'>...</svg>",
                    oninput: move |evt| {
                        custom_svg.set(evt.value().clone());
                        status.set(String::new());
                    }
                }

                div { class: "editor-button-row", style: "display: flex; flex-wrap: wrap; gap: 8px; margin-top: 8px;",
                    button {
                        class: "editor-button editor-button-small",
                        onclick: {
                            let value = current_slot_value.clone();
                            let label = selected_slot_label;
                            move |_| {
                                if is_external_link(&value) {
                                    icon_url.set(css_url_inner(&value));
                                    custom_svg.set(String::new());
                                    status.set(format!("Loaded current {} icon as an external URL.", label));
                                } else {
                                    let raw = mask_uri_to_raw_svg(&value).unwrap_or_else(|| value.clone());
                                    custom_svg.set(raw);
                                    status.set(format!("Loaded current {} icon as raw SVG.", label));
                                }
                            }
                        },
                        "Load Current Slot"
                    }

                    button {
                        class: "editor-button primary",
                        onclick: {
                            let current_config = current_config.clone();
                            move |_| {
                                match normalize_icon_input(&custom_svg()) {
                                    Ok(icon_value) => {
                                        let slot = selected_slot();
                                        let next = set_icon_by_slot(current_config.clone(), &slot, icon_value);
                                        on_apply_theme.call(next);
                                        status.set(format!("Embedded SVG into {}.", slot_label(&slot)));
                                    }
                                    Err(err) => status.set(err),
                                }
                            }
                        },
                        "Convert & Apply"
                    }
                }
            }

            div { class: "editor-field-group", style: "margin-top: 12px;",
                label { class: "editor-field-label", "External SVG URL" }
                input {
                    class: "editor-input",
                    r#type: "url",
                    style: "width: 100%;",
                    value: "{icon_url}",
                    placeholder: "https://example.com/icons/menu.svg",
                    oninput: move |evt| {
                        icon_url.set(evt.value().clone());
                        status.set(String::new());
                    }
                }
                p { class: "editor-help", style: "margin-top: 6px;",
                    "Links the icon instead of embedding it. Must be "
                    code { "https" }
                    " (Blogger blocks mixed http content). The icon keeps its own colors and is loaded from this host by every visitor's browser \u{2014} so use somewhere reliable, since a dead link is a missing icon."
                }

                div { class: "editor-button-row", style: "display: flex; flex-wrap: wrap; gap: 8px; margin-top: 8px;",
                    button {
                        class: "editor-button primary",
                        onclick: {
                            let current_config = current_config.clone();
                            move |_| {
                                match normalize_icon_url(&icon_url()) {
                                    Ok(icon_value) => {
                                        let slot = selected_slot();
                                        let next = set_icon_by_slot(current_config.clone(), &slot, icon_value);
                                        on_apply_theme.call(next);
                                        status.set(format!("Linked external SVG to {}.", slot_label(&slot)));
                                    }
                                    Err(err) => status.set(err),
                                }
                            }
                        },
                        "Link URL to Slot"
                    }
                }
            }

            if !status().is_empty() {
                div { class: "restore-status", style: "margin-top: 10px;", "{status}" }
            }

            div { class: "editor-help", style: "margin-top: 12px;",
                "More SVG icons: "
                a {
                    href: ICON_REPO_URL,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Lucide Icons"
                }
                " \u{00b7} "
                a {
                    href: ALT_ICON_REPO_URL,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Tabler Icons"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct IconPreviewProps {
    label: &'static str,
    value: String,
}

#[component]
fn IconPreview(props: IconPreviewProps) -> Element {
    let style = icon_preview_style(&props.value);
    let kind = if props.value.trim().is_empty() {
        "unset"
    } else if is_external_link(&props.value) {
        "linked"
    } else {
        "embedded"
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; min-height: 24px;",
            span { style: "{style}" }
            span { style: "font-size: 0.85rem;", "{props.label}" }
            span { style: "margin-left: auto; font-size: 0.7rem; opacity: 0.6; text-transform: uppercase; letter-spacing: 0.04em;", "{kind}" }
        }
    }
}

fn icon_value_for_slot<'a>(config: &'a ThemeConfig, slot: &str) -> &'a str {
    match slot {
        "search" => &config.icons.search,
        "menu" => &config.icons.menu,
        "sidebar_left" => &config.icons.sidebar_left,
        "sidebar_right" => &config.icons.sidebar_right,
        _ => &config.icons.panel_close,
    }
}

fn set_icon_by_slot(mut config: ThemeConfig, slot: &str, icon_value: String) -> ThemeConfig {
    match slot {
        "search" => config.icons.search = icon_value,
        "menu" => config.icons.menu = icon_value,
        "sidebar_left" => config.icons.sidebar_left = icon_value,
        "sidebar_right" => config.icons.sidebar_right = icon_value,
        _ => config.icons.panel_close = icon_value,
    }

    config
}

fn slot_label(slot: &str) -> &'static str {
    match slot {
        "search" => "Search",
        "menu" => "Menu",
        "sidebar_left" => "Left Sidebar",
        "sidebar_right" => "Right Sidebar",
        _ => "Panel Close",
    }
}

/// Choose a preview rendering that matches how the icon is meant to be shown:
/// embedded SVGs are tinted as a mask (the way the export tints them today),
/// while external links are drawn as-is on a neutral tile so their real colors
/// are visible. An empty slot shows a dashed placeholder.
fn icon_preview_style(value: &str) -> String {
    let value = value.trim();

    if value.is_empty() {
        return "display:inline-block;width:18px;height:18px;flex:0 0 18px;\
                border:1px dashed rgba(255,255,255,0.25);border-radius:4px;"
            .to_string();
    }

    if is_external_link(value) {
        format!(
            "display:inline-block;width:18px;height:18px;flex:0 0 18px;\
             background-color:#f5f5f5;border:1px solid rgba(0,0,0,0.15);border-radius:4px;\
             background-image:{v};background-size:contain;background-repeat:no-repeat;background-position:center;",
            v = value
        )
    } else {
        format!(
            "display:inline-block;width:18px;height:18px;flex:0 0 18px;\
             background-color:var(--editor-accent, #6eb6ff);\
             -webkit-mask-image:{v};mask-image:{v};\
             -webkit-mask-size:contain;mask-size:contain;\
             -webkit-mask-repeat:no-repeat;mask-repeat:no-repeat;\
             -webkit-mask-position:center;mask-position:center;",
            v = value
        )
    }
}

/// True when the stored CSS value points at a remote URL rather than an
/// embedded `data:` payload.
fn is_external_link(value: &str) -> bool {
    let inner = css_url_inner(value);
    inner.starts_with("https://") || inner.starts_with("http://")
}

/// Strip a `url('...')` wrapper (and surrounding quotes) down to the inner value.
fn css_url_inner(value: &str) -> String {
    let mut v = value.trim();

    if v.starts_with("url(") && v.ends_with(')') {
        v = v[4..v.len() - 1].trim();
    }

    v.trim_matches('"').trim_matches('\'').trim().to_string()
}

/// Validate an external icon URL and wrap it as a CSS `url(...)` value.
fn normalize_icon_url(input: &str) -> Result<String, String> {
    let url = input.trim();

    if url.is_empty() {
        return Err("Enter an https:// SVG URL first.".to_string());
    }

    if url.starts_with("http://") {
        return Err(
            "Use an https:// URL \u{2014} Blogger serves over https and blocks mixed (http) content."
                .to_string(),
        );
    }

    if !url.starts_with("https://") {
        return Err("URL must start with https://".to_string());
    }

    if url.contains(['\'', '"', ' ', '\n', '\r', '\t', '(', ')']) {
        return Err("URL contains characters that aren't safe inside a CSS url() value.".to_string());
    }

    Ok(format!("url('{}')", url))
}

/// Turn whatever the user pasted into a storable CSS value. Accepts an
/// https URL, an already-encoded `url(data:image/svg+xml,...)` value, or raw
/// `<svg>` markup (which gets sanitized and encoded as a data URI).
fn normalize_icon_input(input: &str) -> Result<String, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Paste raw SVG, a url(data:...) value, or an https:// link first.".to_string());
    }

    // External link pasted into the raw box -> route through URL validation.
    if input.starts_with("http://") || input.starts_with("https://") {
        return normalize_icon_url(input);
    }

    // Already an embedded mask value -> accept verbatim.
    if input.starts_with("url(") && input.contains("data:image/svg+xml") {
        return Ok(input.to_string());
    }

    // Otherwise treat it as raw SVG (decoding a data URI back to markup if needed).
    let raw_svg = mask_uri_to_raw_svg(input).unwrap_or_else(|| input.to_string());

    if !raw_svg.contains("<svg") {
        return Err(
            "Paste raw <svg> code, a url(data:image/svg+xml,...) value, or an https:// SVG link."
                .to_string(),
        );
    }

    if svg_has_active_content(&raw_svg) {
        return Err(
            "Rejected SVG because it contains scripts, event handlers, or javascript: links."
                .to_string(),
        );
    }

    Ok(svg_to_mask_uri(&sanitize_svg(&raw_svg)))
}

/// Reject SVGs that can execute code once embedded.
fn svg_has_active_content(raw_svg: &str) -> bool {
    let lower = raw_svg.to_lowercase();
    const BLOCKED: [&str; 7] = [
        "<script",
        "javascript:",
        "onload=",
        "onclick=",
        "onerror=",
        "onmouseover=",
        "<foreignobject",
    ];
    BLOCKED.iter().any(|needle| lower.contains(needle))
}

fn sanitize_svg(input: &str) -> String {
    let mut svg = input
        .trim()
        .trim_start_matches('\u{feff}')
        .replace("]]>", "");

    if svg.starts_with("<?xml") {
        if let Some(end) = svg.find("?>") {
            svg = svg[end + 2..].trim_start().to_string();
        }
    }

    if svg.starts_with("<!DOCTYPE") {
        if let Some(end) = svg.find('>') {
            svg = svg[end + 1..].trim_start().to_string();
        }
    }

    if svg.starts_with("<svg") && !svg.contains("xmlns=") {
        svg = svg.replacen("<svg", "<svg xmlns='http://www.w3.org/2000/svg'", 1);
    }

    svg
}

fn mask_uri_to_raw_svg(input: &str) -> Option<String> {
    let value = css_url_inner(input);
    let payload = svg_payload_from_data_uri(&value)?;
    Some(percent_decode(payload))
}

fn svg_payload_from_data_uri(value: &str) -> Option<&str> {
    const MARKERS: [&str; 3] = [
        "data:image/svg+xml,",
        "data:image/svg+xml;utf8,",
        "data:image/svg+xml;charset=utf-8,",
    ];

    for marker in MARKERS {
        if let Some(index) = value.find(marker) {
            return Some(&value[index + marker.len()..]);
        }
    }

    None
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (hex_value(bytes[i + 1]), hex_value(bytes[i + 2])) {
                output.push((high << 4) | low);
                i += 3;
                continue;
            }
        }

        output.push(bytes[i]);
        i += 1;
    }

    String::from_utf8_lossy(&output).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}