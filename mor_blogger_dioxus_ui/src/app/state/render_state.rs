use std::collections::HashMap;
use std::time::Duration;

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

/// How long a burst of config edits must go quiet before the cold pipeline
/// (full export XML + diagnostics) recomputes.
const COLD_DEBOUNCE_MS: u64 = 200;

/// Cold-path build: full theme render plus the zstd/base64 rehydration inject.
/// Shared by the debounced background pipeline and the on-demand export.
fn build_export_xml(config: &ThemeConfig, vfs: &HashMap<String, String>) -> String {
    let rendered_xml = render_theme(config, vfs);
    let payload =
        mor_blogger_core::utils::rehydration::RehydrationPayload::from_config(config.clone())
            .with_vfs(vfs.clone());
    match mor_blogger_core::utils::rehydration::inject_workspace_state(&rendered_xml, &payload) {
        Ok(xml) => xml,
        Err(err) => {
            log::error!("Failed to inject rehydration state: {}", err);
            rendered_xml
        }
    }
}

#[derive(Clone, Copy)]
pub struct RenderState {
    pub current_config: Memo<ThemeConfig>,
    /// Debounced, computed off the UI thread. May lag the config by up to
    /// [`COLD_DEBOUNCE_MS`] + build time; use [`Self::export_xml_now`] when
    /// freshness matters (file exports).
    pub generated_xml: Signal<String>,
    pub preview_html: Memo<String>,
    pub diag: Signal<DiagnosticResult>,
    vfs: Signal<HashMap<String, String>>,
}

impl RenderState {
    pub fn new(theme: ThemeState, layout: LayoutState, site_data: Signal<SiteData>) -> Self {
        let signals = theme.signals;
        let active_preset = theme.active_preset;
        let active_variant = theme.active_variant;

        let current_config = use_memo(move || {
            let mut config = signals.to_config();
            config.active_preset_id = active_preset().map(|s| s.to_string());
            config.active_variant_id = active_variant().map(|s| s.to_string());
            config
        });

        let vfs = use_context::<VfsDictionary>().0;

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

        let mut generated_xml = use_signal(String::new);
        let mut diag = use_signal(DiagnosticResult::default);

        // Generation counter: bumped on every real config/vfs change. The cold
        // pipeline subscribes to it (not to the config itself, so the expensive
        // clones happen after the debounce, not on every tick) and uses it to
        // discard results that were overtaken while spawn_blocking ran.
        let mut cold_generation = use_signal(|| 0u64);
        use_effect(move || {
            let _ = current_config.read();
            let _ = vfs.read();
            cold_generation += 1;
        });

        // Cold pipeline: debounce -> snapshot -> full export XML + integrity
        // check on a blocking thread. use_resource cancels the in-flight run at
        // the sleep/join await points whenever the generation bumps again.
        let _cold_pipeline = use_resource(move || {
            let generation = cold_generation();
            async move {
                // Skip the debounce for the very first build so the app doesn't
                // sit on an empty export at startup.
                if !generated_xml.peek().is_empty() {
                    tokio::time::sleep(Duration::from_millis(COLD_DEBOUNCE_MS)).await;
                }
                let config = current_config.peek().clone();
                let vfs_snapshot = vfs.peek().clone();
                let built = tokio::task::spawn_blocking(move || {
                    let xml = build_export_xml(&config, &vfs_snapshot);
                    let diag = check_integrity(&xml, &config.template_pack);
                    (xml, diag)
                })
                .await;
                if let Ok((xml, diag_result)) = built {
                    if generation == *cold_generation.peek() {
                        generated_xml.set(xml);
                        diag.set(diag_result);
                    }
                }
            }
        });

        Self {
            current_config,
            generated_xml,
            preview_html,
            diag,
            vfs,
        }
    }

    /// Synchronous, always-fresh export build for user-initiated actions.
    /// Bypasses the debounced pipeline so a click right after an edit can never
    /// export stale XML. Blocks briefly; fine for a rare explicit click.
    pub fn export_xml_now(&self) -> String {
        build_export_xml(&self.current_config.read(), &self.vfs.read())
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
