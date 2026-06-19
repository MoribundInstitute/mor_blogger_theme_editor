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
            blocks: Vec::new(),
            preset_css: String::new(),
            active_preset_id: None,
            enable_image_borders: false,
            custom_border_url: None,
            svg_border_slice: default_slice(),
            image_border_width: default_image_width(),
            target_sidebars: default_true(),
            target_canvas: false,
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
