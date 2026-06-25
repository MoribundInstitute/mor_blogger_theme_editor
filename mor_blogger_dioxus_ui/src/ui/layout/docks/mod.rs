pub mod asset_editor_dock;
pub mod css_dock;
pub mod js_dock;
pub mod site_data_dock;
pub mod theme_palette_dock;
pub mod xml_editor_dock;
pub mod diagnostics_dock;
pub mod plugin_manager_dock;
pub mod css_builder_dock;
pub mod js_builder_dock;
pub mod smart_code_dock;
pub mod template_editor_dock;

pub use asset_editor_dock::{
    resolve_theme_dependencies, resolve_workbench_dependencies, AssetEditorDock,
};
pub use css_dock::CssEditorPanel;
pub use js_dock::JsEditorPanel;
pub use site_data_dock::SiteDataDock;
pub use theme_palette_dock::ThemePaletteDock;
pub use xml_editor_dock::XmlEditorDock;
pub use diagnostics_dock::DiagnosticsDock;
pub use plugin_manager_dock::PluginManagerDock;
pub use css_builder_dock::CssBuilderDock;
pub use js_builder_dock::JsBuilderDock;
pub use template_editor_dock::TemplateModulesDock;
