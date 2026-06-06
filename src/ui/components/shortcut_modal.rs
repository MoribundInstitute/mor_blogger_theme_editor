use dioxus::prelude::*;
use crate::ui::components::modal::Modal;

#[component]
pub fn KeyboardShortcutsModal(open: Signal<bool>) -> Element {
    // Placeholder data until we upgrade the ShortcutRegistry to pass this dynamically
    let shortcut_groups = vec![
        ("Global App", vec![
            ("User Preferences", vec!["Ctrl", "P"]),
            ("Theme Diagnostics", vec!["Ctrl", "D"]),
            ("Toggle Preview", vec!["F9"]),
            ("Exit Architect", vec!["Ctrl", "Q"]),
        ]),
        ("Project & File", vec![
            ("Open Project", vec!["Ctrl", "O"]),
            ("Save Project", vec!["Ctrl", "S"]),
            ("Export Blogger XML", vec!["Shift", "Ctrl", "E"]),
        ]),
        ("Workspace", vec![
            ("Undo", vec!["Ctrl", "Z"]),
            ("Redo", vec!["Ctrl", "Y"]),
            ("Copy Raw XML", vec!["Ctrl", "C"]),
            ("Reset Zoom", vec!["Ctrl", "0"]),
        ]),
    ];

    rsx! {
        Modal {
            open: open,
            title: "Keyboard Shortcuts".to_string(),
            // Slightly wider modal to accommodate the two-column grid
            style: "min-width: 750px; max-width: 850px;".to_string(),
            
            div { class: "mor-shortcuts-wrapper",
                
                // Search bar styled like your other inputs
                div { class: "mor-shortcuts-search",
                    span { class: "search-icon", "🔎" }
                    input { 
                        class: "mor-input", 
                        style: "width: 100%; margin-left: 10px;",
                        placeholder: "Search shortcuts...",
                    }
                }

                // The Nemo-inspired 2-column layout
                div { class: "mor-shortcuts-grid",
                    for (group_name, shortcuts) in shortcut_groups {
                        div { class: "mor-shortcut-group",
                            h4 { class: "mor-shortcut-group-title", "{group_name}" }
                            
                            for (action, keys) in shortcuts {
                                div { class: "mor-shortcut-row",
                                    div { class: "mor-key-cluster",
                                        for (i, key) in keys.iter().enumerate() {
                                            span { class: "mor-keycap", "{key}" }
                                            if i < keys.len() - 1 {
                                                span { class: "mor-key-plus", "+" }
                                            }
                                        }
                                    }
                                    div { class: "mor-action-label", "{action}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
