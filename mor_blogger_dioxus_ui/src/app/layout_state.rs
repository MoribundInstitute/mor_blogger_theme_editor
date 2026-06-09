use dioxus::prelude::*;

use crate::ui::workspace::layout::{PanelLayout, PreviewTemplateMode, PreviewViewport};

#[derive(Clone, Copy)]
pub struct AppLayoutState {
    pub left_layout: Signal<PanelLayout>,
    pub right_layout: Signal<PanelLayout>,
    pub active_left_tab: Signal<&'static str>,
    pub active_right_tab: Signal<&'static str>,
    pub preview_viewport: Signal<PreviewViewport>,
    pub preview_width: Signal<u32>,
    pub preview_template_mode: Signal<PreviewTemplateMode>,
}

pub fn use_app_layout_state() -> AppLayoutState {
    AppLayoutState {
        left_layout: use_signal(|| PanelLayout::Split),
        right_layout: use_signal(|| PanelLayout::Split),
        active_left_tab: use_signal(|| "Presets"),
        active_right_tab: use_signal(|| "Site"),
        preview_viewport: use_signal(|| PreviewViewport::Desktop),
        preview_width: use_signal(|| 1200u32),
        preview_template_mode: use_signal(|| PreviewTemplateMode::Sidebars),
    }
}
