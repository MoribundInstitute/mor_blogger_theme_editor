use crate::app::state::{LayoutState, ThemeState};
use crate::ui::dialogs::about_dialog::AboutDialog;
use crate::ui::dialogs::advanced_colors_dialog::AdvancedColorsDialog;
use crate::ui::dialogs::advanced_presets_dialog::AdvancedPresetsDialog;
use crate::ui::dialogs::advanced_typography_dialog::AdvancedTypographyDialog;
use crate::ui::dialogs::documentation_dialog::DocumentationDialog;
use crate::ui::dialogs::shortcuts_dialog::ShortcutsDialog;
use crate::ui::dialogs::template_grid_dialog::TemplateGridDialog;
use crate::ui::dialogs::user_preferences_dialog::UserPreferencesDialog;
use crate::ui::dialogs::workspace_settings_dialog::{
    WorkspaceAppearancePanel, WorkspaceThemePresets,
};
use crate::ui::dialogs::modal::Modal;
use dioxus::prelude::*;
use mor_blogger_core::utils::svg_icons::*;

#[derive(Props, Clone, PartialEq)]
pub struct MorDialogsProps {
    pub show_about: Signal<bool>,
    pub show_prefs: Signal<bool>,
    pub show_editor_settings: Signal<bool>,
    pub show_shortcuts: Signal<bool>,
    pub show_docs: Signal<bool>,
    pub ui_mode_pref: Signal<String>,
    pub ui_theme_pref: Signal<String>,
    pub active_ui_mode: String,
}

