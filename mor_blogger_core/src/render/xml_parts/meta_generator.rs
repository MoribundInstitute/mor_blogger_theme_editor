use crate::config::fonts::build_google_font_imports;
use crate::config::ThemeConfig;
use crate::render::ads::render_ads_head_script;
use crate::render::util::{escape_attr, first_non_empty};

pub fn render_meta_sockets(mut xml: String, config: &ThemeConfig) -> String {
    let site_home_url = first_non_empty(&config.site.home_url, "/");
    let favicon_url = first_non_empty(&config.assets.favicon_url, "https://imgur.com/QZ7pbY6");
    let social_card_image_url = first_non_empty(&config.assets.social_card_image_url, favicon_url);

    let google_fonts_link = build_google_font_imports(&[
        &config.typography.body_font_stack,
        &config.typography.heading_font_stack,
        &config.typography.mono_font_stack,
    ]);

    xml = xml.replace(
        "{{META_DESCRIPTION}}",
        &escape_attr(&config.seo.meta_description),
    );
    xml = xml.replace("{{META_KEYWORDS}}", &escape_attr(&config.seo.meta_keywords));
    xml = xml.replace("{{CUSTOM_ROBOTS}}", &escape_attr(&config.seo.custom_robots));
    xml = xml.replace("{{AUTHOR_NAME}}", &escape_attr(&config.seo.author_name));
    xml = xml.replace(
        "{{THEME_COLOR}}",
        &escape_attr(&config.colors.bg_panel.to_css()),
    );

    xml = xml.replace("{{OG_IMAGE_URL}}", &escape_attr(social_card_image_url));
    xml = xml.replace(
        "{{SOCIAL_CARD_IMAGE_URL}}",
        &escape_attr(social_card_image_url),
    );

    xml = xml.replace("{{FAVICON_URL}}", &escape_attr(favicon_url));
    xml = xml.replace("{{FAVICON_16_URL}}", &escape_attr(favicon_url));
    xml = xml.replace("{{FAVICON_32_URL}}", &escape_attr(favicon_url));
    xml = xml.replace("{{FAVICON_48_URL}}", &escape_attr(favicon_url));
    xml = xml.replace("{{FAVICON_192_URL}}", &escape_attr(favicon_url));
    xml = xml.replace("{{FAVICON_512_URL}}", &escape_attr(favicon_url));
    xml = xml.replace("{{APPLE_TOUCH_ICON_URL}}", &escape_attr(favicon_url));

    xml = xml.replace("{{CANONICAL_HOME_URL}}", &escape_attr(site_home_url));
    xml = xml.replace("{{LICENSE_URL}}", &escape_attr(&config.seo.license_url));

    xml = xml.replace("{{GOOGLE_FONTS_LINK}}", &google_fonts_link);
    xml = xml.replace("{{GOOGLE_FONT_IMPORTS}}", &google_fonts_link);
    xml = xml.replace("{{ADS_HEAD_SCRIPT}}", &render_ads_head_script(&config.ads));
    xml = xml.replace("{{CUSTOM_HEAD_HTML}}", "");

    xml
}
