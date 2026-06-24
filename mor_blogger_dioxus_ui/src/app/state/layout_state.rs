use dioxus::prelude::*;

use crate::ui::workspace::layout::{PreviewTemplateMode, PreviewViewport};
use crate::app::config_bridge::{CompendiumManifest, PluginState};

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
#[allow(non_camel_case_types)]
pub enum DockPosition {
    mor_panel_left,
    mor_panel_right,
    Floating,
    Hidden,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ContextMenuPayload {
    pub x: f64,
    pub y: f64,
    pub kind: String, // e.g., "svg", "ui_typography", "preview_typography"
    pub target_id: String,
}

#[derive(Clone, Copy, Debug)]
pub struct PluginManagerContext {
    pub launch_plugins: Signal<Vec<PluginState>>,
    pub current_plugins: Signal<Vec<PluginState>>,
    pub compendium_registry: Signal<Vec<CompendiumManifest>>,
}

#[derive(Clone, Copy)]
pub struct LayoutState {
    pub active_left_tab: Signal<&'static str>,
    pub active_right_tab: Signal<&'static str>,
    pub preview_viewport: Signal<PreviewViewport>,
    pub preview_width: Signal<u32>,
    pub preview_template_mode: Signal<PreviewTemplateMode>,
    pub theme_palette_pos: Signal<DockPosition>,
    pub site_data_pos: Signal<DockPosition>,
    pub css_editor_pos: Signal<DockPosition>,
    pub js_editor_pos: Signal<DockPosition>,
    pub diagnostics_pos: Signal<DockPosition>,
    pub plugin_manager_pos: Signal<DockPosition>,
    pub presets_pos: Signal<DockPosition>,
    pub xml_editor_pos: Signal<DockPosition>,
    pub css_builder_pos: Signal<DockPosition>,
    pub js_builder_pos: Signal<DockPosition>,
    pub active_workbench_module: Signal<Option<&'static str>>,

    pub center_view: Signal<CenterView>,
    pub active_static_page: Signal<Option<String>>,
    pub active_context_menu: Signal<Option<ContextMenuPayload>>,
    pub active_icon_picker: Signal<Option<String>>,
    pub show_advanced_modules: Signal<bool>,
    pub pinned_docks: Signal<Vec<String>>,
    pub quick_launch_hidden: Signal<Vec<String>>,
}

impl LayoutState {
    pub fn new() -> Self {
        let layout_prefs = crate::app::config_bridge::LayoutPrefs::load();
        LayoutState {
            active_left_tab: use_signal(|| "Presets"),
            active_right_tab: use_signal(|| "Site"),
            preview_viewport: use_signal(|| PreviewViewport::Desktop),
            preview_width: use_signal(|| 1200u32),
            preview_template_mode: use_signal(|| PreviewTemplateMode::Sidebars),
            theme_palette_pos: use_signal(|| DockPosition::mor_panel_left),
            site_data_pos: use_signal(|| DockPosition::mor_panel_right),
            css_editor_pos: use_signal(|| DockPosition::Hidden),
            js_editor_pos: use_signal(|| DockPosition::Hidden),
            diagnostics_pos: use_signal(|| DockPosition::Hidden),
            plugin_manager_pos: use_signal(|| DockPosition::Hidden),
            presets_pos: use_signal(|| DockPosition::Hidden),
            xml_editor_pos: use_signal(|| DockPosition::Hidden),
            css_builder_pos: use_signal(|| DockPosition::Hidden),
            js_builder_pos: use_signal(|| DockPosition::Hidden),
            active_workbench_module: use_signal(|| None),

            center_view: use_signal(|| CenterView::Preview),
            active_static_page: use_signal(|| None::<String>),
            active_context_menu: use_signal(|| None::<ContextMenuPayload>),
            active_icon_picker: use_signal(|| None::<String>),
            show_advanced_modules: use_signal(|| false),
            pinned_docks: use_signal(|| layout_prefs.pinned_docks.clone()),
            quick_launch_hidden: use_signal(|| layout_prefs.quick_launch_hidden.clone()),
        }
    }

    fn save_layout_prefs(&self) {
        let prefs = crate::app::config_bridge::LayoutPrefs {
            pinned_docks: self.pinned_docks.read().clone(),
            quick_launch_hidden: self.quick_launch_hidden.read().clone(),
        };
        let _ = prefs.save();
    }

    pub fn is_quick_launch_visible(&self, dock_id: &str) -> bool {
        !self
            .quick_launch_hidden
            .read()
            .contains(&dock_id.to_string())
    }

    pub fn set_quick_launch_visible(&self, dock_id: &str, visible: bool) {
        let mut hidden_sig = self.quick_launch_hidden;
        let mut hidden = hidden_sig.write();
        let id = dock_id.to_string();
        if visible {
            hidden.retain(|x| x != &id);
        } else if !hidden.contains(&id) {
            hidden.push(id);
        }
        self.save_layout_prefs();
    }

    pub fn toggle_pinned_dock(&self, dock_name: &str) {
        let mut pinned_docks = self.pinned_docks;
        let mut pinned = pinned_docks.write();
        let name_str = dock_name.to_string();
        if let Some(pos) = pinned.iter().position(|x| x == &name_str) {
            pinned.remove(pos);
        } else {
            pinned.push(name_str);
        }
        self.save_layout_prefs();
    }

    pub fn is_dock_pinned(&self, dock_name: &str) -> bool {
        self.pinned_docks.read().contains(&dock_name.to_string())
    }

    pub fn request_exclusive_dock(&mut self, target_id: &str, requested_pos: DockPosition) {
        let sig = match target_id {
            "theme" | "theme_palette" => &self.theme_palette_pos,
            "site" | "site_data" => &self.site_data_pos,
            "css" | "css_editor" => &self.css_editor_pos,
            "js" | "js_editor" => &self.js_editor_pos,
            "xml" | "xml_editor" => &self.xml_editor_pos,
            "diagnostics" => &self.diagnostics_pos,
            "plugin_manager" => &self.plugin_manager_pos,
            "presets" => &self.presets_pos,
            "css_builder" => &self.css_builder_pos,
            "js_builder" => &self.js_builder_pos,
            _ => return,
        };

        if requested_pos == DockPosition::mor_panel_left
            || requested_pos == DockPosition::mor_panel_right
        {
            let preferred_zone = requested_pos;
            let opposite_zone = if preferred_zone == DockPosition::mor_panel_left {
                DockPosition::mor_panel_right
            } else {
                DockPosition::mor_panel_left
            };

            let other_signals = [
                &self.theme_palette_pos,
                &self.site_data_pos,
                &self.css_editor_pos,
                &self.js_editor_pos,
                &self.xml_editor_pos,
                &self.diagnostics_pos,
                &self.plugin_manager_pos,
                &self.presets_pos,
                &self.css_builder_pos,
                &self.js_builder_pos,
            ];

            let is_occupied = |zone: DockPosition| -> bool {
                for &s in &other_signals {
                    if *s != *sig && *s.read() == zone {
                        return true;
                    }
                }
                false
            };

            let final_pos = if !is_occupied(preferred_zone) {
                preferred_zone
            } else if !is_occupied(opposite_zone) {
                opposite_zone
            } else {
                DockPosition::Floating
            };

            let mut target_sig = *sig;
            target_sig.set(final_pos);
        } else {
            let mut target_sig = *sig;
            target_sig.set(requested_pos);
        }
    }
}
