use crate::app::config_bridge::EditorPrefs;
use crate::app::state::ThemeState;
use crate::ui::components::form::MorCheckbox;
use crate::ui::dialogs::modal::Modal;
use dioxus::prelude::*;

#[component]
pub fn UserPreferencesDialog(
    mut show_prefs: Signal<bool>,
    mut ui_mode_pref: Signal<String>,
    active_ui_mode: String,
) -> Element {
    let theme_state = use_context::<ThemeState>();
    let mut enable_ai_bridge = theme_state.enable_ai_bridge;

    rsx! {
        Modal {
            open: show_prefs,
            title: "User Preferences".to_string(),
            style: "min-width:420px;max-width:560px;".to_string(),
            on_close: move |_| show_prefs.set(false),

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Window Mode" }
                select {
                    class: "editor-select",
                    value: "{ui_mode_pref}",
                    onchange: move |evt| {
                        let new_mode = evt.value();
                        ui_mode_pref.set(new_mode.clone());
                        EditorPrefs::update_ui_mode(new_mode);
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

            div { class: "editor-field-group",
                label { class: "editor-field-label", "AI Live State Bridge" }
                MorCheckbox {
                    label: "Enable AI Bridge (writes live state to /tmp)".to_string(),
                    checked: enable_ai_bridge(),
                    onchange: move |val| {
                        enable_ai_bridge.set(val);
                    }
                }
            }

            div { class: "editor-field-group",
                label { class: "editor-field-label", "Workspace Defaults" }
                button {
                    class: "editor-btn",
                    onclick: move |_| crate::app::config_bridge::EditorPrefs::clear_default_template_pack(),
                    "Clear Default Template"
                }
            }
        }
    }
}
