use crate::config::{BackgroundMode, ButtonConfig, ThemeConfig};
use crate::presets::resolve_palette_pair;
use crate::render::css_builder::{
    icon_or_default, DEFAULT_ICON_MENU, DEFAULT_ICON_PANEL_CLOSE, DEFAULT_ICON_SEARCH,
    DEFAULT_ICON_SIDEBAR_LEFT, DEFAULT_ICON_SIDEBAR_RIGHT,
};
use crate::render::util::{escape_attr, first_non_empty};

fn hex_to_rgba(hex: &str, alpha: f32) -> String {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 {
        return format!("rgba(255, 255, 255, {:.2})", alpha);
    }
    let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(255);
    format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha)
}

fn fluid_glow_css(accent: &str) -> String {
    format!(
        "linear-gradient(110deg, {}, #a371f7, #f778ba)",
        escape_attr(accent)
    )
}

fn sanitize_border_image_repeat(value: &str) -> &str {
    match value.trim() {
        "repeat" => "repeat",
        "round" => "round",
        "space" => "space",
        "stretch" => "stretch",
        _ => "stretch",
    }
}

fn generate_border_image_css(url: &str, slice: &str, repeat: &str) -> String {
    let url = url.trim();
    if url.is_empty() {
        return String::new();
    }
    let slice = first_non_empty(slice, "30%");
    let repeat = sanitize_border_image_repeat(repeat);
    format!(
        "border-style: solid;\n  border-image: url('{}') {} {};",
        escape_attr(url),
        escape_attr(slice),
        escape_attr(repeat)
    )
}

fn generate_border_image_value(url: &str, slice: &str, repeat: &str, accent_color: &str) -> String {
    let url = url.trim();
    if url.is_empty() {
        return "none".to_string();
    }
    let colored_url = url
        .replace("{{ACCENT}}", accent_color)
        .replace("%7B%7BACCENT%7D%7D", accent_color);
    let slice = first_non_empty(slice, "30%");
    let repeat = sanitize_border_image_repeat(repeat);
    format!(
        "url('{}') {} {}",
        escape_attr(&colored_url),
        escape_attr(slice),
        escape_attr(repeat)
    )
}

fn generate_background_css(bg: &BackgroundMode) -> String {
    match bg {
        BackgroundMode::Solid { color } => format!("background-color: {};", escape_attr(color)),
        BackgroundMode::Gradient {
            from,
            to,
            angle_deg,
        } => format!(
            "background: linear-gradient({}deg, {}, {});",
            angle_deg,
            escape_attr(from),
            escape_attr(to)
        ),
        BackgroundMode::Tile { url } if url.trim().is_empty() => String::new(),
        BackgroundMode::Tile { url } => format!(
            "background-image: url('{}');\n  background-repeat: repeat;",
            escape_attr(url)
        ),
    }
}

fn generate_background_value(bg: &BackgroundMode) -> String {
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
        BackgroundMode::Tile { url } if url.trim().is_empty() => "none".to_string(),
        BackgroundMode::Tile { url } => format!("url('{}')", escape_attr(url)),
    }
}

