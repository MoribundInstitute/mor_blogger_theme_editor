use crate::config::{
    AssetConfig, BackgroundConfig, BackgroundMode, ButtonConfig, ColorConfig, FooterConfig,
    MenuLink, PluginConfig, SeoConfig, SiteConfig, SurfaceFill, ThemeConfig, TypographyConfig,
};

pub fn default_theme_config() -> ThemeConfig {
    ThemeConfig {
        site: SiteConfig {
            site_title: "Modern Editorial".to_string(),
            site_subtitle: "A clean Blogger starter for posts, notes, and pages.".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        },
        colors: ColorConfig {
            bg_base: "#18161f".to_string(),
            bg_panel: SurfaceFill::solid("#211d2b"),
            bg_elevated: SurfaceFill::solid("#2a2435"),
            fg_base: "#f2eadf".to_string(),
            fg_muted: "#b9aebf".to_string(),
            accent: "#bc8d6b".to_string(),
            border: "#4a3f59".to_string(),
        },
        buttons: ButtonConfig {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        },
        typography: TypographyConfig {
            body_font_stack: "Georgia, 'Times New Roman', Times, serif".to_string(),
            heading_font_stack: "system-ui, -apple-system, 'Segoe UI', sans-serif".to_string(),
            mono_font_stack: "'Courier New', Courier, monospace".to_string(),
            base_size: "17px".to_string(),
            scale_ratio: "1.25".to_string(),
            line_height: "1.65".to_string(),
            heading_weight: "700".to_string(),
        },
        assets: AssetConfig {
            favicon_url: String::new(),
            social_card_image_url: String::new(),
        },
        background: BackgroundConfig {
            mode: BackgroundMode::Solid {
                color: "#18161f".to_string(),
            },
        },
        seo: SeoConfig {
            meta_description: "A modern Blogger theme generated with Blogger Theme Architect."
                .to_string(),
            meta_keywords: "blog, writing, theme, editorial".to_string(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        },
        menu_links: vec![
            MenuLink { label: "Home".to_string(), url: "/".to_string() },
            MenuLink { label: "Archive".to_string(), url: "/p/archive.html".to_string() },
            MenuLink { label: "About".to_string(), url: "/p/about.html".to_string() },
            MenuLink { label: "Contact".to_string(), url: "/p/contact.html".to_string() },
        ],
        footer: FooterConfig {
            footer_text: "Published with Blogger.".to_string(),
            footer_license_label: "License".to_string(),
            footer_license_url: String::new(),
        },
        plugins: PluginConfig {
            custom_js: String::new(),
        },
        preset_css: String::new(),
    }
}