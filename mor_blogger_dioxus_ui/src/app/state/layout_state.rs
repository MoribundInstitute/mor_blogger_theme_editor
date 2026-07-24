use dioxus::prelude::*;
use std::collections::HashMap;

use crate::ui::workspace::layout::{PreviewTemplateMode, PreviewViewport};
use crate::app::config_bridge::{CompendiumManifest, PluginState};

/// Normalize a pin/icon key (a dock display name OR id) to its canonical dock id,
/// so pins keyed from the activity bar (ids) and from preview icons (names) agree.
pub fn normalize_dock_key(key: &str) -> String {
    match key {
        "Theme Palette" | "theme" => "theme_palette",
        "Site Data" | "site" => "site_data",
        "CSS Editor" | "css" => "css_editor",
        "JS Editor" | "js" => "js_editor",
        "Presets" => "presets",
        "Plugin Manager" => "plugin_manager",
        "Diagnostics" => "diagnostics",
        "CSS Builder" => "css_builder",
        "JS Builder" => "js_builder",
        "Template Modules" => "template_modules",
        "Code Nav" => "code_nav",
        "Static Pages" => "static_pages",
        other => other,
    }
    .to_string()
}

/// Every dock id with its tab/display label, in zone-priority order (workbench
/// companion docks first, so they win the default tab when a workspace opens).
pub const DOCK_REGISTRY: &[(&str, &str)] = &[
    ("template_modules", "Template Modules"),
    ("widgets", "Widgets"),
    ("code_nav", "Code Nav"),
    ("static_pages", "Static Pages"),
    ("css_editor", "CSS Editor"),
    ("js_editor", "JS Editor"),
    ("diagnostics", "Diagnostics"),
    ("plugin_manager", "Plugin Manager"),
    ("css_builder", "CSS Builder"),
    ("js_builder", "JS Builder"),
    ("theme_palette", "Theme Palette"),
    ("site_data", "Site Data"),
    ("presets", "Presets"),
];

pub fn dock_display_name(id: &str) -> &'static str {
    DOCK_REGISTRY
        .iter()
        .find(|(d, _)| *d == id)
        .map(|(_, name)| *name)
        .unwrap_or("Dock")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterView {
    Preview,
    CodeEditor,
    Split,
    Export,
    ModuleWorkbench,
    WidgetWorkbench,
    JsWorkbench,
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
    pub kind: String, // e.g., "preview", "svg", "dock", "ui_typography"
    pub target_id: String,
    /// Rich click context for `kind == "preview"` (everything under the cursor).
    pub preview: Option<PreviewContextInfo>,
}

