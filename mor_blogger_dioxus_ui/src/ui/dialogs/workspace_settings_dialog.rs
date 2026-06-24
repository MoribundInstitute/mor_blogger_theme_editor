use crate::app::config_bridge::EditorPrefs;
use crate::ui::dialogs::modal::Modal;
use crate::ui::layout::theme::{get_native_os_theme, MorTheme, GTK4_DARK_TOML, MAC_OS_LIGHT_TOML};
use dioxus::prelude::*;

fn apply_workspace_theme(mut active_theme_toml: Signal<String>, toml_str: String) {
    active_theme_toml.set(toml_str.clone());
    EditorPrefs::update_workspace_theme(toml_str);
}

/// Quick workspace theme preset picker (GTK4 Dark, macOS Light, System).
#[component]
pub fn WorkspaceThemePresets(active_theme_toml: Signal<String>) -> Element {
    let current = active_theme_toml();
    let is_dark = current == GTK4_DARK_TOML;
    let is_light = current == MAC_OS_LIGHT_TOML;
    let is_sys = current == get_native_os_theme();

    rsx! {
        div {
            class: "editor-field-group",
            label { class: "editor-field-label", "Workspace Theme Preset" }
            div {
                style: "display: flex; gap: 8px; margin-top: 8px; flex-wrap: wrap;",
                ThemeBtn { active: is_dark, label: "GTK4 Dark", theme: GTK4_DARK_TOML, active_theme_toml }
                ThemeBtn { active: is_light, label: "macOS Light", theme: MAC_OS_LIGHT_TOML, active_theme_toml }
                ThemeBtn { active: is_sys, label: "System Default", theme: get_native_os_theme(), active_theme_toml }
            }
        }
    }
}

