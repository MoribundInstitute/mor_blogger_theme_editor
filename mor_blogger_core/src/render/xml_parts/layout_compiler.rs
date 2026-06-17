//! Compiles declarative `LayoutBlock` definitions into strict Blogger XML.

use crate::config::{LayoutBlock, LayoutBlockType, ThemeConfig};
use crate::render::template_resolver::generate_widget_xml;
use crate::render::tracking::widget_title_h2_xml;
use crate::render::util::escape_attr;

const LAYOUT_BLOCKS_SOCKET: &str = "{{LAYOUT_BLOCKS}}";

/// Replace the layout-blocks socket inside an XML fragment.
pub fn render_layout_blocks(mut xml: String, config: &ThemeConfig) -> String {
    if xml.contains(LAYOUT_BLOCKS_SOCKET) {
        let compiled = compile_layout_blocks(&config.blocks);
        xml = xml.replace(LAYOUT_BLOCKS_SOCKET, &compiled);
    }
    xml
}

/// Compile all layout blocks into a sequence of `<b:section>` / `<b:widget>` nodes.
pub fn compile_layout_blocks(blocks: &[LayoutBlock]) -> String {
    let mut out = String::new();
    for block in blocks {
        compile_block(block, &mut out);
    }
    out
}

fn compile_block(block: &LayoutBlock, out: &mut String) {
    let sid = sanitize_block_id(&block.id);
    if sid.is_empty() {
        return;
    }

    let extra = block.section_class.trim();
    let outer_class = if extra.is_empty() {
        format!("mor-block mor-block-{}", block_type_slug(&block.block_type))
    } else {
        format!(
            "mor-block mor-block-{} {}",
            block_type_slug(&block.block_type),
            extra
        )
    };

    push_section_open(out, &format!("block-{sid}"), &outer_class, false);

    match block.block_type {
        LayoutBlockType::TwoColumn => compile_two_column(block, &sid, out),
        LayoutBlockType::HeroImage => compile_hero_image(block, &sid, out),
        LayoutBlockType::Collapsible => compile_collapsible(block, &sid, out),
        LayoutBlockType::WidgetRow => compile_widget_row(&block.widgets, out),
    }

    out.push_str("</b:section>\n");
}

fn compile_two_column(block: &LayoutBlock, sid: &str, out: &mut String) {
    push_section_open(
        out,
        &format!("block-{sid}-left"),
        "mor-block-col mor-block-col-left",
        true,
    );
    compile_widget_row(&block.left_widgets, out);
    out.push_str("</b:section>\n");

    push_section_open(
        out,
        &format!("block-{sid}-right"),
        "mor-block-col mor-block-col-right",
        true,
    );
    let right = if block.right_widgets.is_empty() {
        &block.widgets
    } else {
        &block.right_widgets
    };
    compile_widget_row(right, out);
    out.push_str("</b:section>\n");
}

fn compile_hero_image(block: &LayoutBlock, sid: &str, out: &mut String) {
    let alt = escape_attr(first_non_empty(&block.title, "Hero image"));
    let src = escape_attr(block.image_url.trim());
    let caption = block.content_html.trim();

    let mut body = String::from("<div class='mor-hero-image'>");
    if !src.is_empty() {
        body.push_str("<img class='mor-hero-img' src='");
        body.push_str(&src);
        body.push_str("' alt='");
        body.push_str(&alt);
        body.push_str("'/>");
    }
    if !caption.is_empty() {
        body.push_str("<div class='mor-hero-caption'>");
        body.push_str(&sanitize_cdata(caption));
        body.push_str("</div>");
    }
    body.push_str("</div>");

    let widget_id = format!("Hero{sid}");
    let title = first_non_empty(&block.title, "Hero");
    push_html_widget(out, &widget_id, title, &body);
}

fn compile_collapsible(block: &LayoutBlock, sid: &str, out: &mut String) {
    let title = escape_attr(first_non_empty(&block.title, "Details"));
    let open_attr = if block.collapsed { "" } else { " open='open'" };

    let body = format!(
        "<details class='mor-collapsible'{open_attr}><summary class='mor-collapsible-title'>{title}</summary><div class='mor-collapsible-body'>{}</div></details>",
        sanitize_cdata(block.content_html.trim()),
        open_attr = open_attr,
        title = title,
    );

    let widget_id = format!("Collapse{sid}");
    push_html_widget(out, &widget_id, first_non_empty(&block.title, "Section"), &body);
}

fn compile_widget_row(widget_ids: &[String], out: &mut String) {
    for wid in widget_ids {
        let trimmed = wid.trim();
        if trimmed.is_empty() {
            continue;
        }
        out.push_str(&generate_widget_xml(trimmed));
        out.push('\n');
    }
}

