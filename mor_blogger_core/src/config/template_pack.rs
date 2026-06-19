use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Structural layout primitive compiled into Blogger `<b:section>` / `<b:widget>` trees.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutBlockType {
    /// Horizontal section split into left and right widget columns.
    TwoColumn,
    /// Full-width hero banner with image and optional caption HTML.
    HeroImage,
    /// Native `<details>` disclosure block (no JavaScript).
    Collapsible,
    /// Vertical stack of registered widgets inside a single section.
    WidgetRow,
}

impl Default for LayoutBlockType {
    fn default() -> Self {
        Self::WidgetRow
    }
}

/// Declarative layout block consumed by the XML layout compiler.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LayoutBlock {
    /// Stable identifier; used as a suffix for section and widget ids.
    pub id: String,
    pub block_type: LayoutBlockType,
    /// Widget / section title surfaced in Blogger chrome.
    pub title: String,
    /// Widget ids for WidgetRow blocks (or shared fallback list).
    pub widgets: Vec<String>,
    /// Left column widget ids (TwoColumn).
    pub left_widgets: Vec<String>,
    /// Right column widget ids (TwoColumn).
    pub right_widgets: Vec<String>,
    /// Hero image URL (HeroImage).
    pub image_url: String,
    /// HTML body / caption (HeroImage, Collapsible).
    pub content_html: String,
    /// Start collapsed (Collapsible).
    pub collapsed: bool,
    /// Optional extra section class token appended to the compiler class hook.
    pub section_class: String,
}

impl Default for LayoutBlock {
    fn default() -> Self {
        Self {
            id: "block-1".to_string(),
            block_type: LayoutBlockType::WidgetRow,
            title: String::new(),
            widgets: Vec::new(),
            left_widgets: Vec::new(),
            right_widgets: Vec::new(),
            image_url: String::new(),
            content_html: String::new(),
            collapsed: false,
            section_class: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TemplatePackConfig {
    pub header_variant: String,
    pub main_variant: String,
    pub content_variant: String,
    pub left_sidebar_variant: String,
    pub right_sidebar_variant: String,
    pub footer_variant: String,
    pub script_variant: String,
    pub icon_pack: String,

    /// Ledger tracking which widget IDs live in which socket IDs.
    pub widget_map: HashMap<String, Vec<String>>,
    /// User-editable widget display titles keyed by widget id.
    pub widget_titles: HashMap<String, String>,
}

impl Default for TemplatePackConfig {
    fn default() -> Self {
        Self {
            header_variant: "mor_header_baseline".to_string(),
            main_variant: "sidebars".to_string(),
            content_variant: "blog_standard".to_string(),
            left_sidebar_variant: "blogger_left".to_string(),
            right_sidebar_variant: "toc_right".to_string(),
            footer_variant: "mega".to_string(),
            script_variant: "mor_panels".to_string(),
            icon_pack: "default".to_string(),
            // THE FIX: Populate the standard layout so old configs don't boot empty.
            widget_map: HashMap::from([
                (
                    "sidebar-left".to_string(),
                    vec!["Label1".to_string(), "BlogArchive1".to_string()],
                ),
                ("sidebar-right".to_string(), vec!["HTML1".to_string()]),
            ]),
            widget_titles: HashMap::from([
                ("Label1".to_string(), "Labels".to_string()),
                ("BlogArchive1".to_string(), "Archive".to_string()),
                ("HTML1".to_string(), "Table of Contents".to_string()),
            ]),
        }
    }
}

impl TemplatePackConfig {
    pub fn widget_title<'a>(&'a self, widget_id: &str, fallback: &'a str) -> &'a str {
        self.widget_titles
            .get(widget_id)
            .map(|s| s.as_str())
            .unwrap_or(fallback)
    }
    /// Moves a widget from its current socket array into a new destination socket array.
    pub fn move_widget(&mut self, widget_id: &str, dest_socket: &str) {
        // 1. Hunt and destroy widget ID in all existing sockets
        for widgets in self.widget_map.values_mut() {
            widgets.retain(|id| id != widget_id);
        }

        // 2. Push widget ID into new destination socket
        self.widget_map
            .entry(dest_socket.to_string())
            .or_default()
            .push(widget_id.to_string());
    }
}
