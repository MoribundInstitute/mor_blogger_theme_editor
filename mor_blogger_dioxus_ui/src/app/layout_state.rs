use dioxus::prelude::*;

use crate::ui::workspace::layout::{PanelLayout, PreviewTemplateMode, PreviewViewport};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterView {
    Preview,
    CodeEditor,
    Split,
    Export,
    ModuleWorkbench,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Right,
    Floating,
    Hidden,
}

#[derive(Clone, Copy)]
pub struct AppLayoutState {
    pub left_layout: Signal<PanelLayout>,
    pub right_layout: Signal<PanelLayout>,
    pub active_left_tab: Signal<&'static str>,
    pub active_right_tab: Signal<&'static str>,
    pub preview_viewport: Signal<PreviewViewport>,
    pub preview_width: Signal<u32>,
    pub preview_template_mode: Signal<PreviewTemplateMode>,
    pub theme_palette_pos: Signal<DockPosition>,
    pub site_data_pos: Signal<DockPosition>,
    pub css_editor_pos: Signal<DockPosition>,
    pub diagnostics_pos: Signal<DockPosition>,
    pub plugin_manager_pos: Signal<DockPosition>,
    pub presets_pos: Signal<DockPosition>,
    // LIFTED BLOAT KILLER: Global tracking for the CSS Editor to read
    pub active_workbench_module: Signal<Option<&'static str>>,
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
        theme_palette_pos: use_signal(|| DockPosition::Left),
        site_data_pos: use_signal(|| DockPosition::Right),
        css_editor_pos: use_signal(|| DockPosition::Hidden),
        diagnostics_pos: use_signal(|| DockPosition::Hidden),
        plugin_manager_pos: use_signal(|| DockPosition::Hidden),
        presets_pos: use_signal(|| DockPosition::Hidden),
        active_workbench_module: use_signal(|| None),
    }
}