//! CSS Builder module for sanitizing and concatenating base CSS files.

use crate::config::fonts::resolve_font_stack_with_fallback;
use crate::config::{BackgroundMode, ThemeConfig, TypographyConfig};
use crate::render::util::escape_attr;
use crate::utils::svg_icons::svg_to_data_uri;

fn generate_workspace_background_value(bg: &BackgroundMode) -> String {
    match bg {
        BackgroundMode::Solid { color } => escape_attr(color),
        BackgroundMode::Gradient {
            from,
            to,
            angle_deg,
        } => format!(
            "linear-gradient({}deg, {}, {})",
            angle_deg,
            escape_attr(from),
            escape_attr(to)
        ),
        // Tile stays on the page's lowest layer; panels consuming
        // --bg-workspace must go opaque (base color), not re-tile the image
        // from their own origin. css_generator applies the same rule.
        BackgroundMode::Tile { .. } => "var(--bg-base)".to_string(),
    }
}

/// Cleans user-uploaded or modular CSS by stripping out existing Blogger XML wrappers
/// so we can safely concatenate and re-wrap it later without nesting errors.
pub fn clean_raw_css(input: &str) -> String {
    let mut cleaned = input.to_string();

    cleaned = cleaned.replace("<b:skin>", "");
    cleaned = cleaned.replace("</b:skin>", "");
    cleaned = cleaned.replace("<![CDATA[", "");
    cleaned = cleaned.replace("]]>", "");

    cleaned.trim().to_string()
}

pub const DEFAULT_ICON_SIDEBAR_LEFT: &str = "M9 4v16M6 8h.01M6 12h.01 M3 4h18v16H3z";
pub const DEFAULT_ICON_SIDEBAR_RIGHT: &str = "M15 4v16M18 8h.01M18 12h.01 M3 4h18v16H3z";
pub const DEFAULT_ICON_PANEL_CLOSE: &str = "M18 6 6 18M6 6l12 12";
pub const DEFAULT_ICON_SEARCH: &str =
    "M15.5 10.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0Z M21 21l-5.5-5.5";
pub const DEFAULT_ICON_MENU: &str = "M4 7h16M4 12h16M4 17h16";

/// Returns the configured icon if set, otherwise an inline-encoded fallback
/// generated from a built-in path string.
pub fn icon_or_default(value: &str, default_path_d: &str) -> String {
    if value.trim().is_empty() {
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#000" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><path d="{}"/></svg>"##,
            default_path_d
        );
        svg_to_data_uri(&svg)
    } else {
        value.to_string()
    }
}

/// Per-slot render-mode override. Recolor mode (default / absent / `true`)
/// emits nothing: the icon CSS rules fall back to masking `var(--icon-X)` with
/// the theme colour, exactly as before. As-is mode (`false`) emits the three
/// hook vars that flip that slot to a full-colour background image with no tint
/// — so colour emoji, multicolour SVGs, and logos render untouched.
fn icon_asis_vars(slot_field: &str, recolor: &std::collections::HashMap<String, bool>) -> String {
    if recolor.get(slot_field).copied().unwrap_or(true) {
        return String::new();
    }
    let v = slot_field.replace('_', "-");
    format!(
        "\n  --icon-{v}-tint: transparent;\n  --icon-{v}-bg: var(--icon-{v});\n  --icon-{v}-mask: none;"
    )
}