/// What sits under a right-click inside the preview iframe. All the `closest()`
/// ancestors at once, so the menu can offer every applicable action.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PreviewContextInfo {
    /// Nearest `data-edit-target` ancestor (theme token; `icons.*` = swappable icon).
    pub edit_target: Option<String>,
    /// Nearest `data-field-path` ancestor (inline-editable text).
    pub field_path: Option<String>,
    /// Nearest `data-block-id` ancestor (widget block).
    pub block_id: Option<String>,
    /// Page region: Header / Left / Right / Main / Footer / Layout.
    pub region: String,
    /// Tag name of the clicked element, lowercase.
    pub tag: String,
    /// href of the nearest enclosing link.
    pub link: Option<String>,
    /// Computed text color at the click point (#rrggbb).
    pub color: Option<String>,
    /// Effective (nearest non-transparent) background color (#rrggbb).
    pub bg: Option<String>,
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
    pub css_builder_pos: Signal<DockPosition>,
    pub js_builder_pos: Signal<DockPosition>,
    pub template_modules_pos: Signal<DockPosition>,
    pub widgets_pos: Signal<DockPosition>,
    pub code_nav_pos: Signal<DockPosition>,
    pub static_pages_pos: Signal<DockPosition>,
    /// Shared TOML/XML toggle for the Code Editor, so the Code Nav dock knows
    /// which buffer is showing (false = TOML, true = compiled XML).
    pub code_show_xml: Signal<bool>,
    pub active_workbench_module: Signal<Option<&'static str>>,
    /// One-shot request: open the JS Editor dock on this file (JS workspace
    /// behavior cards set it; the dock consumes it and resets to None).
    pub js_editor_open_file: Signal<Option<String>>,

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
    /// Which dock's tab is focused in each shared zone ("" = first dock in
    /// registry order). Docks that leave the zone need no cleanup here: the
    /// zone falls back to its first dock when the focused id isn't present.
    pub left_dock_focus: Signal<String>,
    pub right_dock_focus: Signal<String>,
    /// X-ray inspect overlay on the preview.
    pub is_xray_active: Signal<bool>,
    /// One-shot "reveal this token" request consumed by the code editor dock.
    pub active_xray_target: Signal<Option<String>>,
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
            css_builder_pos: use_signal(|| DockPosition::Hidden),
            js_builder_pos: use_signal(|| DockPosition::Hidden),
            template_modules_pos: use_signal(|| DockPosition::Hidden),
            widgets_pos: use_signal(|| DockPosition::Hidden),
            code_nav_pos: use_signal(|| DockPosition::Hidden),
            static_pages_pos: use_signal(|| DockPosition::Hidden),
            code_show_xml: use_signal(|| false),
            active_workbench_module: use_signal(|| None),
            js_editor_open_file: use_signal(|| None),

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
                    ["theme_palette", "site_data", "css_editor", "js_editor", "presets"]
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
            left_dock_focus: use_signal(String::new),
            right_dock_focus: use_signal(String::new),
            is_xray_active: use_signal(|| false),
            active_xray_target: use_signal(|| None),
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
        // Static Pages rides along with the Static Page Editor, since that view
        // hides the Theme Palette where the page builder otherwise lives.
        self.static_pages_pos.set(match ws {
            CenterView::StaticPageEditor => DockPosition::mor_panel_left,
            _ => DockPosition::Hidden,
        });
        // The Widgets library rides along with the Widget Workbench, the way
        // Template Modules does for the Module Workbench.
        self.widgets_pos.set(match ws {
            CenterView::WidgetWorkbench => DockPosition::mor_panel_left,
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
            | CenterView::ModuleWorkbench
            | CenterView::WidgetWorkbench
            | CenterView::JsWorkbench => {
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
            "diagnostics" => self.diagnostics_pos,
            "plugin_manager" => self.plugin_manager_pos,
            "presets" => self.presets_pos,
            "css_builder" => self.css_builder_pos,
            "js_builder" => self.js_builder_pos,
            "template_modules" => self.template_modules_pos,
            "widgets" => self.widgets_pos,
            "code_nav" => self.code_nav_pos,
            "static_pages" => self.static_pages_pos,
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

    /// Where a dock opens by default: Site Data is the natural right-hand dock,
    /// the Plugin Manager is a floating dialog, everything else prefers the left.
    pub fn preferred_dock_position(&self, dock_id: &str) -> DockPosition {
        match normalize_dock_key(dock_id).as_str() {
            "site_data" => DockPosition::mor_panel_right,
            "plugin_manager" => DockPosition::Floating,
            _ => DockPosition::mor_panel_left,
        }
    }

    /// Open the dock into its preferred zone if hidden, otherwise hide it.
    pub fn toggle_dock_by_id(&mut self, dock_id: &str) {
        let id = normalize_dock_key(dock_id);
        let Some(mut sig) = self.dock_pos_signal(&id) else {
            return;
        };
        if *sig.read() == DockPosition::Hidden {
            let preferred = self.preferred_dock_position(&id);
            self.request_dock(&id, preferred);
        } else {
            sig.set(DockPosition::Hidden);
        }
    }

    /// Every dock currently at `pos`, in registry (zone-priority) order.
    pub fn docks_at(&self, pos: DockPosition) -> Vec<&'static str> {
        DOCK_REGISTRY
            .iter()
            .filter(|(id, _)| {
                self.dock_pos_signal(id)
                    .map(|s| *s.read() == pos)
                    .unwrap_or(false)
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Move a dock to a position. Zones are shared: a dock landing in an
    /// occupied zone joins it as a tab (and takes focus) instead of bouncing
    /// the occupant out.
    pub fn request_dock(&mut self, target_id: &str, requested_pos: DockPosition) {
        let id = normalize_dock_key(target_id);
        let Some(mut sig) = self.dock_pos_signal(&id) else {
            return;
        };
        sig.set(requested_pos);
        match requested_pos {
            DockPosition::mor_panel_left => {
                let mut focus = self.left_dock_focus;
                focus.set(id);
            }
            DockPosition::mor_panel_right => {
                let mut focus = self.right_dock_focus;
                focus.set(id);
            }
            _ => {}
        }
    }
}
