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
    let mut status = use_signal(String::new);

    let selected_slot_id = selected_slot();
    let selected_slot_label = slot_label(&selected_slot_id);
    let current_slot_value = icon_value_for_slot(&current_config, &selected_slot_id).to_string();
    let current_slot_raw =
        mask_uri_to_raw_svg(&current_slot_value).unwrap_or_else(|| current_slot_value.clone());

    rsx! {
        div { class: "editor-card",
            h3 { class: "editor-card-title", "SVG Icons" }

            p { class: "editor-help",
                "Blogger export uses encoded CSS mask URLs. This panel lets you edit raw SVG, then converts it into a safe "
                code { "url(data:image/svg+xml,...)" }
                " value for the XML."
            }

            div {
                style: "display: grid; grid-template-columns: 1fr; gap: 8px; margin: 12px 0;",
                IconPreview { label: "Panel Close", mask_uri: current_config.icons.panel_close.clone() }
                IconPreview { label: "Search", mask_uri: current_config.icons.search.clone() }
                IconPreview { label: "Menu", mask_uri: current_config.icons.menu.clone() }
                IconPreview { label: "Left Sidebar", mask_uri: current_config.icons.sidebar_left.clone() }
                IconPreview { label: "Right Sidebar", mask_uri: current_config.icons.sidebar_right.clone() }
            }

            div { class: "editor-button-row", style: "display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 12px;",
                button {
                    class: "editor-button",
                    onclick: {
                        let current_config = current_config.clone();
                        let on_apply_theme = on_apply_theme.clone();

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
                        let on_apply_theme = on_apply_theme.clone();

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
                                let on_apply_theme = on_apply_theme.clone();

                                move |_| {
                                    let slot = selected_slot();
                                    let icon_value = svg_to_mask_uri(&sanitize_svg(preset_svg));
                                    let next = set_icon_by_slot(current_config.clone(), &slot, icon_value);
                                    on_apply_theme.call(next);
                                    status.set(format!("Applied {} to {}.", preset_name, slot_label(&slot)));
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
                            let raw = current_slot_raw.clone();
                            move |_| {
                                custom_svg.set(raw.clone());
                                status.set(format!("Loaded current {} icon as raw SVG.", selected_slot_label));
                            }
                        },
                        "Load Current Slot"
                    }

                    button {
                        class: "editor-button primary",
                        onclick: {
                            let current_config = current_config.clone();
                            let on_apply_theme = on_apply_theme.clone();

                            move |_| {
                                let raw = custom_svg();

                                match normalize_icon_input(&raw) {
                                    Ok(icon_value) => {
                                        let slot = selected_slot();
                                        let next = set_icon_by_slot(current_config.clone(), &slot, icon_value);
                                        on_apply_theme.call(next);
                                        status.set(format!("Converted raw SVG and applied it to {}.", slot_label(&slot)));
                                    }
                                    Err(err) => status.set(err),
                                }
                            }
                        },
                        "Convert & Apply"
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
                " · "
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
    mask_uri: String,
}

#[component]
fn IconPreview(props: IconPreviewProps) -> Element {
    let style = mask_style(&props.mask_uri);

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; min-height: 24px;",
            span { style: "{style}" }
            span { style: "font-size: 0.85rem;", "{props.label}" }
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

fn mask_style(mask_uri: &str) -> String {
    format!(
        "display:inline-block;width:18px;height:18px;flex:0 0 18px;background-color:var(--editor-accent, #6eb6ff);-webkit-mask-image:{};mask-image:{};-webkit-mask-size:contain;mask-size:contain;-webkit-mask-repeat:no-repeat;mask-repeat:no-repeat;-webkit-mask-position:center;mask-position:center;",
        mask_uri,
        mask_uri
    )
}

fn normalize_icon_input(input: &str) -> Result<String, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Paste raw SVG first.".to_string());
    }

    if input.starts_with("url(") && input.contains("data:image/svg+xml") {
        return Ok(input.to_string());
    }

    let raw_svg = mask_uri_to_raw_svg(input).unwrap_or_else(|| input.to_string());

    if !raw_svg.contains("<svg") {
        return Err(
            "Paste raw <svg> code or a CSS url(data:image/svg+xml,...) mask value.".to_string(),
        );
    }

    let lower = raw_svg.to_lowercase();

    if lower.contains("<script") || lower.contains("onload=") || lower.contains("onclick=") {
        return Err(
            "Rejected SVG because it contains script or inline event handlers.".to_string(),
        );
    }

    Ok(svg_to_mask_uri(&sanitize_svg(&raw_svg)))
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
    let mut value = input.trim();

    if value.starts_with("url(") && value.ends_with(')') {
        value = value
            .trim_start_matches("url(")
            .trim_end_matches(')')
            .trim();
    }

    value = value.trim_matches('"').trim_matches('\'');

    let payload = svg_payload_from_data_uri(value)?;
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
