use dioxus::prelude::*;

use crate::config::gtk_theme::ImportedGtkPreset;
use crate::config::ThemeConfig;
use crate::presets::all_presets;

use super::card::PresetCard;
use super::drag::PRESET_FLOATING_DRAG_JS;
use super::importers::{
    choose_gtk_theme, fetch_remote_theme, normalize_preset_url, parse_theme_text,
    save_imported_gtk_preset,
};
use super::signals::ThemeSignals;

#[derive(Props, Clone, PartialEq)]
pub struct PresetsPanelProps {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    pub current_config: ThemeConfig,
    pub on_apply_theme: EventHandler<ThemeConfig>,
    pub show_undocked_presets: Signal<bool>,
}

#[component]
pub fn PresetsPanel(props: PresetsPanelProps) -> Element {
    let presets = all_presets();
    let mut active = props.active_preset;
    let mut is_dark_mode = props.signals.is_dark_mode;

    let mut show_import = use_signal(|| false);
    let mut show_undocked_presets = props.show_undocked_presets;
    let mut remote_url = use_signal(String::new);
    let mut pasted_theme = use_signal(String::new);
    let mut import_status = use_signal(String::new);
    let mut last_imported_gtk = use_signal(|| None::<ImportedGtkPreset>);

    let active_label = active()
        .and_then(|active_id| presets.iter().find(|preset| preset.id == active_id))
        .map(|preset| preset.name)
        .unwrap_or("Custom / Imported");

    let preset_css_bytes = props.signals.preset_css.read().len();

    let current_config_for_gtk = props.current_config.clone();
    let on_apply_gtk_theme = props.on_apply_theme;

    let last_gtk_summary = last_imported_gtk().map(|imported| {
        format!(
            "Last GTK import: {} — {}",
            imported.name,
            imported.report.short_status()
        )
    });

    rsx! {
        script { dangerous_inner_html: "{PRESET_FLOATING_DRAG_JS}" }

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

                    button {
                        class: "editor-button editor-button-small",
                        onclick: move |_| {
                            match choose_gtk_theme(&current_config_for_gtk) {
                                Ok(Some(imported)) => {
                                    let status = format!(
                                        "Imported GTK theme '{}' from {}. {}",
                                        imported.name,
                                        imported.source_dir.display(),
                                        imported.report.short_status()
                                    );
                                    on_apply_gtk_theme.call(imported.config.clone());
                                    active.set(None);
                                    last_imported_gtk.set(Some(imported));
                                    import_status.set(status);
                                }
                                Ok(None) => {}
                                Err(err) => {
                                    import_status.set(format!("GTK import failed: {}", err));
                                }
                            }
                        },
                        "Import GTK4"
                    }

                    button {
                        class: if show_undocked_presets() {
                            "editor-button editor-button-small editor-button-active"
                        } else {
                            "editor-button editor-button-small"
                        },
                        onclick: move |_| {
                            show_undocked_presets.set(!show_undocked_presets());
                        },
                        if show_undocked_presets() { "Dock Presets" } else { "Undock Presets" }
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
                if let Some(summary) = last_gtk_summary {
                    div {
                        class: "editor-note-body",
                        "{summary}"
                    }
                }
            }

            if last_imported_gtk().is_some() {
                div {
                    class: "editor-note",
                    style: "margin-bottom: 12px;",

                    h4 {
                        class: "editor-note-title",
                        "Imported GTK Theme"
                    }

                    p {
                        class: "editor-note-body",
                        "This GTK import is currently applied as Custom / Imported. Save it to make it a reusable user preset on disk."
                    }

                    div {
                        class: "editor-row",
                        style: "margin-top: 10px;",

                        button {
                            class: "editor-button",
                            onclick: move |_| {
                                let Some(imported) = last_imported_gtk() else {
                                    import_status.set("No GTK import is available to save.".to_string());
                                    return;
                                };

                                match save_imported_gtk_preset(&imported) {
                                    Ok(report) => {
                                        import_status.set(format!(
                                            "Saved GTK preset '{}' to {} ({} file(s) written). Restart or reload user presets after Phase 5 to see it as a preset card.",
                                            imported.name,
                                            report.bundle_dir.display(),
                                            report.files_written.len()
                                        ));
                                    }
                                    Err(err) => {
                                        import_status.set(format!("Could not save GTK preset: {}", err));
                                    }
                                }
                            },
                            "Save Imported Theme as Preset"
                        }
                    }
                }
            }

            if !import_status().is_empty() {
                div {
                    class: "restore-status",
                    style: "margin-bottom: 12px;",
                    "{import_status}"
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
                                                last_imported_gtk.set(None);
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
                                                            last_imported_gtk.set(None);
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
                                            last_imported_gtk.set(None);
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
                }
            }

            div {
                class: "preset-rail",
                style: "display: flex; gap: 10px; overflow-x: auto; overflow-y: hidden; padding-bottom: 8px; scroll-snap-type: x proximity;",

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