pub fn render_css_sockets(mut xml: String, config: &ThemeConfig) -> String {
    let (light_palette, dark_palette) =
        resolve_palette_pair(config.active_preset_id.as_deref(), config);

    let light_bg_css = generate_background_css(&light_palette.background.mode);
    let dark_bg_css = generate_background_css(&dark_palette.background.mode);
    let light_bg_workspace = generate_background_value(&light_palette.background.mode);
    let dark_bg_workspace = generate_background_value(&dark_palette.background.mode);
    let background_tile_url = match &config.background.mode {
        BackgroundMode::Tile { url } => url.clone(),
        _ => String::new(),
    };

    let body_stack = crate::config::fonts::resolve_font_stack_with_fallback(
        &config.typography.body_font_stack,
        false,
    );
    let heading_stack = if config.typography.heading_font_stack.trim().is_empty() {
        body_stack.clone()
    } else {
        crate::config::fonts::resolve_font_stack_with_fallback(
            &config.typography.heading_font_stack,
            false,
        )
    };
    let mono_stack = crate::config::fonts::resolve_font_stack_with_fallback(
        &config.typography.mono_font_stack,
        true,
    );

    let color_accent_soft = hex_to_rgba(&config.colors.accent, 0.45);
    let color_accent_shadow = hex_to_rgba(&config.colors.accent, 0.25);
    let color_accent_wash = hex_to_rgba(&config.colors.accent, 0.08);
    let fluid_glow = fluid_glow_css(&config.colors.accent);
    let glow_spread = first_non_empty(&config.colors.glow_spread, "8px");

    let panel_border_width = first_non_empty(&config.colors.panel_border_width, "1px");
    let panel_border_image_slice = first_non_empty(&config.colors.panel_border_image_slice, "30%");
    let panel_border_image_repeat =
        sanitize_border_image_repeat(&config.colors.panel_border_image_repeat);
    let panel_border_image_css = generate_border_image_css(
        &config.colors.panel_border_image_url,
        panel_border_image_slice,
        panel_border_image_repeat,
    );
    let panel_border_image_value = generate_border_image_value(
        &config.colors.panel_border_image_url,
        panel_border_image_slice,
        panel_border_image_repeat,
        &config.colors.accent,
    );

    let light_panel_border_image_slice =
        first_non_empty(&light_palette.colors.panel_border_image_slice, "30%");
    let light_panel_border_image_repeat =
        sanitize_border_image_repeat(&light_palette.colors.panel_border_image_repeat);
    let light_panel_border_image_css = generate_border_image_css(
        &light_palette.colors.panel_border_image_url,
        light_panel_border_image_slice,
        light_panel_border_image_repeat,
    );
    let light_panel_border_image_value = generate_border_image_value(
        &light_palette.colors.panel_border_image_url,
        light_panel_border_image_slice,
        light_panel_border_image_repeat,
        &light_palette.colors.accent,
    );

    let dark_panel_border_image_slice =
        first_non_empty(&dark_palette.colors.panel_border_image_slice, "30%");
    let dark_panel_border_image_repeat =
        sanitize_border_image_repeat(&dark_palette.colors.panel_border_image_repeat);
    let dark_panel_border_image_css = generate_border_image_css(
        &dark_palette.colors.panel_border_image_url,
        dark_panel_border_image_slice,
        dark_panel_border_image_repeat,
    );
    let dark_panel_border_image_value = generate_border_image_value(
        &dark_palette.colors.panel_border_image_url,
        dark_panel_border_image_slice,
        dark_panel_border_image_repeat,
        &dark_palette.colors.accent,
    );

    // Apply strict structural variable replacements directly into the string payload
    xml = xml.replace(
        "{{LIGHT_BG_BASE}}",
        &escape_attr(&light_palette.colors.bg_base),
    );
    xml = xml.replace(
        "{{LIGHT_BG_PANEL}}",
        &escape_attr(&light_palette.colors.bg_panel.to_css()),
    );
    xml = xml.replace(
        "{{LIGHT_BG_ELEVATED}}",
        &escape_attr(&light_palette.colors.bg_elevated.to_css()),
    );
    xml = xml.replace(
        "{{LIGHT_FG_BASE}}",
        &escape_attr(&light_palette.colors.fg_base),
    );
    xml = xml.replace(
        "{{LIGHT_FG_MUTED}}",
        &escape_attr(&light_palette.colors.fg_muted),
    );
    xml = xml.replace(
        "{{LIGHT_ACCENT}}",
        &escape_attr(&light_palette.colors.accent),
    );
    xml = xml.replace(
        "{{LIGHT_BORDER}}",
        &escape_attr(&light_palette.colors.border),
    );
    xml = xml.replace(
        "{{LIGHT_PANEL_BORDER_WIDTH}}",
        &escape_attr(&light_palette.colors.panel_border_width),
    );
    xml = xml.replace(
        "{{LIGHT_GLOW_SPREAD}}",
        &escape_attr(&light_palette.colors.glow_spread),
    );
    xml = xml.replace(
        "{{LIGHT_HOVER_SCALE}}",
        &escape_attr(&light_palette.colors.hover_scale),
    );
    xml = xml.replace(
        "{{LIGHT_PANEL_BORDER_IMAGE_URL}}",
        &escape_attr(&light_palette.colors.panel_border_image_url),
    );
    xml = xml.replace(
        "{{LIGHT_PANEL_BORDER_IMAGE_SLICE}}",
        &escape_attr(light_panel_border_image_slice),
    );
    xml = xml.replace(
        "{{LIGHT_PANEL_BORDER_IMAGE_REPEAT}}",
        &escape_attr(light_panel_border_image_repeat),
    );
    xml = xml.replace(
        "{{LIGHT_PANEL_BORDER_IMAGE_CSS}}",
        &light_panel_border_image_css,
    );
    xml = xml.replace(
        "{{LIGHT_PANEL_BORDER_IMAGE_VALUE}}",
        &light_panel_border_image_value,
    );
    xml = xml.replace("{{LIGHT_BACKGROUND_TILE_CSS}}", &light_bg_css);
    xml = xml.replace("{{LIGHT_BG_WORKSPACE}}", &light_bg_workspace);

    xml = xml.replace(
        "{{DARK_BG_BASE}}",
        &escape_attr(&dark_palette.colors.bg_base),
    );
    xml = xml.replace(
        "{{DARK_BG_PANEL}}",
        &escape_attr(&dark_palette.colors.bg_panel.to_css()),
    );
    xml = xml.replace(
        "{{DARK_BG_ELEVATED}}",
        &escape_attr(&dark_palette.colors.bg_elevated.to_css()),
    );
    xml = xml.replace(
        "{{DARK_FG_BASE}}",
        &escape_attr(&dark_palette.colors.fg_base),
    );
    xml = xml.replace(
        "{{DARK_FG_MUTED}}",
        &escape_attr(&dark_palette.colors.fg_muted),
    );
    xml = xml.replace("{{DARK_ACCENT}}", &escape_attr(&dark_palette.colors.accent));
    xml = xml.replace("{{DARK_BORDER}}", &escape_attr(&dark_palette.colors.border));
    xml = xml.replace(
        "{{DARK_PANEL_BORDER_WIDTH}}",
        &escape_attr(&dark_palette.colors.panel_border_width),
    );
    xml = xml.replace(
        "{{DARK_GLOW_SPREAD}}",
        &escape_attr(&dark_palette.colors.glow_spread),
    );
    xml = xml.replace(
        "{{DARK_HOVER_SCALE}}",
        &escape_attr(&dark_palette.colors.hover_scale),
    );
    xml = xml.replace(
        "{{DARK_PANEL_BORDER_IMAGE_URL}}",
        &escape_attr(&dark_palette.colors.panel_border_image_url),
    );
    xml = xml.replace(
        "{{DARK_PANEL_BORDER_IMAGE_SLICE}}",
        &escape_attr(dark_panel_border_image_slice),
    );
    xml = xml.replace(
        "{{DARK_PANEL_BORDER_IMAGE_REPEAT}}",
        &escape_attr(dark_panel_border_image_repeat),
    );
    xml = xml.replace(
        "{{DARK_PANEL_BORDER_IMAGE_CSS}}",
        &dark_panel_border_image_css,
    );
    xml = xml.replace(
        "{{DARK_PANEL_BORDER_IMAGE_VALUE}}",
        &dark_panel_border_image_value,
    );
    xml = xml.replace("{{DARK_BACKGROUND_TILE_CSS}}", &dark_bg_css);
    xml = xml.replace("{{DARK_BG_WORKSPACE}}", &dark_bg_workspace);

    xml = xml.replace("{{COLOR_ACCENT_SOFT}}", &escape_attr(&color_accent_soft));
    xml = xml.replace(
        "{{COLOR_ACCENT_SHADOW}}",
        &escape_attr(&color_accent_shadow),
    );
    xml = xml.replace("{{COLOR_ACCENT_WASH}}", &escape_attr(&color_accent_wash));
    xml = xml.replace("{{FLUID_GLOW_CSS}}", &fluid_glow);
    xml = xml.replace("{{GLOBAL_CURSOR_CSS}}", &escape_attr(&config.cursor_style));
    xml = xml.replace("{{CURSOR_DEFAULT}}", &config.cursor_style);
    xml = xml.replace("{{CURSOR_POINTER}}", "pointer");
    xml = xml.replace("{{SCROLLBAR_WIDTH}}", &escape_attr(&config.scrollbar_width));
    xml = xml.replace(
        "{{SCROLLBAR_TRACK}}",
        &escape_attr(&config.scrollbar_track_color),
    );
    xml = xml.replace(
        "{{SCROLLBAR_THUMB}}",
        &escape_attr(&config.scrollbar_thumb_color),
    );
    xml = xml.replace(
        "{{SCROLLBAR_THUMB_HOVER}}",
        &escape_attr(&config.scrollbar_thumb_hover_color),
    );

    xml = xml.replace("{{FONT_BODY}}", &escape_attr(&body_stack));
    xml = xml.replace("{{FONT_HEADING}}", &escape_attr(&heading_stack));
    xml = xml.replace("{{FONT_MONO}}", &escape_attr(&mono_stack));
    xml = xml.replace("{{BASE_SIZE}}", &escape_attr(&config.typography.base_size));
    xml = xml.replace(
        "{{SCALE_RATIO}}",
        &escape_attr(&config.typography.scale_ratio),
    );
    xml = xml.replace(
        "{{LINE_HEIGHT}}",
        &escape_attr(&config.typography.line_height),
    );
    xml = xml.replace(
        "{{HEADING_WEIGHT}}",
        &escape_attr(&config.typography.heading_weight),
    );

    xml = xml.replace("{{BTN_RADIUS}}", &escape_attr(&config.buttons.radius));
    xml = xml.replace(
        "{{BTN_BORDER_WIDTH}}",
        &escape_attr(&config.buttons.border_width),
    );
    xml = xml.replace(
        "{{BTN_TEXT_TRANSFORM}}",
        &escape_attr(&config.buttons.text_transform),
    );
    // Computed button styling block (fill/elevation/hover need conditional CSS
    // that token replacement can't express). Sits in core CSS so presets can
    // still override it via preset_css.
    xml = xml.replace("{{BUTTON_STYLES}}", &render_button_styles(&config.buttons, ""));

    xml = xml.replace("{{PANEL_BORDER_WIDTH}}", &escape_attr(panel_border_width));
    xml = xml.replace(
        "{{CONTAINER_BORDER_WIDTH}}",
        &escape_attr(panel_border_width),
    );
    xml = xml.replace("{{FRAME_BORDER_WIDTH}}", &escape_attr(panel_border_width));

    xml = xml.replace(
        "{{PANEL_BORDER_IMAGE_URL}}",
        &escape_attr(&config.colors.panel_border_image_url),
    );
    xml = xml.replace(
        "{{PANEL_BORDER_IMAGE_SLICE}}",
        &escape_attr(panel_border_image_slice),
    );
    xml = xml.replace(
        "{{PANEL_BORDER_IMAGE_REPEAT}}",
        &escape_attr(panel_border_image_repeat),
    );
    xml = xml.replace("{{PANEL_BORDER_IMAGE_CSS}}", &panel_border_image_css);
    xml = xml.replace("{{PANEL_BORDER_IMAGE_VALUE}}", &panel_border_image_value);
    xml = xml.replace("{{CONTAINER_BORDER_IMAGE_CSS}}", &panel_border_image_css);
    xml = xml.replace("{{FRAME_BORDER_IMAGE_CSS}}", &panel_border_image_css);

    xml = xml.replace("{{GLOW_SPREAD}}", &escape_attr(glow_spread));
    xml = xml.replace("{{HOVER_SCALE}}", &escape_attr(&config.colors.hover_scale));
    xml = xml.replace("{{SIDEBAR_WIDTH}}", "300px");

    let active_glow = if config.colors.glow_color.is_empty() {
        &config.colors.accent
    } else {
        &config.colors.glow_color
    };
    let custom_glow_soft = hex_to_rgba(active_glow, 0.45);
    let soft_glow = format!(
        "0 0 calc({} / 2) {}",
        escape_attr(glow_spread),
        escape_attr(&custom_glow_soft)
    );
    let strong_glow = format!(
        "0 0 {} {}",
        escape_attr(glow_spread),
        escape_attr(active_glow)
    );
    xml = xml.replace("{{GLOW_SOFT}}", &soft_glow);
    xml = xml.replace("{{GLOW_STRONG}}", &strong_glow);
    xml = xml.replace("{{SHADOW_ELEVATED}}", "0 18px 45px rgba(0, 0, 0, 0.65)");

    xml = xml.replace(
        "{{BACKGROUND_TILE_URL}}",
        &escape_attr(&background_tile_url),
    );

    xml = xml.replace(
        "{{ICON_SIDEBAR_LEFT}}",
        &icon_or_default(&config.icons.sidebar_left, DEFAULT_ICON_SIDEBAR_LEFT),
    );
    xml = xml.replace(
        "{{ICON_SIDEBAR_RIGHT}}",
        &icon_or_default(&config.icons.sidebar_right, DEFAULT_ICON_SIDEBAR_RIGHT),
    );
    xml = xml.replace(
        "{{ICON_PANEL_CLOSE}}",
        &icon_or_default(&config.icons.panel_close, DEFAULT_ICON_PANEL_CLOSE),
    );
    xml = xml.replace(
        "{{ICON_SEARCH}}",
        &icon_or_default(&config.icons.search, DEFAULT_ICON_SEARCH),
    );
    xml = xml.replace(
        "{{ICON_MENU}}",
        &icon_or_default(&config.icons.menu, DEFAULT_ICON_MENU),
    );

    // Remaining structural sizing
    xml = xml.replace("{{HEADER_LOGO_SIZE}}", "72px");
    xml = xml.replace(
        "{{HEADER_LOGO_RADIUS}}",
        &escape_attr(&config.buttons.radius),
    );
    xml = xml.replace("{{SITE_TITLE_SIZE}}", "1.05rem");
    xml = xml.replace("{{SITE_TITLE_LETTER_SPACING}}", "2px");
    xml = xml.replace("{{SEARCH_INPUT_WIDTH}}", "180px");
    xml = xml.replace("{{SEARCH_INPUT_FOCUS_WIDTH}}", "240px");
    xml = xml.replace("{{CONTENT_MAX_WIDTH}}", "1200px");
    xml = xml.replace("{{POST_TITLE_SIZE}}", "1.5rem");
    xml = xml.replace("{{HEADER_LOGO_SIZE_MOBILE}}", "48px");
    xml = xml.replace(
        "{{HEADER_LOGO_RADIUS_MOBILE}}",
        &escape_attr(&config.buttons.radius),
    );
    xml = xml.replace("{{SITE_TITLE_SIZE_MOBILE}}", "0.95rem");
    xml = xml.replace("{{SITE_TITLE_LETTER_SPACING_MOBILE}}", "1px");
    xml = xml.replace("{{POST_TITLE_SIZE_MOBILE}}", "1.25rem");
    xml = xml.replace("{{HEADER_LOGO_SIZE_SMALL}}", "38px");
    xml = xml.replace(
        "{{HEADER_LOGO_RADIUS_SMALL}}",
        &escape_attr(&config.buttons.radius),
    );
    xml = xml.replace("{{SITE_TITLE_SIZE_SMALL}}", "0.8rem");

    xml
}

