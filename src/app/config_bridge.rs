use crate::config::ThemeConfig;
use crate::ui::workspace::layout::PanelLayout;

pub fn menu_label(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|link| link.label.clone())
        .unwrap_or_default()
}

pub fn menu_url(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|link| link.url.clone())
        .unwrap_or_default()
}

pub fn panel_layout_class(layout: PanelLayout) -> &'static str {
    match layout {
        PanelLayout::Split => "split",
        PanelLayout::Wide => "wide",
        PanelLayout::Floating => "floating",
        PanelLayout::Hidden => "hidden",
    }
}
