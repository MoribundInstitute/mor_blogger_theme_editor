use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
                ("sidebar-left".to_string(), vec!["Label1".to_string(), "BlogArchive1".to_string()]),
                ("sidebar-right".to_string(), vec!["HTML1".to_string()]),
            ]),
        }
    }
}

impl TemplatePackConfig {
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