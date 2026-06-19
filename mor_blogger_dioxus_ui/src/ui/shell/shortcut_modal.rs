use crate::ui::components::modal::Modal;
use dioxus::prelude::*;

#[component]
pub fn KeyboardShortcutsModal(open: Signal<bool>) -> Element {
    let mut shortcuts = use_signal(|| crate::app::config_bridge::ShortcutPrefs::load());

    macro_rules! keys {
        ($field:ident, $default:literal) => {
            shortcuts()
                .$field
                .clone()
                .unwrap_or_else(|| $default.to_string())
                .split('+')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };
    }

    let shortcut_groups = vec![
        (
            "Global App",
            vec![
                ("User Preferences", keys!(user_prefs, "Ctrl+P")),
                ("Theme Diagnostics", keys!(theme_diagnostics, "Ctrl+D")),
                ("Toggle Preview", keys!(toggle_preview, "F9")),
                ("Exit Architect", keys!(exit_architect, "Ctrl+Q")),
            ],
        ),
        (
            "Project & File",
            vec![
                ("Open Project", keys!(open_project, "Ctrl+O")),
                ("Save Project", keys!(save_project, "Ctrl+S")),
                ("Export Blogger XML", keys!(export_xml, "Shift+Ctrl+E")),
            ],
        ),
        (
            "Workspace",
            vec![
                ("Undo", keys!(undo, "Ctrl+Z")),
                ("Redo", keys!(redo, "Ctrl+Y")),
                ("Copy Raw XML", keys!(copy_raw_xml, "Ctrl+C")),
                ("Toggle Left Dock", keys!(toggle_left_dock, "Ctrl+B")),
                ("Toggle Right Dock", keys!(toggle_right_dock, "Ctrl+E")),
                ("Reset Zoom", keys!(reset_zoom, "Ctrl+0")),
            ],
        ),
    ];

    rsx! {
        Modal {
            open: open,
            title: "Keyboard Shortcuts".to_string(),
            style: "min-width: 750px; max-width: 850px;".to_string(),

            div { class: "mor-shortcuts-wrapper",
                div { class: "mor-shortcuts-search",
                    span { class: "search-icon", "🔎" }
                    input {
                        class: "mor-input",
                        style: "width: 100%; margin-left: 10px;",
                        placeholder: "Search shortcuts...",
                    }
                }

                div { class: "mor-shortcuts-grid",
                    for (group_name, shortcuts_list) in shortcut_groups {
                        div { class: "mor-shortcut-group",
                            h4 { class: "mor-shortcut-group-title", "{group_name}" }

                            for (action, keys) in shortcuts_list {
                                div { class: "mor-shortcut-row",
                                    input {
                                        class: "mor-input",
                                        style: "width: 120px; text-align: center; font-family: monospace;",
                                        value: "{keys.join(\"+\")}",
                                        oninput: move |evt| {
                                            let mut sc = shortcuts.write();
                                            let val = Some(evt.value());
                                            match action {
                                                "Undo"              => sc.undo = val,
                                                "Redo"              => sc.redo = val,
                                                "Copy Raw XML"      => sc.copy_raw_xml = val,
                                                "Toggle Left Dock"  => sc.toggle_left_dock = val,
                                                "Toggle Right Dock" => sc.toggle_right_dock = val,
                                                "User Preferences"  => sc.user_prefs = val,
                                                "Theme Diagnostics" => sc.theme_diagnostics = val,
                                                "Toggle Preview"    => sc.toggle_preview = val,
                                                "Exit Architect"    => sc.exit_architect = val,
                                                "Open Project"      => sc.open_project = val,
                                                "Save Project"      => sc.save_project = val,
                                                "Export Blogger XML"=> sc.export_xml = val,
                                                "Reset Zoom"        => sc.reset_zoom = val,
                                                _ => {}
                                            }
                                            let _ = sc.save();
                                        }
                                    }
                                    div { class: "mor-action-label", "{action}" }
                                }
                            }
                        }
                    }
                }

                div {
                    style: "margin-top: 20px; display: flex; justify-content: flex-end;",
                    button {
                        class: "editor-button",
                        onclick: move |_| {
                            let path = mor_blogger_core::config::prefs::shortcuts_path();
                            let _ = std::process::Command::new("xdg-open").arg(path).spawn();
                        },
                        "Open Config File (.toml)"
                    }
                }
            }
        }
    }
}
