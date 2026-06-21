use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct SiteState {
    pub site_title: Signal<String>,
    pub site_subtitle: Signal<String>,
    pub header_logo_url: Signal<String>,
    pub home_url: Signal<String>,
    pub meta_description: Signal<String>,
    pub meta_keywords: Signal<String>,
    pub custom_robots: Signal<String>,
    pub license_url: Signal<String>,
    pub author_name: Signal<String>,
    pub enable_ai_bridge: Signal<bool>,
}

impl SiteState {
    pub fn new(signals: crate::app::theme_signals::ThemeSignals) -> Self {
        Self {
            site_title: signals.site_title,
            site_subtitle: signals.site_subtitle,
            header_logo_url: signals.header_logo_url,
            home_url: signals.home_url,
            meta_description: signals.meta_description,
            meta_keywords: signals.meta_keywords,
            custom_robots: signals.custom_robots,
            license_url: signals.license_url,
            author_name: signals.author_name,
            enable_ai_bridge: use_signal(|| false),
        }
    }
}
