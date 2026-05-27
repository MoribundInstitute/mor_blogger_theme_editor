use dioxus::prelude::*;
use serde::Deserialize;

use crate::config::{AdsConfig, BackgroundConfig, SurfaceFill, ThemeConfig};
use crate::presets::{all_presets, Preset, PresetPalette};

#[derive(Clone, Copy, PartialEq)]
pub struct ThemeSignals {
    pub is_dark_mode: Signal<bool>,

    // Site identity
    pub site_title: Signal<String>,
    pub site_subtitle: Signal<String>,
    pub header_logo_url: Signal<String>,
    pub home_url: Signal<String>,

    // Color palette — bg_panel and bg_elevated are SurfaceFill.
    pub bg_base: Signal<String>,
    pub bg_panel: Signal<SurfaceFill>,
    pub bg_elevated: Signal<SurfaceFill>,
    pub fg_base: Signal<String>,
    pub fg_muted: Signal<String>,
    pub accent: Signal<String>,
    pub border: Signal<String>,

    // Buttons
    pub btn_radius: Signal<String>,
    pub btn_border_width: Signal<String>,
    pub btn_text_transform: Signal<String>,

    // Typography
    pub body_font_stack: Signal<String>,
    pub heading_font_stack: Signal<String>,
    pub mono_font_stack: Signal<String>,
    pub base_size: Signal<String>,
    pub scale_ratio: Signal<String>,
    pub line_height: Signal<String>,
    pub heading_weight: Signal<String>,

    // Media/assets
    pub background: Signal<BackgroundConfig>,
    pub favicon_url: Signal<String>,
    pub social_card_image_url: Signal<String>,

    // SEO
    pub meta_description: Signal<String>,
    pub meta_keywords: Signal<String>,
    pub custom_robots: Signal<String>,
    pub license_url: Signal<String>,
    pub author_name: Signal<String>,

    // Menu: four slots for the GUI.
    pub menu_1_label: Signal<String>,
    pub menu_1_url: Signal<String>,
    pub menu_2_label: Signal<String>,
    pub menu_2_url: Signal<String>,
    pub menu_3_label: Signal<String>,
    pub menu_3_url: Signal<String>,
    pub menu_4_label: Signal<String>,
    pub menu_4_url: Signal<String>,

    // Footer
    pub footer_text: Signal<String>,
    pub footer_license_label: Signal<String>,
    pub footer_license_url: Signal<String>,

    // Plugins
    pub custom_js: Signal<String>,

    // Active preset CSS.
    //
    // This is the important export/preview bridge:
    // - presets/css/*.css lives in the preset module through include_str!
    // - selecting a preset must copy that string here
    // - ThemeConfig must then carry this into render/xml_generator.rs
    // - xml_generator.rs must replace {{PRESET_CSS}} with this value
    pub preset_css: Signal<String>,

    // Static pages
    pub static_pages: Signal<crate::config::StaticPagesConfig>,

    // Ads
    pub ads: Signal<AdsConfig>,
}

impl ThemeSignals {
    pub fn apply_preset(&self, preset: &Preset) {
        let is_dark = *self.is_dark_mode.read();
        let palette = if is_dark { &preset.dark } else { &preset.light };

        self.swap_palette(palette);

        let base = &preset.base_config;
        self.apply_config_except_palette(base);

        self.apply_preset_css(preset);
    }

    pub fn apply_config(&self, config: &ThemeConfig) {
        self.bg_base.clone().set(config.colors.bg_base.clone());
        self.bg_panel.clone().set(config.colors.bg_panel.clone());
        self.bg_elevated
            .clone()
            .set(config.colors.bg_elevated.clone());
        self.fg_base.clone().set(config.colors.fg_base.clone());
        self.fg_muted.clone().set(config.colors.fg_muted.clone());
        self.accent.clone().set(config.colors.accent.clone());
        self.border.clone().set(config.colors.border.clone());
        self.background.clone().set(config.background.clone());

        self.apply_config_except_palette(config);

        // Imported/restored themes may include preset CSS. Preserve it.
        self.preset_css.clone().set(config.preset_css.clone());
    }

