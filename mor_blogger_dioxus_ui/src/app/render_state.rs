use dioxus::prelude::*;

use mor_blogger_core::diagnostics::{check_integrity, DiagnosticResult};
use mor_blogger_core::render::{render_preview_html, render_theme};

use super::layout_state::AppLayoutState;
use super::state::ThemeAppState;
use crate::ui::docks::css_dock::VfsDictionary;

#[derive(Clone, Copy)]
pub struct AppRenderState {
    pub generated_xml: Memo<String>,
    pub preview_html: Memo<String>,
    pub diag: Signal<DiagnosticResult>,
}

pub fn use_app_render_state(theme: ThemeAppState, layout: AppLayoutState) -> AppRenderState {
    let current_config_for_xml = theme.current_config;
    let vfs = use_context::<VfsDictionary>().0;

    let generated_xml = use_memo(move || {
        let config = current_config_for_xml();
        let rendered_xml = render_theme(&config, &*vfs.read());
        match mor_blogger_core::utils::rehydration::inject_state(&rendered_xml, &config) {
            Ok(xml) => xml,
            Err(err) => {
                log::error!("Failed to inject rehydration state: {}", err);
                rendered_xml
            }
        }
    });

    let current_config_for_preview = theme.current_config;
    let preview_template_mode = layout.preview_template_mode;
    let is_dark_mode = theme.signals.is_dark_mode;

    let preview_html = use_memo(move || {
        render_preview_html(
            &current_config_for_preview(),
            preview_template_mode(),
            is_dark_mode(),
            &*vfs.peek(),
        )
    });

    let current_config_for_diag_init = theme.current_config;
    let current_config_for_diag_effect = theme.current_config;

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

    AppRenderState {
        generated_xml,
        preview_html,
        diag,
    }
}
