use crate::config::ThemeConfig;
use crate::render::ads::render_ads_widget_sidebar;

pub fn render_sidebar_sockets(mut xml: String, config: &ThemeConfig, side: &str) -> String {
    if side == "LEFT" {
        xml = xml.replace("{{LEFT_PANEL_TITLE}}", "Browse");
        xml = xml.replace("{{LEFT_PANEL_CLOSE_LABEL}}", "Close");
    } else {
        xml = xml.replace("{{RIGHT_PANEL_TITLE}}", "Contents");
        xml = xml.replace("{{RIGHT_PANEL_CLOSE_LABEL}}", "Close");
    }

    xml = xml.replace("{{WIDGET_ADSENSE_SIDEBAR}}", &render_ads_widget_sidebar(&config.ads));

    xml = xml.replace("{{LABEL_WIDGET_TITLE}}", "Labels");
    xml = xml.replace("{{LABEL_SORTING}}", "ALPHA");
    xml = xml.replace("{{LABEL_DISPLAY}}", "LIST");
    xml = xml.replace("{{LABEL_SHOW_TYPE}}", "ALL");
    xml = xml.replace("{{LABEL_SHOW_FREQ_NUMBERS}}", "false");

    xml = xml.replace("{{ARCHIVE_WIDGET_TITLE}}", "Archive");
    xml = xml.replace("{{ARCHIVE_SHOW_STYLE}}", "HIERARCHY");
    xml = xml.replace("{{ARCHIVE_YEAR_PATTERN}}", "yyyy");
    xml = xml.replace("{{ARCHIVE_SHOW_WEEK_END}}", "true");
    xml = xml.replace("{{ARCHIVE_MONTH_PATTERN}}", "MMMM");
    xml = xml.replace("{{ARCHIVE_DAY_PATTERN}}", "MMM dd");
    xml = xml.replace("{{ARCHIVE_WEEK_PATTERN}}", "MM/dd");
    xml = xml.replace("{{ARCHIVE_CHRONOLOGICAL}}", "false");
    xml = xml.replace("{{ARCHIVE_SHOW_POSTS}}", "true");
    xml = xml.replace("{{ARCHIVE_FREQUENCY}}", "MONTHLY");

    xml = xml.replace("{{RIGHT_WIDGET_TITLE}}", "Table of Contents");
    xml = xml.replace("{{TOC_LOADING_MESSAGE}}", "Building contents...");
    xml = xml.replace("{{TOC_WAITING_MESSAGE}}", "Waiting for document...");
    xml = xml.replace("{{TOC_EMPTY_MESSAGE}}", "No anchor points found in document.");
    xml = xml.replace("{{TOC_HEADING_SELECTOR}}", "h2, h3, h4, h5");
    xml = xml.replace("{{TOC_INDENT_STEP}}", "15");
    xml = xml.replace("{{TOC_PRIMARY_MARKER}}", ">");
    xml = xml.replace("{{TOC_CHILD_MARKER}}", "-");
    xml = xml.replace("{{TOC_ITEM_MARGIN_BOTTOM}}", "8px");
    xml = xml.replace("{{TOC_ITEM_FONT_SIZE}}", "0.85rem");

    xml
}