/// Full appearance customizer: colors, typography, and geometry.
#[component]
pub fn WorkspaceAppearancePanel(active_theme_toml: Signal<String>) -> Element {
    let mut current_tab = use_signal(|| 0);
    let theme_struct = MorTheme::from_toml(&active_theme_toml()).unwrap_or_default();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            div {
                class: "mor-tabs",
                style: "display: flex; gap: 4px; margin-bottom: 4px;",
                button {
                    class: if current_tab() == 0 { "mor-tab active" } else { "mor-tab" },
                    onclick: move |_| current_tab.set(0),
                    "Colors"
                }
                button {
                    class: if current_tab() == 1 { "mor-tab active" } else { "mor-tab" },
                    onclick: move |_| current_tab.set(1),
                    "Typography & Geometry"
                }
            }

            if current_tab() == 0 {
                ColorCustomizerSection {
                    theme: theme_struct.clone(),
                    on_change: move |new_theme: MorTheme| {
                        let toml_str = new_theme.to_toml();
                        apply_workspace_theme(active_theme_toml, toml_str);
                    }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    TypographySection {
                        theme: theme_struct.clone(),
                        on_change: move |new_theme: MorTheme| {
                            let toml_str = new_theme.to_toml();
                            apply_workspace_theme(active_theme_toml, toml_str);
                        }
                    }
                    GeometrySection {
                        theme: theme_struct,
                        on_change: move |new_theme: MorTheme| {
                            let toml_str = new_theme.to_toml();
                            apply_workspace_theme(active_theme_toml, toml_str);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct WorkspaceSettingsDialogProps {
    pub open: Signal<bool>,
    pub active_theme_toml: Signal<String>,
}

/// Standalone modal wrapper — prefer `EditorSettingsDialog` in the shell.
#[component]
pub fn WorkspaceSettingsDialog(props: WorkspaceSettingsDialogProps) -> Element {
    let mut open = props.open;
    let active_theme_toml = props.active_theme_toml;

    rsx! {
        Modal {
            open,
            title: "Workspace Editor Settings".to_string(),
            style: "min-width: 520px; max-width: 640px;".to_string(),
            on_close: move |_| open.set(false),

            div {
                style: "display: flex; flex-direction: column; gap: 16px; overflow-y: auto; max-height: 520px; padding-right: 4px;",
                WorkspaceThemePresets { active_theme_toml }
                WorkspaceAppearancePanel { active_theme_toml }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ThemeBtnProps {
    active: bool,
    label: &'static str,
    theme: &'static str,
    active_theme_toml: Signal<String>,
}

#[component]
fn ThemeBtn(props: ThemeBtnProps) -> Element {
    let cls = if props.active {
        "editor-button editor-button-active"
    } else {
        "editor-button"
    };
    rsx! {
        button {
            class: "{cls}",
            onclick: move |_| {
                apply_workspace_theme(props.active_theme_toml, props.theme.to_string());
            },
            "{props.label}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ColorInputProps {
    label: &'static str,
    value: String,
    on_change: EventHandler<String>,
}

#[component]
fn ColorInput(props: ColorInputProps) -> Element {
    let val_clone = props.value.clone();
    rsx! {
        div {
            style: "display: grid; grid-template-columns: 140px 1fr; align-items: center; gap: 12px; margin: 4px 0;",
            label { class: "editor-field-label", "{props.label}" }
            div {
                style: "display: flex; align-items: center; gap: 8px; width: 100%;",
                input {
                    r#type: "color",
                    class: "editor-color-field",
                    style: "width: 38px; height: 32px; border: 1px solid var(--editor-border); border-radius: var(--radius-sm); cursor: pointer; background: transparent; padding: 0;",
                    value: "{props.value}",
                    oninput: move |evt| props.on_change.call(evt.value()),
                }
                input {
                    r#type: "text",
                    class: "editor-field",
                    style: "flex: 1; font-family: monospace; font-size: 13px;",
                    value: "{val_clone}",
                    oninput: move |evt| props.on_change.call(evt.value()),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ColorCustomizerSectionProps {
    theme: MorTheme,
    on_change: EventHandler<MorTheme>,
}

#[component]
fn ColorCustomizerSection(props: ColorCustomizerSectionProps) -> Element {
    let theme = props.theme;
    rsx! {
        div {
            class: "editor-field-group",
            style: "border-top: 1px solid var(--editor-border-soft); padding-top: 12px;",
            label {
                class: "editor-field-label",
                style: "font-weight: 600; margin-bottom: 8px; color: var(--editor-accent-warm);",
                "Aesthetics & Colors"
            }

            div { style: "font-size: 11px; font-weight: 600; color: var(--mor-text-muted); margin-top: 12px; margin-bottom: 6px; text-transform: uppercase; letter-spacing: 0.5px;", "Surfaces" }
            ColorInput { label: "Workspace Background", value: theme.bg.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.bg = v; props.on_change.call(nt); } } }
            ColorInput { label: "Side Panels", value: theme.panel.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.panel = v; props.on_change.call(nt); } } }
            ColorInput { label: "Title Headerbar", value: theme.header.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.header = v; props.on_change.call(nt); } } }

            div { style: "font-size: 11px; font-weight: 600; color: var(--mor-text-muted); margin-top: 12px; margin-bottom: 6px; text-transform: uppercase; letter-spacing: 0.5px;", "Typography & Borders" }
            ColorInput { label: "Normal Text", value: theme.text.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.text = v; props.on_change.call(nt); } } }
            ColorInput { label: "Muted Text", value: theme.text_muted.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.text_muted = v; props.on_change.call(nt); } } }
            ColorInput { label: "Base Border", value: theme.border.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.border = v; props.on_change.call(nt); } } }
            ColorInput { label: "Soft Border", value: theme.border_light.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.border_light = v; props.on_change.call(nt); } } }

            div { style: "font-size: 11px; font-weight: 600; color: var(--mor-text-muted); margin-top: 12px; margin-bottom: 6px; text-transform: uppercase; letter-spacing: 0.5px;", "Actions & Status" }
            ColorInput { label: "Theme Accent", value: theme.accent.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.accent = v; props.on_change.call(nt); } } }
            ColorInput { label: "Accent Hover", value: theme.accent_hover.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.accent_hover = v; props.on_change.call(nt); } } }
            ColorInput { label: "Button Background", value: theme.btn.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.btn = v; props.on_change.call(nt); } } }
            ColorInput { label: "Button Hover", value: theme.btn_hover.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.btn_hover = v; props.on_change.call(nt); } } }
            ColorInput { label: "Danger/Delete", value: theme.destructive.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.destructive = v; props.on_change.call(nt); } } }
            ColorInput { label: "Success/Go", value: theme.success.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.success = v; props.on_change.call(nt); } } }
            ColorInput { label: "Warning", value: theme.warning.clone(), on_change: { let theme = theme.clone(); move |v| { let mut nt = theme.clone(); nt.warning = v; props.on_change.call(nt); } } }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TypographySectionProps {
    theme: MorTheme,
    on_change: EventHandler<MorTheme>,
}

