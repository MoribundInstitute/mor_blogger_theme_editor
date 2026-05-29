use dioxus::prelude::*;

use crate::diagnostics::{check_integrity, DiagnosticResult};
use crate::render::{render_preview_html, render_theme};

use super::layout_state::AppLayoutState;
use super::state::ThemeAppState;

#[derive(Clone, Copy)]
pub struct AppRenderState {
    pub generated_xml: Memo<String>,
    pub preview_html: Memo<String>,
    pub diag: Signal<DiagnosticResult>,
}

pub fn use_app_render_state(theme: ThemeAppState, layout: AppLayoutState) -> AppRenderState {
    let current_config_for_xml = theme.current_config;
    let active_preset = theme.active_preset;

    let generated_xml = use_memo(move || {
        let config = current_config_for_xml();
        let (light, dark) = crate::presets::resolve_palette_pair(active_preset(), &config);
        render_theme(&config, &light, &dark)
    });

    let current_config_for_preview = theme.current_config;
    let preview_template_mode = layout.preview_template_mode;

    let preview_html =
        use_memo(move || render_preview_html(&current_config_for_preview(), preview_template_mode()));

    let mut diag = use_signal(|| check_integrity(&generated_xml()));
    use_effect(move || {
        diag.set(check_integrity(&generated_xml()));
    });

    AppRenderState {
        generated_xml,
        preview_html,
        diag,
    }
}
