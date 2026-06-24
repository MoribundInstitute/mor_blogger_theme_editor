pub mod ads;
pub mod defaults;
pub mod fonts;
pub mod gtk_theme;
pub mod pages;
pub mod prefs;
pub mod styling;
pub mod template_pack;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------
// Re-export these as PUBLIC (pub use) so the rest of the
// application can reach the config subtypes directly.
// ---------------------------------------------------------
pub use ads::*;
pub use defaults::*;
pub use fonts::*;
pub use gtk_theme::*;
pub use pages::*;
pub use styling::*;
pub use template_pack::*;

fn default_slice() -> String {
    "30".to_string()
}

fn default_image_width() -> String {
    "20px".to_string()
}

fn default_true() -> bool {
    true
}

fn default_cursor_style() -> String {
    "auto".to_string()
}

fn default_scrollbar_width() -> String {
    "6px".to_string()
}

fn default_scrollbar_track_color() -> String {
    "transparent".to_string()
}

fn default_scrollbar_thumb_color() -> String {
    "#4c566a".to_string()
}

fn default_scrollbar_thumb_hover_color() -> String {
    "#81a1c1".to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub site: SiteConfig,
    pub colors: ColorConfig,
    pub icons: IconConfig,
    pub assets: AssetConfig,
    pub background: BackgroundConfig,
    pub buttons: ButtonConfig,
    pub typography: TypographyConfig,
    pub seo: SeoConfig,
    pub menu_links: Vec<MenuLink>,
    pub footer: FooterConfig,
    pub plugins: PluginConfig,
    pub static_pages: StaticPagesConfig,
    pub ads: AdsConfig,
    #[serde(default = "default_cursor_style")]
    pub cursor_style: String,
    #[serde(default = "default_scrollbar_width")]
    pub scrollbar_width: String,
    #[serde(default = "default_scrollbar_track_color")]
    pub scrollbar_track_color: String,
    #[serde(default = "default_scrollbar_thumb_color")]
    pub scrollbar_thumb_color: String,
    #[serde(default = "default_scrollbar_thumb_hover_color")]
    pub scrollbar_thumb_hover_color: String,
    #[serde(default)]
    pub template_pack: TemplatePackConfig,
    /// Structural layout blocks compiled into Blogger sections/widgets.
    #[serde(default)]
    pub blocks: Vec<LayoutBlock>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub preset_css: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_preset_id: Option<String>,
    #[serde(default)]
    pub enable_image_borders: bool,
    #[serde(default)]
    pub custom_border_url: Option<String>,
    #[serde(default = "default_slice")]
    pub svg_border_slice: String,
    #[serde(default = "default_image_width")]
    pub image_border_width: String,
    #[serde(default = "default_true")]
    pub target_sidebars: bool,
    #[serde(default)]
    pub target_canvas: bool,
    #[serde(default)]
    pub scripts: ScriptBehaviorConfig,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            site: SiteConfig::default(),
            colors: ColorConfig::default(),
            icons: IconConfig::default(),
            assets: AssetConfig::default(),
            background: BackgroundConfig::default(),
            buttons: ButtonConfig::default(),
            typography: TypographyConfig::default(),
            seo: SeoConfig::default(),
            menu_links: vec![
                MenuLink {
                    label: "Home".to_string(),
                    url: "/".to_string(),
                },
                MenuLink {
                    label: "Archive".to_string(),
                    url: "/p/archive.html".to_string(),
                },
            ],
            footer: FooterConfig::default(),
            plugins: PluginConfig::default(),
            static_pages: StaticPagesConfig::default(),
            ads: AdsConfig::default(),
            cursor_style: default_cursor_style(),
            scrollbar_width: default_scrollbar_width(),
            scrollbar_track_color: default_scrollbar_track_color(),
            scrollbar_thumb_color: default_scrollbar_thumb_color(),
            scrollbar_thumb_hover_color: default_scrollbar_thumb_hover_color(),
            template_pack: TemplatePackConfig::default(),
            blocks: Vec::new(),
            preset_css: String::new(),
            active_preset_id: None,
            enable_image_borders: false,
            custom_border_url: None,
            svg_border_slice: default_slice(),
            image_border_width: default_image_width(),
            target_sidebars: default_true(),
            target_canvas: false,
            scripts: ScriptBehaviorConfig::default(),
        }
    }
}

