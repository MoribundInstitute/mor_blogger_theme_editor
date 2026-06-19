use crate::config::ThemeConfig;
use crate::render::util::{escape_attr, escape_html};

pub fn render_footer_sockets(mut xml: String, config: &ThemeConfig) -> String {
    xml = xml.replace(
        "{{FOOTER_SYS_MESSAGE}}",
        &escape_html(&config.footer.footer_text),
    );
    xml = xml.replace(
        "{{FOOTER_LICENSE_URL}}",
        &escape_attr(&config.footer.footer_license_url),
    );
    xml = xml.replace(
        "{{FOOTER_LICENSE_LABEL}}",
        &escape_html(&config.footer.footer_license_label),
    );
    xml = xml.replace("{{BACK_TO_TOP_LABEL}}", "Back to Top");

    for (col_idx, col) in config.footer.columns.iter().enumerate() {
        let c = col_idx + 1;
        xml = xml.replace(
            &format!("{{{{FOOTER_COL_{}_HEADING}}}}", c),
            &escape_html(&col.heading),
        );

        for (link_idx, link) in col.links.iter().enumerate() {
            let l = link_idx + 1;
            xml = xml.replace(
                &format!("{{{{FOOTER_{}_LINK_{}_LABEL}}}}", c, l),
                &escape_html(&link.label),
            );
            xml = xml.replace(
                &format!("{{{{FOOTER_{}_LINK_{}_URL}}}}", c, l),
                &escape_attr(&link.url),
            );
        }
    }

    for (leg_idx, link) in config.footer.legal_links.iter().enumerate() {
        let l = leg_idx + 1;
        xml = xml.replace(
            &format!("{{{{FOOTER_LEGAL_{}_LABEL}}}}", l),
            &escape_html(&link.label),
        );
        xml = xml.replace(
            &format!("{{{{FOOTER_LEGAL_{}_URL}}}}", l),
            &escape_attr(&link.url),
        );
    }

    // Scrub untouched placeholders
    for c in 1..=6 {
        xml = xml.replace(&format!("{{{{FOOTER_COL_{}_HEADING}}}}", c), "");
        for l in 1..=5 {
            xml = xml.replace(&format!("{{{{FOOTER_{}_LINK_{}_LABEL}}}}", c, l), "");
            xml = xml.replace(&format!("{{{{FOOTER_{}_LINK_{}_URL}}}}", c, l), "#");
        }
    }

    for l in 1..=4 {
        xml = xml.replace(&format!("{{{{FOOTER_LEGAL_{}_LABEL}}}}", l), "");
        xml = xml.replace(&format!("{{{{FOOTER_LEGAL_{}_URL}}}}", l), "#");
    }

    xml
}
