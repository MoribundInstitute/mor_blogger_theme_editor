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

#[derive(Clone, PartialEq, Debug)]
pub struct ContextMenuPayload {
    pub x: f64,
    pub y: f64,
    pub kind: String, // e.g., "svg", "ui_typography", "preview_typography"
    pub target_id: String,
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
    pub active_workbench_module: Signal<Option<&'static str>>,

    pub center_view: Signal<CenterView>,
    pub active_static_page: Signal<Option<String>>,
    pub active_context_menu: Signal<Option<ContextMenuPayload>>,
    pub active_icon_picker: Signal<Option<String>>,
    pub show_advanced_modules: Signal<bool>,
}

impl LayoutState {
    pub fn new() -> Self {
        LayoutState {
            active_left_tab: use_signal(|| "Presets"),
            active_right_tab: use_signal(|| "Site"),
            preview_viewport: use_signal(|| PreviewViewport::Desktop),
            preview_width: use_signal(|| 1200u32),
            preview_template_mode: use_signal(|| PreviewTemplateMode::Sidebars),
            theme_palette_pos: use_signal(|| DockPosition::Left),
            site_data_pos: use_signal(|| DockPosition::Right),
            css_editor_pos: use_signal(|| DockPosition::Hidden),
            js_editor_pos: use_signal(|| DockPosition::Hidden),
            diagnostics_pos: use_signal(|| DockPosition::Hidden),
            plugin_manager_pos: use_signal(|| DockPosition::Hidden),
            presets_pos: use_signal(|| DockPosition::Hidden),
            active_workbench_module: use_signal(|| None),

            center_view: use_signal(|| CenterView::Preview),
            active_static_page: use_signal(|| None::<String>),
            active_context_menu: use_signal(|| None::<ContextMenuPayload>),
            active_icon_picker: use_signal(|| None::<String>),
            show_advanced_modules: use_signal(|| false),
        }
    }

    pub fn request_exclusive_dock(&mut self, target_id: &str, requested_pos: DockPosition) {
        if requested_pos == DockPosition::Left || requested_pos == DockPosition::Right {
            if target_id != "theme"
                && target_id != "theme_palette"
                && *self.theme_palette_pos.read() == requested_pos
            {
                self.theme_palette_pos.set(DockPosition::Floating);
            }
            if target_id != "site"
                && target_id != "site_data"
                && *self.site_data_pos.read() == requested_pos
            {
                self.site_data_pos.set(DockPosition::Floating);
            }
            if target_id != "css"
                && target_id != "css_editor"
                && *self.css_editor_pos.read() == requested_pos
            {
                self.css_editor_pos.set(DockPosition::Floating);
            }
            if target_id != "js"
                && target_id != "js_editor"
                && *self.js_editor_pos.read() == requested_pos
            {
                self.js_editor_pos.set(DockPosition::Floating);
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
            "js" | "js_editor" => self.js_editor_pos.set(requested_pos),
            "diagnostics" => self.diagnostics_pos.set(requested_pos),
            "plugin_manager" => self.plugin_manager_pos.set(requested_pos),
            "presets" => self.presets_pos.set(requested_pos),
            _ => {}
        }
    }

    pub fn toggle_editor_smartly(&mut self, target: &str) {
        // Determine which signal we are targeting
        let (is_hidden, _preferred_pos) = match target {
            "css" => (*self.css_editor_pos.read() == DockPosition::Hidden, DockPosition::Right),
            "js" => (*self.js_editor_pos.read() == DockPosition::Hidden, DockPosition::Right),
            _ => return,
        };

        // If it's already open, just hide it
        if !is_hidden {
            match target {
                "css" => self.css_editor_pos.set(DockPosition::Hidden),
                "js" => self.js_editor_pos.set(DockPosition::Hidden),
                _ => {}
            }
            return;
        }

        // It is hidden. Check if the side panes are currently occupying the left/right slots.
        let right_occupied = *self.site_data_pos.read() == DockPosition::Right 
                          || *self.theme_palette_pos.read() == DockPosition::Right;
                          
        let left_occupied = *self.theme_palette_pos.read() == DockPosition::Left 
                         || *self.site_data_pos.read() == DockPosition::Left;

        // Determine the fallback position
        let final_pos = if left_occupied && right_occupied {
            DockPosition::Floating // Both sides are pinned, float it
        } else if !right_occupied {
            DockPosition::Right    // Prefer right if open
        } else {
            DockPosition::Left     // Fallback to left if right is taken but left is open
        };

        // Apply the position
        match target {
            "css" => self.css_editor_pos.set(final_pos),
            "js" => self.js_editor_pos.set(final_pos),
            _ => {}
        }
    }
}