    pub fn apply_preset_css(&self, preset: &Preset) {
        self.preset_css
            .clone()
            .set(preset.preset_css.to_string());
    }

    fn apply_config_except_palette(&self, config: &ThemeConfig) {
        self.site_title.clone().set(config.site.site_title.clone());
        self.site_subtitle
            .clone()
            .set(config.site.site_subtitle.clone());
        self.header_logo_url
            .clone()
            .set(config.site.header_logo_url.clone());
        self.home_url.clone().set(config.site.home_url.clone());

        self.btn_radius.clone().set(config.buttons.radius.clone());
        self.btn_border_width
            .clone()
            .set(config.buttons.border_width.clone());
        self.btn_text_transform
            .clone()
            .set(config.buttons.text_transform.clone());

        self.body_font_stack
            .clone()
            .set(config.typography.body_font_stack.clone());
        self.heading_font_stack
            .clone()
            .set(config.typography.heading_font_stack.clone());
        self.mono_font_stack
            .clone()
            .set(config.typography.mono_font_stack.clone());
        self.base_size
            .clone()
            .set(config.typography.base_size.clone());
        self.scale_ratio
            .clone()
            .set(config.typography.scale_ratio.clone());
        self.line_height
            .clone()
            .set(config.typography.line_height.clone());
        self.heading_weight
            .clone()
            .set(config.typography.heading_weight.clone());

        self.favicon_url
            .clone()
            .set(config.assets.favicon_url.clone());
        self.social_card_image_url
            .clone()
            .set(config.assets.social_card_image_url.clone());

        self.meta_description
            .clone()
            .set(config.seo.meta_description.clone());
        self.meta_keywords
            .clone()
            .set(config.seo.meta_keywords.clone());
        self.custom_robots
            .clone()
            .set(config.seo.custom_robots.clone());
        self.license_url.clone().set(config.seo.license_url.clone());
        self.author_name.clone().set(config.seo.author_name.clone());

        let menu_pairs = [
            (self.menu_1_label, self.menu_1_url),
            (self.menu_2_label, self.menu_2_url),
            (self.menu_3_label, self.menu_3_url),
            (self.menu_4_label, self.menu_4_url),
        ];

        for (i, (mut label_sig, mut url_sig)) in menu_pairs.into_iter().enumerate() {
            let (label, url) = config
                .menu_links
                .get(i)
                .map(|menu| (menu.label.clone(), menu.url.clone()))
                .unwrap_or_default();

            label_sig.set(label);
            url_sig.set(url);
        }

        self.footer_text
            .clone()
            .set(config.footer.footer_text.clone());
        self.footer_license_label
            .clone()
            .set(config.footer.footer_license_label.clone());
        self.footer_license_url
            .clone()
            .set(config.footer.footer_license_url.clone());

        self.custom_js.clone().set(config.plugins.custom_js.clone());
        self.static_pages.clone().set(config.static_pages.clone());
        self.ads.clone().set(config.ads.clone());
    }

