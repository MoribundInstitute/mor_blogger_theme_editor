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

    xml = xml.replace("{{MAIN_NAV_LINKS}}", &render_main_nav_links(config));
    xml = xml.replace("{{HEADER_LOGO_IMG}}", &header_logo_img);

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
    xml = xml.replace("{{NAV_MERCH_LABEL}}", "Shop");
    xml = xml.replace("{{NAV_MERCH_URL}}", "#");
    xml = xml.replace("{{NAV_POSTS_LABEL}}", "Posts");
    xml = xml.replace("{{NAV_POSTS_URL}}", "/search");
    xml = xml.replace("{{NAV_CATEGORIES_LABEL}}", "Categories");
    xml = xml.replace("{{NAV_CATEGORIES_URL}}", "#");
    xml = xml.replace("{{NAV_ARCHIVE_LABEL}}", "Archive");
    xml = xml.replace("{{NAV_ARCHIVE_URL}}", "#");

    xml = xml.replace("{{SEARCH_ACTION_URL}}", "/search");
    xml = xml.replace("{{SEARCH_ACTION_URL_ATTR}}", "/search");
    xml = xml.replace("{{SEARCH_PROMPT}}", "Search");
    xml = xml.replace("{{SEARCH_PLACEHOLDER}}", "Search...");
    xml = xml.replace("{{SEARCH_PLACEHOLDER_ATTR}}", "Search...");
    xml = xml.replace("{{SEARCH_BUTTON_LABEL}}", "Search");

    xml = xml.replace("{{CATALOG_TRIGGER_LABEL}}", "Catalog");
    xml = xml.replace("{{CATALOG_ALL_LABEL}}", "See All");
    xml = xml.replace("{{CATALOG_ALL_URL}}", "#");
    xml = xml.replace("{{CATALOG_SUBJECTS_LABEL}}", "Categories");
    xml = xml.replace("{{CATALOG_SUBJECTS_URL}}", "#");
    xml = xml.replace("{{CATALOG_LEXICON_LABEL}}", "Glossary");
    xml = xml.replace("{{CATALOG_LEXICON_URL}}", "#");
    xml = xml.replace("{{CATALOG_MEDIA_LABEL}}", "Media");
    xml = xml.replace("{{CATALOG_MEDIA_URL}}", "#");
    xml = xml.replace("{{CATALOG_WIKI_LABEL}}", "Wiki");
    xml = xml.replace("{{CATALOG_WIKI_URL}}", "#");
    xml = xml.replace("{{CATALOG_PROJECTS_LABEL}}", "Projects");
    xml = xml.replace("{{CATALOG_PROJECTS_URL}}", "#");
    xml = xml.replace("{{CATALOG_PROGRESS_LABEL}}", "Status");
    xml = xml.replace("{{CATALOG_PROGRESS_URL}}", "#");

    xml = xml.replace("{{SUBJECT_LIST}}", "");
    xml = xml.replace("{{LEXICON_LIST}}", "");
    xml = xml.replace("{{MEDIA_LIST}}", "");
    xml = xml.replace("{{WIKI_LIST}}", "");
    xml = xml.replace("{{PROJECTS_LIST}}", "");
    xml = xml.replace("{{PROGRESS_LIST}}", "");

    xml = xml.replace("{{SUBJECT_000_LABEL}}", "000 General");
    xml = xml.replace("{{SUBJECT_000_URL}}", "#");
    xml = xml.replace("{{SUBJECT_100_LABEL}}", "100 Philosophy");
    xml = xml.replace("{{SUBJECT_100_URL}}", "#");
    xml = xml.replace("{{SUBJECT_200_LABEL}}", "200 Religion");
    xml = xml.replace("{{SUBJECT_200_URL}}", "#");
    xml = xml.replace("{{SUBJECT_300_LABEL}}", "300 Social");
    xml = xml.replace("{{SUBJECT_300_URL}}", "#");
    xml = xml.replace("{{SUBJECT_400_LABEL}}", "400 Language");
    xml = xml.replace("{{SUBJECT_400_URL}}", "#");
    xml = xml.replace("{{SUBJECT_500_LABEL}}", "500 Science");
    xml = xml.replace("{{SUBJECT_500_URL}}", "#");
    xml = xml.replace("{{SUBJECT_600_LABEL}}", "600 Technology");
    xml = xml.replace("{{SUBJECT_600_URL}}", "#");
    xml = xml.replace("{{SUBJECT_700_LABEL}}", "700 Arts");
    xml = xml.replace("{{SUBJECT_700_URL}}", "#");
    xml = xml.replace("{{SUBJECT_800_LABEL}}", "800 Literature");
    xml = xml.replace("{{SUBJECT_800_URL}}", "#");
    xml = xml.replace("{{SUBJECT_900_LABEL}}", "900 History");
    xml = xml.replace("{{SUBJECT_900_URL}}", "#");

    xml = xml.replace("{{LEXICON_MORDICTIONARY_LABEL}}", "Dictionary");
    xml = xml.replace("{{LEXICON_MORDICTIONARY_URL}}", "#");
    xml = xml.replace("{{LEXICON_WEAR_YOUR_DICTIONARY_LABEL}}", "Apparel");
    xml = xml.replace("{{LEXICON_WEAR_YOUR_DICTIONARY_URL}}", "#");
    xml = xml.replace("{{LEXICON_VOCABULARY_LABEL}}", "Vocabulary");
    xml = xml.replace("{{LEXICON_VOCABULARY_URL}}", "#");
    xml = xml.replace("{{LEXICON_ETYMOLOGY_LABEL}}", "Etymology");
    xml = xml.replace("{{LEXICON_ETYMOLOGY_URL}}", "#");
    xml = xml.replace("{{LEXICON_WORDPLAY_LABEL}}", "Wordplay");
    xml = xml.replace("{{LEXICON_WORDPLAY_URL}}", "#");
    xml = xml.replace("{{LEXICON_LANGUAGE_LABEL}}", "Language");
    xml = xml.replace("{{LEXICON_LANGUAGE_URL}}", "#");

    xml = xml.replace("{{MEDIA_AUDIOBOOK_GAMING_LABEL}}", "Audiobooks");
    xml = xml.replace("{{MEDIA_AUDIOBOOK_GAMING_URL}}", "#");
    xml = xml.replace("{{MEDIA_WATCHLISTS_LABEL}}", "Watchlists");
    xml = xml.replace("{{MEDIA_WATCHLISTS_URL}}", "#");
    xml = xml.replace("{{MEDIA_READING_LABEL}}", "Reading");
    xml = xml.replace("{{MEDIA_READING_URL}}", "#");
    xml = xml.replace("{{MEDIA_LISTENING_LABEL}}", "Listening");
    xml = xml.replace("{{MEDIA_LISTENING_URL}}", "#");
    xml = xml.replace("{{MEDIA_SOCIAL_SCIENCE_LABEL}}", "Social Science");
    xml = xml.replace("{{MEDIA_SOCIAL_SCIENCE_URL}}", "#");
    xml = xml.replace("{{MEDIA_DIET_LABEL}}", "Media Diet");
    xml = xml.replace("{{MEDIA_DIET_URL}}", "#");

    xml = xml.replace("{{WIKI_START_LABEL}}", "Start Here");
    xml = xml.replace("{{WIKI_START_URL}}", "#");
    xml = xml.replace("{{WIKI_ALL_POSTS_LABEL}}", "All Posts");
    xml = xml.replace("{{WIKI_ALL_POSTS_URL}}", "#");
    xml = xml.replace("{{WIKI_TRAILS_LABEL}}", "Trails");
    xml = xml.replace("{{WIKI_TRAILS_URL}}", "#");
    xml = xml.replace("{{WIKI_WALKING_LABEL}}", "Walking");
    xml = xml.replace("{{WIKI_WALKING_URL}}", "#");
    xml = xml.replace("{{WIKI_VIDEO_COMMENTARY_LABEL}}", "Video");
    xml = xml.replace("{{WIKI_VIDEO_COMMENTARY_URL}}", "#");
    xml = xml.replace("{{WIKI_LEXICOGRAPHY_LABEL}}", "Lexicography");
    xml = xml.replace("{{WIKI_LEXICOGRAPHY_URL}}", "#");
    xml = xml.replace("{{WIKI_BLOG_LABEL}}", "Wiki Blog");
    xml = xml.replace("{{WIKI_BLOG_URL}}", "#");
    xml = xml.replace("{{WIKI_OFFICIAL_LABEL}}", "Official Wiki");
    xml = xml.replace("{{WIKI_OFFICIAL_URL}}", "#");
    xml = xml.replace("{{WIKI_YOUTUBE_LABEL}}", "YouTube");
    xml = xml.replace("{{WIKI_YOUTUBE_URL}}", "#");
    xml = xml.replace("{{WIKI_REDDIT_LABEL}}", "Community");
    xml = xml.replace("{{WIKI_REDDIT_URL}}", "#");

    xml = xml.replace("{{PROJECTS_ALL_LABEL}}", "All Projects");
    xml = xml.replace("{{PROJECTS_ALL_URL}}", "#");
    xml = xml.replace("{{PROJECTS_INSTITUTE_LABEL}}", "Main Project");
    xml = xml.replace("{{PROJECTS_INSTITUTE_URL}}", "#");
    xml = xml.replace("{{PROJECTS_WEAR_YOUR_WORDS_LABEL}}", "Shop");
    xml = xml.replace("{{PROJECTS_WEAR_YOUR_WORDS_URL}}", "#");
    xml = xml.replace("{{PROJECTS_MORBLOCKS_LABEL}}", "Blocks");
    xml = xml.replace("{{PROJECTS_MORBLOCKS_URL}}", "#");
    xml = xml.replace("{{PROJECTS_MORLESSONBUILDER_LABEL}}", "Builder");
    xml = xml.replace("{{PROJECTS_MORLESSONBUILDER_URL}}", "#");
    xml = xml.replace("{{PROJECTS_LOG_LABEL}}", "Logs");
    xml = xml.replace("{{PROJECTS_LOG_URL}}", "#");

    xml = xml.replace("{{PROGRESS_DASHBOARD_LABEL}}", "Dashboard");
    xml = xml.replace("{{PROGRESS_DASHBOARD_URL}}", "#");
    xml = xml.replace("{{PROGRESS_OFFLINE_TRACKER_LABEL}}", "Tracker");
    xml = xml.replace("{{PROGRESS_OFFLINE_TRACKER_URL}}", "#");
    xml = xml.replace("{{PROGRESS_CATALOG_LABEL}}", "Catalog");
    xml = xml.replace("{{PROGRESS_CATALOG_URL}}", "#");
    xml = xml.replace("{{PROGRESS_LESSONS_LABEL}}", "Lessons");
    xml = xml.replace("{{PROGRESS_LESSONS_URL}}", "#");
    xml = xml.replace("{{PROGRESS_ACTIVITIES_LABEL}}", "Activities");
    xml = xml.replace("{{PROGRESS_ACTIVITIES_URL}}", "#");

    xml
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