#[component]
fn TypographySection(props: TypographySectionProps) -> Element {
    let theme = props.theme;
    let font_presets = vec![
        "Cantarell, system-ui, sans-serif",
        "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
        "'Segoe UI Variable', 'Segoe UI', system-ui, sans-serif",
        "JetBrains Mono, monospace",
        "Fira Code, monospace",
        "Source Code Pro, monospace",
        "Inter, sans-serif",
        "Ubuntu, sans-serif",
        "Fira Sans, sans-serif",
    ];
    let current_font = theme.font_family.clone();

    rsx! {
        div {
            class: "editor-field-group",
            label {
                class: "editor-field-label",
                style: "font-weight: 600; margin-bottom: 8px; color: var(--editor-accent-warm);",
                "Typography"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { class: "editor-field-label", "Select Preset Font" }
                    select {
                        class: "editor-select",
                        value: "{current_font}",
                        onchange: {
                            let theme = theme.clone();
                            move |evt| {
                                let mut new_theme = theme.clone();
                                new_theme.font_family = evt.value();
                                props.on_change.call(new_theme);
                            }
                        },
                        for opt in font_presets.clone() {
                            option { value: "{opt}", "{opt}" }
                        }
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { class: "editor-field-label", "Custom Font Family" }
                    input {
                        r#type: "text",
                        class: "editor-field",
                        placeholder: "e.g. system-ui, sans-serif",
                        value: "{theme.font_family}",
                        oninput: {
                            let theme = theme.clone();
                            move |evt| {
                                let mut new_theme = theme.clone();
                                new_theme.font_family = evt.value();
                                props.on_change.call(new_theme);
                            }
                        }
                    }
                    span { class: "editor-mini-label", "Type any system font family name or fallback list." }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct GeometrySectionProps {
    theme: MorTheme,
    on_change: EventHandler<MorTheme>,
}

#[component]
fn GeometrySection(props: GeometrySectionProps) -> Element {
    let theme = props.theme;

    rsx! {
        div {
            class: "editor-field-group",
            style: "border-top: 1px solid var(--editor-border-soft); padding-top: 12px;",
            label {
                class: "editor-field-label",
                style: "font-weight: 600; margin-bottom: 8px; color: var(--editor-accent-warm);",
                "Geometry & Layout"
            }
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { class: "editor-field-label", "Base Font Size" }
                    input {
                        r#type: "text",
                        class: "editor-field",
                        placeholder: "e.g. 13px",
                        value: "{theme.font_size_base}",
                        oninput: {
                            let theme = theme.clone();
                            move |evt| {
                                let mut new_theme = theme.clone();
                                new_theme.font_size_base = evt.value();
                                props.on_change.call(new_theme);
                            }
                        }
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { class: "editor-field-label", "Heading Font Size" }
                    input {
                        r#type: "text",
                        class: "editor-field",
                        placeholder: "e.g. 20px",
                        value: "{theme.font_size_h1}",
                        oninput: {
                            let theme = theme.clone();
                            move |evt| {
                                let mut new_theme = theme.clone();
                                new_theme.font_size_h1 = evt.value();
                                props.on_change.call(new_theme);
                            }
                        }
                    }
                }
            }
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-top: 10px;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { class: "editor-field-label", "Control Padding" }
                    input {
                        r#type: "text",
                        class: "editor-field",
                        placeholder: "e.g. 8px",
                        value: "{theme.padding_base}",
                        oninput: {
                            let theme = theme.clone();
                            move |evt| {
                                let mut new_theme = theme.clone();
                                new_theme.padding_base = evt.value();
                                props.on_change.call(new_theme);
                            }
                        }
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { class: "editor-field-label", "Border Radius" }
                    input {
                        r#type: "text",
                        class: "editor-field",
                        placeholder: "e.g. 6px",
                        value: "{theme.border_radius}",
                        oninput: {
                            let theme = theme.clone();
                            move |evt| {
                                let mut new_theme = theme.clone();
                                new_theme.border_radius = evt.value();
                                props.on_change.call(new_theme);
                            }
                        }
                    }
                }
            }
        }
    }
}