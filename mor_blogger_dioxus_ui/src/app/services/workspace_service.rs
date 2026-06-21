use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::utils::svg_icons::{is_svg, svg_to_data_uri};
use std::collections::HashMap;

pub fn set_icon_slot(config: &mut ThemeConfig, slot: &str, mask: String) {
    match slot {
        "icons.panel_close" => config.icons.panel_close = mask,
        "icons.search" => config.icons.search = mask,
        "icons.menu" => config.icons.menu = mask,
        "icons.sidebar_left" => config.icons.sidebar_left = mask,
        "icons.sidebar_right" => config.icons.sidebar_right = mask,
        "icons.archive" => config.icons.archive = mask,
        "icons.label" => config.icons.label = mask,
        "icons.share" => config.icons.share = mask,
        "icons.user" => config.icons.user = mask,
        "icons.comment" => config.icons.comment = mask,
        "icons.arrow_up" => config.icons.arrow_up = mask,
        "icons.external_link" => config.icons.external_link = mask,
        _ => {}
    }
}

pub fn build_fresh_export_xml(
    config_toml: &str,
    vfs: &HashMap<String, String>,
) -> Result<String, String> {
    let config = toml::from_str::<ThemeConfig>(config_toml)
        .map_err(|err| format!("could not parse TOML: {}", err))?;

    let rendered_xml = mor_blogger_core::render::render_theme(&config, vfs);
    mor_blogger_core::utils::rehydration::inject_state(&rendered_xml, &config)
}

pub fn handle_text_edit(target: &str, val: String, cfg: &str) -> Option<ThemeConfig> {
    if target.is_empty() {
        return None;
    }
    let mut config = toml::from_str::<ThemeConfig>(cfg).unwrap_or_default();
    if let Some(widget_id) = target
        .strip_prefix("widget.")
        .and_then(|s| s.strip_suffix(".title"))
    {
        config
            .template_pack
            .widget_titles
            .insert(widget_id.to_string(), val);
    } else {
        match target {
            "site.site_title" => config.site.site_title = val,
            "site.site_subtitle" => config.site.site_subtitle = val,
            "footer.footer_text" => config.footer.footer_text = val,
            "typography.body_font_stack" => config.typography.body_font_stack = val,
            "typography.heading_font_stack" => config.typography.heading_font_stack = val,
            "typography.mono_font_stack" => config.typography.mono_font_stack = val,
            _ => return None,
        }
    }
    Some(config)
}

pub fn handle_widget_move(id: &str, dest: &str, cfg: &str) -> Option<ThemeConfig> {
    if id.is_empty() || dest.is_empty() {
        return None;
    }
    let mut config = toml::from_str::<ThemeConfig>(cfg).unwrap_or_default();
    config.template_pack.move_widget(id, dest);
    Some(config)
}

pub fn handle_drop_svg(target: &str, content: &str, cfg: &str) -> Option<ThemeConfig> {
    if target.is_empty() || !is_svg(content) {
        return None;
    }
    let mask = svg_to_data_uri(content);
    let mut config = toml::from_str::<ThemeConfig>(cfg).unwrap_or_default();
    set_icon_slot(&mut config, target, mask);
    Some(config)
}

pub async fn handle_xml_export_to_disk(xml: String) -> Result<String, String> {
    if let Some(handle) = rfd::AsyncFileDialog::new()
        .set_file_name("theme.xml")
        .add_filter("XML", &["xml"])
        .save_file()
        .await
    {
        mor_blogger_core::render::save_xml_to_disk(&xml, handle.path())
            .map(|_| "Exported to disk.".to_string())
            .map_err(|err| format!("Export failed: {}", err))
    } else {
        Err("Cancelled".to_string())
    }
}

pub async fn handle_zip_export_to_disk(xml: String, cfg_str: String) -> Result<String, String> {
    if let Some(handle) = rfd::AsyncFileDialog::new()
        .set_file_name("theme_bundle.zip")
        .add_filter("ZIP", &["zip"])
        .save_file()
        .await
    {
        mor_blogger_core::render::save_bundle_to_disk(&xml, &cfg_str, handle.path())
            .map(|_| "Bundle exported.".to_string())
            .map_err(|err| format!("Bundle failed: {}", err))
    } else {
        Err("Cancelled".to_string())
    }
}

pub fn handle_copy_xml(xml: String, mut status_msg: Signal<String>) {
    crate::utils::clipboard::copy_to_clipboard(xml);
    status_msg.set("Theme XML copied to clipboard!".to_string());
}

#[allow(dead_code)]
pub fn handle_xml_export(xml: String, mut status_msg: Signal<String>) {
    spawn(async move {
        match handle_xml_export_to_disk(xml).await {
            Ok(msg) => status_msg.set(msg),
            Err(err) => {
                if err != "Cancelled" {
                    status_msg.set(err);
                }
            }
        }
    });
}

pub fn handle_xml_export_state(
    render: &crate::app::state::RenderState,
    mut status_msg: Signal<String>,
) {
    let xml = (render.generated_xml)();
    spawn(async move {
        match handle_xml_export_to_disk(xml).await {
            Ok(msg) => status_msg.set(msg),
            Err(err) => {
                if err != "Cancelled" {
                    status_msg.set(err);
                }
            }
        }
    });
}

pub fn handle_zip_export(xml: String, cfg_str: String, mut status_msg: Signal<String>) {
    spawn(async move {
        match handle_zip_export_to_disk(xml, cfg_str).await {
            Ok(msg) => status_msg.set(msg),
            Err(err) => {
                if err != "Cancelled" {
                    status_msg.set(err);
                }
            }
        }
    });
}
