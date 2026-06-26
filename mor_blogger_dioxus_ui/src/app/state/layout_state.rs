use dioxus::prelude::*;
use std::collections::HashMap;

use crate::ui::workspace::layout::{PreviewTemplateMode, PreviewViewport};
use crate::app::config_bridge::{CompendiumManifest, PluginState};

/// Normalize a pin/icon key (a dock display name OR id) to its canonical dock id,
/// so pins keyed from the activity bar (ids) and from preview icons (names) agree.
pub fn normalize_dock_key(key: &str) -> String {
    match key {
        "Theme Palette" => "theme_palette",
        "Site Data" => "site_data",
        "CSS Editor" => "css_editor",
        "JS Editor" => "js_editor",
        "XML Editor" => "xml_editor",
        "Presets" => "presets",
        "Plugin Manager" => "plugin_manager",
        "Diagnostics" => "diagnostics",
        "CSS Builder" => "css_builder",
        "JS Builder" => "js_builder",
        "Template Modules" => "template_modules",
        "Code Nav" => "code_nav",
        other => other,
    }
    .to_string()
}

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
    pub template_modules_pos: Signal<DockPosition>,
    pub code_nav_pos: Signal<DockPosition>,
    /// Shared TOML/XML toggle for the Code Editor, so the Code Nav dock knows
    /// which buffer is showing (false = TOML, true = compiled XML).
    pub code_show_xml: Signal<bool>,
    pub active_workbench_module: Signal<Option<&'static str>>,

    pub center_view: Signal<CenterView>,
    pub active_static_page: Signal<Option<String>>,
    pub active_context_menu: Signal<Option<ContextMenuPayload>>,
    pub active_icon_picker: Signal<Option<String>>,
    pub show_advanced_modules: Signal<bool>,
    pub pinned_docks: Signal<Vec<String>>,
    pub quick_launch_hidden: Signal<Vec<String>>,
    /// Per-dock activity-bar icon overrides (dock id -> tagged spec string).
    pub activity_icons: Signal<HashMap<String, String>>,
    /// Dock id whose activity-bar icon is being edited (drives the picker modal).
    pub active_activity_icon_picker: Signal<Option<String>>,
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
            template_modules_pos: use_signal(|| DockPosition::Hidden),
            code_nav_pos: use_signal(|| DockPosition::Hidden),
            code_show_xml: use_signal(|| false),
            active_workbench_module: use_signal(|| None),

            center_view: use_signal(|| CenterView::Preview),
            active_static_page: use_signal(|| None::<String>),
            active_context_menu: use_signal(|| None::<ContextMenuPayload>),
            active_icon_picker: use_signal(|| None::<String>),
            show_advanced_modules: use_signal(|| false),
            pinned_docks: use_signal(|| {
                // Migrate any legacy name-based pins to canonical ids and de-dup, so
                // existing prefs actually register as pinned in the activity bar.
                let mut seen: Vec<String> = Vec::new();
                for s in &layout_prefs.pinned_docks {
                    let id = normalize_dock_key(s);
                    if !seen.contains(&id) {
                        seen.push(id);
                    }
                }
                // First run (or unpinned-to-empty): seed a sensible starter set so the
                // activity bar isn't blank. ponytail: re-seeds if you unpin everything;
                // add a "configured" flag if that ever annoys.
                if seen.is_empty() {
                    ["theme_palette", "site_data", "xml_editor", "css_editor", "js_editor", "presets"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                } else {
                    seen
                }
            }),
            quick_launch_hidden: use_signal(|| layout_prefs.quick_launch_hidden.clone()),
            activity_icons: use_signal(|| layout_prefs.activity_icons.clone()),
            active_activity_icon_picker: use_signal(|| None::<String>),
        }
    }

    fn save_layout_prefs(&self) {
        let prefs = crate::app::config_bridge::LayoutPrefs {
            pinned_docks: self.pinned_docks.read().clone(),
            quick_launch_hidden: self.quick_launch_hidden.read().clone(),
            activity_icons: self.activity_icons.read().clone(),
        };
        let _ = prefs.save();
    }

    /// Set or clear (None) a dock's activity-bar icon override, then persist.
    pub fn set_activity_icon(&self, dock_id: &str, spec: Option<String>) {
        let key = normalize_dock_key(dock_id);
        let mut icons = self.activity_icons;
        match spec {
            Some(s) => {
                icons.write().insert(key, s);
            }
            None => {
                icons.write().remove(&key);
            }
        }
        self.save_layout_prefs();
    }

    pub fn toggle_pinned_dock(&self, dock_key: &str) {
        let id = normalize_dock_key(dock_key);
        let mut pinned_docks = self.pinned_docks;
        let mut pinned = pinned_docks.write();
        if let Some(pos) = pinned.iter().position(|x| x == &id) {
            pinned.remove(pos);
        } else {
            pinned.push(id);
        }
        drop(pinned);
        self.save_layout_prefs();
    }

    pub fn is_dock_pinned(&self, dock_key: &str) -> bool {
        self.pinned_docks
            .read()
            .contains(&normalize_dock_key(dock_key))
    }

    /// Switch the center workspace and apply that workspace's default dock layout
    /// in one shot (called on the switcher click, not via a use_effect). Module
    /// Workbench opens the Template Modules dock on the left; other views hide it.
    pub fn enter_workspace(&mut self, ws: CenterView) {
        self.center_view.set(ws);
        self.template_modules_pos.set(match ws {
            CenterView::ModuleWorkbench => DockPosition::mor_panel_left,
            _ => DockPosition::Hidden,
        });
        // Code Nav rides along with the Code Editor view, like Template Modules
        // does for Module Workbench.
        self.code_nav_pos.set(match ws {
            CenterView::CodeEditor => DockPosition::mor_panel_left,
            _ => DockPosition::Hidden,
        });
        // Only Preview is about theme/content editing, so the Theme Palette and
        // Site Data docks default to visible there; every other workspace hides
        // them (Module Workbench drives its own Template Modules dock instead).
        match ws {
            CenterView::Preview | CenterView::Split => {
                self.theme_palette_pos.set(DockPosition::mor_panel_left);
                self.site_data_pos.set(DockPosition::mor_panel_right);
            }
            CenterView::CodeEditor
            | CenterView::Export
            | CenterView::StaticPageEditor
            | CenterView::ModuleWorkbench => {
                self.theme_palette_pos.set(DockPosition::Hidden);
                self.site_data_pos.set(DockPosition::Hidden);
            }
        }
    }

    /// The position signal backing a dock id (canonical or short alias), if any.
    fn dock_pos_signal(&self, dock_id: &str) -> Option<Signal<DockPosition>> {
        Some(match dock_id {
            "theme" | "theme_palette" => self.theme_palette_pos,
            "site" | "site_data" => self.site_data_pos,
            "css" | "css_editor" => self.css_editor_pos,
            "js" | "js_editor" => self.js_editor_pos,
            "xml" | "xml_editor" => self.xml_editor_pos,
            "diagnostics" => self.diagnostics_pos,
            "plugin_manager" => self.plugin_manager_pos,
            "presets" => self.presets_pos,
            "css_builder" => self.css_builder_pos,
            "js_builder" => self.js_builder_pos,
            "template_modules" => self.template_modules_pos,
            "code_nav" => self.code_nav_pos,
            _ => return None,
        })
    }

    /// Toggle the dock pinned at `index` in the activity bar (0-based, top→bottom).
    pub fn toggle_dock_by_index(&mut self, index: usize) {
        let Some(id) = self.pinned_docks.read().get(index).cloned() else {
            return;
        };
        self.toggle_dock_by_id(&id);
    }

    /// Open the dock into a free zone if hidden, otherwise hide it.
    pub fn toggle_dock_by_id(&mut self, dock_id: &str) {
        let id = normalize_dock_key(dock_id);
        let Some(mut sig) = self.dock_pos_signal(&id) else {
            return;
        };
        if *sig.read() == DockPosition::Hidden {
            // Site Data is the natural right-hand dock; everything else prefers
            // the left and falls through to right/floating if that zone is taken.
            let preferred = if id == "site_data" {
                DockPosition::mor_panel_right
            } else {
                DockPosition::mor_panel_left
            };
            self.request_exclusive_dock(&id, preferred);
        } else {
            sig.set(DockPosition::Hidden);
        }
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
            "template_modules" => &self.template_modules_pos,
            "code_nav" => &self.code_nav_pos,
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
                &self.template_modules_pos,
                &self.code_nav_pos,
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
