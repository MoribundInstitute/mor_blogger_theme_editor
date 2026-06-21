use crate::app::state::{LayoutState, RenderState, ThemeState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::panels::theme_palette::static_pages_panel::inject_static_page;
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
    let layout = use_context::<LayoutState>();

    let active_page = layout.active_static_page;
    let mut raw_html_signal = use_signal(String::new);
    let mut save_status = use_signal(String::new);

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

    rsx! {
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
                // Left Pane - XML/HTML Text Area (40% width)
                div {
                    style: "width: 40%; flex-shrink: 0; border-right: 1px solid var(--editor-border); display: flex; flex-direction: column; min-width: 0;",

                    // Top header row with Save Button & Status
                    div {
                        style: "padding: 8px 12px; border-bottom: 1px solid var(--editor-border-soft); background: rgba(0,0,0,0.2); display: flex; align-items: center; gap: 8px; justify-content: space-between;",
                        span {
                            style: "font-size: 0.8rem; color: var(--editor-accent-warm); font-family: var(--font-mono); font-weight: bold;",
                            "{page_name} Page Content"
                        }
                        div {
                            style: "display: flex; align-items: center; gap: 8px;",
                            button {
                                class: "editor-mini-button",
                                title: "Save customized HTML back to system pages directory",
                                onclick: move |_| {
                                    let name = page_name.clone();
                                    let content = raw_html_signal();
                                    match fs_bridge::save_custom_page(&name, &content) {
                                        Ok(path) => save_status.set(format!("Saved to {}", path.file_name().and_then(|f| f.to_str()).unwrap_or("file"))),
                                        Err(e) => save_status.set(format!("Save failed: {}", e)),
                                    }
                                },
                                "Save Page"
                            }
                        }
                    }

                    // Success / Error status banner
                    if !save_status().is_empty() {
                        div {
                            style: "padding: 6px 12px; font-size: 0.75rem; color: var(--editor-accent-warm); background: rgba(0,0,0,0.15); font-family: var(--font-mono); border-bottom: 1px solid var(--editor-border-soft); display: flex; justify-content: space-between; align-items: center;",
                            span { "{save_status}" }
                            button {
                                style: "background: none; border: none; color: var(--fg-muted); cursor: pointer; font-size: 0.9rem; padding: 0 2px;",
                                onclick: move |_| save_status.set(String::new()),
                                "×"
                            }
                        }
                    }

                    // Text Area Editor
                    CodeEditor {
                        value: raw_html_signal,
                        mode: "html".to_string(),
                        on_change: move |new_val| {
                            raw_html_signal.set(new_val);
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