    pub fn swap_palette(&self, palette: &PresetPalette) {
        self.bg_base.clone().set(palette.colors.bg_base.clone());
        self.bg_panel.clone().set(palette.colors.bg_panel.clone());
        self.bg_elevated
            .clone()
            .set(palette.colors.bg_elevated.clone());
        self.fg_base.clone().set(palette.colors.fg_base.clone());
        self.fg_muted.clone().set(palette.colors.fg_muted.clone());
        self.accent.clone().set(palette.colors.accent.clone());
        self.border.clone().set(palette.colors.border.clone());
        self.background.clone().set(palette.background.clone());
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PresetsPanelProps {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
}

#[component]
pub fn PresetsPanel(props: PresetsPanelProps) -> Element {
    let presets = all_presets();
    let mut active = props.active_preset;
    let mut is_dark_mode = props.signals.is_dark_mode;

    let mut show_import = use_signal(|| false);
    let mut remote_url = use_signal(String::new);
    let mut pasted_theme = use_signal(String::new);
    let mut import_status = use_signal(String::new);

    let active_label = active()
        .and_then(|active_id| presets.iter().find(|preset| preset.id == active_id))
        .map(|preset| preset.name)
        .unwrap_or("Custom / Imported");

    let preset_css_bytes = props.signals.preset_css.read().len();

    rsx! {
        section {
            class: "editor-card preset-panel",

            div {
                class: "preset-panel-header",

                h3 {
                    class: "editor-card-title",
                    style: "margin-bottom: 0; padding-bottom: 0; border-bottom: none;",
                    "Theme Presets"
                }

                div {
                    class: "editor-row",

                    button {
                        class: if show_import() {
                            "editor-button editor-button-small editor-button-active"
                        } else {
                            "editor-button editor-button-small"
                        },
                        onclick: move |_| {
                            show_import.set(!show_import());
                        },
                        "Import JSON"
                    }

                    button {
                        class: "editor-button editor-button-small",
                        onclick: move |_| {
                            let new_mode = !is_dark_mode();
                            is_dark_mode.set(new_mode);

                            if let Some(active_id) = active() {
                                if let Some(preset) = presets.iter().find(|preset| preset.id == active_id) {
                                    if new_mode {
                                        props.signals.swap_palette(&preset.dark);
                                    } else {
                                        props.signals.swap_palette(&preset.light);
                                    }

                                    // Important: changing dark/light mode must not accidentally
                                    // leave the export config with empty preset_css.
                                    props.signals.apply_preset_css(preset);
                                }
                            }
                        },
                        if is_dark_mode() { "☾ Dark Mode" } else { "☀ Light Mode" }
                    }
                }
            }

            p {
                class: "preset-panel-copy",
                "One click to swap the entire theme. Edit any field afterward to customize."
            }

            div {
                class: "editor-note",
                style: "margin-bottom: 12px;",
                div {
                    class: "editor-note-body",
                    "Active preset: {active_label}"
                }
                div {
                    class: "editor-note-body",
                    "Preset CSS bytes: {preset_css_bytes}"
                }
            }

            if show_import() {
                div {
                    class: "editor-note",

                    h4 {
                        class: "editor-note-title",
                        "Import Compendium Theme"
                    }

                    p {
                        class: "editor-note-body",
                        "Paste a raw GitHub/compendium JSON URL, paste raw JSON/TOML, or load a local .json/.toml file."
                    }

                    div {
                        class: "editor-field-group",
                        label {
                            class: "editor-field-label",
                            "Remote JSON URL"
                        }

                        div {
                            class: "editor-row-stretch",

                            input {
                                class: "editor-field editor-flex-1",
                                r#type: "text",
                                placeholder: "https://raw.githubusercontent.com/.../theme.json",
                                value: "{remote_url}",
                                oninput: move |evt| {
                                    remote_url.set(evt.value());
                                    import_status.set(String::new());
                                }
                            }

                            button {
                                class: "editor-button",
                                onclick: move |_| {
                                    let url = normalize_preset_url(&remote_url());
                                    let signals = props.signals;

                                    async move {
                                        if url.trim().is_empty() {
                                            import_status.set("Paste a remote JSON URL first.".to_string());
                                            return;
                                        }

                                        match fetch_remote_theme(&url).await {
                                            Ok(config) => {
                                                signals.apply_config(&config);
                                                active.set(None);
                                                import_status.set("Imported remote theme.".to_string());
                                            }
                                            Err(err) => {
                                                import_status.set(format!("Import failed: {}", err));
                                            }
                                        }
                                    }
                                },
                                "Import URL"
                            }
                        }
                    }

                    div {
                        class: "editor-field-group",

                        label {
                            class: "editor-field-label",
                            "Local JSON/TOML File"
                        }

                        label {
                            class: "editor-button",
                            "Load JSON/TOML File"

                            input {
                                r#type: "file",
                                accept: ".json,.toml,.txt,application/json,text/plain",
                                style: "display: none;",
                                onchange: move |evt| {
                                    let signals = props.signals;

                                    async move {
                                        if let Some(file_engine) = evt.files() {
                                            if let Some(file_name) = file_engine.files().first() {
                                                match file_engine.read_file_to_string(file_name).await {
                                                    Some(contents) => match parse_theme_text(&contents) {
                                                        Ok(config) => {
                                                            signals.apply_config(&config);
                                                            active.set(None);
                                                            import_status.set(format!("Imported {}", file_name));
                                                        }
                                                        Err(err) => {
                                                            import_status.set(format!("Import failed: {}", err));
                                                        }
                                                    },
                                                    None => {
                                                        import_status.set(format!("Could not read {}", file_name));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "editor-field-group",

                        label {
                            class: "editor-field-label",
                            "Paste JSON or TOML"
                        }

                        textarea {
                            class: "editor-textarea",
                            style: "min-height: 90px; resize: vertical;",
                            placeholder: "Paste exported ThemeConfig JSON/TOML here...",
                            value: "{pasted_theme}",
                            oninput: move |evt| {
                                pasted_theme.set(evt.value());
                                import_status.set(String::new());
                            }
                        }

                        div {
                            class: "editor-row",

                            button {
                                class: "editor-button",
                                onclick: move |_| {
                                    let pasted = pasted_theme();

                                    match parse_theme_text(&pasted) {
                                        Ok(config) => {
                                            props.signals.apply_config(&config);
                                            active.set(None);
                                            import_status.set("Imported pasted theme.".to_string());
                                        }
                                        Err(err) => {
                                            import_status.set(format!("Import failed: {}", err));
                                        }
                                    }
                                },
                                "Import Pasted Theme"
                            }

                            button {
                                class: "editor-button editor-button-small",
                                onclick: move |_| {
                                    pasted_theme.set(String::new());
                                    import_status.set(String::new());
                                },
                                "Clear"
                            }
                        }
                    }

                    if !import_status().is_empty() {
                        div {
                            class: "restore-status",
                            "{import_status}"
                        }
                    }
                }
            }

            div {
                class: "preset-rail",

                for preset in presets.iter() {
                    PresetCard {
                        key: "{preset.id}",
                        preset: preset.clone(),
                        is_active: active.read().map(|id| id == preset.id).unwrap_or(false),
                        signals: props.signals,
                        active_preset: active,
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PresetCardProps {
    preset: Preset,
    is_active: bool,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
}

#[component]
fn PresetCard(props: PresetCardProps) -> Element {
    let preset = &props.preset;
    let mut active = props.active_preset;
    let preset_for_click = preset.clone();

    let is_dark = *props.signals.is_dark_mode.read();
    let palette = if is_dark { &preset.dark } else { &preset.light };
    let colors = &palette.colors;

    let bg_panel_css = colors.bg_panel.to_css();
    let bg_elevated_css = colors.bg_elevated.to_css();

    let active_class = if props.is_active {
        "preset-card preset-card-active"
    } else {
        "preset-card"
    };

    let sample_label = preset
        .base_config
        .menu_links
        .first()
        .map(|menu| menu.label.as_str())
        .filter(|label| !label.is_empty())
        .unwrap_or("Home");

    let sample_radius = &preset.base_config.buttons.radius;
    let sample_border_width = &preset.base_config.buttons.border_width;
    let sample_text_transform = &preset.base_config.buttons.text_transform;

    let body_font = &preset.base_config.typography.body_font_stack;
    let heading_font_raw = &preset.base_config.typography.heading_font_stack;
    let heading_font: &str = if heading_font_raw.trim().is_empty() {
        body_font
    } else {
        heading_font_raw
    };

    rsx! {
        button {
            class: "{active_class}",
            style: "--preset-accent: {colors.accent};",
            onclick: move |_| {
                props.signals.apply_preset(&preset_for_click);
                active.set(Some(preset_for_click.id));
            },

            div {
                style: "padding: 10px 12px; background: {bg_panel_css}; border-bottom: 1px solid {colors.border};",

                div {
                    style: "color: {colors.accent}; font-family: {heading_font}; font-size: 0.9rem; font-weight: bold; line-height: 1.2;",
                    "{preset.base_config.site.site_title}"
                }

                if !preset.base_config.site.site_subtitle.is_empty() {
                    div {
                        style: "color: {colors.fg_muted}; font-family: {body_font}; font-size: 0.65rem; margin-top: 2px; line-height: 1.2; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        "{preset.base_config.site.site_subtitle}"
                    }
                }

                div {
                    style: "display: inline-block; margin-top: 6px; padding: 2px 6px; border: {sample_border_width} solid {colors.border}; border-radius: {sample_radius}; color: {colors.fg_base}; font-family: {body_font}; font-size: 0.65rem; text-transform: {sample_text_transform};",
                    "{sample_label}"
                }
            }

            div {
                style: "padding: 8px 12px; background: {bg_elevated_css}; min-height: 24px;",

                div {
                    style: "color: {colors.fg_muted}; font-family: {body_font}; font-size: 0.6rem; line-height: 1.3;",
                    "Sample content"
                }
            }

            div {
                class: "preset-footer",

                div {
                    class: "preset-name",
                    "{preset.name}"
                }

                div {
                    class: "preset-description",
                    "{preset.description}"
                }
            }
        }
    }
}

async fn fetch_remote_theme(url: &str) -> Result<ThemeConfig, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("request failed: {}", err))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let text = response
        .text()
        .await
        .map_err(|err| format!("could not read response: {}", err))?;

    parse_theme_text(&text)
}

fn parse_theme_text(text: &str) -> Result<ThemeConfig, String> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return Err("theme text is empty".to_string());
    }

    if let Ok(config) = serde_json::from_str::<ThemeConfig>(trimmed) {
        return Ok(config);
    }

    if let Ok(config) = parse_json_wrapped_theme(trimmed) {
        return Ok(config);
    }

    if let Ok(config) = toml::from_str::<ThemeConfig>(trimmed) {
        return Ok(config);
    }

    Err(
        "expected a ThemeConfig JSON/TOML document, or a JSON object with base_config/config/theme"
            .to_string(),
    )
}

#[derive(Deserialize)]
struct WrappedTheme {
    #[serde(default)]
    base_config: Option<ThemeConfig>,

    #[serde(default)]
    config: Option<ThemeConfig>,

    #[serde(default)]
    theme: Option<ThemeConfig>,

    #[serde(default)]
    preset_css: Option<String>,
}

fn parse_json_wrapped_theme(text: &str) -> Result<ThemeConfig, String> {
    let wrapped = serde_json::from_str::<WrappedTheme>(text).map_err(|err| err.to_string())?;

    let mut config = wrapped
        .base_config
        .or(wrapped.config)
        .or(wrapped.theme)
        .ok_or_else(|| "no base_config/config/theme object found".to_string())?;

    if config.preset_css.trim().is_empty() {
        if let Some(css) = wrapped.preset_css {
            config.preset_css = css;
        }
    }

    Ok(config)
}

fn normalize_preset_url(input: &str) -> String {
    let url = input.trim();

    if url.contains("github.com/") && url.contains("/blob/") {
        let without_scheme = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        let parts: Vec<&str> = without_scheme.split('/').collect();

        if parts.len() >= 6 && parts[0] == "github.com" && parts[3] == "blob" {
            let owner = parts[1];
            let repo = parts[2];
            let branch = parts[4];
            let path = parts[5..].join("/");

            return format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, branch, path
            );
        }
    }

    url.to_string()
}