use crate::config::ThemeConfig;
use crate::render::util::{escape_html, escape_attr, first_non_empty};

pub fn render_content_sockets(mut xml: String, config: &ThemeConfig) -> String {
    let favicon_url = first_non_empty(&config.assets.favicon_url, "https://imgur.com/QZ7pbY6");
    let social_card_image_url = first_non_empty(&config.assets.social_card_image_url, favicon_url);

    xml = xml.replace("{{BLOG_WIDGET_TITLE}}", "Blog Posts");
    xml = xml.replace("{{BLOG_COMMENT_LABEL}}", "Comment");
    xml = xml.replace("{{BLOG_AUTHOR_LABEL}}", &format!("By {}", escape_html(&config.seo.author_name)));
    xml = xml.replace("{{BLOG_TIMESTAMP_FORMAT}}", "d MMM, yyyy");
    xml = xml.replace("{{POST_TAGS_PREFIX}}", "Tags: ");
    
    xml = xml.replace("{{PAGER_NEWER_LABEL}}", "Newer");
    xml = xml.replace("{{PAGER_HOME_LABEL}}", "Home");
    xml = xml.replace("{{PAGER_OLDER_LABEL}}", "Older");
    
    xml = xml.replace("{{POST_METADATA_FALLBACK_IMAGE_URL}}", &escape_attr(social_card_image_url));
    
    xml = xml.replace("{{PUBLISHER_NAME}}", &escape_attr(&config.site.site_title));
    xml = xml.replace("{{PUBLISHER_LOGO_URL}}", &escape_attr(&config.site.header_logo_url));
    xml = xml.replace("{{PUBLISHER_LOGO_WIDTH}}", "206");
    xml = xml.replace("{{PUBLISHER_LOGO_HEIGHT}}", "60");

    xml
}