//! Low-level Blogger XML generator.
//!
//! This module now treats the modular files in `src/template_parts/` as the
//! source of truth. The old `src/template.xml` monolith should be kept only as
//! a reference/legacy fallback, not as the export input.

use crate::config::fonts::{build_google_font_imports, resolve_font_stack_with_fallback};
use crate::config::{BackgroundMode, ThemeConfig};
use crate::presets::PresetPalette;
use crate::render::template_resolver::resolve_template_parts;

use super::plugins::CORE_FRAMEWORK_JS;
use super::ads::{
    render_ads_consent_banner, render_ads_head_script, render_ads_runtime_script,
    render_ads_widget_sidebar,
};
use super::util::{escape_attr, escape_html, menu_link_or_empty};
use super::xml_parts::header_generator::render_header_sockets;

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

fn first_non_empty<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if primary.trim().is_empty() {
        fallback
    } else {
        primary
    }
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

// NEW: Generates pure values safe for CSS Custom Properties
fn generate_border_image_value(url: &str, slice: &str, repeat: &str) -> String {
    let url = url.trim();
    if url.is_empty() {
        return "none".to_string();
    }
    
    let slice = first_non_empty(slice, "30%");
    let repeat = sanitize_border_image_repeat(repeat);

    format!(
        "url('{}') {} {}",
        escape_attr(url),
        escape_attr(slice),
        escape_attr(repeat)
    )
}

