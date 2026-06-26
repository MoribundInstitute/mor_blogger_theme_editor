use crate::app::state::{LayoutState, RenderState, ThemeState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::panels::theme_palette::static_pages_panel::inject_static_page;
use crate::ui::workspace::layout::{apply_preview_viewport, clamp_preview_width, PreviewViewport};
use crate::ui::workspace::preview_canvas::PreviewCanvas;
use dioxus::prelude::*;
use mor_blogger_core::render::pages::{
    generate_about_html, generate_archive_html, generate_categories_html,
    generate_course_catalog_html, generate_portfolio_html,
};
use mor_blogger_core::utils::fs_bridge;

#[component]
pub fn StaticPageEditor(preview_html: Signal<String>) -> Element {
    let theme = use_context::<ThemeState>();
    let render = use_context::<RenderState>();
    let mut layout = use_context::<LayoutState>();

    let active_page = layout.active_static_page;
    let mut raw_html_signal = use_signal(String::new);
    let mut save_status = use_signal(String::new);
    let mut is_editor_takeover = use_signal(|| false);
    let mut is_preview_takeover = use_signal(|| false);

    let page_name = active_page().clone().unwrap_or_default();

    // Synchronize textarea raw HTML when the active page changes
    use_effect(move || {
        if let Some(ref name) = active_page() {
            let pages = (theme.signals.static_pages)();
            let default_html = match name.as_str() {
                "Archive" => generate_archive_html(&pages.archive),
                "Directory" => generate_categories_html(&pages.categories),
                "Portfolio" => generate_portfolio_html(&pages.portfolio),
                "About" => generate_about_html(&pages.about),
                "LMS" => generate_course_catalog_html(&pages.lms),
                _ => String::new(),
            };
            raw_html_signal.set(default_html.clone());
            let base = (render.preview_html)();
            preview_html.set(inject_static_page(&base, &default_html));
            save_status.set(String::new());
        } else {
            raw_html_signal.set(String::new());
            save_status.set(String::new());
        }
    });

    // Reactively update preview HTML when raw_html_signal edits occur
    use_effect(move || {
        let val = raw_html_signal();
        let base = (render.preview_html)();
        preview_html.set(inject_static_page(&base, &val));
    });

    // Save buffer to this page's canonical custom override. Copy closure so it's
    // reusable in both the pane header and the editor-takeover bar.
    let save_page = move |_: Event<MouseData>| {
        let Some(name) = active_page() else { return; };
        let content = raw_html_signal();
        let mut status = save_status;
        match fs_bridge::save_custom_page(&name, &content) {
            Ok(path) => status.set(format!(
                "Saved to {}",
                path.file_name().and_then(|f| f.to_str()).unwrap_or("file")
            )),
            Err(e) => status.set(format!("Save failed: {}", e)),
        }
    };
    // Save buffer to a user-named new HTML file (lets you spin off variants).
    let save_page_as_new = move |_: Event<MouseData>| {
        let Some(name) = active_page() else { return; };
        let content = raw_html_signal();
        let default_name = format!("{}_custom.html", name.to_lowercase());
        let start_dir = fs_bridge::pages_root();
        let mut status = save_status;
        spawn(async move {
            let mut dlg = rfd::AsyncFileDialog::new()
                .add_filter("Static page HTML", &["html"])
                .set_file_name(default_name);
            if let Some(dir) = start_dir {
                dlg = dlg.set_directory(dir);
            }
            if let Some(handle) = dlg.save_file().await {
                let path = handle.path().to_path_buf();
                match std::fs::write(&path, &content) {
                    Ok(_) => status.set(format!("Saved new page → {}", path.display())),
                    Err(e) => status.set(format!("Save failed: {}", e)),
                }
            }
        });
    };

    rsx! {
        if is_editor_takeover() {
            // ── Editor Takeover ─ Full-viewport focused HTML editor ───────
            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",
                div {
                    style: "flex-shrink: 0; display: flex; align-items: center; gap: 8px; padding: 6px 12px; background: var(--bg-elevated); border-bottom: 1px solid var(--editor-border);",
                    span {
                        style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);",
                        "{page_name} Page Content"
                    }
                    div { style: "flex: 1;" }
                    button {
                        class: "editor-mini-button",
                        title: "Save to this page's custom override",
                        onclick: save_page,
                        "Save"
                    }
                    button {
                        class: "editor-mini-button",
                        title: "Save the buffer as a new named HTML file",
                        onclick: save_page_as_new,
                        "Save as New"
                    }
                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 2px;" }
                    button {
                        class: "editor-mini-button",
                        onclick: move |_| is_editor_takeover.set(false),
                        "Editor ×"
                    }
                }
                div {
                    style: "flex: 1; min-height: 0; display: flex; flex-direction: column;",
                    CodeEditor {
                        value: (raw_html_signal)(),
                        mode: "html".to_string(),
                        minimap_key: Some("static_page".to_string()),
                        on_change: move |new_val| { raw_html_signal.set(new_val); }
                    }
                }
            }
        } else if is_preview_takeover() {
            // ── Preview Takeover ─ Full-viewport isolated page preview ─────
            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",
                div {
                    style: "flex-shrink: 0; display: flex; align-items: center; gap: 6px; padding: 6px 12px; background: var(--bg-elevated); border-bottom: 1px solid var(--editor-border);",
                    button {
                        class: if (layout.preview_viewport)() == PreviewViewport::Desktop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Desktop, layout.preview_width); layout.preview_viewport.set(PreviewViewport::Desktop); },
                        "Desktop"
                    }
                    button {
                        class: if (layout.preview_viewport)() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Laptop, layout.preview_width); layout.preview_viewport.set(PreviewViewport::Laptop); },
                        "Laptop"
                    }
                    button {
                        class: if (layout.preview_viewport)() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Tablet, layout.preview_width); layout.preview_viewport.set(PreviewViewport::Tablet); },
                        "Tablet"
                    }
                    button {
                        class: if (layout.preview_viewport)() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Phone, layout.preview_width); layout.preview_viewport.set(PreviewViewport::Phone); },
                        "Phone"
                    }
                    button {
                        class: if (layout.preview_viewport)() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Fit, layout.preview_width); layout.preview_viewport.set(PreviewViewport::Fit); },
                        "Fit"
                    }
                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 2px;" }
                    label {
                        class: "preview-width-control",
                        span { class: "preview-width-label", "Width" }
                        input {
                            class: "preview-width-input",
                            r#type: "number", min: "240", max: "2400", step: "10",
                            value: "{(layout.preview_width)()}",
                            oninput: move |evt| {
                                if let Ok(w) = evt.value().parse::<u32>() {
                                    layout.preview_width.set(clamp_preview_width(w));
                                    layout.preview_viewport.set(PreviewViewport::Custom);
                                }
                            }
                        }
                    }
                    div { style: "flex: 1;" }
                    button {
                        class: "editor-mini-button",
                        onclick: move |_| is_preview_takeover.set(false),
                        "Editor ×"
                    }
                }
                PreviewCanvas {
                    preview_viewport: layout.preview_viewport,
                    preview_width: layout.preview_width,
                    preview_html: preview_html(),
                }
            }
        } else {
        div {
            class: "export-viewport",
            style: "display: flex; flex-direction: row; flex: 1; min-height: 0; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel); height: 100%;",

            if active_page().is_none() {
                // No active page state
                div {
                    style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; color: var(--fg-muted); font-family: var(--font-mono); text-align: center; gap: 16px;",
                    div {
                        style: "font-size: 2.5rem; filter: drop-shadow(0 0 10px rgba(var(--accent-rgb), 0.2));",
                        "📄"
                    }
                    h3 { style: "margin: 0; font-size: 1.1rem; color: var(--fg-base);", "Static Page Editor" }
                    p {
                        style: "margin: 0; font-size: 0.85rem; max-width: 420px; line-height: 1.5;",
                        "Select a static page layout from the Left Dock (Theme Palette > Static Pages) to start editing its raw HTML content live."
                    }
                }
            } else {
                // Split Editor View
                // Left Pane - HTML editor (40% width)
                div {
                    style: "width: 40%; flex-shrink: 0; border-right: 1px solid var(--editor-border); display: flex; flex-direction: column; min-width: 0;",
                    div {
                        class: "editor-pane",
                        style: "display: flex; flex-direction: column; width: 100%; height: 100%; background: var(--bg-base); border-radius: 6px; overflow: hidden; border: 1px solid var(--border-color);",

                        div {
                            class: "editor-pane-header",
                            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: rgba(0,0,0,0.2); border-bottom: 1px solid var(--border-color); flex-shrink: 0;",
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);", "{page_name} Page Content" }
                                span {
                                    style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                                    "Live Preview"
                                }
                            }
                            div {
                                style: "display: flex; align-items: center; gap: 6px;",
                                button {
                                    class: "editor-mini-button",
                                    title: "Expand the editor to a full-viewport focused stage",
                                    onclick: move |_| is_editor_takeover.set(true),
                                    "Takeover"
                                }
                                button {
                                    class: "editor-mini-button",
                                    title: "Save current HTML buffer to this page's custom override",
                                    onclick: save_page,
                                    "Save"
                                }
                                button {
                                    class: "editor-mini-button",
                                    title: "Save the buffer as a new named HTML file",
                                    onclick: save_page_as_new,
                                    "Save as New"
                                }
                            }
                        }

                        if !save_status().is_empty() {
                            div {
                                style: "padding: 4px 12px; font-size: 0.75rem; color: var(--editor-accent-warm); background: rgba(0,0,0,0.15); font-family: var(--font-mono); border-bottom: 1px solid var(--editor-border-soft); display: flex; justify-content: space-between; align-items: center; flex-shrink: 0;",
                                span { "{save_status()}" }
                                button {
                                    style: "background: none; border: none; color: var(--fg-muted); cursor: pointer; font-size: 0.9rem; padding: 0 2px;",
                                    onclick: move |_| save_status.set(String::new()),
                                    "×"
                                }
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",
                            CodeEditor {
                                value: (raw_html_signal)(),
                                mode: "html".to_string(),
                                minimap_key: Some("static_page".to_string()),
                                on_change: move |new_val| {
                                    raw_html_signal.set(new_val);
                                }
                            }
                        }
                    }
                }

                // Right Pane - Live Preview (60% width)
                div {
                    style: "width: 60%; flex-grow: 1; display: flex; flex-direction: column; min-width: 0;",
                    div {
                        style: "padding: 8px 12px; border-bottom: 1px solid var(--editor-border-soft); background: rgba(0,0,0,0.2); display: flex; align-items: center; justify-content: space-between;",
                        span {
                            style: "font-size: 0.8rem; color: var(--accent); font-family: var(--font-mono);",
                            "Live Preview"
                        }
                        button {
                            class: "editor-mini-button",
                            onclick: move |_| is_preview_takeover.set(true),
                            "Takeover"
                        }
                    }
                    PreviewCanvas {
                        preview_viewport: layout.preview_viewport,
                        preview_width: layout.preview_width,
                        preview_html: preview_html(),
                    }
                }
            }
        }
        }
    }
}