fn push_html_widget(out: &mut String, widget_id: &str, title: &str, content: &str) {
    let wid = escape_attr(widget_id);
    let title_attr = escape_attr(title);
    let cdata = sanitize_cdata(content);

    out.push_str("<b:widget data-block-id='");
    out.push_str(&wid);
    out.push_str("' id='");
    out.push_str(&wid);
    out.push_str("' locked='false' title='");
    out.push_str(&title_attr);
    out.push_str("' type='HTML' version='1' visible='true'>\n  <b:widget-settings>\n    <b:widget-setting name='content'><![CDATA[");
    out.push_str(&cdata);
    let title_h2 = widget_title_h2_xml(widget_id);
    out.push_str("]]></b:widget-setting>\n  </b:widget-settings>\n  <b:includable id='main'>\n    <b:if cond='data:title != &quot;&quot;'>\n      ");
    out.push_str(&title_h2);
    out.push_str("\n    </b:if>\n    <div class='widget-content'>\n      <data:content/>\n    </div>\n  </b:includable>\n</b:widget>\n");
}

fn push_section_open(out: &mut String, id: &str, class: &str, show_add: bool) {
    out.push_str("<b:section class='");
    out.push_str(class);
    out.push_str("' id='");
    out.push_str(&escape_attr(id));
    out.push_str("' showaddelement='");
    out.push_str(if show_add { "yes" } else { "no" });
    out.push_str("'>\n");
}

fn block_type_slug(block_type: &LayoutBlockType) -> &'static str {
    match block_type {
        LayoutBlockType::TwoColumn => "two-column",
        LayoutBlockType::HeroImage => "hero-image",
        LayoutBlockType::Collapsible => "collapsible",
        LayoutBlockType::WidgetRow => "widget-row",
    }
}

fn sanitize_block_id(raw: &str) -> String {
    let mut out = String::new();
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if c == '-' || c == '_' {
            out.push('-');
        }
    }
    out
}

fn sanitize_cdata(raw: &str) -> String {
    raw.replace("]]>", "]]&gt;")
}

fn first_non_empty<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if primary.trim().is_empty() {
        fallback
    } else {
        primary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LayoutBlockType;

    #[test]
    fn compiles_two_column_sections() {
        let blocks = vec![LayoutBlock {
            id: "intro".to_string(),
            block_type: LayoutBlockType::TwoColumn,
            left_widgets: vec!["Label1".to_string()],
            right_widgets: vec!["HTML1".to_string()],
            ..LayoutBlock::default()
        }];

        let xml = compile_layout_blocks(&blocks);
        assert!(xml.contains("<b:section class='mor-block mor-block-two-column' id='block-intro'"));
        assert!(xml.contains("id='block-intro-left'"));
        assert!(xml.contains("id='block-intro-right'"));
        assert!(xml.contains("id='Label1'"));
        assert!(xml.contains("id='HTML1'"));
        assert!(!xml.contains("style="));
        assert!(!xml.contains("<script"));
    }

    #[test]
    fn compiles_hero_without_inline_css() {
        let blocks = vec![LayoutBlock {
            id: "banner".to_string(),
            block_type: LayoutBlockType::HeroImage,
            title: "Welcome".to_string(),
            image_url: "https://example.com/hero.jpg".to_string(),
            content_html: "<p>Caption</p>".to_string(),
            ..LayoutBlock::default()
        }];

        let xml = compile_layout_blocks(&blocks);
        assert!(xml.contains("type='HTML'"));
        assert!(xml.contains("class='mor-hero-image'"));
        assert!(xml.contains("src='https://example.com/hero.jpg'"));
        assert!(!xml.contains("style="));
    }

    #[test]
    fn compiles_collapsible_with_details_element() {
        let blocks = vec![LayoutBlock {
            id: "faq".to_string(),
            block_type: LayoutBlockType::Collapsible,
            title: "FAQ".to_string(),
            content_html: "<p>Answer text</p>".to_string(),
            collapsed: true,
            ..LayoutBlock::default()
        }];

        let xml = compile_layout_blocks(&blocks);
        assert!(xml.contains("<details class='mor-collapsible'>"));
        assert!(xml.contains("<summary class='mor-collapsible-title'>FAQ</summary>"));
        assert!(!xml.contains("<script"));
    }

    #[test]
    fn socket_replacement_in_layout_template() {
        let config = ThemeConfig {
            blocks: vec![LayoutBlock {
                id: "row".to_string(),
                block_type: LayoutBlockType::WidgetRow,
                widgets: vec!["Label1".to_string()],
                ..LayoutBlock::default()
            }],
            ..ThemeConfig::default()
        };

        let raw = "<div>{{LAYOUT_BLOCKS}}</div>";
        let out = render_layout_blocks(raw.to_string(), &config);
        assert!(!out.contains("{{LAYOUT_BLOCKS}}"));
        assert!(out.contains("id='Label1'"));
    }
}