/// Builds the master CSS baseline from modular chunks or user uploads,
/// ensuring it is perfectly wrapped for Blogger and dynamically applies user settings.
pub fn build_master_css(base_css_chunks: &[&str], config: &ThemeConfig) -> String {
    let mut combined_css = String::new();

    for chunk in base_css_chunks {
        let cleaned = clean_raw_css(chunk);
        if !cleaned.is_empty() {
            combined_css.push_str(&cleaned);
            combined_css.push_str("\n\n");
        }
    }

    let font_body = resolve_font_stack_with_fallback(&config.typography.body_font_stack, false);
    let font_heading = if config.typography.heading_font_stack.trim().is_empty() {
        font_body.clone()
    } else {
        resolve_font_stack_with_fallback(&config.typography.heading_font_stack, false)
    };
    let font_mono = resolve_font_stack_with_fallback(&config.typography.mono_font_stack, true);

    let mut custom_vars = format!(
        r#":root {{
  --bg-base: {bg_base};
  --bg-panel: {bg_panel};
  --bg-elevated: {bg_elevated};
  --bg-highlight: {bg_elevated};
  --bg-workspace: {bg_workspace};
  --fg-base: {fg_base};
  --fg-dim: {fg_muted};
  --fg-muted: {fg_muted};
  --accent: {accent};
  --border-color: {border};
  --border-soft: {border};
  --theme-border-color: {border};
  --panel-border-width: {panel_border_width};
  --glow: 0 0 {glow_spread} {accent};
  --glow-strong: 0 0 calc({glow_spread} * 2) {accent};
  --btn-radius: {btn_radius};
  --btn-border-width: {btn_border_width};
  --btn-text-transform: {btn_text_transform};
  --font-body: {font_body};
  --font-heading: {font_heading};
  --font-mono: {font_mono};
  --font-size-base: {font_size_base};
  --line-height-body: {line_height_body};
  --heading-weight: {heading_weight};"#,
        bg_base = config.colors.bg_base,
        bg_panel = config.colors.bg_panel.to_css(),
        bg_elevated = config.colors.bg_elevated.to_css(),
        bg_workspace = generate_workspace_background_value(&config.background.mode),
        fg_base = config.colors.fg_base,
        fg_muted = config.colors.fg_muted,
        accent = config.colors.accent,
        border = config.colors.border,
        panel_border_width = config.colors.panel_border_width,
        glow_spread = config.colors.glow_spread,
        btn_radius = config.buttons.radius,
        btn_border_width = config.buttons.border_width,
        btn_text_transform = config.buttons.text_transform,
        font_body = font_body,
        font_heading = font_heading,
        font_mono = font_mono,
        font_size_base = config.typography.base_size,
        line_height_body = config.typography.line_height,
        heading_weight = config.typography.heading_weight,
    );

    custom_vars.push_str(&format!(
        "\n  --icon-sidebar-left: {};",
        icon_or_default(&config.icons.sidebar_left, DEFAULT_ICON_SIDEBAR_LEFT)
    ));
    custom_vars.push_str(&icon_asis_vars("sidebar_left", &config.icons.recolor));
    custom_vars.push_str(&format!(
        "\n  --icon-sidebar-right: {};",
        icon_or_default(&config.icons.sidebar_right, DEFAULT_ICON_SIDEBAR_RIGHT)
    ));
    custom_vars.push_str(&icon_asis_vars("sidebar_right", &config.icons.recolor));
    custom_vars.push_str(&format!(
        "\n  --icon-panel-close: {};",
        icon_or_default(&config.icons.panel_close, DEFAULT_ICON_PANEL_CLOSE)
    ));
    custom_vars.push_str(&icon_asis_vars("panel_close", &config.icons.recolor));
    custom_vars.push_str(&format!(
        "\n  --icon-search: {};",
        icon_or_default(&config.icons.search, DEFAULT_ICON_SEARCH)
    ));
    custom_vars.push_str(&icon_asis_vars("search", &config.icons.recolor));
    custom_vars.push_str(&format!(
        "\n  --icon-menu: {};",
        icon_or_default(&config.icons.menu, DEFAULT_ICON_MENU)
    ));
    // Widget-heading icons: 14-Widgets-Sidebars.css masks archive/label/TOC
    // titles with these vars. They were referenced but never emitted, so the
    // masks resolved to none and the headings painted as solid squares.
    custom_vars.push_str(&format!(
        "\n  --icon-archive: {};",
        icon_or_default(&config.icons.archive, crate::config::styling::ICON_ARCHIVE_PATH)
    ));
    custom_vars.push_str(&format!(
        "\n  --icon-label: {};",
        icon_or_default(&config.icons.label, crate::config::styling::ICON_LABEL_PATH)
    ));
    custom_vars.push_str(&format!(
        "\n  --icon-toc: {};",
        icon_or_default(&config.icons.toc, crate::config::styling::ICON_TOC_PATH)
    ));

    for (key, svg_data) in &config.icons.custom_icons {
        if svg_data.trim().is_empty() {
            continue;
        }
        let safe_key = key.replace(' ', "-").to_lowercase();
        custom_vars.push_str(&format!("\n  --icon-{safe_key}: {svg_data};"));
    }

    // The frame is active whenever a non-empty image URL is set; the
    // per-region toggles (target_sidebars / target_canvas) decide WHERE it
    // applies. (No separate "enable" gate — pasting a URL is the enable, which
    // matches what the panel's live Preview Frame already shows.)
    let frame_on = config
        .custom_border_url
        .as_ref()
        .is_some_and(|u| !u.trim().is_empty());
    // Sidebars consume --mor-border-image (see 10-Side-Panels.css). Honor the
    // "Apply to Sidebars" toggle instead of always framing them.
    if frame_on && config.target_sidebars {
        let url = config.custom_border_url.as_ref().unwrap();
        custom_vars.push_str(&format!(
            "\n  --mor-border-image: url(\"{}\");\n  --mor-border-slice: {};\n  --mor-border-width-img: {};",
            escape_attr(url),
            escape_attr(&config.svg_border_slice),
            escape_attr(&config.image_border_width),
        ));
    } else {
        custom_vars.push_str(
            "\n  --mor-border-image: none;\n  --mor-border-slice: 0;\n  --mor-border-width-img: var(--panel-border-width);",
        );
    }

    custom_vars.push_str("\n}\n");

    /* Widget header icons via CSS mask (scoped per widget ID/class for .widget h2::before) */
    custom_vars.push_str("#Label1, .Label { --widget-icon: var(--icon-label); }\n");
    custom_vars.push_str("#BlogArchive1, .BlogArchive { --widget-icon: var(--icon-archive); }\n");

    // Prepend custom_vars so 00-Root-Section.css's :root {} (filled with the preset's designed
    // light palette via {{LIGHT_*}} substitution) overrides the color vars here. Icon and button
    // vars are not in 00-Root-Section.css, so they survive as fallback from this block.
    let mut result = custom_vars;
    result.push_str("\n\n");
    result.push_str(&combined_css);

    // Per-element typography overrides. Emitted AFTER the base CSS so simple
    // element selectors (h1, p, …) win over 02-Typography-Links.css and UA
    // defaults, but still before preset_css/user CSS below.
    result.push_str(&build_element_typography_css(&config.typography));

    // Generated button styling, emitted AFTER all core CSS so it is the
    // authoritative button layer (beats base rules in 13-Pagination, 18-Footer,
    // 07-Catalog, etc.). Sits before preset_css so any unavoidable preset-
    // specific button CSS can still override. Replaced in render_css_sockets.
    result.push_str("\n\n/* --- Generated buttons (config-driven) --- */\n{{BUTTON_STYLES}}\n");

    if !config.icons.custom_icons.is_empty() {
        result.push_str("\n/* --- Custom Icon Utilities --- */\n");
        for (key, value) in &config.icons.custom_icons {
            if value.trim().is_empty() {
                continue;
            }
            let safe_key = key.replace(' ', "-").to_lowercase();
            result.push_str(&format!(
                ".custom-icon-{safe_key} {{\n  background-image: var(--icon-{safe_key});\n  background-size: contain;\n  background-repeat: no-repeat;\n  background-position: center;\n}}\n"
            ));
        }
    }

    if !config.preset_css.is_empty() {
        result.push_str("\n\n/* --- User Custom CSS --- */\n");
        result.push_str(&config.preset_css);
    }

    // Image frame — emitted LAST, with !important, so an explicitly-enabled
    // frame wins over preset CSS that hard-sets borders on these elements (e.g.
    // the glassmorphism preset's `.canvas-core/.mor-panel { border: ... !important }`).
    // The `border` shorthand in those presets resets border-image, so we must
    // re-assert the longhands here. Applies to both preview and exported XML.
    if frame_on {
        let url = config.custom_border_url.as_ref().unwrap();
        let decl = format!(
            "border-style: solid !important; border-width: {w} !important; border-image-source: url(\"{u}\") !important; border-image-slice: {s} !important; border-image-repeat: round !important; border-radius: 0 !important;",
            w = escape_attr(&config.image_border_width),
            u = escape_attr(url),
            s = escape_attr(&config.svg_border_slice),
        );
        result.push_str("\n\n/* --- Image frame (wins over preset borders) --- */\n");
        if config.target_canvas {
            result.push_str(&format!(".canvas-core {{ {decl} }}\n"));
        }
        if config.target_sidebars {
            result.push_str(&format!(".mor-panel {{ {decl} }}\n"));
        }
    }

    result
}

