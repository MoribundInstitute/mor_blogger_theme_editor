use serde::{Deserialize, Serialize};

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
}

// These defaults point exactly to your newly moved core layout files
impl Default for TemplatePackConfig {
    fn default() -> Self {
        Self {
            header_variant: "mor".to_string(),
            main_variant: "sidebars".to_string(),
            content_variant: "blog_standard".to_string(),
            left_sidebar_variant: "blogger_left".to_string(),
            right_sidebar_variant: "toc_right".to_string(),
            footer_variant: "mor".to_string(),
            script_variant: "mor_panels".to_string(),
            icon_pack: "default".to_string(),
        }
    }
}