//! Widget property schemas: which `<b:widget-setting>` entries exist per
//! widget type, curated for the Layout-style property sheets (C5). The catalog
//! is data (`widget_settings.toml`, embedded), so covering a new widget type
//! means adding TOML, not Rust.

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WidgetSchema {
    /// Matches the `type` attribute on `<b:widget>` (e.g. "Blog", "HTML").
    #[serde(rename = "type")]
    pub widget_type: String,
    #[serde(default, rename = "setting")]
    pub settings: Vec<SettingSchema>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SettingSchema {
    /// The `name` attribute of the `<b:widget-setting>` entry. Dotted names
    /// like `style.textcolor` are opaque strings to Blogger — no nesting.
    pub name: String,
    #[serde(rename = "type")]
    pub kind: SettingKind,
    pub label: String,
    /// TOML value because the schema writes `default = true` and
    /// `default = "#000000"` alike; [`Self::default_text`] gives the canonical
    /// XML text form.
    #[serde(default)]
    pub default: Option<toml::Value>,
    /// `select` settings only: (XML value, display label) pairs.
    #[serde(default)]
    pub options: Vec<(String, String)>,
}

impl SettingSchema {
    /// The default as it would appear as `<b:widget-setting>` inner text.
    pub fn default_text(&self) -> String {
        match &self.default {
            Some(toml::Value::String(s)) => s.clone(),
            Some(toml::Value::Boolean(b)) => b.to_string(),
            Some(toml::Value::Integer(n)) => n.to_string(),
            Some(toml::Value::Float(f)) => f.to_string(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettingKind {
    Boolean,
    Color,
    #[serde(alias = "string")]
    Text,
    Select,
}

#[derive(Deserialize)]
struct SchemaFile {
    #[serde(default, rename = "widget")]
    widgets: Vec<WidgetSchema>,
}

/// Every widget type with a curated settings schema.
pub fn all_widget_schemas() -> &'static [WidgetSchema] {
    static CACHE: OnceLock<Vec<WidgetSchema>> = OnceLock::new();
    CACHE.get_or_init(|| {
        toml::from_str::<SchemaFile>(include_str!("widget_settings.toml"))
            .expect("embedded widget_settings.toml must parse")
            .widgets
    })
}

/// Schema for one `<b:widget type='..'>`, or None for types without one
/// (the property sheet simply doesn't render for those).
pub fn schema_for(widget_type: &str) -> Option<&'static WidgetSchema> {
    all_widget_schemas()
        .iter()
        .find(|w| w.widget_type == widget_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_parses_and_covers_core_types() {
        for t in ["Blog", "HTML", "Label"] {
            let schema = schema_for(t).unwrap_or_else(|| panic!("no schema for {t}"));
            assert!(!schema.settings.is_empty());
        }
        let blog = schema_for("Blog").unwrap();
        let date_header = blog
            .settings
            .iter()
            .find(|s| s.name == "showDateHeader")
            .unwrap();
        assert_eq!(date_header.kind, SettingKind::Boolean);
        assert_eq!(date_header.default_text(), "true");
    }

    #[test]
    fn select_settings_carry_options() {
        let label = schema_for("Label").unwrap();
        let display = label.settings.iter().find(|s| s.name == "display").unwrap();
        assert_eq!(display.kind, SettingKind::Select);
        assert!(display.options.iter().any(|(v, _)| v == "CLOUD"));
    }
}