/// Map a Typography panel element key to the CSS selector(s) it targets.
/// `:root` prefix lifts specificity to (0,1,1) so a panel-set override ties
/// contextual base rules like `.widget h2` and wins on source order (overrides
/// are emitted after base CSS) — a bare `h2` would lose its padding to
/// `.widget h2 { padding-bottom: 5px }` and plaques went lopsided.
fn element_selector(key: &str) -> Option<&'static str> {
    match key {
        "h1" => Some(":root h1"),
        "h2" => Some(":root h2"),
        "h3" => Some(":root h3"),
        "h4" => Some(":root h4"),
        "h5" => Some(":root h5"),
        "h6" => Some(":root h6"),
        "p" | "body" => Some(".post-body, .post-body p, .post-body li"),
        "blockquote" => Some(":root blockquote"),
        "code" => Some(":root code, :root pre, :root kbd, :root samp"),
        _ => None,
    }
}

/// Build CSS rules for per-element typography overrides. Each [`ElementStyle`]
/// emits one rule containing only its non-empty properties; an all-default
/// entry (or an unknown selector) emits nothing.
pub fn build_element_typography_css(typo: &TypographyConfig) -> String {
    let mut out = String::new();
    for el in &typo.elements {
        let Some(selector) = element_selector(el.selector.trim()) else {
            continue;
        };

        let mut decls = String::new();
        let mut push = |prop: &str, val: &str| {
            let v = val.trim();
            if !v.is_empty() {
                decls.push_str(&format!(" {}: {};", prop, escape_attr(v)));
            }
        };
        push("font-size", &el.font_size);
        push("font-weight", &el.font_weight);
        push("line-height", &el.line_height);
        push("letter-spacing", &el.letter_spacing);
        push("color", &el.color);
        push("background", &el.background);
        push("padding", &el.padding);
        push("border-radius", &el.border_radius);
        push("text-align", &el.text_align);
        if el.italic {
            decls.push_str(" font-style: italic;");
        }

        if !decls.is_empty() {
            out.push_str(&format!("{} {{{} }}\n", selector, decls));
        }
    }
    if out.is_empty() {
        out
    } else {
        format!("\n\n/* --- Per-element typography --- */\n{}", out)
    }
}

