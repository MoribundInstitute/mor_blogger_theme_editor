use dioxus::prelude::*;

#[component]
pub fn MenuBar(
    mut show_prefs: Signal<bool>,
    mut show_about: Signal<bool>,
) -> Element {
    rsx! {
        nav { class: "mor-menu-bar",
            // 1. FILE
            div { class: "mor-menu-item", "File",
                div { class: "mor-menu-dropdown",
                    button { "New Workspace" }
                    div { class: "mor-menu-divider" }
                    button { "Load Theme (.toml)" }
                    button { "Save Theme (.toml)" }
                    div { class: "mor-menu-divider" }
                    button { "Import Data (.json)" }
                    button { "Export Data (.json)" }
                    div { class: "mor-menu-divider" }
                    button { "Export Blogger XML" }
                    button { "Export Theme Bundle (.zip)" }
                    div { class: "mor-menu-divider" }
                    button {
                        onclick: move |_| {
                            std::process::exit(0);
                        },
                        "Exit"
                    }
                }
            }

            // 2. EDIT
            div { class: "mor-menu-item", "Edit",
                div { class: "mor-menu-dropdown",
                    button { "Undo" }
                    button { "Redo" }
                    div { class: "mor-menu-divider" }
                    button { "Copy Raw XML to Clipboard" }
                }
            }

            // 3. VIEW
            div { class: "mor-menu-item", "View",
                div { class: "mor-menu-dropdown",
                    button { "Toggle Preview Monitor" }
                    button { "Toggle Code Split" }
                    button { "Reset Viewport Scale" }
                }
            }

            // 4. DOCKS
            div { class: "mor-menu-item", "Docks",
                div { class: "mor-menu-dropdown",
                    button { "Theme Palette (Left)" }
                    button { "Site Data (Right)" }
                    div { class: "mor-menu-divider" }
                    button { "Lock Docks" }
                }
            }

            // 5. PROFILE
            div { class: "mor-menu-item", "Profile",
                div { class: "mor-menu-dropdown",
                    button {
                        onclick: move |_| show_prefs.set(true),
                        "User Preferences"
                    }
                    button { "Editor Settings" }
                }
            }

            // 6. TOOLS
            div { class: "mor-menu-item", "Tools",
                div { class: "mor-menu-dropdown",
                    button { "Theme Diagnostics" }
                    button { "CSS Token Builder" }
                }
            }

            // 7. HELP
            div { class: "mor-menu-item", "Help",
                div { class: "mor-menu-dropdown",
                    button { "Documentation" }
                    button { "Keyboard Shortcuts" }
                    div { class: "mor-menu-divider" }
                    button {
                        onclick: move |_| show_about.set(true),
                        "About MorBlogger"
                    }
                }
            }
        }
    }
}
