use dioxus::html::HasFileData;
use dioxus::prelude::*;

use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::presets::{all_presets, Preset};
use mor_blogger_core::utils::rehydration::extract_and_decode;

use super::importers::{
    fetch_remote_theme, normalize_preset_url, parse_theme_text, save_imported_gtk_preset,
};
use super::morph_preview_from_preset;
use crate::app::state::{DockPosition, LayoutState, ThemeState};
use crate::ui_kit::MorPanelWrapper;
use crate::app::theme_signals::ThemeSignals;

const PRESET_FLOATING_DRAG_JS: &str = r#"
(function () {
    if (window.__morPresetFloatingDragInstalled) return;
    window.__morPresetFloatingDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const bar = e.target.closest('.preset-floating-window-bar');
        if (!bar) return;
        if (e.target.closest('button, input, textarea, select, a, label')) return;

        const panel = bar.closest('.preset-floating-window');
        if (!panel) return;

        e.preventDefault();

        const startX = e.clientX;
        const startY = e.clientY;
        const rect = panel.getBoundingClientRect();
        const startLeft = rect.left;
        const startTop = rect.top;

        document.body.classList.add('editor-floating-dragging');

        const onMove = function (moveEvt) {
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;

            const maxLeft = Math.max(0, window.innerWidth - 160);
            const maxTop = Math.max(0, window.innerHeight - 80);

            const nextLeft = Math.max(0, Math.min(startLeft + dx, maxLeft));
            const nextTop = Math.max(0, Math.min(startTop + dy, maxTop));

            document.documentElement.style.setProperty('--preset-floating-x', nextLeft + 'px');
            document.documentElement.style.setProperty('--preset-floating-y', nextTop + 'px');
        };

        const onUp = function () {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
            document.body.classList.remove('editor-floating-dragging');
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

#[derive(Props, Clone, PartialEq)]
pub struct PresetsPanelProps {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    pub current_config: ThemeConfig,
    pub on_apply_theme: EventHandler<ThemeConfig>,
    pub show_undocked_presets: Signal<bool>,
    #[props(default = false)]
    pub is_embedded: bool,
}

#[component]
pub fn PresetsPanel(props: PresetsPanelProps) -> Element {
    let mut theme = use_context::<ThemeState>();
    let mut layout = use_context::<LayoutState>();
    let presets = all_presets();
    let mut active = props.active_preset;

    let mut show_import = use_signal(|| false);
    let mut remote_url = use_signal(String::new);
    let mut pasted_theme = use_signal(String::new);

    let active_label = active()
        .and_then(|active_id| presets.iter().find(|preset| preset.id == active_id))
        .map(|preset| preset.name)
        .unwrap_or("Default Base Theme");
    let active_scheme = (theme.active_variant)().unwrap_or("Default scheme");

    let preset_css_bytes = props.signals.preset_css.read().len();

    let last_gtk_summary = (theme.last_imported_gtk)().map(|imported| {
        format!(
            "Last GTK import: {} — {}",
            imported.name,
            imported.report.short_status()
        )
    });

    let inner_content = rsx! {
        script { dangerous_inner_html: "{PRESET_FLOATING_DRAG_JS}" }

        section {
            class: "editor-card preset-panel",

            div { class: "preset-panel-header",
                if !props.is_embedded {
                    h3 { class: "editor-card-title", style: "margin-bottom: 0; padding-bottom: 0; border-bottom: none;", "Theme Presets" }
                }

                div { class: "editor-row", style: "flex-wrap: wrap; gap: 4px; margin-left: auto;",
                    button {
                        class: if show_import() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        title: "Import JSON",
                        onclick: move |_| show_import.set(!show_import()),
                        "⤓"
                    }

                    button {
                        class: "editor-mini-button",
                        title: if (props.signals.is_dark_mode)() { "Switch to light mode" } else { "Switch to dark mode" },
                        onclick: move |_| {
                            theme.perform_dark_mode_toggle();
                        },
                        if (props.signals.is_dark_mode)() { "☾" } else { "☀" }
                    }

                    button {
                        class: if (theme.show_advanced_presets)() { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        title: "Advanced presets",
                        onclick: move |_| theme.show_advanced_presets.set(true),
                        "⚙"
                    }

                    if !props.is_embedded {
                        button {
                            class: "editor-mini-button",
                            title: "Close panel",
                            onclick: move |_| {
                                layout.presets_pos.set(DockPosition::Hidden);
                            },
                            "✕"
                        }
                    }
                }
            }


            p { class: "preset-panel-copy", "One click to swap the entire theme. Edit any field afterward to customize." }

            div { class: "editor-note", style: "margin-bottom: 12px;",
                div { class: "editor-note-body", "Active preset: {active_label} — {active_scheme}" }
                div { class: "editor-note-body", "Preset CSS bytes: {preset_css_bytes}" }
                if let Some(summary) = last_gtk_summary { div { class: "editor-note-body", "{summary}" } }
            }

            if (theme.last_imported_gtk)().is_some() {
                div { class: "editor-note", style: "margin-bottom: 12px;",
                    h4 { class: "editor-note-title", "Imported GTK Theme" }
                    p { class: "editor-note-body", "This GTK import is currently applied as Custom / Imported. Save it to make it a reusable user preset on disk." }
                    div { class: "editor-row", style: "margin-top: 10px;",
                        button {
                            class: "editor-button",
                            onclick: move |_| {
                                let Some(imported) = (theme.last_imported_gtk)() else { return };
                                match save_imported_gtk_preset(&imported) {
                                    // We now receive a simple string message from the importer
                                    Ok(success_msg) => theme.import_status.set(success_msg),
                                    Err(err) => theme.import_status.set(format!("Could not save GTK preset: {}", err)),
                                }
                            },
                            "Save Imported Theme as Preset"
                        }
                    }
                }
            }

            if !(theme.import_status)().is_empty() { div { class: "restore-status", style: "margin-bottom: 12px;", "{theme.import_status}" } }

            if show_import() {
                div { class: "editor-note",
                    h4 { class: "editor-note-title", "Import Compendium Theme" }
                    p { class: "editor-note-body", "Paste remote URL or local file below." }

                    div { class: "editor-field-group",
                        label { class: "editor-field-label", "Remote JSON URL" }
                        div { class: "editor-row-stretch",
                            input {
                                class: "editor-field editor-flex-1", r#type: "text", placeholder: "https://github.com/...", value: "{remote_url}",
                                oninput: move |evt| { remote_url.set(evt.value()); theme.import_status.set(String::new()); }
                            }
                            button {
                                class: "editor-button",
                                onclick: move |_| {
                                    let url = normalize_preset_url(&remote_url());
                                    let signals = props.signals;
                                    async move {
                                        if url.trim().is_empty() { theme.import_status.set("Paste URL first.".to_string()); return; }
                                        match fetch_remote_theme(&url).await {
                                            Ok(config) => { signals.apply_config(&config); active.set(None); theme.commit(); theme.last_imported_gtk.set(None); theme.import_status.set("Imported remote theme.".to_string()); }
                                            Err(err) => theme.import_status.set(format!("Import failed: {}", err)),
                                        }
                                    }
                                },
                                "Import URL"
                            }
                        }
                    }

                    div { class: "editor-field-group",
                        label { class: "editor-field-label", "Local JSON/TOML File" }
                        label { class: "editor-button", "Load JSON/TOML File"
                            input {
                                r#type: "file", accept: ".json,.toml,.txt,application/json,text/plain", style: "display: none;",
                                onchange: move |evt| {
                                    let signals = props.signals;
                                    async move {
                                        if let Some(file) = evt.files().first() {
                                            if let Ok(bytes) = file.read_bytes().await {
                                                match parse_theme_text(&String::from_utf8_lossy(&bytes)) {
                                                    Ok(config) => { signals.apply_config(&config); active.set(None); theme.commit(); theme.last_imported_gtk.set(None); theme.import_status.set("Imported local theme file.".to_string()); }
                                                    Err(err) => theme.import_status.set(format!("Import failed: {}", err)),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "editor-field-group",
                        label { class: "editor-field-label", "Paste JSON or TOML" }
                        textarea {
                            class: "editor-textarea", style: "min-height: 90px; resize: vertical;", placeholder: "Paste JSON/TOML here...", value: "{pasted_theme}",
                            oninput: move |evt| { pasted_theme.set(evt.value()); theme.import_status.set(String::new()); }
                        }
                        div { class: "editor-row",
                            button {
                                class: "editor-button",
                                onclick: move |_| {
                                    match parse_theme_text(&pasted_theme()) {
                                        Ok(config) => { props.signals.apply_config(&config); active.set(None); theme.commit(); theme.last_imported_gtk.set(None); theme.import_status.set("Imported pasted theme.".to_string()); }
                                        Err(err) => theme.import_status.set(format!("Import failed: {}", err)),
                                    }
                                },
                                "Import Pasted Theme"
                            }
                            button { class: "editor-button editor-button-small", onclick: move |_| { pasted_theme.set(String::new()); theme.import_status.set(String::new()); }, "Clear" }
                        }
                    }
                }
            }

            // Compact rows: the whole catalog scannable in ~1/4 the height of
            // preview cards. Rich cards live in the floating preset browser.
            div {
                style: "display: flex; flex-direction: column; gap: 6px; margin-top: 14px; max-height: 420px; overflow-y: auto; padding: 2px;",
                for preset in presets.iter() {
                    PresetRow { key: "{preset.id}", preset: preset.clone(), is_active: active() == Some(preset.id), signals: props.signals, active_preset: active }
                }
            }
        }
    };

    if props.is_embedded {
        rsx! { {inner_content} }
    } else {
        let pos = (layout.presets_pos)();
        if pos == DockPosition::Hidden {
            return rsx! {};
        }

        rsx! {
            MorPanelWrapper {
                position: pos,
                default_position: DockPosition::mor_panel_left,
                title: "Presets",
                {inner_content}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct PresetCardProps {
    preset: Preset,
    is_active: bool,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
}

#[component]
pub(crate) fn PresetCard(props: PresetCardProps) -> Element {
    let mut theme = use_context::<ThemeState>();
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
        .map(|m| m.label.as_str())
        .filter(|l| !l.is_empty())
        .unwrap_or("Home");
    let sample_radius = &preset.base_config.buttons.radius;
    let sample_border_width = &preset.base_config.buttons.border_width;
    let sample_text_transform = &preset.base_config.buttons.text_transform;

    let body_font = &preset.base_config.typography.body_font_stack;
    let heading_font_raw = &preset.base_config.typography.heading_font_stack;
    let heading_font = if heading_font_raw.trim().is_empty() {
        body_font
    } else {
        heading_font_raw
    };

    rsx! {
        button {
            class: "{active_class}",
            style: "--preset-accent: {colors.accent}; flex: 0 0 240px; min-width: 240px; scroll-snap-align: start;",
            onclick: move |_| {
                let is_dark = *props.signals.is_dark_mode.read();
                props.signals.apply_preset(&preset_for_click);
                active.set(Some(preset_for_click.id));
                theme.active_variant.set(None);
                theme.commit();
                morph_preview_from_preset(&preset_for_click, is_dark);
            },
            div { style: "padding: 10px 12px; background: {bg_panel_css}; border-bottom: 1px solid {colors.border};",
                div { style: "color: {colors.accent}; font-family: {heading_font}; font-size: 0.9rem; font-weight: bold; line-height: 1.2;", "{preset.base_config.site.site_title}" }
                if !preset.base_config.site.site_subtitle.is_empty() {
                    div { style: "color: {colors.fg_muted}; font-family: {body_font}; font-size: 0.65rem; margin-top: 2px; line-height: 1.2; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;", "{preset.base_config.site.site_subtitle}" }
                }
                div { style: "display: inline-block; margin-top: 6px; padding: 2px 6px; border: {sample_border_width} solid {colors.border}; border-radius: {sample_radius}; color: {colors.fg_base}; font-family: {body_font}; font-size: 0.65rem; text-transform: {sample_text_transform};", "{sample_label}" }
            }
            div { style: "padding: 8px 12px; background: {bg_elevated_css}; min-height: 24px;",
                div { style: "color: {colors.fg_muted}; font-family: {body_font}; font-size: 0.6rem; line-height: 1.3;", "Sample content" }
            }
            div { class: "preset-footer",
                div { class: "preset-name", "{preset.name}" }
                div { class: "preset-description", "{preset.description}" }
                if !preset.variants.is_empty() {
                    SchemeDots { preset: preset.clone(), is_active: props.is_active, signals: props.signals, active_preset: active }
                }
            }
        }
    }
}

/// Base + variant color-scheme dots for a preset. Click applies the preset
/// with that palette; the active scheme gets an accent ring.
#[component]
pub(crate) fn SchemeDots(
    preset: Preset,
    is_active: bool,
    signals: ThemeSignals,
    active_preset: Signal<Option<&'static str>>,
) -> Element {
    let mut theme = use_context::<ThemeState>();
    let mut active = active_preset;
    let is_dark = *signals.is_dark_mode.read();
    let palette = if is_dark { &preset.dark } else { &preset.light };
    let colors = palette.colors.clone();
    let active_variant = (theme.active_variant)();
    let base_active = is_active && active_variant.is_none();
    let ring = |on: bool| {
        if on {
            "outline: 2px solid var(--editor-accent, #e8a04c); outline-offset: 2px;"
        } else {
            ""
        }
    };

    rsx! {
        div {
            style: "display: flex; gap: 7px; margin-top: 6px; align-items: center;",
            span {
                role: "button",
                title: "{preset.name} — default scheme",
                style: "width: 14px; height: 14px; border-radius: 50%; background: {colors.accent}; border: 2px solid {colors.border}; cursor: pointer; display: inline-block; flex-shrink: 0; {ring(base_active)}",
                onclick: {
                    let preset_for_base = preset.clone();
                    move |evt: MouseEvent| {
                        evt.stop_propagation();
                        let is_dark = *signals.is_dark_mode.read();
                        signals.apply_preset(&preset_for_base);
                        active.set(Some(preset_for_base.id));
                        theme.active_variant.set(None);
                        theme.commit();
                        morph_preview_from_preset(&preset_for_base, is_dark);
                    }
                },
            }
            for v in preset.variants.iter() {
                {
                    let vname = v.name;
                    let vaccent = if is_dark { v.dark.colors.accent.clone() } else { v.light.colors.accent.clone() };
                    let preset_for_variant = preset.clone();
                    let v_active = is_active && active_variant == Some(vname);
                    rsx! {
                        span {
                            key: "{preset.id}-{vname}",
                            role: "button",
                            title: "{preset.name} — {vname}",
                            style: "width: 14px; height: 14px; border-radius: 50%; background: {vaccent}; border: 2px solid {colors.border}; cursor: pointer; display: inline-block; flex-shrink: 0; {ring(v_active)}",
                            onclick: move |evt: MouseEvent| {
                                evt.stop_propagation();
                                let is_dark = *signals.is_dark_mode.read();
                                // Structure + CSS from the preset, then the variant palette.
                                signals.apply_preset(&preset_for_variant);
                                let (light, dark) = preset_for_variant.palette_pair(Some(vname));
                                signals.swap_palette(if is_dark { dark } else { light });
                                active.set(Some(preset_for_variant.id));
                                theme.active_variant.set(Some(vname));
                                theme.commit();
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Compact one-line preset entry for the docked panel: theme-tinted row with
/// name + scheme dots. The rich preview cards stay in the floating browser.
#[component]
pub(crate) fn PresetRow(props: PresetCardProps) -> Element {
    let mut theme = use_context::<ThemeState>();
    let preset = &props.preset;
    let mut active = props.active_preset;
    let preset_for_click = preset.clone();

    let is_dark = *props.signals.is_dark_mode.read();
    let palette = if is_dark { &preset.dark } else { &preset.light };
    let colors = &palette.colors;
    let bg_panel_css = colors.bg_panel.to_css();

    let heading_font_raw = &preset.base_config.typography.heading_font_stack;
    let heading_font = if heading_font_raw.trim().is_empty() {
        &preset.base_config.typography.body_font_stack
    } else {
        heading_font_raw
    };

    let border = if props.is_active {
        "2px solid var(--editor-accent, #e8a04c)".to_string()
    } else {
        format!("1px solid {}", colors.border)
    };

    rsx! {
        button {
            title: "{preset.description}",
            style: "display: flex; align-items: center; justify-content: space-between; gap: 10px; width: 100%; padding: 7px 10px; background: {bg_panel_css}; border: {border}; border-radius: 8px; cursor: pointer; text-align: left;",
            onclick: move |_| {
                let is_dark = *props.signals.is_dark_mode.read();
                props.signals.apply_preset(&preset_for_click);
                active.set(Some(preset_for_click.id));
                theme.active_variant.set(None);
                theme.commit();
                morph_preview_from_preset(&preset_for_click, is_dark);
            },
            span {
                style: "color: {colors.accent}; font-family: {heading_font}; font-size: 0.82rem; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0;",
                "{preset.name}"
            }
            span { style: "flex-shrink: 0; margin-top: -6px;",
                SchemeDots { preset: preset.clone(), is_active: props.is_active, signals: props.signals, active_preset: active }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PresetFloatingWindowProps {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    pub show_undocked_presets: Signal<bool>,
}

#[component]
pub fn PresetFloatingWindow(props: PresetFloatingWindowProps) -> Element {
    let mut show_undocked_presets = props.show_undocked_presets;
    let active = props.active_preset;
    rsx! {
        script { dangerous_inner_html: "{PRESET_FLOATING_DRAG_JS}" }
        div { class: "preset-floating-window",
            div { class: "preset-floating-window-bar floating-editor-window-bar",
                div { class: "preset-floating-window-title-group",
                    span { class: "floating-editor-grip", "⠿" }
                    div {
                        div { class: "preset-floating-window-title", "Theme Presets" }
                        div { class: "preset-floating-window-subtitle", "Detached preset browser" }
                    }
                }
                div { class: "floating-editor-window-actions",
                    button { class: "editor-mini-button", onclick: move |_| show_undocked_presets.set(false), "Dock" }
                }
            }
            div { class: "preset-floating-window-body",
                div { class: "preset-rail preset-rail-undocked",
                    for preset in all_presets().iter() {
                        PresetCard { key: "undocked-{preset.id}", preset: preset.clone(), is_active: active() == Some(preset.id), signals: props.signals, active_preset: active }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ThemeRestoreDropZone(
    on_restore: EventHandler<ThemeConfig>,
    on_close: EventHandler<()>,
) -> Element {
    let mut is_hovered = use_signal(|| false);
    let mut import_text = use_signal(|| "".to_string());
    let mut import_status = use_signal(|| "".to_string());

    let restore_from_xml = move |contents: String, source_label: String| {
        import_text.set(contents.clone());
        match extract_and_decode(&contents) {
            Ok(config) => {
                on_restore.call(config);
                import_status.set(format!("Workspace restored from {}!", source_label));
                import_text.set("".to_string());
            }
            Err(err) => import_status.set(format!("Error restoring {}: {}", source_label, err)),
        }
    };

    rsx! {
        div {
            class: if is_hovered() { "restore-workspace-drawer hovered" } else { "restore-workspace-drawer" },
            ondragover: move |evt| { evt.prevent_default(); is_hovered.set(true); import_status.set("Drop XML file to restore.".to_string()); },
            ondragenter: move |evt| { evt.prevent_default(); is_hovered.set(true); },
            ondragleave: move |_| is_hovered.set(false),
            ondrop: {
                let mut restore_from_xml = restore_from_xml.clone();
                move |evt| {
                    async move {
                        evt.prevent_default(); is_hovered.set(false); import_status.set("Reading file...".to_string());
                        if let Some(file) = evt.files().first() {
                            if let Ok(bytes) = file.read_bytes().await {
                                restore_from_xml(String::from_utf8_lossy(&bytes).into_owned(), file.name());
                            }
                        }
                    }
                }
            },
            div { class: "restore-header",
                h4 { class: "restore-title", "Restore Workspace from Blogger XML" }
                button { class: "editor-mini-button", onclick: move |_| on_close.call(()), "Close" }
            }
            p { class: "restore-copy", "Paste exported Blogger XML, drag and drop an .xml file, or use Choose XML." }
            div { class: "editor-row-stretch",
                textarea { class: "editor-textarea restore-textarea", placeholder: "Paste Blogger XML here...", value: "{import_text}", oninput: move |evt| { import_text.set(evt.value()); import_status.set(String::new()); } }
                div { class: "restore-button-stack",
                    label { class: "editor-button restore-button", "Choose XML"
                        input {
                            r#type: "file", accept: ".xml", style: "display: none;",
                            onchange: {
                                let mut restore_from_xml = restore_from_xml.clone();
                                move |evt| async move {
                                    if let Some(file) = evt.files().first() {
                                        if let Ok(bytes) = file.read_bytes().await {
                                            restore_from_xml(String::from_utf8_lossy(&bytes).into_owned(), file.name());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    button {
                        class: "editor-button restore-button",
                        onclick: {
                            let mut restore_from_xml = restore_from_xml.clone();
                            move |_| {
                                if import_text().trim().is_empty() { import_status.set("Paste XML first.".to_string()); return; }
                                restore_from_xml(import_text(), "pasted XML".to_string());
                            }
                        },
                        "Rehydrate"
                    }
                }
            }
            if !import_status().is_empty() { div { class: "restore-status", "{import_status}" } }
        }
    }
}