#[cfg(test)]
mod element_typography_tests {
    use super::*;
    use crate::config::{ElementStyle, TypographyConfig};

    #[test]
    fn populated_element_emits_rule() {
        let mut typo = TypographyConfig::default();
        typo.elements.push(ElementStyle {
            selector: "h1".into(),
            font_size: "2.5rem".into(),
            font_weight: "600".into(),
            italic: true,
            ..Default::default()
        });
        let css = build_element_typography_css(&typo);
        assert!(css.contains("h1 {"));
        assert!(css.contains("font-size: 2.5rem;"));
        assert!(css.contains("font-weight: 600;"));
        assert!(css.contains("font-style: italic;"));
    }

    #[test]
    fn plaque_box_properties_emit() {
        let mut typo = TypographyConfig::default();
        typo.elements.push(ElementStyle {
            selector: "h2".into(),
            background: "repeating-linear-gradient(135deg, #0c0c0c, #1c1c1d 40%, #0c0c0c 80%)".into(),
            padding: "20px".into(),
            border_radius: "8px".into(),
            text_align: "center".into(),
            ..Default::default()
        });
        let css = build_element_typography_css(&typo);
        assert!(css.contains("background: repeating-linear-gradient(135deg, #0c0c0c, #1c1c1d 40%, #0c0c0c 80%);"));
        assert!(css.contains("padding: 20px;"));
        assert!(css.contains("border-radius: 8px;"));
        assert!(css.contains("text-align: center;"));
    }

    #[test]
    fn all_default_element_emits_nothing() {
        let mut typo = TypographyConfig::default();
        typo.elements.push(ElementStyle {
            selector: "h2".into(),
            ..Default::default()
        });
        assert!(build_element_typography_css(&typo).is_empty());
    }

