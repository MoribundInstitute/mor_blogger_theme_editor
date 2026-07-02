use crate::app::config_bridge::ShortcutPrefs;
use crate::ui::dialogs::modal::Modal;
use crate::ui::layout::shortcut::{normalize_combo, ShortcutRegistry};
use dioxus::prelude::*;

/// One rebindable action: (stable id, display label, group).
const ACTIONS: &[(&str, &str, &str)] = &[
    ("user_prefs", "User Preferences", "Global App"),
    ("theme_diagnostics", "Theme Diagnostics", "Global App"),
    ("toggle_preview", "Toggle Preview", "Global App"),
    ("exit_architect", "Exit Architect", "Global App"),
    ("open_project", "Load Theme (.toml)", "Project & File"),
    ("save_project", "Save Theme (.toml)", "Project & File"),
    ("export_xml", "Export Blogger XML", "Project & File"),
    ("undo", "Undo", "Workspace"),
    ("redo", "Redo", "Workspace"),
    ("copy_raw_xml", "Copy Raw XML", "Workspace"),
    ("toggle_left_dock", "Toggle Left Dock", "Workspace"),
    ("toggle_right_dock", "Toggle Right Dock", "Workspace"),
    ("close_left_dock", "Close Left Pane", "Workspace"),
    ("close_right_dock", "Close Right Pane", "Workspace"),
    ("reset_zoom", "Reset Zoom", "Workspace"),
];

fn field_mut<'a>(sc: &'a mut ShortcutPrefs, id: &str) -> Option<&'a mut Option<String>> {
    Some(match id {
        "user_prefs" => &mut sc.user_prefs,
        "theme_diagnostics" => &mut sc.theme_diagnostics,
        "toggle_preview" => &mut sc.toggle_preview,
        "exit_architect" => &mut sc.exit_architect,
        "open_project" => &mut sc.open_project,
        "save_project" => &mut sc.save_project,
        "export_xml" => &mut sc.export_xml,
        "undo" => &mut sc.undo,
        "redo" => &mut sc.redo,
        "copy_raw_xml" => &mut sc.copy_raw_xml,
        "toggle_left_dock" => &mut sc.toggle_left_dock,
        "toggle_right_dock" => &mut sc.toggle_right_dock,
        "close_left_dock" => &mut sc.close_left_dock,
        "close_right_dock" => &mut sc.close_right_dock,
        "reset_zoom" => &mut sc.reset_zoom,
        _ => return None,
    })
}

fn field(sc: &ShortcutPrefs, id: &str) -> Option<String> {
    let mut sc = sc.clone();
    field_mut(&mut sc, id).and_then(|f| f.clone())
}

/// Turn a captured keydown into the canonical combo, or None while only
/// modifiers are held.
fn combo_from_event(evt: &Event<KeyboardData>) -> Option<String> {
    let key = match evt.key() {
        dioxus::html::Key::Character(c) => c.to_uppercase(),
        dioxus::html::Key::Control | dioxus::html::Key::Shift | dioxus::html::Key::Alt => {
            return None
        }
        other => other.to_string().to_uppercase(),
    };
    let mut combo = String::new();
    if evt.modifiers().ctrl() {
        combo.push_str("Ctrl+");
    }
    if evt.modifiers().shift() {
        combo.push_str("Shift+");
    }
    if evt.modifiers().alt() {
        combo.push_str("Alt+");
    }
    combo.push_str(&key);
    Some(combo)
}

