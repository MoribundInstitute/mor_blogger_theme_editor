use mor_blogger_core::config::ThemeConfig;
use crate::ui::panels::theme_palette::presets::ThemeSignals;

pub fn apply_hotswap_json(signals: ThemeSignals, json_text: String) {
    // 1. Instantly parse the JSON straight into your master config struct
    match serde_json::from_str::<ThemeConfig>(&json_text) {
        Ok(config) => {
            // 2. Push the loaded config into the Dioxus UI signals using your original method!
            signals.apply_config(&config);
            log::info!("Hotswap successful: Project loaded and UI updated.");
        }
        Err(e) => {
            log::error!("Load Project failed: invalid JSON payload. Error: {}", e);
        }
    }
}

/// Injects CSS variables into the preview iframe, targeting both :root and html[data-theme='...']
/// to ensure high specificity over the iframe's internal theme styles.
pub fn execute_theme_morph(css_vars: &str, is_light_mode: bool) {
    let theme = if is_light_mode { "light" } else { "dark" };
    // One flat JS template string. No wrappers.
    let js = format!(
        r#"let f=document.getElementById('mor-preview-frame');if(f&&f.contentWindow){{let d=f.contentDocument||f.contentWindow.document;let t='{theme}';d.documentElement.setAttribute('data-theme',t);let s=d.getElementById('mor-hotswap-style');if(!s){{s=d.createElement('style');s.id='mor-hotswap-style';d.head.appendChild(s);}}s.textContent=`:root,html[data-theme='${{t}}']{{ {css_vars} }}`;}}"#,
    );
    let _ = dioxus::document::eval(&js);
}