#[component]
pub fn MorDialogs(props: MorDialogsProps) -> Element {
    let layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();

    rsx! {
        AboutDialog { open: props.show_about }

        UserPreferencesDialog {
            show_prefs: props.show_prefs,
            ui_mode_pref: props.ui_mode_pref,
            active_ui_mode: props.active_ui_mode,
        }

        EditorSettingsDialog {
            open: props.show_editor_settings,
            active_theme_toml: props.ui_theme_pref,
        }

        ShortcutsDialog { open: props.show_shortcuts }

        DocumentationDialog { open: props.show_docs }

        AdvancedPresetsDialog {
            open: theme.show_advanced_presets,
        }

        TemplateGridDialog {
            open: layout.show_advanced_modules,
        }

        AdvancedColorsDialog {
            open_signal: theme.show_advanced_colors,
        }

        AdvancedTypographyDialog {
            open_signal: theme.show_advanced_typography,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum SettingsTab {
    General,
    Appearance,
    QuickLaunch,
}

#[component]
pub fn EditorSettingsDialog(mut open: Signal<bool>, active_theme_toml: Signal<String>) -> Element {
    let mut active_tab = use_signal(|| SettingsTab::General);

    rsx! {
        Modal {
            open: open,
            title: "Editor Settings".to_string(),
            style: "width: 700px; height: 500px; max-width: 800px;".to_string(),
            on_close: move |_| open.set(false),

            div {
                class: "split-pane-container",
                style: "display: flex; height: 430px; min-height: 0;",

                nav {
                    style: "flex: 0 0 200px; display: flex; flex-direction: column; \
                            border-right: 1px solid var(--editor-border-soft, rgba(236, 231, 218, 0.18)); padding: 18px;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 4px; flex: 1 1 auto;",
                        CategoryButton {
                            label: "General",
                            active: active_tab() == SettingsTab::General,
                            on_click: move |_| active_tab.set(SettingsTab::General)
                        }
                        CategoryButton {
                            label: "Appearance",
                            active: active_tab() == SettingsTab::Appearance,
                            on_click: move |_| active_tab.set(SettingsTab::Appearance)
                        }
                        CategoryButton {
                            label: "Quick Launch Bar",
                            active: active_tab() == SettingsTab::QuickLaunch,
                            on_click: move |_| active_tab.set(SettingsTab::QuickLaunch)
                        }
                    }
                }

                div {
                    style: "flex: 1 1 auto; overflow-y: auto; padding: 24px; background: var(--editor-bg, #0f0e0c); color: var(--editor-fg, #f4f1ea);",

                    match active_tab() {
                        SettingsTab::General => rsx! {
                            GeneralSettings { active_theme_toml }
                        },
                        SettingsTab::Appearance => rsx! {
                            AppearanceSettings { active_theme_toml }
                        },
                        SettingsTab::QuickLaunch => rsx! {
                            QuickLaunchSettings {}
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn GeneralSettings(active_theme_toml: Signal<String>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            div {
                h3 {
                    style: "margin: 0 0 4px 0; font-size: 1.05rem; font-weight: 600;",
                    "General"
                }
                p {
                    style: "margin: 0 0 16px 0; font-size: 0.85rem; color: #a8a294;",
                    "Workspace theme presets. Changes apply immediately and persist to editor_prefs.toml."
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; background: rgba(0,0,0,0.2); padding: 16px; border-radius: 6px; border: 1px solid var(--editor-border-soft, rgba(236, 231, 218, 0.12));",
                    WorkspaceThemePresets { active_theme_toml }
                }
            }
        }
    }
}

#[component]
fn AppearanceSettings(active_theme_toml: Signal<String>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            div {
                h3 {
                    style: "margin: 0 0 4px 0; font-size: 1.05rem; font-weight: 600;",
                    "Appearance"
                }
                p {
                    style: "margin: 0 0 16px 0; font-size: 0.85rem; color: #a8a294;",
                    "Customize editor colors, typography, and layout geometry."
                }
                div {
                    style: "background: rgba(0,0,0,0.2); padding: 16px; border-radius: 6px; border: 1px solid var(--editor-border-soft, rgba(236, 231, 218, 0.12));",
                    WorkspaceAppearancePanel { active_theme_toml }
                }
            }
        }
    }
}

const QUICK_LAUNCH_ITEMS: &[(&str, &str, &str)] = &[
    ("theme_palette", "Theme Palette", ICON_CSS),
    ("site_data", "Site Data", ICON_PAGE),
    ("css_editor", "CSS Editor", ICON_CSS),
    ("js_editor", "JavaScript Config", ICON_JS),
    ("xml_editor", "XML Structure", ICON_XML),
    ("presets", "Presets", ICON_SETTINGS),
    ("plugin_manager", "Plugin Manager", ICON_SETTINGS),
    ("diagnostics", "Diagnostics", ICON_SETTINGS),
    ("css_builder", "CSS Builder", ICON_CSS),
    ("js_builder", "JS Builder", ICON_JS),
];

#[component]
fn QuickLaunchSettings() -> Element {
    let layout = use_context::<LayoutState>();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            div {
                h3 {
                    style: "margin: 0 0 4px 0; font-size: 1.05rem; font-weight: 600;",
                    "Quick Launch Bar"
                }
                p {
                    style: "margin: 0 0 16px 0; font-size: 0.85rem; color: #a8a294;",
                    "Choose which tools appear in the activity bar. Hidden items remain available from the menu."
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; background: rgba(0,0,0,0.2); padding: 16px; border-radius: 6px; border: 1px solid var(--editor-border-soft, rgba(236, 231, 218, 0.12));",
                    for (dock_id, label, icon) in QUICK_LAUNCH_ITEMS {
                        QuickLaunchToggle {
                            key: "{dock_id}",
                            label,
                            icon,
                            visible: layout.is_quick_launch_visible(dock_id),
                            on_change: move |visible| layout.set_quick_launch_visible(dock_id, visible),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuickLaunchToggle(
    label: &'static str,
    icon: &'static str,
    visible: bool,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        label {
            style: "display: flex; align-items: center; gap: 12px; cursor: pointer; padding: 8px; border-radius: 4px; transition: background 0.2s;",
            input {
                r#type: "checkbox",
                checked: visible,
                onchange: move |evt| on_change.call(evt.checked()),
                style: "cursor: pointer; width: 16px; height: 16px; accent-color: #73c991;",
            }
            span { style: "font-size: 18px;", "{icon}" }
            span { style: "font-size: 0.95rem;", "{label}" }
        }
    }
}

#[component]
fn CategoryButton(
    label: &'static str,
    active: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            style: format!(
                "text-align: left; padding: 8px 12px; border: none; cursor: pointer; border-radius: 3px; \
                 font-family: inherit; font-size: 0.95rem; transition: background 0.2s; \
                 background: {}; color: {}; font-weight: {};",
                if active { "rgba(236, 231, 218, 0.08)" } else { "transparent" },
                if active { "#f4f1ea" } else { "#a8a294" },
                if active { "600" } else { "400" }
            ),
            onclick: move |evt| on_click.call(evt),
            "{label}"
        }
    }
}