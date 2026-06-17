use dioxus::prelude::*;
use crate::ui::shell::theme::{get_native_os_theme, GTK4_DARK_TOML, MAC_OS_LIGHT_TOML};

#[component]
pub fn UserPreferencesModal(
    mut show_prefs: Signal<bool>,
    mut active_theme_toml: Signal<String>,
    mut ui_mode_pref: Signal<String>,
    active_ui_mode: String,
) -> Element {
    if !show_prefs() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            style: "position:fixed;inset:0;z-index:9999;background:rgba(0,0,0,0.7);display:flex;align-items:center;justify-content:center;",
            onclick: move |_| show_prefs.set(false),

            div {
                class: "mor-modal",
                style: "min-width:420px;max-width:560px;",
                onclick: |e| e.stop_propagation(),

                div { class: "mor-modal-header",
                    span { "User Preferences" }
                    div {
                        class: "mor-modal-close",
                        onclick: move |_| show_prefs.set(false),
                        "×"
                    }
                }

                div { class: "mor-modal-body",

                    div { class: "editor-field-group",
                        label { class: "editor-field-label", "Window Mode" }
                        select {
                            class: "editor-select",
                            value: "{ui_mode_pref}",
                            onchange: move |evt| {
                                let new_mode = evt.value();
                                ui_mode_pref.set(new_mode.clone());
                                let json = format!(r#"{{ "ui_mode": "{}" }}"#, new_mode);
                                let _ = std::fs::write("editor_prefs.json", json);
                            },
                            option { value: "frameless", "Frameless (Custom OS Buttons)" }
                            option { value: "native", "Native OS Window" }
                            option { value: "tiling", "Tiling WM (No Buttons)" }
                        }
                        if ui_mode_pref() != active_ui_mode {
                            div { class: "editor-note", style: "margin-top:12px;border-color:var(--editor-warning);background:rgba(210,153,34,0.05);",
                                p { class: "editor-note-title", style: "color:var(--editor-warning);", "Restart Required" }
                                p { class: "editor-note-body", "Restart to apply the new window border setting." }
                            }
                        }
                    }

                    div { class: "editor-field-group", style: "margin-top:20px;",
                        label { class: "editor-field-label", "Workspace Theme" }
                        div { style: "display:flex;flex-wrap:wrap;gap:8px;margin-top:8px;",
                            button {
                                class: if active_theme_toml() == GTK4_DARK_TOML { "editor-button editor-button-active" } else { "editor-button" },
                                onclick: move |_| active_theme_toml.set(GTK4_DARK_TOML.to_string()),
                                "GTK4 Dark"
                            }
                            button {
                                class: if active_theme_toml() == MAC_OS_LIGHT_TOML { "editor-button editor-button-active" } else { "editor-button" },
                                onclick: move |_| active_theme_toml.set(MAC_OS_LIGHT_TOML.to_string()),
                                "macOS Light"
                            }
                            button {
                                class: if active_theme_toml() == get_native_os_theme() { "editor-button editor-button-active" } else { "editor-button" },
                                onclick: move |_| active_theme_toml.set(get_native_os_theme().to_string()),
                                "System Default"
                            }
                        }
                    }
                }
            }
        }
    }
}