#[component]
pub fn ShortcutsDialog(open: Signal<bool>) -> Element {
    let mut prefs = use_context::<Signal<ShortcutPrefs>>();
    let registry = try_consume_context::<Signal<ShortcutRegistry>>();
    // Action id currently capturing its new combo, if any.
    let mut capturing = use_signal(|| None::<&'static str>);
    let mut filter = use_signal(String::new);

    let mut apply = move |id: &'static str, new_combo: String| {
        let mut sc = prefs.write();
        let Some(slot) = field_mut(&mut sc, id) else { return };
        let old = slot.clone();
        *slot = Some(new_combo.clone());
        let _ = sc.save();
        drop(sc);

        // Live-rekey the registry so the change applies without a restart.
        // (Registrations bind once at mount; the JS dock map re-reads prefs.)
        if let (Some(mut reg), Some(old)) = (registry, old) {
            let old_key = normalize_combo(&old);
            let mut reg = reg.write();
            if let Some(mut meta) = reg.binds.remove(&old_key) {
                meta.keys = new_combo.clone();
                reg.binds.insert(normalize_combo(&new_combo), meta);
            }
        }
    };

    let sc = prefs();
    let query = filter().to_lowercase();
    let groups = ["Global App", "Project & File", "Workspace"];

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
                        value: "{filter}",
                        oninput: move |e| filter.set(e.value()),
                    }
                }

                p { style: "margin: 8px 0 0 0; font-size: 0.75rem; color: var(--fg-muted);",
                    "Click a key chip, then press the new combination. Esc cancels."
                }

                div { class: "mor-shortcuts-grid",
                    for group in groups {
                        div { class: "mor-shortcut-group",
                            h4 { class: "mor-shortcut-group-title", "{group}" }

                            for (id, label, _) in ACTIONS.iter().filter(|(id, label, g)| {
                                *g == group
                                    && (query.is_empty()
                                        || label.to_lowercase().contains(&query)
                                        || field(&sc, id).unwrap_or_default().to_lowercase().contains(&query))
                            }) {
                                {
                                    let id: &'static str = id;
                                    let is_capturing = capturing() == Some(id);
                                    let keys = field(&sc, id).unwrap_or_default();
                                    let display = if is_capturing { "press keys…".to_string() } else { keys };
                                    rsx! {
                                        div { class: "mor-shortcut-row", key: "{id}",
                                            button {
                                                class: "mor-input",
                                                style: format!(
                                                    "width: 150px; text-align: center; font-family: monospace; cursor: pointer; {}",
                                                    if is_capturing { "outline: 2px solid var(--accent);" } else { "" }
                                                ),
                                                onclick: move |_| capturing.set(Some(id)),
                                                onkeydown: move |evt: Event<KeyboardData>| {
                                                    if capturing() != Some(id) { return; }
                                                    evt.prevent_default();
                                                    evt.stop_propagation();
                                                    if evt.key() == dioxus::html::Key::Escape {
                                                        capturing.set(None);
                                                        return;
                                                    }
                                                    if let Some(combo) = combo_from_event(&evt) {
                                                        apply(id, combo);
                                                        capturing.set(None);
                                                    }
                                                },
                                                onblur: move |_| {
                                                    if capturing() == Some(id) { capturing.set(None); }
                                                },
                                                "{display}"
                                            }
                                            div { class: "mor-action-label", "{label}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "mor-shortcut-group",
                        h4 { class: "mor-shortcut-group-title", "Asset Editors (fixed)" }
                        div { class: "mor-shortcut-row",
                            input {
                                class: "mor-input",
                                style: "width: 150px; text-align: center; font-family: monospace; pointer-events: none;",
                                readonly: true,
                                value: "Alt+Left / Right",
                            }
                            div { class: "mor-action-label", "Previous / Next File Tab" }
                        }
                        div { class: "mor-shortcut-row",
                            input {
                                class: "mor-input",
                                style: "width: 150px; text-align: center; font-family: monospace; pointer-events: none;",
                                readonly: true,
                                value: "Ctrl+Shift+1…9",
                            }
                            div { class: "mor-action-label", "Toggle Pinned Dock (activity-bar order)" }
                        }
                    }
                }

                div {
                    style: "margin-top: 20px; display: flex; justify-content: space-between;",
                    button {
                        class: "editor-button",
                        onclick: move |_| {
                            let defaults = ShortcutPrefs::default();
                            for (id, _, _) in ACTIONS {
                                if let Some(d) = field(&defaults, id) {
                                    apply(id, d);
                                }
                            }
                        },
                        "Reset All to Defaults"
                    }
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
