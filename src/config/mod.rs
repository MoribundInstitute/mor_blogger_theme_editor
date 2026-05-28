pub mod ads;
pub mod gtk_theme;
pub mod pages;
pub mod styling;
pub mod template_pack;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------
// Re-export these as PUBLIC (pub use) so the rest of the
// application can reach the config subtypes directly.
// ---------------------------------------------------------
pub use ads::*;
pub use pages::*;
pub use styling::*;
pub use template_pack::*;
pub use gtk_theme::*; // Added to expose the refactored GTK module types

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
    #[serde(default)]
    pub template_pack: TemplatePackConfig,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub preset_css: String,
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
            template_pack: TemplatePackConfig::default(),
            preset_css: String::new(),
        }
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
pub struct FooterConfig {
    pub footer_text: String,
    pub footer_license_label: String,
    pub footer_license_url: String,
}

impl Default for FooterConfig {
    fn default() -> Self {
        Self {
            footer_text: "Published with Blogger.".to_string(),
            footer_license_label: "License".to_string(),
            footer_license_url: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PluginConfig {
    pub custom_js: String,
}