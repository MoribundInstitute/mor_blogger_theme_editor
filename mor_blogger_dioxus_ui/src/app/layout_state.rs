use dioxus::prelude::*;

use crate::ui::workspace::layout::{PreviewTemplateMode, PreviewViewport};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterView {
    Preview,
    CodeEditor,
    Split,
    Export,
    ModuleWorkbench,
    StaticPageEditor,
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

impl AppLayoutState {
    pub fn request_exclusive_dock(&mut self, target_id: &str, requested_pos: DockPosition) {
        if requested_pos == DockPosition::Left || requested_pos == DockPosition::Right {
            if target_id != "theme" && target_id != "theme_palette" && *self.theme_palette_pos.read() == requested_pos {
                self.theme_palette_pos.set(DockPosition::Floating);
            }
            if target_id != "site" && target_id != "site_data" && *self.site_data_pos.read() == requested_pos {
                self.site_data_pos.set(DockPosition::Floating);
            }
            if target_id != "css" && target_id != "css_editor" && *self.css_editor_pos.read() == requested_pos {
                self.css_editor_pos.set(DockPosition::Floating);
            }
            if target_id != "diagnostics" && *self.diagnostics_pos.read() == requested_pos {
                self.diagnostics_pos.set(DockPosition::Floating);
            }
            if target_id != "plugin_manager" && *self.plugin_manager_pos.read() == requested_pos {
                self.plugin_manager_pos.set(DockPosition::Floating);
            }
            if target_id != "presets" && *self.presets_pos.read() == requested_pos {
                self.presets_pos.set(DockPosition::Floating);
            }
        }

        match target_id {
            "theme" | "theme_palette" => self.theme_palette_pos.set(requested_pos),
            "site" | "site_data" => self.site_data_pos.set(requested_pos),
            "css" | "css_editor" => self.css_editor_pos.set(requested_pos),
            "diagnostics" => self.diagnostics_pos.set(requested_pos),
            "plugin_manager" => self.plugin_manager_pos.set(requested_pos),
            "presets" => self.presets_pos.set(requested_pos),
            _ => {}
        }
    }
}