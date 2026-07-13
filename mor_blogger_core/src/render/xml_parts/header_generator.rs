use crate::config::{MenuLink, ThemeConfig};
use crate::render::tracking::menu_link_anchor;
use crate::render::util::{escape_attr, escape_html, first_non_empty};

fn menu_link_or_empty(config: &ThemeConfig, index: usize) -> MenuLink {
    config.menu_links.get(index).cloned().unwrap_or_default()
}

pub fn render_header_sockets(mut xml: String, config: &ThemeConfig) -> String {
    let site_home_url = first_non_empty(&config.site.home_url, "/");
    let header_logo_img = if config.site.header_logo_url.trim().is_empty() {
        String::new()
    } else {
        format!(
            "<img alt=\"{} logo\" class='institute-logo' src=\"{}\"/>",
            escape_attr(&config.site.site_title),
            escape_attr(&config.site.header_logo_url)
        )
    };

    let menu_1 = menu_link_or_empty(config, 0);
    let menu_2 = menu_link_or_empty(config, 1);
    let menu_3 = menu_link_or_empty(config, 2);
    let menu_4 = menu_link_or_empty(config, 3);

    // Logo if set, otherwise the site title — mirrors the preview's branding so
    // the centered-search header shows a brand even with no logo configured.
    let header_branding = if header_logo_img.is_empty() {
        format!(
            "<span class='institute-title' data-field-path='site.site_title'>{}</span>",
            escape_html(&config.site.site_title)
        )
    } else {
        header_logo_img.clone()
    };

    xml = xml.replace("{{MAIN_NAV_LINKS}}", &render_main_nav_links(config));
    xml = xml.replace("{{HEADER_LOGO_IMG}}", &header_logo_img);
    xml = xml.replace("{{HEADER_BRANDING}}", &header_branding);
    // These were referenced by gtk_headerbar.xml but never substituted.
    xml = xml.replace("{{SITE_TITLE}}", &escape_html(&config.site.site_title));
    xml = xml.replace("{{SITE_TITLE_ATTR}}", &escape_attr(&config.site.site_title));
    xml = xml.replace("{{SITE_HOME_URL_ATTR}}", &escape_attr(site_home_url));

    // FIX: Safely evaluate if the icon is a full SVG string or just a path data string,
    // and inject it directly into the DOM tree instead of hiding it in a style attribute!
    let left_icon = if config.icons.sidebar_left.trim().starts_with("<svg") {
        config.icons.sidebar_left.clone()
    } else {
        format!("<svg fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" viewBox=\"0 0 24 24\" height=\"1.2em\" width=\"1.2em\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"{}\"/></svg>", escape_attr(&config.icons.sidebar_left))
    };

    let right_icon = if config.icons.sidebar_right.trim().starts_with("<svg") {
        config.icons.sidebar_right.clone()
    } else {
        format!("<svg fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" viewBox=\"0 0 24 24\" height=\"1.2em\" width=\"1.2em\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"{}\"/></svg>", escape_attr(&config.icons.sidebar_right))
    };

    let left_panel_html = format!(
        "<span data-edit-target=\"icons.sidebar_left\" style=\"display: inline-flex; align-items: center; gap: 6px; cursor: pointer;\">{}<span class=\"visually-hidden\">Browse</span></span>",
        left_icon
    );

    let right_panel_html = format!(
        "<span data-edit-target=\"icons.sidebar_right\" style=\"display: inline-flex; align-items: center; gap: 6px; cursor: pointer;\"><span class=\"visually-hidden\">Contents</span>{}</span>",
        right_icon
    );

    xml = xml.replace("{{LEFT_PANEL_OPEN_LABEL}}", &left_panel_html);
    xml = xml.replace("{{RIGHT_PANEL_OPEN_LABEL}}", &right_panel_html);

    let catalog_label = escape_html("Catalog");
    xml = xml.replace("{{CATALOG_OPEN_LABEL}}", &catalog_label);

    xml = xml.replace(
        "{{NAV_HOME_LABEL}}",
        &escape_html(first_non_empty(&menu_1.label, "Home")),
    );
    xml = xml.replace(
        "{{NAV_HOME_URL}}",
        &escape_attr(first_non_empty(&menu_1.url, site_home_url)),
    );
    xml = xml.replace(
        "{{NAV_ABOUT_LABEL}}",
        &escape_html(first_non_empty(&menu_2.label, "About")),
    );
    xml = xml.replace(
        "{{NAV_ABOUT_URL}}",
        &escape_attr(first_non_empty(&menu_2.url, "#")),
    );
    xml = xml.replace(
        "{{NAV_PROJECTS_LABEL}}",
        &escape_html(first_non_empty(&menu_3.label, "Projects")),
    );
    xml = xml.replace(
        "{{NAV_PROJECTS_URL}}",
        &escape_attr(first_non_empty(&menu_3.url, "#")),
    );
    xml = xml.replace(
        "{{NAV_CONTACT_LABEL}}",
        &escape_html(first_non_empty(&menu_4.label, "Contact")),
    );
    xml = xml.replace(
        "{{NAV_CONTACT_URL}}",
        &escape_attr(first_non_empty(&menu_4.url, "#")),
    );
    const DEFAULTS: &[(&str, &str)] = &[
        ("{{NAV_MERCH_LABEL}}", "Shop"),
        ("{{NAV_MERCH_URL}}", "#"),
        ("{{NAV_POSTS_LABEL}}", "Posts"),
        ("{{NAV_POSTS_URL}}", "/search"),
        ("{{NAV_CATEGORIES_LABEL}}", "Categories"),
        ("{{NAV_CATEGORIES_URL}}", "#"),
        ("{{NAV_ARCHIVE_LABEL}}", "Archive"),
        ("{{NAV_ARCHIVE_URL}}", "#"),
        ("{{SEARCH_ACTION_URL}}", "/search"),
        ("{{SEARCH_ACTION_URL_ATTR}}", "/search"),
        ("{{SEARCH_PROMPT}}", "Search"),
        ("{{SEARCH_PLACEHOLDER}}", "Search..."),
        ("{{SEARCH_PLACEHOLDER_ATTR}}", "Search..."),
        ("{{SEARCH_BUTTON_LABEL}}", "Search"),
        ("{{CATALOG_TRIGGER_LABEL}}", "Catalog"),
        ("{{CATALOG_ALL_LABEL}}", "See All"),
        ("{{CATALOG_ALL_URL}}", "#"),
        ("{{CATALOG_SUBJECTS_LABEL}}", "Categories"),
        ("{{CATALOG_SUBJECTS_URL}}", "#"),
        ("{{CATALOG_LEXICON_LABEL}}", "Glossary"),
        ("{{CATALOG_LEXICON_URL}}", "#"),
        ("{{CATALOG_MEDIA_LABEL}}", "Media"),
        ("{{CATALOG_MEDIA_URL}}", "#"),
        ("{{CATALOG_WIKI_LABEL}}", "Wiki"),
        ("{{CATALOG_WIKI_URL}}", "#"),
        ("{{CATALOG_PROJECTS_LABEL}}", "Projects"),
        ("{{CATALOG_PROJECTS_URL}}", "#"),
        ("{{CATALOG_PROGRESS_LABEL}}", "Status"),
        ("{{CATALOG_PROGRESS_URL}}", "#"),
        ("{{SUBJECT_LIST}}", ""),
        ("{{LEXICON_LIST}}", ""),
        ("{{MEDIA_LIST}}", ""),
        ("{{WIKI_LIST}}", ""),
        ("{{PROJECTS_LIST}}", ""),
        ("{{PROGRESS_LIST}}", ""),
        ("{{SUBJECT_000_LABEL}}", "000 General"),
        ("{{SUBJECT_000_URL}}", "#"),
        ("{{SUBJECT_100_LABEL}}", "100 Philosophy"),
        ("{{SUBJECT_100_URL}}", "#"),
        ("{{SUBJECT_200_LABEL}}", "200 Religion"),
        ("{{SUBJECT_200_URL}}", "#"),
        ("{{SUBJECT_300_LABEL}}", "300 Social"),
        ("{{SUBJECT_300_URL}}", "#"),
        ("{{SUBJECT_400_LABEL}}", "400 Language"),
        ("{{SUBJECT_400_URL}}", "#"),
        ("{{SUBJECT_500_LABEL}}", "500 Science"),
        ("{{SUBJECT_500_URL}}", "#"),
        ("{{SUBJECT_600_LABEL}}", "600 Technology"),
        ("{{SUBJECT_600_URL}}", "#"),
        ("{{SUBJECT_700_LABEL}}", "700 Arts"),
        ("{{SUBJECT_700_URL}}", "#"),
        ("{{SUBJECT_800_LABEL}}", "800 Literature"),
        ("{{SUBJECT_800_URL}}", "#"),
        ("{{SUBJECT_900_LABEL}}", "900 History"),
        ("{{SUBJECT_900_URL}}", "#"),
        ("{{LEXICON_MORDICTIONARY_LABEL}}", "Dictionary"),
        ("{{LEXICON_MORDICTIONARY_URL}}", "#"),
        ("{{LEXICON_WEAR_YOUR_DICTIONARY_LABEL}}", "Apparel"),
        ("{{LEXICON_WEAR_YOUR_DICTIONARY_URL}}", "#"),
        ("{{LEXICON_VOCABULARY_LABEL}}", "Vocabulary"),
        ("{{LEXICON_VOCABULARY_URL}}", "#"),
        ("{{LEXICON_ETYMOLOGY_LABEL}}", "Etymology"),
        ("{{LEXICON_ETYMOLOGY_URL}}", "#"),
        ("{{LEXICON_WORDPLAY_LABEL}}", "Wordplay"),
        ("{{LEXICON_WORDPLAY_URL}}", "#"),
        ("{{LEXICON_LANGUAGE_LABEL}}", "Language"),
        ("{{LEXICON_LANGUAGE_URL}}", "#"),
        ("{{MEDIA_AUDIOBOOK_GAMING_LABEL}}", "Audiobooks"),
        ("{{MEDIA_AUDIOBOOK_GAMING_URL}}", "#"),
        ("{{MEDIA_WATCHLISTS_LABEL}}", "Watchlists"),
        ("{{MEDIA_WATCHLISTS_URL}}", "#"),
        ("{{MEDIA_READING_LABEL}}", "Reading"),
        ("{{MEDIA_READING_URL}}", "#"),
        ("{{MEDIA_LISTENING_LABEL}}", "Listening"),
        ("{{MEDIA_LISTENING_URL}}", "#"),
        ("{{MEDIA_SOCIAL_SCIENCE_LABEL}}", "Social Science"),
        ("{{MEDIA_SOCIAL_SCIENCE_URL}}", "#"),
        ("{{MEDIA_DIET_LABEL}}", "Media Diet"),
        ("{{MEDIA_DIET_URL}}", "#"),
        ("{{WIKI_START_LABEL}}", "Start Here"),
        ("{{WIKI_START_URL}}", "#"),
        ("{{WIKI_ALL_POSTS_LABEL}}", "All Posts"),
        ("{{WIKI_ALL_POSTS_URL}}", "#"),
        ("{{WIKI_TRAILS_LABEL}}", "Trails"),
        ("{{WIKI_TRAILS_URL}}", "#"),
        ("{{WIKI_WALKING_LABEL}}", "Walking"),
        ("{{WIKI_WALKING_URL}}", "#"),
        ("{{WIKI_VIDEO_COMMENTARY_LABEL}}", "Video"),
        ("{{WIKI_VIDEO_COMMENTARY_URL}}", "#"),
        ("{{WIKI_LEXICOGRAPHY_LABEL}}", "Lexicography"),
        ("{{WIKI_LEXICOGRAPHY_URL}}", "#"),
        ("{{WIKI_BLOG_LABEL}}", "Wiki Blog"),
        ("{{WIKI_BLOG_URL}}", "#"),
        ("{{WIKI_OFFICIAL_LABEL}}", "Official Wiki"),
        ("{{WIKI_OFFICIAL_URL}}", "#"),
        ("{{WIKI_YOUTUBE_LABEL}}", "YouTube"),
        ("{{WIKI_YOUTUBE_URL}}", "#"),
        ("{{WIKI_REDDIT_LABEL}}", "Community"),
        ("{{WIKI_REDDIT_URL}}", "#"),
        ("{{PROJECTS_ALL_LABEL}}", "All Projects"),
        ("{{PROJECTS_ALL_URL}}", "#"),
        ("{{PROJECTS_INSTITUTE_LABEL}}", "Main Project"),
        ("{{PROJECTS_INSTITUTE_URL}}", "#"),
        ("{{PROJECTS_WEAR_YOUR_WORDS_LABEL}}", "Shop"),
        ("{{PROJECTS_WEAR_YOUR_WORDS_URL}}", "#"),
        ("{{PROJECTS_MORBLOCKS_LABEL}}", "Blocks"),
        ("{{PROJECTS_MORBLOCKS_URL}}", "#"),
        ("{{PROJECTS_MORLESSONBUILDER_LABEL}}", "Builder"),
        ("{{PROJECTS_MORLESSONBUILDER_URL}}", "#"),
        ("{{PROJECTS_LOG_LABEL}}", "Logs"),
        ("{{PROJECTS_LOG_URL}}", "#"),
        ("{{PROGRESS_DASHBOARD_LABEL}}", "Dashboard"),
        ("{{PROGRESS_DASHBOARD_URL}}", "#"),
        ("{{PROGRESS_OFFLINE_TRACKER_LABEL}}", "Tracker"),
        ("{{PROGRESS_OFFLINE_TRACKER_URL}}", "#"),
        ("{{PROGRESS_CATALOG_LABEL}}", "Catalog"),
        ("{{PROGRESS_CATALOG_URL}}", "#"),
        ("{{PROGRESS_LESSONS_LABEL}}", "Lessons"),
        ("{{PROGRESS_LESSONS_URL}}", "#"),
        ("{{PROGRESS_ACTIVITIES_LABEL}}", "Activities"),
        ("{{PROGRESS_ACTIVITIES_URL}}", "#"),
    ];
    for (key, value) in DEFAULTS {
        xml = xml.replace(key, value);
    }

    xml
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ThemeConfig;
    use crate::render::template_resolver::HEADER_REGISTRY;

    #[test]
    fn centered_search_header_renders_cleanly() {
        let cfg = ThemeConfig::default();
        let raw = HEADER_REGISTRY
            .iter()
            .find(|c| c.id == "mor_search_center")
            .expect("centered-search header registered")
            .xml_content;
        let out = render_header_sockets(raw.to_string(), &cfg);

        // Modifier class + search markup present.
        assert!(out.contains("main-header search-centered"));
        assert!(out.contains("class=\"mor-search\""));
        // Branding placeholder substituted (title fallback when no logo).
        assert!(out.contains("institute-title"));
        // Previously-unsubstituted placeholders are now filled.
        assert!(!out.contains("{{HEADER_BRANDING}}"));
        assert!(!out.contains("{{SITE_HOME_URL_ATTR}}"));
        assert!(!out.contains("{{SEARCH_PLACEHOLDER_ATTR}}"));
        // No stray template tokens left — except plugin-widget sockets, which are
        // resolved downstream in xml_generator after the full template is assembled.
        let residual = out.replace("{{PLUGIN_WIDGET_HEADER}}", "");
        assert!(!residual.contains("{{"), "unsubstituted placeholder remains: {residual}");
    }

    #[test]
    fn centered_search_css_bundled_when_variant_selected() {
        use std::collections::HashMap;
        let mut cfg = ThemeConfig::default();
        cfg.template_pack.header_variant = "mor_search_center".to_string();
        let vfs = HashMap::new();
        let parts = crate::render::template_resolver::resolve_template_parts(&cfg, &vfs);
        // Both preview and export run css through render_css_sockets(parts.css).
        let css = crate::render::xml_parts::css_generator::render_css_sockets(parts.css, &cfg);
        assert!(
            css.contains(".search-centered"),
            "centered-search CSS must be bundled when the variant is active"
        );
        assert!(parts.header.contains("search-centered"));
    }
}

fn render_main_nav_links(config: &ThemeConfig) -> String {
    config
        .menu_links
        .iter()
        .enumerate()
        .filter_map(|(index, link)| render_menu_link(index, link))
        .collect::<Vec<_>>()
        .join("\n      ")
}

fn render_menu_link(index: usize, link: &MenuLink) -> Option<String> {
    let label = link.label.trim();
    let url = link.url.trim();

    if label.is_empty() || url.is_empty() {
        return None;
    }

    Some(menu_link_anchor(index, url, label))
}
