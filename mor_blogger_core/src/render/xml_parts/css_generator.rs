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

/// Per-target glow CSS driven by the Advanced Glow toggles. Each enabled target
/// gets a box/text-shadow using its own color (falling back to the global glow
/// color, then the accent). Appended last so it overrides the base rules.
/// Expand a (possibly comma-separated) selector into its hover/focus form,
/// attaching the pseudo-classes to EACH selector — `a, b` → `a:hover,
/// a:focus-visible, b:hover, b:focus-visible` (not just the last one).
fn hover_selectors(sel: &str) -> String {
    sel.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .flat_map(|s| [format!("{s}:hover"), format!("{s}:focus-visible")])
        .collect::<Vec<_>>()
        .join(", ")
}

fn build_target_glow_css(config: &ThemeConfig) -> String {
    let c = &config.colors;
    let spread = first_non_empty(&c.glow_spread, "8px");
    let global = if c.glow_color.trim().is_empty() {
        c.accent.as_str()
    } else {
        c.glow_color.as_str()
    };
    // Stacked layers at increasing blur give the deep neon bloom (1–4 layers).
    let layers: usize = c.glow_intensity.trim().parse::<usize>().unwrap_or(2).clamp(1, 4);
    // Each layer fades as it spreads — inner layer brightest, outer ones soft —
    // so the glow reads as a natural halo instead of a hard solid block.
    // `extra` widens each layer's blur (integer multipliers keep the CSS clean).
    let shadow = |col: &str, extra: usize| -> String {
        (1..=layers)
            .map(|i| {
                let alpha = (0.70 / i as f32).max(0.14);
                format!("0 0 calc({spread} * {}) {}", i + extra, hex_to_rgba(col, alpha))
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    // Smooth fade is this blog's signature — the glow eases in/out instead of
    // snapping. margin/transform are included so a glow target that also animates
    // (panel collapse, hover-scale) keeps doing so. ponytail: this replaces a few
    // minor per-element transitions (e.g. search-btn filter) — fine for a uniform,
    // smooth glow; the key collapse/scale animations are preserved.
    let transition =
        "transition: text-shadow 0.3s ease, box-shadow 0.3s ease, transform 0.2s ease, margin 0.2s ease-in-out;";
    let rule = |on: bool, color: &str, sel: &str, prop: &str| -> String {
        if !on {
            return String::new();
        }
        let col = first_non_empty(color, global);
        let hov = hover_selectors(sel);
        if c.glow_hover {
            // Hover-only: no glow at rest, but arm the transition so it fades.
            format!("{sel} {{ {transition} }}\n{hov} {{ {prop}: {} !important; }}\n", shadow(col, 0))
        } else {
            // Always on, with a wider bloom on hover/focus; both fade smoothly.
            format!(
                "{sel} {{ {prop}: {} !important; {transition} }}\n{hov} {{ {prop}: {} !important; }}\n",
                shadow(col, 0),
                shadow(col, 1)
            )
        }
    };
    let mut out = String::new();
    out.push_str(&rule(c.glow_logo, &c.glow_logo_color, ".institute-logo, .footer-logo", "box-shadow"));
    out.push_str(&rule(c.glow_title, &c.glow_title_color, ".post-title a, .post-title", "text-shadow"));
    out.push_str(&rule(c.glow_toc, &c.glow_toc_color, ".mor-toc-container, .mor-toc-list a", "text-shadow"));
    // Glow the OUTER side panel, not each inner widget box (nested box-shadow
    // halos on every widget look bad — this blog glows the sidebar itself).
    out.push_str(&rule(c.glow_sidebar, &c.glow_sidebar_color, ".mor-panel", "box-shadow"));
    // Typography also covers sidebar widget headings/links, so "glow the text"
    // works inside panels without boxing the containers.
    out.push_str(&rule(c.glow_text, &c.glow_text_color, ".post-body, .post-body p, .post-body h2, .post-body h3, .mor-panel .widget h2, .mor-panel .widget a", "text-shadow"));
    // Content cards only — not inner .widget boxes.
    out.push_str(&rule(c.glow_containers, &c.glow_containers_color, ".mor-post, .mor-catalog-dropdown", "box-shadow"));
    out.push_str(&rule(c.glow_icons, &c.glow_icons_color, ".icon-search-btn, .mor-nav a, .header-panel-toggle, .back-to-top-btn", "box-shadow"));
    out.push_str(&rule(c.glow_footer, &c.glow_footer_color, ".mor-footer, .mor-footer-mega, .mor-footer-social, .mor-footer-compact", "box-shadow"));
    out.push_str(&rule(c.glow_header, &c.glow_header_color, ".main-header, .gtk-headerbar", "box-shadow"));
    out.push_str(&rule(c.glow_main, &c.glow_main_color, ".canvas-core, .canvas-content", "box-shadow"));
    if out.is_empty() {
        String::new()
    } else {
        format!("\n/* ===== Per-target glow (Advanced Glow) ===== */\n{out}")
    }
}

pub fn render_css_sockets(mut xml: String, config: &ThemeConfig) -> String {
    let (light_palette, dark_palette) =
        resolve_palette_pair(
        config.active_preset_id.as_deref(),
        config.active_variant_id.as_deref(),
        config,
    );

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
    let light_glow = if light_palette.colors.glow_color.trim().is_empty() {
        &light_palette.colors.accent
    } else {
        &light_palette.colors.glow_color
    };
    xml = xml.replace("{{LIGHT_GLOW_COLOR}}", &escape_attr(light_glow));
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
    let dark_glow = if dark_palette.colors.glow_color.trim().is_empty() {
        &dark_palette.colors.accent
    } else {
        &dark_palette.colors.glow_color
    };
    xml = xml.replace("{{DARK_GLOW_COLOR}}", &escape_attr(dark_glow));
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
    // URL-encoded palette colors for inline-SVG cursors in preset CSS, so
    // themed cursors follow the active color scheme instead of baking a hex.
    let accent_urlenc = config.colors.accent.replace('#', "%23");
    let glow_urlenc = if config.colors.glow_color.trim().is_empty() {
        accent_urlenc.clone()
    } else {
        config.colors.glow_color.replace('#', "%23")
    };
    xml = xml.replace("{{ACCENT_URLENCODED}}", &accent_urlenc);
    xml = xml.replace("{{GLOW_URLENCODED}}", &glow_urlenc);
    let cs = &config.cursor_set;
    xml = xml.replace("{{CURSOR_POINTER}}", &cs.pointer);
    xml = xml.replace("{{CURSOR_TEXT}}", &cs.text);
    xml = xml.replace("{{CURSOR_HELP}}", &cs.help);
    xml = xml.replace("{{CURSOR_WAIT}}", &cs.wait);
    xml = xml.replace("{{CURSOR_NOT_ALLOWED}}", &cs.not_allowed);
    xml = xml.replace("{{CURSOR_CROSSHAIR}}", &cs.crosshair);
    xml = xml.replace("{{CURSOR_MOVE}}", &cs.move_);
    xml = xml.replace("{{CURSOR_GRAB}}", &cs.grab);
    xml = xml.replace("{{CURSOR_ZOOM}}", &cs.zoom_in);
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

    xml.push_str(&build_target_glow_css(config));

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
        let easing = if b.easing.trim().is_empty() {
            "ease"
        } else {
            b.easing.trim()
        };
        format!("  transition: all {} {};\n", b.transition_ms, easing)
    };

    let hover_extra = match b.hover_effect.as_str() {
        "lift" => "  transform: translateY(-2px);\n  box-shadow: 0 4px 10px rgba(0,0,0,0.3);\n".to_string(),
        "grow" => "  transform: scale(1.04);\n".to_string(),
        "brighten" => "  filter: brightness(1.12);\n".to_string(),
        // Customizable glow, also intensifies neon on hover.
        "glow" => format!("  box-shadow: 0 0 {g} {gc}, 0 0 calc({g} * 2) color-mix(in srgb, {gc} 50%, transparent);\n"),
        // Neon fill flip: button lights up to the accent with contrasting text,
        // a subtle scale, and a glow — the classic "fill on hover" link/button.
        "invert" => format!("  background: var(--accent);\n  color: var(--bg-base);\n  border-color: var(--accent);\n  transform: scale(1.03);\n  box-shadow: 0 0 {g} {gc};\n"),
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
mod glow_tests {
    use super::*;

    #[test]
    fn hover_only_mode_emits_no_resting_glow() {
        let mut config = ThemeConfig::default();
        config.colors.glow_title = true;
        config.colors.glow_spread = "10px".to_string();
        config.colors.glow_intensity = "3".to_string();
        config.colors.glow_hover = true; // hover-only (default)
        let css = build_target_glow_css(&config);
        // Three stacked layers, only inside a hover/focus rule.
        assert!(css.contains("calc(10px * 3)"));
        // :hover attaches to EACH selector in the list, not just the last.
        assert!(css.contains(".post-title a:hover"));
        assert!(css.contains(".post-title:hover"));
        assert!(css.contains(":focus-visible"));
        // Resting rule only arms the smooth transition — no glow shadow at rest.
        assert!(css.contains("transition: text-shadow 0.3s ease"));
        assert!(css.contains(".post-title a, .post-title { transition:"));
        assert!(!css.contains(".post-title a, .post-title { text-shadow"));
        // No widened hover layer in hover-only mode.
        assert!(!css.contains("calc(10px * 4)"));
    }

    #[test]
    fn always_mode_emits_resting_plus_wider_hover() {
        let mut config = ThemeConfig::default();
        config.colors.glow_title = true;
        config.colors.glow_spread = "10px".to_string();
        config.colors.glow_intensity = "3".to_string();
        config.colors.glow_hover = false; // always on
        let css = build_target_glow_css(&config);
        // Resting rule present, plus a wider hover bloom (3 + 1 layers).
        assert!(css.contains(".post-title a, .post-title {"));
        assert!(css.contains("calc(10px * 3)"));
        assert!(css.contains("calc(10px * 4)"));

        // Intensity clamps junk to 1; 1 layer = no second layer.
        config.colors.glow_intensity = "1".to_string();
        let soft = build_target_glow_css(&config);
        assert!(soft.contains("calc(10px * 1)"));
        assert!(!soft.contains("calc(10px * 3)"));
    }

    #[test]
    fn invert_hover_fills_with_accent() {
        let mut b = ButtonConfig::default();
        b.hover_effect = "invert".into();
        let css = render_button_styles(&b, "");
        // On hover the button flips to the accent fill with contrasting text.
        assert!(css.contains(":hover"));
        assert!(css.contains("background: var(--accent);"));
        assert!(css.contains("color: var(--bg-base);"));
        assert!(css.contains("transform: scale("));
    }

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
