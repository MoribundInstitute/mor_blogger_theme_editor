use crate::ui::shell::shortcut::use_shortcut;
use dioxus::prelude::*; 

// =========================================================================
// 1. GENERIC BUILDING BLOCKS
// =========================================================================
#[derive(Props, Clone, PartialEq)]
pub struct MorMenuDropdownProps {
    pub label: String,
    pub children: Element,
}

#[component]
pub fn MorMenuDropdown(props: MorMenuDropdownProps) -> Element {
    rsx! {
        div { class: "mor-menu-item",
            "{props.label}"
            div { class: "mor-menu-dropdown",
                {props.children}
            }
        }
    }
}

#[component]
pub fn MorMenuBar(children: Element) -> Element {
    rsx! {
        nav { class: "mor-menu-bar",
            {children}
        }
    }
}

#[component]
pub fn MenuItem(
    label: String,
    #[props(default = None)] shortcut: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = None)] on_action: Option<EventHandler<()>>,
) -> Element {
    let bind_shortcut = if disabled { None } else { shortcut.clone() };
    use_shortcut(bind_shortcut, on_action.clone());

    rsx! {
        button {
            class: if disabled { "mor-menu-item disabled" } else { "mor-menu-item" },
            onmousedown: move |evt| evt.stop_propagation(), 
            onclick: move |e| {
                e.stop_propagation();
                if !disabled {
                    if let Some(h) = on_action { h.call(()); }
                }
            },
            span { "{label}" }
            if let Some(sc) = shortcut {
                span { class: "shortcut", "{sc}" }
            }
        }
    }
}

#[component]
pub fn MenuSeparator() -> Element {
    rsx! { div { class: "mor-menu-divider" } }
}

// =========================================================================
// 2. THE APP MENU INSTANCE
// =========================================================================
#[component]
pub fn AppMenuBar(
    mut show_prefs: Signal<bool>,
    mut show_about: Signal<bool>,
    mut show_shortcuts: Signal<bool>,
    mut show_plugins: Signal<bool>, 
    on_load_theme: EventHandler<()>,
    on_save_theme: EventHandler<()>,
    on_load_data: EventHandler<()>,
    on_save_data: EventHandler<()>,
    on_export_xml: EventHandler<()>,
    on_export_zip: EventHandler<()>,
) -> Element {
    rsx! {
        MorMenuBar {
            // 1. FILE
            MorMenuDropdown { label: "File".to_string(),
                MenuItem { label: "New Workspace".to_string() }
                MenuSeparator {}
                MenuItem {
                    label: "Load Theme (.toml)".to_string(),
                    on_action: move |_| on_load_theme.call(())
                }
                MenuItem {
                    label: "Save Theme (.toml)".to_string(),
                    on_action: move |_| on_save_theme.call(())
                }
                MenuSeparator {}
                MenuItem { 
                    label: "Import Sample Data (.json)".to_string(),
                    on_action: move |_| on_load_data.call(())
                }
                MenuItem { 
                    label: "Export Sample Data (.json)".to_string(),
                    on_action: move |_| on_save_data.call(())
                }
                MenuSeparator {}
                MenuItem {
                    label: "Export Blogger XML".to_string(),
                    on_action: move |_| on_export_xml.call(())
                }
                MenuItem {
                    label: "Export Theme Bundle (.zip)".to_string(),
                    on_action: move |_| on_export_zip.call(())
                }
                MenuSeparator {}
                MenuItem {
                    label: "Exit".to_string(),
                    on_action: move |_| -> () { std::process::exit(0); }
                }
            }

            // 2. EDIT
            MorMenuDropdown { label: "Edit".to_string(),
                MenuItem { label: "Undo".to_string(), shortcut: "Ctrl+Z".to_string() }
                MenuItem { label: "Redo".to_string(), shortcut: "Ctrl+Y".to_string() }
                MenuSeparator {}
                MenuItem { label: "Copy Raw XML".to_string() }
            }

            // 3. VIEW
            MorMenuDropdown { label: "View".to_string(),
                MenuItem { label: "Toggle Preview Monitor".to_string() }
                MenuItem { label: "Toggle Code Split".to_string() }
                MenuItem { label: "Reset Viewport Scale".to_string() }
            }

            // 4. DOCKS
            MorMenuDropdown { label: "Docks".to_string(),
                MenuItem { label: "Theme Palette (Left)".to_string() }
                MenuItem { label: "Site Data (Right)".to_string() }
                MenuSeparator {}
                MenuItem { label: "Lock Docks".to_string() }
            }

            // 5. PROFILE
            MorMenuDropdown { label: "Profile".to_string(),
                MenuItem {
                    label: "User Preferences".to_string(),
                    on_action: move |_| show_prefs.set(true)
                }
                MenuItem { label: "Editor Settings".to_string() }
            }

            // 6. TOOLS
            MorMenuDropdown { label: "Tools".to_string(),
                MenuItem { label: "Theme Diagnostics".to_string() }
                MenuItem { label: "CSS Token Builder".to_string() }
                MenuSeparator {}
                MenuItem {
                    label: "Plugin Manager".to_string(),
                    on_action: move |_| show_plugins.set(true)
                }
            }

            // 7. HELP
            MorMenuDropdown { label: "Help".to_string(),
                MenuItem { label: "Documentation".to_string() }
                MenuItem {
                    label: "Keyboard Shortcuts".to_string(),
                    on_action: move |_| show_shortcuts.set(true)
                }
                MenuSeparator {}
                MenuItem {
                    label: "About MorBlogger".to_string(),
                    on_action: move |_| show_about.set(true)
                }
            }
        }
    }
}