    #[test]
    fn per_element_rule_reaches_exported_xml() {
        let mut config = crate::config::ThemeConfig::default();
        config.typography.elements.push(ElementStyle {
            selector: "h1".into(),
            font_size: "2.5rem".into(),
            ..Default::default()
        });
        let xml = crate::render::render_theme(&config, &std::collections::HashMap::new());
        assert!(xml.contains("h1 {"));
        assert!(xml.contains("font-size: 2.5rem;"));
    }

    #[test]
    fn frame_emits_from_url_without_enable_flag() {
        let mut config = crate::config::ThemeConfig::default();
        config.enable_image_borders = false; // legacy gate must NOT matter
        config.custom_border_url = Some("https://example.com/frame.png".to_string());
        config.target_canvas = true;
        config.target_sidebars = true;
        let css = build_master_css(&[], &config);
        // Both regions get an !important frame rule emitted AFTER preset_css so
        // it beats preset borders. Sidebars also get the legacy var.
        assert!(css.contains("--mor-border-image: url(\"https://example.com/frame.png\")"));
        assert!(css.contains(".canvas-core { border-style: solid !important;"));
        assert!(css.contains(".mor-panel { border-style: solid !important;"));
        assert!(css.contains("border-image-source: url(\"https://example.com/frame.png\") !important;"));
    }

    #[test]
    fn frame_beats_preset_border_none() {
        // The frame rule must come AFTER preset_css that kills the border.
        let mut config = crate::config::ThemeConfig::default();
        config.custom_border_url = Some("https://example.com/frame.png".to_string());
        config.target_canvas = true;
        config.preset_css = ".canvas-core { border: none !important; }".to_string();
        let css = build_master_css(&[], &config);
        let preset_pos = css.find("border: none !important").unwrap();
        let frame_pos = css.find(".canvas-core { border-style: solid !important;").unwrap();
        assert!(frame_pos > preset_pos, "frame must be emitted after preset_css");
    }

    #[test]
    fn icon_recolor_off_emits_asis_vars() {
        let mut config = crate::config::ThemeConfig::default();
        // Opt one slot out of recolor → as-is full-colour override vars appear.
        config.icons.recolor.insert("panel_close".to_string(), false);
        let css = build_master_css(&[], &config);
        assert!(css.contains("--icon-panel-close-bg: var(--icon-panel-close);"));
        assert!(css.contains("--icon-panel-close-mask: none;"));
        assert!(css.contains("--icon-panel-close-tint: transparent;"));
        // A slot left at default stays in recolor (mask) mode — no override vars.
        assert!(!css.contains("--icon-search-mask: none;"));
    }

    #[test]
    fn glow_wins_over_preset_box_shadow_on_any_preset() {
        // Per-target glow is config-driven (preset-independent) and emitted LAST,
        // so it beats a preset's own box-shadow — i.e. it shows on every preset.
        let mut config = crate::config::ThemeConfig::default();
        config.colors.glow_containers = true;
        config.colors.glow_hover = false; // always-on -> a resting rule exists
        config.preset_css = ".mor-panel { box-shadow: 0 0 0 99px red; }".to_string();
        let xml = crate::render::render_theme(&config, &std::collections::HashMap::new());
        let preset_pos = xml.find("box-shadow: 0 0 0 99px red").unwrap();
        let glow_pos = xml.find("Per-target glow").unwrap();
        assert!(
            glow_pos > preset_pos,
            "glow must be emitted after preset_css so it wins on every preset"
        );
    }

    #[test]
    fn frame_off_when_url_blank() {
        let mut config = crate::config::ThemeConfig::default();
        config.custom_border_url = Some("   ".to_string());
        config.target_canvas = true;
        let css = build_master_css(&[], &config);
        assert!(css.contains("--mor-border-image: none"));
        assert!(!css.contains("border-style: solid !important;"));
    }

    #[test]
    fn unknown_selector_skipped() {
        let mut typo = TypographyConfig::default();
        typo.elements.push(ElementStyle {
            selector: "marquee".into(),
            font_size: "9px".into(),
            ..Default::default()
        });
        assert!(build_element_typography_css(&typo).is_empty());
    }
}
