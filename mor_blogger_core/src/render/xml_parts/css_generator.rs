use crate::config::{BackgroundMode, ThemeConfig};
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

    let soft_glow = format!(
        "0 0 calc({} / 2) {}",
        escape_attr(glow_spread),
        escape_attr(&color_accent_soft)
    );
    let strong_glow = format!(
        "0 0 {} {}",
        escape_attr(glow_spread),
        escape_attr(&config.colors.accent)
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