/// Text buttons in the rendered theme. Icon toggles (`.panel-toggle`) are
/// excluded so their square hit-areas aren't restyled.
const BTN_PARTS: [&str; 9] = [
    "button:not(.panel-toggle)",
    ".pager-btn",
    ".back-to-top-btn",
    ".toc-jump-link",
    ".mor-catalog-trigger",
    ".read-more",
    ".comment-form-submit",
    "input[type='submit']",
    "input[type='button']",
];

/// Join the button selectors, optionally prefixed by a scope (e.g. a preview
/// container class) so the same rules can style just one region.
fn scoped_sel(scope: &str) -> String {
    let scope = scope.trim();
    if scope.is_empty() {
        BTN_PARTS.join(", ")
    } else {
        BTN_PARTS
            .iter()
            .map(|p| format!("{scope} {p}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Build the full button rule set from [`ButtonConfig`]. Handles the fill,
/// elevation, and hover variants that can't be expressed as flat token swaps.
/// Empty color fields derive from the active theme vars. `scope` is "" for the
/// global theme output, or a container selector to scope a preview.
pub fn render_button_styles(b: &ButtonConfig, scope: &str) -> String {
    let sel = scoped_sel(scope);

    // Effect parameters for glass / neon / glow.
    let gc = if b.glow_color.trim().is_empty() {
        "var(--accent)".to_string()
    } else {
        b.glow_color.clone()
    };
    let g = if b.glow_strength.trim().is_empty() {
        "12px".to_string()
    } else {
        b.glow_strength.clone()
    };
    let blur = if b.glass_blur.trim().is_empty() {
        "12px".to_string()
    } else {
        b.glass_blur.clone()
    };
    let op = {
        let v: f32 = b.glass_opacity.trim().parse().unwrap_or(0.4);
        (v.clamp(0.0, 1.0) * 100.0).round() as i32
    };
    let op_hi = (op + 18).min(100);

    let g_angle = if b.gradient_angle.trim().is_empty() {
        "180deg".to_string()
    } else {
        b.gradient_angle.clone()
    };
    let g_from = if b.gradient_from.trim().is_empty() {
        "var(--bg-panel)".to_string()
    } else {
        b.gradient_from.clone()
    };
    let g_to = if b.gradient_to.trim().is_empty() {
        "var(--bg-base)".to_string()
    } else {
        b.gradient_to.clone()
    };
    let gradient = format!("linear-gradient({g_angle}, {g_from}, {g_to})");

    // (bg, text, border, default-hover-bg) per fill style.
    let (mut bg, mut fg, mut border, hover_bg_default) = match b.fill.as_str() {
        "solid" => (
            "var(--accent)".to_string(),
            "var(--bg-base)".to_string(),
            "var(--accent)".to_string(),
            "color-mix(in srgb, var(--accent) 85%, #000)".to_string(),
        ),
        "soft" => (
            "color-mix(in srgb, var(--accent) 16%, transparent)".to_string(),
            "var(--accent)".to_string(),
            "transparent".to_string(),
            "color-mix(in srgb, var(--accent) 28%, transparent)".to_string(),
        ),
        "ghost" => (
            "transparent".to_string(),
            "var(--fg-base)".to_string(),
            "transparent".to_string(),
            "color-mix(in srgb, var(--fg-base) 12%, transparent)".to_string(),
        ),
        "glass" => (
            format!("color-mix(in srgb, var(--bg-base) {op}%, transparent)"),
            "var(--fg-base)".to_string(),
            "color-mix(in srgb, var(--fg-base) 14%, transparent)".to_string(),
            format!("color-mix(in srgb, var(--bg-base) {op_hi}%, transparent)"),
        ),
        "neon" => (
            "transparent".to_string(),
            gc.clone(),
            gc.clone(),
            format!("color-mix(in srgb, {gc} 16%, transparent)"),
        ),
        "glossy" => (
            "var(--accent)".to_string(),
            "#ffffff".to_string(),
            "color-mix(in srgb, var(--fg-base) 12%, transparent)".to_string(),
            "color-mix(in srgb, var(--accent) 88%, #000)".to_string(),
        ),
        "gradient" => (
            gradient.clone(),
            "var(--fg-base)".to_string(),
            "var(--border-color)".to_string(),
            gradient.clone(),
        ),
        // "outline" / default — neutral bordered button (historical look).
        _ => (
            "transparent".to_string(),
            "var(--fg-base)".to_string(),
            "var(--border-color)".to_string(),
            "var(--fg-base)".to_string(),
        ),
    };
    // Outline inverts text on hover; other fills keep their text color.
    let mut hover_fg = if b.fill == "outline" || b.fill.is_empty() {
        "var(--bg-base)".to_string()
    } else {
        fg.clone()
    };

    if !b.bg_color.trim().is_empty() {
        bg = b.bg_color.clone();
    }
    if !b.text_color.trim().is_empty() {
        fg = b.text_color.clone();
        hover_fg = fg.clone();
    }
    if !b.border_color.trim().is_empty() {
        border = b.border_color.clone();
    }
    let hover_bg = if b.hover_bg_color.trim().is_empty() {
        hover_bg_default
    } else {
        b.hover_bg_color.clone()
    };

    let shadow = match b.elevation.as_str() {
        "subtle" => "0 1px 2px rgba(0,0,0,0.18)",
        "raised" => "0 2px 5px rgba(0,0,0,0.28), inset 0 1px 0 rgba(255,255,255,0.06)",
        _ => "none",
    };

    // Surface effects: extra declarations layered after the base (so they win).
    let effect_extra = match b.fill.as_str() {
        "glass" => format!(
            "  backdrop-filter: blur({blur}) saturate(140%);\n  -webkit-backdrop-filter: blur({blur}) saturate(140%);\n  box-shadow: inset 0 1px 0 color-mix(in srgb, var(--fg-base) 16%, transparent);\n"
        ),
        "neon" => format!(
            "  box-shadow: 0 0 {g} {gc}, inset 0 0 calc({g} / 2) color-mix(in srgb, {gc} 35%, transparent);\n  text-shadow: 0 0 6px {gc};\n"
        ),
        "glossy" => {
            let base = if b.bg_color.trim().is_empty() {
                "var(--accent)".to_string()
            } else {
                b.bg_color.clone()
            };
            format!(
                "  background-image: linear-gradient(180deg, color-mix(in srgb, {base} 80%, #fff), {base});\n  box-shadow: inset 0 1px 0 rgba(255,255,255,0.45), 0 1px 3px rgba(0,0,0,0.3);\n"
            )
        }
        _ => String::new(),
    };

    let mut decls = String::new();
    if !b.font_size.trim().is_empty() {
        decls.push_str(&format!("  font-size: {};\n", b.font_size));
    }
    if !b.font_weight.trim().is_empty() {
        decls.push_str(&format!("  font-weight: {};\n", b.font_weight));
    }
    if !b.letter_spacing.trim().is_empty() {
        decls.push_str(&format!("  letter-spacing: {};\n", b.letter_spacing));
    }
    if b.full_width {
        decls.push_str("  width: 100%;\n");
    }
    let transition = if b.transition_ms.trim().is_empty() || b.transition_ms.trim() == "0ms" {
        String::new()
    } else {
        format!("  transition: all {} ease;\n", b.transition_ms)
    };

    let hover_extra = match b.hover_effect.as_str() {
        "lift" => "  transform: translateY(-2px);\n  box-shadow: 0 4px 10px rgba(0,0,0,0.3);\n".to_string(),
        "grow" => "  transform: scale(1.04);\n".to_string(),
        "brighten" => "  filter: brightness(1.12);\n".to_string(),
        // Customizable glow, also intensifies neon on hover.
        "glow" => format!("  box-shadow: 0 0 {g} {gc}, 0 0 calc({g} * 2) color-mix(in srgb, {gc} 50%, transparent);\n"),
        _ => String::new(),
    };
    let pressed = if b.pressed_feedback {
        // Outset borders flip to inset on press for the classic 3D button feel.
        let bevel = if b.border_style == "outset" {
            " border-style: inset;"
        } else {
            ""
        };
        format!("{sel}:active {{ transform: translateY(1px) scale(0.99);{bevel} }}\n")
    } else {
        String::new()
    };
    let focus_color = if b.focus_ring_color.trim().is_empty() {
        "var(--fg-dim)".to_string()
    } else {
        b.focus_ring_color.clone()
    };

    // Raw box-shadow override wins over elevation + effect shadows when set.
    let shadow_override = if b.box_shadow.trim().is_empty() {
        String::new()
    } else {
        format!("  box-shadow: {};\n", b.box_shadow)
    };
    // Gradient/glossy/glass keep their surface on hover (rely on hover_effect)
    // unless an explicit hover color is given.
    let keeps_surface = matches!(b.fill.as_str(), "gradient" | "glossy" | "glass")
        && b.hover_bg_color.trim().is_empty();
    let hover_bg_decl = if keeps_surface {
        String::new()
    } else {
        format!("  background: {};\n", hover_bg)
    };

    format!(
        "/* --- Computed button styles --- */\n\
{sel} {{\n  background: {bg};\n  color: {fg};\n  border: {bw} {bstyle} {border};\n  border-radius: {radius};\n  padding: {py} {px};\n  text-transform: {tt};\n  box-shadow: {shadow};\n{decls}{transition}{effect_extra}{shadow_override}}}\n\
{sel}:hover, {sel}:focus {{\n{hover_bg_decl}  color: {hover_fg};\n{hover_extra}}}\n\
{sel}:focus-visible {{ outline: none; box-shadow: 0 0 0 {fw} {focus_color}; }}\n\
{pressed}",
        sel = sel,
        bg = bg,
        fg = fg,
        bw = b.border_width,
        bstyle = b.border_style,
        border = border,
        radius = b.radius,
        py = b.padding_y,
        px = b.padding_x,
        tt = b.text_transform,
        shadow = shadow,
        decls = decls,
        transition = transition,
        effect_extra = effect_extra,
        shadow_override = shadow_override,
        hover_bg_decl = hover_bg_decl,
        hover_fg = hover_fg,
        hover_extra = hover_extra,
        fw = b.focus_ring_width,
        focus_color = focus_color,
        pressed = pressed,
    )
}

#[cfg(test)]
mod button_tests {
    use super::*;

    #[test]
    fn button_styles_cover_variants() {
        // Default (outline) reproduces the neutral bordered look.
        let css = render_button_styles(&ButtonConfig::default(), "");
        assert!(css.contains("background: transparent;"));
        assert!(css.contains("color: var(--fg-base);"));
        assert!(css.contains("box-shadow: none;")); // flat elevation

        // Solid + raised + lift + override emits the expected pieces.
        let mut b = ButtonConfig::default();
        b.fill = "solid".into();
        b.elevation = "raised".into();
        b.hover_effect = "lift".into();
        b.bg_color = "#c2622a".into();
        b.transition_ms = "150ms".into();
        let css = render_button_styles(&b, "");
        assert!(css.contains("background: #c2622a;")); // override wins
        assert!(css.contains("inset 0 1px 0")); // raised shadow
        assert!(css.contains("translateY(-2px)")); // lift hover
        assert!(css.contains("transition: all 150ms ease;"));
    }

    #[test]
    fn effect_fills_emit_their_css() {
        let mut b = ButtonConfig::default();
        b.fill = "glass".into();
        b.glass_blur = "20px".into();
        let glass = render_button_styles(&b, "");
        assert!(glass.contains("backdrop-filter: blur(20px)"));

        b.fill = "neon".into();
        b.glow_color = "#00eaff".into();
        b.glow_strength = "14px".into();
        let neon = render_button_styles(&b, "");
        assert!(neon.contains("text-shadow: 0 0 6px #00eaff"));
        assert!(neon.contains("box-shadow: 0 0 14px #00eaff"));

        b.fill = "glossy".into();
        let glossy = render_button_styles(&b, "");
        assert!(glossy.contains("linear-gradient(180deg"));
    }

    #[test]
    fn gradient_bevel_and_shadow_override() {
        let mut b = ButtonConfig::default();
        b.fill = "gradient".into();
        b.gradient_from = "#aaa".into();
        b.gradient_to = "#333".into();
        b.gradient_angle = "180deg".into();
        b.box_shadow = "inset 1px 1px 0 #fff".into();
        let css = render_button_styles(&b, "");
        assert!(css.contains("background: linear-gradient(180deg, #aaa, #333)"));
        assert!(css.contains("box-shadow: inset 1px 1px 0 #fff;")); // override
        // Gradient keeps its surface on hover: hover rule has no background reset.
        assert!(css.contains(":focus {\n  color:"), "hover should not reset background");

        // Outset border flips to inset on press.
        let mut w = ButtonConfig::default();
        w.border_style = "outset".into();
        w.pressed_feedback = true;
        let wcss = render_button_styles(&w, "");
        assert!(wcss.contains("border: 1px outset"));
        assert!(wcss.contains("border-style: inset;"));
    }

    #[test]
    fn scope_prefixes_selectors_for_preview() {
        let scoped = render_button_styles(&ButtonConfig::default(), ".mor-btn-preview");
        assert!(scoped.contains(".mor-btn-preview .pager-btn"));
        assert!(scoped.contains(".mor-btn-preview button:not(.panel-toggle)"));
        // The global (export) form must NOT carry the preview scope.
        let global = render_button_styles(&ButtonConfig::default(), "");
        assert!(!global.contains(".mor-btn-preview"));
    }

    #[test]
    fn button_styles_socket_is_replaced() {
        let cfg = ThemeConfig::default();
        let out = render_css_sockets("a {{BUTTON_STYLES}} b".to_string(), &cfg);
        assert!(!out.contains("{{BUTTON_STYLES}}"), "token left unreplaced");
        assert!(out.contains("Computed button styles"), "block not injected");
    }

    // End-to-end: in the full assembled stylesheet the config-driven block is
    // present, the token is gone, and it lands AFTER the base .pager-btn CSS so
    // the config is authoritative.
    #[test]
    fn generated_block_follows_base_button_css() {
        use crate::render::template_resolver::resolve_template_parts;
        use std::collections::HashMap;
        let mut cfg = ThemeConfig::default();
        cfg.buttons.fill = "gradient".into();
        cfg.buttons.gradient_from = "#abc".into();
        cfg.buttons.gradient_to = "#123".into();
        let parts = resolve_template_parts(&cfg, &HashMap::new());
        let css = render_css_sockets(parts.css, &cfg);
        assert!(!css.contains("{{BUTTON_STYLES}}"), "socket not replaced");
        assert!(css.contains("Generated buttons (config-driven)"));
        assert!(css.contains("linear-gradient(180deg, #abc, #123)"));
        let block = css.find("Generated buttons (config-driven)").unwrap();
        let base = css.find(".pager-btn").unwrap();
        assert!(block > base, "generated block must follow base button CSS");
    }
}