fn generate_background_css(bg: &BackgroundMode) -> String {
    match bg {
        BackgroundMode::Solid { color } => format!("background-color: {};", escape_attr(color)),
        BackgroundMode::Gradient { from, to, angle_deg } => format!(
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

fn assemble_template(
    meta: &str,
    css: &str,
    header: &str,
    sidebar_left: &str,
    main: &str,
    sidebar_right: &str,
    javascript: &str,
    ads_consent_banner: &str,
    ads_runtime_script: &str,
) -> String {
    format!(
        "{meta}\n{css}\n<b:template-skin><![CDATA[]]></b:template-skin>\n</head>\n<body>\n{header}\n<div class='mor-workspace'>\n{left}\n{main}\n{right}\n</div>\n{ads_consent_banner}\n{ads_runtime_script}\n{js}\n</body>\n</html>",
        meta = meta,
        css = css,
        header = header,
        left = sidebar_left,
        main = main,
        right = sidebar_right,
        ads_consent_banner = ads_consent_banner,
        ads_runtime_script = ads_runtime_script,
        js = javascript,
    )
}

pub(super) fn render_template(
    config: &ThemeConfig,
    light_palette: &PresetPalette,
    dark_palette: &PresetPalette,
) -> String {
    let light_bg_css = generate_background_css(&light_palette.background.mode);
    let dark_bg_css = generate_background_css(&dark_palette.background.mode);

    let background_tile_url = match &config.background.mode {
        BackgroundMode::Tile { url } => url.clone(),
        _ => String::new(),
    };

    let body_stack =
        resolve_font_stack_with_fallback(&config.typography.body_font_stack, "serif");

    let heading_stack = if config.typography.heading_font_stack.trim().is_empty() {
        body_stack.clone()
    } else {
        resolve_font_stack_with_fallback(&config.typography.heading_font_stack, "serif")
    };

    let mono_stack =
        resolve_font_stack_with_fallback(&config.typography.mono_font_stack, "monospace");

    let google_fonts_link = build_google_font_imports(&[
        &config.typography.body_font_stack,
        &config.typography.heading_font_stack,
        &config.typography.mono_font_stack,
    ]);

    let menu_1 = menu_link_or_empty(config, 0);
    let menu_2 = menu_link_or_empty(config, 1);
    let menu_3 = menu_link_or_empty(config, 2);
    let menu_4 = menu_link_or_empty(config, 3);

    let ads_widget_sidebar = render_ads_widget_sidebar(&config.ads);
    let ads_head_script = render_ads_head_script(&config.ads);
    let ads_consent_banner = render_ads_consent_banner(&config.ads);
    let ads_runtime_script = render_ads_runtime_script(&config.ads);

    let color_accent_soft = hex_to_rgba(&config.colors.accent, 0.45);
    let color_accent_shadow = hex_to_rgba(&config.colors.accent, 0.25);
    let color_accent_wash = hex_to_rgba(&config.colors.accent, 0.08);
    let fluid_glow = fluid_glow_css(&config.colors.accent);
    let glow_spread = first_non_empty(&config.colors.glow_spread, "8px");
    
    // Border Variables
    let panel_border_width = first_non_empty(&config.colors.panel_border_width, "1px");
    let panel_border_image_slice = first_non_empty(&config.colors.panel_border_image_slice, "30%");
    let panel_border_image_repeat = sanitize_border_image_repeat(&config.colors.panel_border_image_repeat);
    let panel_border_image_css = generate_border_image_css(&config.colors.panel_border_image_url, panel_border_image_slice, panel_border_image_repeat);
    let panel_border_image_value = generate_border_image_value(&config.colors.panel_border_image_url, panel_border_image_slice, panel_border_image_repeat);

    let light_panel_border_image_slice = first_non_empty(&light_palette.colors.panel_border_image_slice, "30%");
    let light_panel_border_image_repeat = sanitize_border_image_repeat(&light_palette.colors.panel_border_image_repeat);
    let light_panel_border_image_css = generate_border_image_css(&light_palette.colors.panel_border_image_url, light_panel_border_image_slice, light_panel_border_image_repeat);
    let light_panel_border_image_value = generate_border_image_value(&light_palette.colors.panel_border_image_url, light_panel_border_image_slice, light_panel_border_image_repeat);

    let dark_panel_border_image_slice = first_non_empty(&dark_palette.colors.panel_border_image_slice, "30%");
    let dark_panel_border_image_repeat = sanitize_border_image_repeat(&dark_palette.colors.panel_border_image_repeat);
    let dark_panel_border_image_css = generate_border_image_css(&dark_palette.colors.panel_border_image_url, dark_panel_border_image_slice, dark_panel_border_image_repeat);
    let dark_panel_border_image_value = generate_border_image_value(&dark_palette.colors.panel_border_image_url, dark_panel_border_image_slice, dark_panel_border_image_repeat);

    let site_home_url = first_non_empty(&config.site.home_url, "/");
    let favicon_url = first_non_empty(&config.assets.favicon_url, "https://imgur.com/QZ7pbY6");
    let social_card_image_url = first_non_empty(&config.assets.social_card_image_url, favicon_url);

    let parts = resolve_template_parts(config);

    let base_xml = assemble_template(
        &parts.meta,
        &parts.css,
        &parts.header,
        &parts.sidebar_left,
        &parts.main,
        &parts.sidebar_right,
        &parts.javascript,
        &ads_consent_banner,
        &ads_runtime_script,
    );

    let rendered = base_xml
        .replace("{{MAIN_CONTENT_MODULE}}", parts.content)
        .replace("{{FOOTER_MODULE}}", parts.footer)
        .replace("{{WIDGET_ADSENSE_SIDEBAR}}", &ads_widget_sidebar)
        .replace("{{GOOGLE_FONTS_LINK}}", &google_fonts_link)
        .replace("{{GOOGLE_FONT_IMPORTS}}", &google_fonts_link)
        .replace("{{ADS_HEAD_SCRIPT}}", &ads_head_script)
        .replace("{{CUSTOM_HEAD_HTML}}", "")
        .replace("{{META_DESCRIPTION}}", &escape_attr(&config.seo.meta_description))
        .replace("{{META_KEYWORDS}}", &escape_attr(&config.seo.meta_keywords))
        .replace("{{CUSTOM_ROBOTS}}", &escape_attr(&config.seo.custom_robots))
        .replace("{{AUTHOR_NAME}}", &escape_attr(&config.seo.author_name))
        .replace("{{THEME_COLOR}}", &escape_attr(&config.colors.bg_panel.to_css()))
        .replace("{{OG_IMAGE_URL}}", &escape_attr(social_card_image_url))
        .replace("{{SOCIAL_CARD_IMAGE_URL}}", &escape_attr(social_card_image_url))
        .replace("{{FAVICON_URL}}", &escape_attr(favicon_url))
        .replace("{{FAVICON_16_URL}}", &escape_attr(favicon_url))
        .replace("{{FAVICON_32_URL}}", &escape_attr(favicon_url))
        .replace("{{FAVICON_48_URL}}", &escape_attr(favicon_url))
        .replace("{{FAVICON_192_URL}}", &escape_attr(favicon_url))
        .replace("{{FAVICON_512_URL}}", &escape_attr(favicon_url))
        .replace("{{APPLE_TOUCH_ICON_URL}}", &escape_attr(favicon_url))
        .replace("{{CANONICAL_HOME_URL}}", &escape_attr(site_home_url))
        .replace("{{LICENSE_URL}}", &escape_attr(&config.seo.license_url))
        .replace("{{SITE_TITLE}}", &escape_html(&config.site.site_title))
        .replace("{{SITE_TITLE_ATTR}}", &escape_attr(&config.site.site_title))
        .replace("{{SITE_SUBTITLE}}", &escape_html(&config.site.site_subtitle))
        .replace("{{SITE_SUBTITLE_ATTR}}", &escape_attr(&config.site.site_subtitle))
        .replace("{{HEADER_LOGO_URL}}", &escape_attr(&config.site.header_logo_url))
        .replace("{{HEADER_LOGO_URL_ATTR}}", &escape_attr(&config.site.header_logo_url))
        .replace("{{SITE_HOME_URL}}", &escape_attr(site_home_url))
        .replace("{{SITE_HOME_URL_ATTR}}", &escape_attr(site_home_url))
        .replace("{{HOME_URL}}", &escape_attr(site_home_url))
        .replace("{{LEFT_PANEL_OPEN_LABEL}}", "Browse")
        .replace("{{RIGHT_PANEL_OPEN_LABEL}}", "Contents")
        .replace("{{NAV_HOME_LABEL}}", &escape_html(first_non_empty(&menu_1.label, "~/home")))
        .replace("{{NAV_HOME_URL}}", &escape_attr(first_non_empty(&menu_1.url, site_home_url)))
        .replace("{{NAV_ABOUT_LABEL}}", &escape_html(first_non_empty(&menu_2.label, "~/about")))
        .replace("{{NAV_ABOUT_URL}}", &escape_attr(first_non_empty(&menu_2.url, "/p/about.html")))
        .replace("{{NAV_PROJECTS_LABEL}}", &escape_html(first_non_empty(&menu_3.label, "~/projects")))
        .replace("{{NAV_PROJECTS_URL}}", &escape_attr(first_non_empty(&menu_3.url, "/p/projects.html")))
        .replace("{{NAV_CONTACT_LABEL}}", &escape_html(first_non_empty(&menu_4.label, "~/contact")))
        .replace("{{NAV_CONTACT_URL}}", &escape_attr(first_non_empty(&menu_4.url, "/p/contact.html")))
        .replace("{{NAV_MERCH_LABEL}}", "~/merch")
        .replace("{{NAV_MERCH_URL}}", "https://www.redbubble.com/people/WearYourWords/shop")
        .replace("{{NAV_POSTS_LABEL}}", "~/posts")
        .replace("{{NAV_POSTS_URL}}", "/search")
        .replace("{{NAV_CATEGORIES_LABEL}}", "~/categories")
        .replace("{{NAV_CATEGORIES_URL}}", "/p/categories.html")
        .replace("{{NAV_ARCHIVE_LABEL}}", "~/archive")
        .replace("{{NAV_ARCHIVE_URL}}", "/p/archive.html")
        .replace("{{SEARCH_ACTION_URL}}", "/search")
        .replace("{{SEARCH_ACTION_URL_ATTR}}", "/search")
        .replace("{{SEARCH_PROMPT}}", "root@moribund:~$")
        .replace("{{SEARCH_PLACEHOLDER}}", "Search...")
        .replace("{{SEARCH_PLACEHOLDER_ATTR}}", "Search...")
        .replace("{{SEARCH_BUTTON_LABEL}}", "Search")
        .replace("{{CATALOG_TRIGGER_LABEL}}", "~/catalog")
        .replace("{{CATALOG_ALL_LABEL}}", "See full catalog")
        .replace("{{CATALOG_ALL_URL}}", "/p/catalog.html")
        .replace("{{CATALOG_SUBJECTS_LABEL}}", "School Subjects")
        .replace("{{CATALOG_SUBJECTS_URL}}", "/p/catalog.html")
        .replace("{{CATALOG_LEXICON_LABEL}}", "Lexicon Labs")
        .replace("{{CATALOG_LEXICON_URL}}", "/p/mordictionary.html")
        .replace("{{CATALOG_MEDIA_LABEL}}", "Media Quests")
        .replace("{{CATALOG_MEDIA_URL}}", "/p/audiobook-gaming.html")
        .replace("{{CATALOG_WIKI_LABEL}}", "WanderingWikis")
        .replace("{{CATALOG_WIKI_URL}}", "/p/wandering-wikis.html")
        .replace("{{CATALOG_PROJECTS_LABEL}}", "Institute Projects")
        .replace("{{CATALOG_PROJECTS_URL}}", "/p/projects.html")
        .replace("{{CATALOG_PROGRESS_LABEL}}", "Progress &amp; Tracker")
        .replace("{{CATALOG_PROGRESS_URL}}", "/p/progress.html")
        .replace("{{SUBJECT_000_LABEL}}", "000 General Works &amp; Knowledge Systems")
        .replace("{{SUBJECT_000_URL}}", "/p/000-general-works.html")
        .replace("{{SUBJECT_100_LABEL}}", "100 Philosophy &amp; Psychology")
        .replace("{{SUBJECT_100_URL}}", "/p/100-philosophy.html")
        .replace("{{SUBJECT_200_LABEL}}", "200 Religion &amp; Theology")
        .replace("{{SUBJECT_200_URL}}", "/p/200-religion.html")
        .replace("{{SUBJECT_300_LABEL}}", "300 Social Sciences")
        .replace("{{SUBJECT_300_URL}}", "/p/300-social-sciences.html")
        .replace("{{SUBJECT_400_LABEL}}", "400 Language &amp; Lexicography")
        .replace("{{SUBJECT_400_URL}}", "/p/400-language.html")
        .replace("{{SUBJECT_500_LABEL}}", "500 Science &amp; Mathematics")
        .replace("{{SUBJECT_500_URL}}", "/p/500-science.html")
        .replace("{{SUBJECT_600_LABEL}}", "600 Technology &amp; Applied Arts")
        .replace("{{SUBJECT_600_URL}}", "/p/600-technology.html")
        .replace("{{SUBJECT_700_LABEL}}", "700 Arts, Media &amp; Games")
        .replace("{{SUBJECT_700_URL}}", "/p/700-arts.html")
        .replace("{{SUBJECT_800_LABEL}}", "800 Literature &amp; Rhetoric")
        .replace("{{SUBJECT_800_URL}}", "/p/800-literature.html")
        .replace("{{SUBJECT_900_LABEL}}", "900 History &amp; Geography")
        .replace("{{SUBJECT_900_URL}}", "/p/900-history.html")
        .replace("{{LEXICON_MORDICTIONARY_LABEL}}", "MorDictionary")
        .replace("{{LEXICON_MORDICTIONARY_URL}}", "/p/mordictionary.html")
        .replace("{{LEXICON_WEAR_YOUR_DICTIONARY_LABEL}}", "WearYourDictionary")
        .replace("{{LEXICON_WEAR_YOUR_DICTIONARY_URL}}", "/p/wearyourdictionary.html")
        .replace("{{LEXICON_VOCABULARY_LABEL}}", "Vocabulary Projects")
        .replace("{{LEXICON_VOCABULARY_URL}}", "/search/label/Vocabulary")
        .replace("{{LEXICON_ETYMOLOGY_LABEL}}", "Etymology Notes")
        .replace("{{LEXICON_ETYMOLOGY_URL}}", "/search/label/Etymology")
        .replace("{{LEXICON_WORDPLAY_LABEL}}", "Wordplay &amp; Coinages")
        .replace("{{LEXICON_WORDPLAY_URL}}", "/search/label/Wordplay")
        .replace("{{LEXICON_LANGUAGE_LABEL}}", "Language Experiments")
        .replace("{{LEXICON_LANGUAGE_URL}}", "/search/label/Language")
        .replace("{{MEDIA_AUDIOBOOK_GAMING_LABEL}}", "Audiobook Gaming")
        .replace("{{MEDIA_AUDIOBOOK_GAMING_URL}}", "/p/audiobook-gaming.html")
        .replace("{{MEDIA_WATCHLISTS_LABEL}}", "Watchlists")
        .replace("{{MEDIA_WATCHLISTS_URL}}", "/search/label/Watchlists")
        .replace("{{MEDIA_READING_LABEL}}", "Reading Trails")
        .replace("{{MEDIA_READING_URL}}", "/search/label/Reading")
        .replace("{{MEDIA_LISTENING_LABEL}}", "Listening Projects")
        .replace("{{MEDIA_LISTENING_URL}}", "/search/label/Listening")
        .replace("{{MEDIA_SOCIAL_SCIENCE_LABEL}}", "Social Science Watch")
        .replace("{{MEDIA_SOCIAL_SCIENCE_URL}}", "/search/label/Social%20Science")
        .replace("{{MEDIA_DIET_LABEL}}", "Media Diet Diaries")
        .replace("{{MEDIA_DIET_URL}}", "/search/label/Media%20Diet")
        .replace("{{WIKI_START_LABEL}}", "Start Here")
        .replace("{{WIKI_START_URL}}", "/p/wandering-wikis.html")
        .replace("{{WIKI_ALL_POSTS_LABEL}}", "All Moribund Institute Posts")
        .replace("{{WIKI_ALL_POSTS_URL}}", "/search/label/Wandering%20Wikis")
        .replace("{{WIKI_TRAILS_LABEL}}", "Wiki Trails")
        .replace("{{WIKI_TRAILS_URL}}", "/search/label/Wiki%20Trails")
        .replace("{{WIKI_WALKING_LABEL}}", "Wiki Walking")
        .replace("{{WIKI_WALKING_URL}}", "/search/label/Wiki%20Walking")
        .replace("{{WIKI_VIDEO_COMMENTARY_LABEL}}", "Video Commentary")
        .replace("{{WIKI_VIDEO_COMMENTARY_URL}}", "/search/label/Video%20Commentary")
        .replace("{{WIKI_LEXICOGRAPHY_LABEL}}", "Lexicographical Riffs")
        .replace("{{WIKI_LEXICOGRAPHY_URL}}", "/search/label/Lexicography")
        .replace("{{WIKI_BLOG_LABEL}}", "Wandering Wikis Blog")
        .replace("{{WIKI_BLOG_URL}}", "https://wanderingwikis.blogspot.com/")
        .replace("{{WIKI_OFFICIAL_LABEL}}", "Official Wiki")
        .replace("{{WIKI_OFFICIAL_URL}}", "https://wanderingwikis.org/wiki/Main_Page")
        .replace("{{WIKI_YOUTUBE_LABEL}}", "YouTube")
        .replace("{{WIKI_YOUTUBE_URL}}", "https://www.youtube.com/@wanderingwikis")
        .replace("{{WIKI_REDDIT_LABEL}}", "Reddit Community")
        .replace("{{WIKI_REDDIT_URL}}", "https://www.reddit.com/r/WanderingWikis/")
        .replace("{{PROJECTS_ALL_LABEL}}", "All Projects")
        .replace("{{PROJECTS_ALL_URL}}", "/p/projects.html")
        .replace("{{PROJECTS_INSTITUTE_LABEL}}", "The Moribund Institute")
        .replace("{{PROJECTS_INSTITUTE_URL}}", "/p/moribund-institute.html")
        .replace("{{PROJECTS_WEAR_YOUR_WORDS_LABEL}}", "Wear Your Words")
        .replace("{{PROJECTS_WEAR_YOUR_WORDS_URL}}", "/p/wear-your-words.html")
        .replace("{{PROJECTS_MORBLOCKS_LABEL}}", "MorBlocks")
        .replace("{{PROJECTS_MORBLOCKS_URL}}", "/p/morblocks.html")
        .replace("{{PROJECTS_MORLESSONBUILDER_LABEL}}", "Mor Lesson Builder")
        .replace("{{PROJECTS_MORLESSONBUILDER_URL}}", "/p/morlessonbuilder.html")
        .replace("{{PROJECTS_LOG_LABEL}}", "Project Logs")
        .replace("{{PROJECTS_LOG_URL}}", "/search/label/Project%20Log")
        .replace("{{PROGRESS_DASHBOARD_LABEL}}", "Progress Dashboard")
        .replace("{{PROGRESS_DASHBOARD_URL}}", "/p/progress.html")
        .replace("{{PROGRESS_OFFLINE_TRACKER_LABEL}}", "MorLMS Offline Tracker")
        .replace("{{PROGRESS_OFFLINE_TRACKER_URL}}", "/p/offline-tracker.html")
        .replace("{{PROGRESS_CATALOG_LABEL}}", "Browse Catalog")
        .replace("{{PROGRESS_CATALOG_URL}}", "/p/catalog.html")
        .replace("{{PROGRESS_LESSONS_LABEL}}", "Lessons")
        .replace("{{PROGRESS_LESSONS_URL}}", "/search/label/Lesson")
        .replace("{{PROGRESS_ACTIVITIES_LABEL}}", "Activities")
        .replace("{{PROGRESS_ACTIVITIES_URL}}", "/search/label/Activity")

        // --- INJECT LIGHT PALETTE ---
        .replace("{{LIGHT_BG_BASE}}", &escape_attr(&light_palette.colors.bg_base))
        .replace("{{LIGHT_BG_PANEL}}", &escape_attr(&light_palette.colors.bg_panel.to_css()))
        .replace("{{LIGHT_BG_ELEVATED}}", &escape_attr(&light_palette.colors.bg_elevated.to_css()))
        .replace("{{LIGHT_FG_BASE}}", &escape_attr(&light_palette.colors.fg_base))
        .replace("{{LIGHT_FG_MUTED}}", &escape_attr(&light_palette.colors.fg_muted))
        .replace("{{LIGHT_ACCENT}}", &escape_attr(&light_palette.colors.accent))
        .replace("{{LIGHT_BORDER}}", &escape_attr(&light_palette.colors.border))
        .replace("{{LIGHT_PANEL_BORDER_WIDTH}}", &escape_attr(&light_palette.colors.panel_border_width))
        .replace("{{LIGHT_GLOW_SPREAD}}", &escape_attr(&light_palette.colors.glow_spread))
        .replace("{{LIGHT_HOVER_SCALE}}", &escape_attr(&light_palette.colors.hover_scale))
        .replace("{{LIGHT_PANEL_BORDER_IMAGE_URL}}", &escape_attr(&light_palette.colors.panel_border_image_url))
        .replace("{{LIGHT_PANEL_BORDER_IMAGE_SLICE}}", &escape_attr(light_panel_border_image_slice))
        .replace("{{LIGHT_PANEL_BORDER_IMAGE_REPEAT}}", &escape_attr(light_panel_border_image_repeat))
        .replace("{{LIGHT_PANEL_BORDER_IMAGE_CSS}}", &light_panel_border_image_css)
        .replace("{{LIGHT_PANEL_BORDER_IMAGE_VALUE}}", &light_panel_border_image_value)
        .replace("{{LIGHT_BACKGROUND_TILE_CSS}}", &light_bg_css)

        // --- INJECT DARK PALETTE ---
        .replace("{{DARK_BG_BASE}}", &escape_attr(&dark_palette.colors.bg_base))
        .replace("{{DARK_BG_PANEL}}", &escape_attr(&dark_palette.colors.bg_panel.to_css()))
        .replace("{{DARK_BG_ELEVATED}}", &escape_attr(&dark_palette.colors.bg_elevated.to_css()))
        .replace("{{DARK_FG_BASE}}", &escape_attr(&dark_palette.colors.fg_base))
        .replace("{{DARK_FG_MUTED}}", &escape_attr(&dark_palette.colors.fg_muted))
        .replace("{{DARK_ACCENT}}", &escape_attr(&dark_palette.colors.accent))
        .replace("{{DARK_BORDER}}", &escape_attr(&dark_palette.colors.border))
        .replace("{{DARK_PANEL_BORDER_WIDTH}}", &escape_attr(&dark_palette.colors.panel_border_width))
        .replace("{{DARK_GLOW_SPREAD}}", &escape_attr(&dark_palette.colors.glow_spread))
        .replace("{{DARK_HOVER_SCALE}}", &escape_attr(&dark_palette.colors.hover_scale))
        .replace("{{DARK_PANEL_BORDER_IMAGE_URL}}", &escape_attr(&dark_palette.colors.panel_border_image_url))
        .replace("{{DARK_PANEL_BORDER_IMAGE_SLICE}}", &escape_attr(dark_panel_border_image_slice))
        .replace("{{DARK_PANEL_BORDER_IMAGE_REPEAT}}", &escape_attr(dark_panel_border_image_repeat))
        .replace("{{DARK_PANEL_BORDER_IMAGE_CSS}}", &dark_panel_border_image_css)
        .replace("{{DARK_PANEL_BORDER_IMAGE_VALUE}}", &dark_panel_border_image_value)
        .replace("{{DARK_BACKGROUND_TILE_CSS}}", &dark_bg_css)

        // Base Config Fallbacks
        .replace("{{COLOR_ACCENT_SOFT}}", &escape_attr(&color_accent_soft))
        .replace("{{COLOR_ACCENT_SHADOW}}", &escape_attr(&color_accent_shadow))
        .replace("{{COLOR_ACCENT_WASH}}", &escape_attr(&color_accent_wash))
        .replace("{{FLUID_GLOW_CSS}}", &fluid_glow)
        .replace("{{FONT_BODY}}", &escape_attr(&body_stack))
        .replace("{{FONT_HEADING}}", &escape_attr(&heading_stack))
        .replace("{{FONT_MONO}}", &escape_attr(&mono_stack))
        .replace("{{BASE_SIZE}}", &escape_attr(&config.typography.base_size))
        .replace("{{SCALE_RATIO}}", &escape_attr(&config.typography.scale_ratio))
        .replace("{{LINE_HEIGHT}}", &escape_attr(&config.typography.line_height))
        .replace("{{HEADING_WEIGHT}}", &escape_attr(&config.typography.heading_weight))
        .replace("{{BTN_RADIUS}}", &escape_attr(&config.buttons.radius))
        .replace("{{BTN_BORDER_WIDTH}}", &escape_attr(&config.buttons.border_width))
        .replace("{{BTN_TEXT_TRANSFORM}}", &escape_attr(&config.buttons.text_transform))
        .replace("{{PANEL_BORDER_WIDTH}}", &escape_attr(panel_border_width))
        .replace("{{CONTAINER_BORDER_WIDTH}}", &escape_attr(panel_border_width))
        .replace("{{FRAME_BORDER_WIDTH}}", &escape_attr(panel_border_width))
        .replace("{{PANEL_BORDER_IMAGE_URL}}", &escape_attr(&config.colors.panel_border_image_url))
        .replace("{{PANEL_BORDER_IMAGE_SLICE}}", &escape_attr(panel_border_image_slice))
        .replace("{{PANEL_BORDER_IMAGE_REPEAT}}", &escape_attr(panel_border_image_repeat))
        .replace("{{PANEL_BORDER_IMAGE_CSS}}", &panel_border_image_css)
        .replace("{{PANEL_BORDER_IMAGE_VALUE}}", &panel_border_image_value)
        .replace("{{CONTAINER_BORDER_IMAGE_CSS}}", &panel_border_image_css)
        .replace("{{FRAME_BORDER_IMAGE_CSS}}", &panel_border_image_css)
        .replace("{{GLOW_SPREAD}}", &escape_attr(glow_spread))
        .replace("{{HOVER_SCALE}}", &escape_attr(&config.colors.hover_scale))
        .replace("{{SIDEBAR_WIDTH}}", "300px")
        .replace(
            "{{GLOW_SOFT}}",
            &format!(
                "0 0 calc({} / 2) {}",
                escape_attr(glow_spread),
                escape_attr(&color_accent_soft)
            ),
        )
        .replace(
            "{{GLOW_STRONG}}",
            &format!(
                "0 0 {} {}",
                escape_attr(glow_spread),
                escape_attr(&config.colors.accent)
            ),
        )
        .replace("{{SHADOW_ELEVATED}}", "0 18px 45px rgba(0, 0, 0, 0.65)")
        .replace("{{HEADER_LOGO_SIZE}}", "72px")
        .replace("{{HEADER_LOGO_RADIUS}}", &escape_attr(&config.buttons.radius))
        .replace("{{SITE_TITLE_SIZE}}", "1.05rem")
        .replace("{{SITE_TITLE_LETTER_SPACING}}", "2px")
        .replace("{{SEARCH_INPUT_WIDTH}}", "180px")
        .replace("{{SEARCH_INPUT_FOCUS_WIDTH}}", "240px")
        .replace("{{CONTENT_MAX_WIDTH}}", "1200px")
        .replace("{{POST_TITLE_SIZE}}", "1.5rem")
        .replace("{{HEADER_LOGO_SIZE_MOBILE}}", "48px")
        .replace("{{HEADER_LOGO_RADIUS_MOBILE}}", &escape_attr(&config.buttons.radius))
        .replace("{{SITE_TITLE_SIZE_MOBILE}}", "0.95rem")
        .replace("{{SITE_TITLE_LETTER_SPACING_MOBILE}}", "1px")
        .replace("{{POST_TITLE_SIZE_MOBILE}}", "1.25rem")
        .replace("{{HEADER_LOGO_SIZE_SMALL}}", "38px")
        .replace("{{HEADER_LOGO_RADIUS_SMALL}}", &escape_attr(&config.buttons.radius))
        .replace("{{SITE_TITLE_SIZE_SMALL}}", "0.8rem")
        .replace("{{BACKGROUND_TILE_URL}}", &escape_attr(&background_tile_url))
        .replace("{{ICON_SIDEBAR_LEFT}}", &config.icons.sidebar_left)
        .replace("{{ICON_SIDEBAR_RIGHT}}", &config.icons.sidebar_right)
        .replace("{{ICON_PANEL_CLOSE}}", &config.icons.panel_close)
        .replace("{{ICON_SEARCH}}", &config.icons.search)
        .replace("{{ICON_MENU}}", &config.icons.menu)
        .replace("{{PRESET_CSS}}", &config.preset_css)
        // Left sidebar.
        .replace("{{LEFT_PANEL_TITLE}}", "Browse")
        .replace("{{LEFT_PANEL_CLOSE_LABEL}}", "Close")
        .replace("{{LABEL_WIDGET_TITLE}}", "Labels")
        .replace("{{LABEL_SORTING}}", "ALPHA")
        .replace("{{LABEL_DISPLAY}}", "LIST")
        .replace("{{LABEL_SHOW_TYPE}}", "ALL")
        .replace("{{LABEL_SHOW_FREQ_NUMBERS}}", "false")
        .replace("{{ARCHIVE_WIDGET_TITLE}}", "Archive")
        .replace("{{ARCHIVE_SHOW_STYLE}}", "HIERARCHY")
        .replace("{{ARCHIVE_YEAR_PATTERN}}", "yyyy")
        .replace("{{ARCHIVE_SHOW_WEEK_END}}", "true")
        .replace("{{ARCHIVE_MONTH_PATTERN}}", "MMMM")
        .replace("{{ARCHIVE_DAY_PATTERN}}", "MMM dd")
        .replace("{{ARCHIVE_WEEK_PATTERN}}", "MM/dd")
        .replace("{{ARCHIVE_CHRONOLOGICAL}}", "false")
        .replace("{{ARCHIVE_SHOW_POSTS}}", "true")
        .replace("{{ARCHIVE_FREQUENCY}}", "MONTHLY")
        // Right sidebar / table of contents.
        .replace("{{RIGHT_PANEL_TITLE}}", "Contents")
        .replace("{{RIGHT_PANEL_CLOSE_LABEL}}", "Close")
        .replace("{{RIGHT_WIDGET_TITLE}}", "Table of Contents")
        .replace("{{TOC_LOADING_MESSAGE}}", "Building contents...")
        .replace("{{TOC_WAITING_MESSAGE}}", "Waiting for document...")
        .replace("{{TOC_EMPTY_MESSAGE}}", "No anchor points found in document.")
        .replace("{{TOC_HEADING_SELECTOR}}", "h2, h3, h4, h5")
        .replace("{{TOC_INDENT_STEP}}", "15")
        .replace("{{TOC_PRIMARY_MARKER}}", ">")
        .replace("{{TOC_CHILD_MARKER}}", "-")
        .replace("{{TOC_ITEM_MARGIN_BOTTOM}}", "8px")
        .replace("{{TOC_ITEM_FONT_SIZE}}", "0.85rem")
        // Blog and footer.
        .replace("{{BLOG_WIDGET_TITLE}}", "Blog Posts")
        .replace("{{BLOG_COMMENT_LABEL}}", "Comment")
        .replace("{{BLOG_AUTHOR_LABEL}}", &format!("By {}", escape_html(&config.seo.author_name)))
        .replace("{{BLOG_TIMESTAMP_FORMAT}}", "d MMM, yyyy")
        .replace("{{POST_TAGS_PREFIX}}", "Tags: ")
        .replace("{{PAGER_NEWER_LABEL}}", "Newer")
        .replace("{{PAGER_HOME_LABEL}}", "Home")
        .replace("{{PAGER_OLDER_LABEL}}", "Older")
        .replace("{{POST_METADATA_FALLBACK_IMAGE_URL}}", &escape_attr(social_card_image_url))
        .replace("{{PUBLISHER_NAME}}", &escape_attr(&config.site.site_title))
        .replace("{{PUBLISHER_LOGO_URL}}", &escape_attr(&config.site.header_logo_url))
        .replace("{{PUBLISHER_LOGO_WIDTH}}", "206")
        .replace("{{PUBLISHER_LOGO_HEIGHT}}", "60")
        .replace("{{FOOTER_SYS_MESSAGE}}", &escape_html(&config.footer.footer_text))
        .replace("{{FOOTER_LICENSE_URL}}", &escape_attr(&config.footer.footer_license_url))
        .replace("{{FOOTER_LICENSE_LABEL}}", &escape_html(&config.footer.footer_license_label))
        .replace("{{BACK_TO_TOP_LABEL}}", "Back to Top")
        // 1. Inject the Core Client OS Framework
        // 2. Append the user's custom JS right below it
        .replace(
            "{{CUSTOM_BEFORE_BODY_JS}}",
            &format!("{}\n{}", CORE_FRAMEWORK_JS, &config.plugins.custom_js),
        );

    let mut final_xml = rendered;

    for (col_idx, col) in config.footer.columns.iter().enumerate() {
        let c = col_idx + 1; // 1-indexed for XML
        final_xml = final_xml.replace(
            &format!("{{{{FOOTER_COL_{}_HEADING}}}}", c),
            &escape_html(&col.heading),
        );

        for (link_idx, link) in col.links.iter().enumerate() {
            let l = link_idx + 1; // 1-indexed for XML
            final_xml = final_xml.replace(
                &format!("{{{{FOOTER_{}_LINK_{}_LABEL}}}}", c, l),
                &escape_html(&link.label),
            );
            final_xml = final_xml.replace(
                &format!("{{{{FOOTER_{}_LINK_{}_URL}}}}", c, l),
                &escape_attr(&link.url),
            );
        }
    }

    for (leg_idx, link) in config.footer.legal_links.iter().enumerate() {
        let l = leg_idx + 1;
        final_xml = final_xml.replace(
            &format!("{{{{FOOTER_LEGAL_{}_LABEL}}}}", l),
            &escape_html(&link.label),
        );
        final_xml = final_xml.replace(
            &format!("{{{{FOOTER_LEGAL_{}_URL}}}}", l),
            &escape_attr(&link.url),
        );
    }

    for c in 1..=6 {
        final_xml = final_xml.replace(&format!("{{{{FOOTER_COL_{}_HEADING}}}}", c), "");
        for l in 1..=5 {
            final_xml = final_xml.replace(
                &format!("{{{{FOOTER_{}_LINK_{}_LABEL}}}}", c, l),
                "",
            );
            final_xml = final_xml.replace(
                &format!("{{{{FOOTER_{}_LINK_{}_URL}}}}", c, l),
                "#",
            );
        }
    }

    for l in 1..=4 {
        final_xml = final_xml.replace(&format!("{{{{FOOTER_LEGAL_{}_LABEL}}}}", l), "");
        final_xml = final_xml.replace(&format!("{{{{FOOTER_LEGAL_{}_URL}}}}", l), "#");
    }

    render_header_sockets(final_xml, config)
}