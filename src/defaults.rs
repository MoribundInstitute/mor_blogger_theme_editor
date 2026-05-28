use crate::config::{
    AssetConfig, ButtonConfig, ColorConfig, FooterConfig, PluginConfig, SeoConfig, SiteConfig,
    TemplatePackConfig, ThemeConfig, TypographyConfig,
    SurfaceFill, BackgroundConfig, // TODO: remove any that don't live in crate::config
};
use crate::config::pages::StaticPagesConfig;
use crate::config::ads::AdsConfig;

pub fn default_theme_config() -> ThemeConfig {
    ThemeConfig {
        site: SiteConfig {
            site_title: "Moribund Institute".to_string(),
            site_subtitle: "A minimal approach to lexicography.".to_string(),
            header_logo_url: "".to_string(),
            home_url: "/".to_string(),
        },
        colors: ColorConfig {
            bg_base:     "#1e2229".to_string(),
            bg_panel:    SurfaceFill::default(),
            bg_elevated: SurfaceFill::default(),
            fg_base:     "#d9dce3".to_string(),
            fg_muted:    "#495265".to_string(),
            accent:      "#5271ad".to_string(),
            border:      "#3b4252".to_string(),
        },
        typography: TypographyConfig {
            body_font_stack:    "monospace".to_string(),
            heading_font_stack: "sans-serif".to_string(),
            mono_font_stack:    "monospace".to_string(),
            base_size:          "14px".to_string(),
            scale_ratio:        "1.25".to_string(),
            line_height:        "1.6".to_string(),
            heading_weight:     "600".to_string(),
        },
        buttons: ButtonConfig {
            radius:         "4px".to_string(),
            border_width:   "1px".to_string(),
            text_transform: "none".to_string(),
        },
        background:   BackgroundConfig::default(),
        assets:       AssetConfig::default(),
        seo:          SeoConfig::default(),
        menu_links:   vec![],
        footer:       FooterConfig::default(),
        plugins:      PluginConfig::default(),
        static_pages: StaticPagesConfig::default(),
        ads:          AdsConfig::default(),

        template_pack: TemplatePackConfig {
            header_variant:        "mor".to_string(),
            main_variant:          "sidebars".to_string(),
            content_variant:       "blog_standard".to_string(),
            left_sidebar_variant:  "blogger_left".to_string(),
            right_sidebar_variant: "toc_right".to_string(),
            footer_variant:        "mor".to_string(),
            script_variant:        "mor_panels".to_string(),
            icon_pack:             "default".to_string(),
        },

        preset_css: "".to_string(),
        icons: crate::config::styling::IconConfig { // Adjust path if your IconConfig is elsewhere
            sidebar_left: "url(\"data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20stroke='currentColor'%20stroke-width='1.5'%20stroke-linecap='round'%20stroke-linejoin='round'%3E%3Crect%20x='1.5'%20y='2.5'%20width='13'%20height='11'%20rx='2'%20/%3E%3Cpath%20d='M5.5%202.5v11'%20/%3E%3C/svg%3E\")".to_string(),
            sidebar_right: "url(\"data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20stroke='currentColor'%20stroke-width='1.5'%20stroke-linecap='round'%20stroke-linejoin='round'%3E%3Crect%20x='1.5'%20y='2.5'%20width='13'%20height='11'%20rx='2'%20/%3E%3Cpath%20d='M10.5%202.5v11'%20/%3E%3C/svg%3E\")".to_string(),
            panel_close: "url(\"data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20stroke='currentColor'%20stroke-width='1.5'%20stroke-linecap='round'%20stroke-linejoin='round'%3E%3Cpath%20d='M4.5%204.5l7%207M11.5%204.5l-7%207'%20/%3E%3C/svg%3E\")".to_string(),
            search: "url(\"data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20stroke='currentColor'%20stroke-width='1.5'%20stroke-linecap='round'%20stroke-linejoin='round'%3E%3Ccircle%20cx='7.5'%20cy='7.5'%20r='5'%20/%3E%3Cpath%20d='M11%2011l3.5%203.5'%20/%3E%3C/svg%3E\")".to_string(),
            menu: "url(\"data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20stroke='currentColor'%20stroke-width='1.5'%20stroke-linecap='round'%20stroke-linejoin='round'%3E%3Cpath%20d='M2.5%208h11M2.5%204h11M2.5%2012h11'%20/%3E%3C/svg%3E\")".to_string(),
        },
    }
}