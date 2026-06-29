use dioxus::prelude::*;
use std::path::{Path, PathBuf};

/// Standardized location for hand-editable component TOML snippets, grouped by
/// type (e.g. `components/buttons/`). Created on demand.
pub fn components_dir(sub: &str) -> PathBuf {
    let base = mor_blogger_core::config::prefs::editor_prefs_path()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("components")
        .join(sub);
    let _ = std::fs::create_dir_all(&base);
    base
}

fn open_in_file_manager(path: &Path) {
    let cmd = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "explorer"
    } else {
        "xdg-open"
    };
    let _ = std::process::Command::new(cmd).arg(path).spawn();
}

const BTN: &str = "padding: 5px 10px; font-size: 12px; background: #2C2C2E; border: 1px solid #3A3A3C; color: #E5E5EA; border-radius: 4px; cursor: pointer;";

/// Power-user TOML view shared by the advanced dialogs. Edits a local buffer;
/// Apply hands the text back to the parent (which parses into its own config
/// type). Load/Save/Open Folder operate on the standardized `components/<sub>/`.
#[component]
pub fn TomlQuickEdit(toml_text: String, subfolder: String, on_apply: EventHandler<String>) -> Element {
    let mut buffer = use_signal(|| toml_text.clone());
    let mut status = use_signal(String::new);
    let dir = components_dir(&subfolder);

    // Captured clones for the closures.
    let reload_src = toml_text.clone();
    let dir_save = dir.clone();
    let dir_open = dir.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
                button { style: "{BTN}", onclick: move |_| { on_apply.call(buffer()); status.set("Applied".into()); }, "Apply" }
                button { style: "{BTN}", onclick: move |_| { buffer.set(reload_src.clone()); status.set("Reloaded from controls".into()); }, "↻ Reload" }
                button { style: "{BTN}",
                    onclick: move |_| {
                        if let Some(p) = rfd::FileDialog::new().add_filter("TOML", &["toml"]).set_directory(&dir).pick_file() {
                            match std::fs::read_to_string(&p) {
                                Ok(c) => { buffer.set(c.clone()); on_apply.call(c); status.set(format!("Loaded {}", p.display())); }
                                Err(e) => status.set(format!("Load failed: {e}")),
                            }
                        }
                    },
                    "Load…"
                }
                button { style: "{BTN}",
                    onclick: move |_| {
                        if let Some(p) = rfd::FileDialog::new().add_filter("TOML", &["toml"]).set_directory(&dir_save).set_file_name("untitled.toml").save_file() {
                            match std::fs::write(&p, buffer()) {
                                Ok(_) => status.set(format!("Saved {}", p.display())),
                                Err(e) => status.set(format!("Save failed: {e}")),
                            }
                        }
                    },
                    "Save…"
                }
                button { style: "{BTN}", onclick: move |_| open_in_file_manager(&dir_open), "Open Folder" }
            }
            textarea {
                style: "width: 100%; min-height: 150px; background: #1C1C1E; border: 1px solid #3A3A3C; color: #d6d6d6; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 12px; resize: vertical;",
                spellcheck: "false",
                value: "{buffer}",
                oninput: move |e| buffer.set(e.value()),
            }
            if !status().is_empty() {
                span { style: "font-size: 11px; color: #8e8e93;", "{status}" }
            }
        }
    }
}