impl ThemeConfig {
    /// Hotswaps the Site Data profile while preserving the current Visual Theme.
    pub fn apply_site_data(&mut self, new_data: &ThemeConfig) {
        self.site = new_data.site.clone(); // Site Identity
        self.assets = new_data.assets.clone(); // Images & Assets
        self.menu_links = new_data.menu_links.clone(); // Navigation Menu
        self.seo = new_data.seo.clone(); // SEO
        self.footer = new_data.footer.clone(); // Footer
        self.ads = new_data.ads.clone(); // Advertising Policy
        self.plugins = new_data.plugins.clone(); // Custom Scripts
        self.static_pages = new_data.static_pages.clone(); // Static Pages
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteConfig {
    pub site_title: String,
    pub site_subtitle: String,
    pub header_logo_url: String,
    pub home_url: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            site_title: "Modern Editorial".to_string(),
            site_subtitle: "A clean Blogger starter for posts, notes, and pages.".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SeoConfig {
    pub meta_description: String,
    pub meta_keywords: String,
    pub custom_robots: String,
    pub license_url: String,
    pub author_name: String,
}

impl Default for SeoConfig {
    fn default() -> Self {
        Self {
            meta_description: "A modern Blogger theme generated with Blogger Theme Architect."
                .to_string(),
            meta_keywords: "blog, writing, theme, editorial".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MenuLink {
    pub label: String,
    pub url: String,
}

impl Default for MenuLink {
    fn default() -> Self {
        Self {
            label: "Link".to_string(),
            url: "#".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FooterColumn {
    pub heading: String,
    pub links: Vec<MenuLink>,
}

impl Default for FooterColumn {
    fn default() -> Self {
        Self {
            heading: "Column".to_string(),
            links: vec![
                MenuLink {
                    label: "Link 1".to_string(),
                    url: "#".to_string(),
                },
                MenuLink {
                    label: "Link 2".to_string(),
                    url: "#".to_string(),
                },
                MenuLink {
                    label: "Link 3".to_string(),
                    url: "#".to_string(),
                },
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FooterConfig {
    pub footer_text: String,
    pub footer_license_label: String,
    pub footer_license_url: String,
    pub columns: Vec<FooterColumn>,
    pub legal_links: Vec<MenuLink>,
}

impl Default for FooterConfig {
    fn default() -> Self {
        Self {
            footer_text: "Published with Blogger.".to_string(),
            footer_license_label: "License".to_string(),
            footer_license_url: String::new(),
            columns: vec![
                FooterColumn {
                    heading: "Introduction".to_string(),
                    links: vec![MenuLink {
                        label: "Getting Started".to_string(),
                        url: "/".to_string(),
                    }],
                },
                FooterColumn {
                    heading: "Downloads".to_string(),
                    links: vec![MenuLink {
                        label: "Source Code".to_string(),
                        url: "/".to_string(),
                    }],
                },
                FooterColumn {
                    heading: "Documentation".to_string(),
                    links: vec![MenuLink {
                        label: "Guides".to_string(),
                        url: "/".to_string(),
                    }],
                },
                FooterColumn {
                    heading: "News".to_string(),
                    links: vec![MenuLink {
                        label: "Announcements".to_string(),
                        url: "/".to_string(),
                    }],
                },
                FooterColumn {
                    heading: "Team".to_string(),
                    links: vec![MenuLink {
                        label: "Core Team".to_string(),
                        url: "/".to_string(),
                    }],
                },
                FooterColumn {
                    heading: "Donate".to_string(),
                    links: vec![MenuLink {
                        label: "How to contribute".to_string(),
                        url: "/".to_string(),
                    }],
                },
            ],
            legal_links: vec![
                MenuLink {
                    label: "Website source code".to_string(),
                    url: "#".to_string(),
                },
                MenuLink {
                    label: "Privacy policy".to_string(),
                    url: "#".to_string(),
                },
                MenuLink {
                    label: "Terms of use".to_string(),
                    url: "#".to_string(),
                },
                MenuLink {
                    label: "Sitemap".to_string(),
                    url: "#".to_string(),
                },
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PluginConfig {
    pub custom_js: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ScriptBehaviorConfig {
    pub mobile_breakpoint: f64,        // e.g., 900.0
    pub panels_collapsed_mobile: bool, // e.g., true
    pub enable_theme_toggle: bool,     // e.g., true
    pub enable_share_actions: bool,    // e.g., true
}

impl Default for ScriptBehaviorConfig {
    fn default() -> Self {
        Self {
            mobile_breakpoint: 900.0,
            panels_collapsed_mobile: true,
            enable_theme_toggle: true,
            enable_share_actions: true,
        }
    }
}

pub fn update_toml_preserve_comments(original_toml: &str, updated_config: &ThemeConfig) -> String {
    let mut doc = match original_toml.parse::<toml_edit::DocumentMut>() {
        Ok(d) => d,
        Err(_) => {
            return toml::to_string_pretty(updated_config).unwrap_or_default();
        }
    };

    let updated_toml_str = match toml::to_string_pretty(updated_config) {
        Ok(s) => s,
        Err(_) => return original_toml.to_string(),
    };

    let updated_doc = match updated_toml_str.parse::<toml_edit::DocumentMut>() {
        Ok(d) => d,
        Err(_) => return original_toml.to_string(),
    };

    let mut orig_item = toml_edit::Item::Table(doc.as_table().clone());
    let upd_item = toml_edit::Item::Table(updated_doc.as_table().clone());
    merge_items(&mut orig_item, &upd_item);

    if let toml_edit::Item::Table(merged_tbl) = orig_item {
        *doc.as_table_mut() = merged_tbl;
    }

    doc.to_string()
}

fn merge_items(original: &mut toml_edit::Item, updated: &toml_edit::Item) {
    match (original, updated) {
        (toml_edit::Item::Table(orig_table), toml_edit::Item::Table(upd_table)) => {
            let keys_to_remove: Vec<String> = orig_table
                .iter()
                .map(|(k, _)| k.to_string())
                .filter(|k| !upd_table.contains_key(k))
                .collect();
            for key in keys_to_remove {
                orig_table.remove(&key);
            }

            for (key, upd_val) in upd_table.iter() {
                if let Some(orig_val) = orig_table.get_mut(key) {
                    merge_items(orig_val, upd_val);
                } else {
                    orig_table.insert(key, upd_val.clone());
                }
            }
        }
        (toml_edit::Item::ArrayOfTables(orig_aot), toml_edit::Item::ArrayOfTables(upd_aot)) => {
            let mut i = 0;
            while i < orig_aot.len() && i < upd_aot.len() {
                if let (Some(orig_tbl), Some(upd_tbl)) = (orig_aot.get_mut(i), upd_aot.get(i)) {
                    let mut orig_item = toml_edit::Item::Table(orig_tbl.clone());
                    let upd_item = toml_edit::Item::Table(upd_tbl.clone());
                    merge_items(&mut orig_item, &upd_item);
                    if let toml_edit::Item::Table(merged_tbl) = orig_item {
                        *orig_tbl = merged_tbl;
                    }
                }
                i += 1;
            }
            while i < upd_aot.len() {
                if let Some(upd_tbl) = upd_aot.get(i) {
                    orig_aot.push(upd_tbl.clone());
                }
                i += 1;
            }
            while orig_aot.len() > upd_aot.len() {
                orig_aot.remove(orig_aot.len() - 1);
            }
        }
        (orig_item, upd_item) => {
            if let (Some(orig_val), Some(upd_val)) = (orig_item.as_value_mut(), upd_item.as_value())
            {
                let decor = orig_val.decor().clone();
                *orig_val = upd_val.clone();
                *orig_val.decor_mut() = decor;
            } else {
                *orig_item = upd_item.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preserve_comments() {
        let original = r##"# This is a comment at the top
[site]
# The title of the site
site_title = "Original Title"
site_subtitle = "Original Subtitle"

[colors]
# Accent color
accent = "#ff0000"
"##;

        let mut config = ThemeConfig::default();
        config.site.site_title = "New Title".to_string();
        config.site.site_subtitle = "Original Subtitle".to_string();
        config.colors.accent = "#00ff00".to_string();

        let updated = update_toml_preserve_comments(original, &config);
        println!("Updated:\n{}", updated);

        assert!(updated.contains("# This is a comment at the top"));
        assert!(updated.contains("# The title of the site"));
        assert!(updated.contains("# Accent color"));
        assert!(updated.contains("site_title = \"New Title\""));
        assert!(updated.contains("accent = \"#00ff00\""));
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub snippet: String,
    pub featured_image: Option<String>,
    pub body: String,
    pub url: String,
    pub author_name: String,
}
