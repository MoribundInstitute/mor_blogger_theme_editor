use dioxus::prelude::*;

use crate::app::state::layout_state::LayoutState;
use crate::app::state::theme_state::ThemeState;
use crate::app::state::SiteData;
use crate::app::vfs::VfsDictionary;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::diagnostics::{check_integrity, DiagnosticResult};
use mor_blogger_core::render::template_resolver::{
    ComponentManifest, CONTENT_REGISTRY, FOOTER_REGISTRY, HEADER_REGISTRY, LAYOUT_REGISTRY,
    SIDEBAR_LEFT_REGISTRY, SIDEBAR_RIGHT_REGISTRY,
};
use mor_blogger_core::render::{render_preview_html, render_theme};

#[derive(Clone, Copy)]
pub struct RenderState {
    pub current_config: Memo<ThemeConfig>,
    pub generated_xml: Memo<String>,
    pub preview_html: Memo<String>,
    pub diag: Signal<DiagnosticResult>,
}

impl RenderState {
    pub fn new(theme: ThemeState, layout: LayoutState, site_data: Signal<SiteData>) -> Self {
        let signals = theme.signals;
        let active_preset = theme.active_preset;

        let current_config = use_memo(move || {
            let mut config = signals.to_config();
            config.active_preset_id = active_preset().map(|s| s.to_string());
            config
        });

        let current_config_for_xml = current_config;
        let vfs = use_context::<VfsDictionary>().0;

        let generated_xml = use_memo(move || {
            let config = current_config_for_xml();
            let rendered_xml = render_theme(&config, &*vfs.read());
            let payload = mor_blogger_core::utils::rehydration::RehydrationPayload::from_config(
                config.clone(),
            )
            .with_vfs(vfs.read().clone());
            match mor_blogger_core::utils::rehydration::inject_workspace_state(
                &rendered_xml,
                &payload,
            ) {
                Ok(xml) => xml,
                Err(err) => {
                    log::error!("Failed to inject rehydration state: {}", err);
                    rendered_xml
                }
            }
        });

        let current_config_for_preview = current_config;
        let preview_template_mode = layout.preview_template_mode;
        let is_dark_mode = theme.signals.is_dark_mode;

        let preview_html = use_memo(move || {
            render_preview_html(
                &current_config_for_preview(),
                &site_data.read().posts,
                preview_template_mode(),
                is_dark_mode(),
                &*vfs.read(),
            )
        });

        let current_config_for_diag_init = current_config;
        let current_config_for_diag_effect = current_config;

        let generated_xml_for_diag_init = generated_xml;
        let generated_xml_for_diag_effect = generated_xml;

        let mut diag = use_signal(move || {
            let config = current_config_for_diag_init();
            check_integrity(&generated_xml_for_diag_init(), &config.template_pack)
        });

        use_effect(move || {
            let config = current_config_for_diag_effect();
            diag.set(check_integrity(
                &generated_xml_for_diag_effect(),
                &config.template_pack,
            ));
        });

        Self {
            current_config,
            generated_xml,
            preview_html,
            diag,
        }
    }

    pub fn get_manifest(&self, registry_type: &str, id: &str) -> Option<ComponentManifest> {
        let registry = match registry_type {
            "header" => HEADER_REGISTRY,
            "layout" => LAYOUT_REGISTRY,
            "content" => CONTENT_REGISTRY,
            "sidebar_left" => SIDEBAR_LEFT_REGISTRY,
            "sidebar_right" => SIDEBAR_RIGHT_REGISTRY,
            "footer" => FOOTER_REGISTRY,
            _ => return None,
        };
        registry.iter().find(|c| c.id == id).cloned()
    }
}
