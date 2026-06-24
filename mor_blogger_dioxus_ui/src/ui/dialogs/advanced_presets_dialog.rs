use crate::app::state::{RenderState, ThemeState};
use crate::ui::dialogs::modal::Modal;
use crate::ui::panels::theme_palette::presets::importers::choose_gtk_theme;
use dioxus::prelude::*;

#[component]
pub fn AdvancedPresetsDialog(mut open: Signal<bool>) -> Element {
    let mut theme = use_context::<ThemeState>();
    let render = use_context::<RenderState>();

    rsx! {
        Modal {
            open: open,
            title: "Advanced Preset Options",
            style: "width: 400px; height: auto;".to_string(),
            on_close: move |_| open.set(false),

            div {
                style: "display: flex; flex-direction: column; gap: 12px; padding: 8px;",

                button {
                    class: "mor-btn-secondary",
                    onclick: move |_| {
                        theme.show_undocked_presets.set(!(theme.show_undocked_presets)());
                    },
                    if (theme.show_undocked_presets)() { "Dock Presets" } else { "Undock Presets" }
                }

                h4 { style: "color: var(--fg-muted); margin-top: 8px; margin-bottom: 4px;", "External Imports" }
                button {
                    class: "mor-btn",
                    onclick: move |_| {
                        match choose_gtk_theme(&(render.current_config)()) {
                            Ok(Some(imported)) => {
                                let status = format!(
                                    "Imported GTK theme '{}' from {}. {}",
                                    imported.name,
                                    imported.source_dir.display(),
                                    imported.report.short_status()
                                );
                                theme.signals.apply_config(&imported.config);
                                theme.active_preset.set(None);
                                theme.commit();
                                theme.last_imported_gtk.set(Some(imported));
                                theme.import_status.set(status);
                                open.set(false);
                            }
                            Ok(None) => {}
                            Err(err) => {
                                theme.import_status.set(format!("GTK import failed: {}", err));
                                open.set(false);
                            }
                        }
                    },
                    "Import GTK4 Theme"
                }
                button {
                    class: "mor-btn-outline",
                    onclick: move |_| {
                        let _ = std::process::Command::new("xdg-open")
                            .arg("https://www.gnome-look.org/browse/")
                            .spawn();
                    },
                    "Get GTK4 Themes..."
                }

                h4 { style: "color: var(--fg-muted); margin-top: 8px; margin-bottom: 4px;", "Developer Tools" }
                button {
                    class: "mor-btn",
                    onclick: move |_| {
                        let _ = std::process::Command::new("xdg-open")
                            .arg(mor_blogger_core::presets::get_canonical_presets_dir())
                            .spawn();
                    },
                    "Edit Presets (.toml)"
                }
                button {
                    class: "mor-btn-outline",
                    onclick: move |_| {
                        let _ = std::process::Command::new("xdg-open")
                            .arg("https://github.com/MoribundInstitute/mor-blogger-theme-preset-compendium")
                            .spawn();
                    },
                    "Theme Compendium (GitHub)"
                }
            }
        }
    }
}
