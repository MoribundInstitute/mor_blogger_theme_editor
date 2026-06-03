use dioxus::prelude::*;

use crate::clipboard::copy_to_clipboard;
use crate::config::pages::StaticPagesConfig;
use crate::render::pages::{
    generate_about_html, generate_archive_html, generate_categories_html,
    generate_course_catalog_html, generate_portfolio_html, generate_syllabus_html,
};
use crate::ui::panels::presets_panel::ThemeSignals;

// (tab id, button label)
const TABS: &[(&str, &str)] = &[
    ("Archive", "Archive"),
    ("Directory", "Directory"),
    ("About", "About Me"),
    ("Portfolio", "Portfolio"),
    ("LMS", "Courses"),
];

fn preview_html_for_tab(id: &str, pages: &StaticPagesConfig) -> String {
    match id {
        "Archive" => generate_archive_html(&pages.archive),
        "Directory" => generate_categories_html(&pages.categories),
        "Portfolio" => generate_portfolio_html(&pages.portfolio),
        "About" => generate_about_html(&pages.about),
        "LMS" => generate_course_catalog_html(&pages.lms),
        _ => String::new(),
    }
}

#[component]
pub fn StaticPagesFloatingWindow(
    signals: ThemeSignals,
    mut show_undocked_pages: Signal<bool>,
    mut preview_html: Signal<String>,
    base_preview_html: ReadSignal<String>,
) -> Element {
    rsx! {
        div {
            class: "preset-floating-window",
            style: "position: fixed; top: 120px; left: 380px; width: 400px; max-height: 80vh; background: var(--editor-bg-base); border: 1px solid var(--editor-border-soft); box-shadow: 0 10px 30px rgba(0,0,0,0.5); z-index: 1000; display: flex; flex-direction: column; border-radius: 8px; overflow: hidden;",
            
            div {
                class: "preset-floating-drag-handle",
                style: "padding: 10px 16px; background: var(--editor-bg-panel); border-bottom: 1px solid var(--editor-border-soft); display: flex; justify-content: space-between; align-items: center; cursor: move;",
                
                h3 { style: "margin: 0; font-size: 14px;", "Static Pages" }
                
                button {
                    class: "editor-mini-button",
                    onclick: move |_| show_undocked_pages.set(false),
                    "Dock"
                }
            }
            
            div {
                style: "padding: 16px; overflow-y: auto;",
                StaticPagesPanel {
                    signals,
                    show_undocked_pages,
                    preview_html,
                    base_preview_html,
                }
            }
        }
    }
}

#[component]
pub fn StaticPagesPanel(
    signals: ThemeSignals,
    mut show_undocked_pages: Signal<bool>,
    mut preview_html: Signal<String>,
    base_preview_html: ReadSignal<String>,
) -> Element {
    let mut pages = signals.static_pages;
    let status = use_signal(String::new);
    let mut active_tab = use_signal(|| "Archive");

    // Wrap the selected static page inside the active generated theme preview.
    // This keeps the iframe CSS/fonts/colors intact and mocks Blogger feed calls offline.
    use_effect(move || {
        let base = base_preview_html();
        let pages_snapshot = pages();
        let new_html = preview_html_for_tab(active_tab(), &pages_snapshot);
        preview_html.set(inject_static_page(&base, &new_html));
    });

    rsx! {
        div { class: "editor-panel",
            
            div { class: "editor-row", style: "margin-bottom: 12px;",
                button {
                    class: if show_undocked_pages() { "editor-button editor-button-small editor-button-active" } else { "editor-button editor-button-small" },
                    onclick: move |_| show_undocked_pages.set(!show_undocked_pages()),
                    if show_undocked_pages() { "Dock Pages" } else { "Undock Pages" }
                }
            }

            div { class: "editor-help-text",
                "Select a page template to generate its HTML. Paste this directly into Blogger's Pages editor (HTML View) to automatically match your active theme colors."
            }

            // Tab navigation
            div {
                style: "display: flex; gap: 8px; margin: 20px 0; border-bottom: 1px solid var(--border-color); padding-bottom: 12px; overflow-x: auto;",
                for (id, label) in TABS.iter().copied() {
                    button {
                        key: "{id}",
                        class: "editor-button",
                        onclick: move |_| active_tab.set(id),
                        "{label}"
                    }
                }
            }

            // Active builder canvas
            match active_tab() {
                "Archive" => rsx! {
                    SinglePageBuilder {
                        heading: "Archive Page Settings",
                        title: pages().archive.title,
                        include_in_bundle: pages().archive.include_in_bundle,
                        html: generate_archive_html(&pages().archive),
                        copy_label: "Copy Archive HTML",
                        copied_msg: "Archive HTML copied to clipboard!",
                        status,
                        on_title: move |v| { let mut c = pages(); c.archive.title = v; pages.set(c); },
                        on_toggle_bundle: move |v| { let mut c = pages(); c.archive.include_in_bundle = v; pages.set(c); }
                    }
                },
                "Directory" => rsx! {
                    SinglePageBuilder {
                        heading: "Directory Settings",
                        title: pages().categories.title,
                        include_in_bundle: pages().categories.include_in_bundle,
                        html: generate_categories_html(&pages().categories),
                        copy_label: "Copy Directory HTML",
                        copied_msg: "Directory HTML copied to clipboard!",
                        status,
                        on_title: move |v| { let mut c = pages(); c.categories.title = v; pages.set(c); },
                        on_toggle_bundle: move |v| { let mut c = pages(); c.categories.include_in_bundle = v; pages.set(c); }
                    }
                },
                "Portfolio" => rsx! {
                    SinglePageBuilder {
                        heading: "Art Portfolio Settings",
                        title: pages().portfolio.title,
                        include_in_bundle: pages().portfolio.include_in_bundle,
                        html: generate_portfolio_html(&pages().portfolio),
                        copy_label: "Copy Portfolio HTML",
                        copied_msg: "Portfolio HTML copied to clipboard!",
                        status,
                        on_title: move |v| { let mut c = pages(); c.portfolio.title = v; pages.set(c); },
                        on_toggle_bundle: move |v| { let mut c = pages(); c.portfolio.include_in_bundle = v; pages.set(c); }
                    }
                },
                "About" => rsx! { AboutBuilder { config: pages, status } },
                "LMS" => rsx! { LmsBuilder { config: pages, status } },
                _ => rsx! {}
            }

            if !status().is_empty() {
                div {
                    class: "export-status",
                    style: "margin-top: 15px; color: #3fb950; font-weight: bold;",
                    "{status}"
                }
            }
        }
    }
}

// ---------------------------------------------------------
// Shared building blocks
// ---------------------------------------------------------

/// A labelled text input or textarea wired to an `on_change` handler.
#[component]
fn TextField(
    label: String,
    value: String,
    #[props(default)] multiline: bool,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        label {
            span { class: "editor-label-text", "{label}" }
            if multiline {
                textarea {
                    class: "editor-textarea", rows: 4, value: "{value}",
                    oninput: move |evt| on_change.call(evt.value()),
                }
            } else {
                input {
                    class: "editor-input", r#type: "text", value: "{value}",
                    oninput: move |evt| on_change.call(evt.value()),
                }
            }
        }
    }
}

/// A button that copies `html` and reports `copied_msg` to the shared status line.
#[component]
fn CopyButton(
    html: String,
    status: Signal<String>,
    copied_msg: String,
    label: String,
) -> Element {
    let mut status = status;
    rsx! {
        button {
            class: "editor-button",
            onclick: move |_| {
                copy_to_clipboard(html.clone());
                status.set(copied_msg.clone());
            },
            "{label}"
        }
    }
}

// ---------------------------------------------------------
// Builders
// ---------------------------------------------------------

/// Archive / Directory / Portfolio: title field, bundle checkbox, copy button.
#[component]
fn SinglePageBuilder(
    heading: String,
    title: String,
    include_in_bundle: bool,
    html: String,
    copy_label: String,
    copied_msg: String,
    status: Signal<String>,
    on_title: EventHandler<String>,
    on_toggle_bundle: EventHandler<bool>,
) -> Element {
    rsx! {
        div { class: "editor-field-group",
            h4 { "{heading}" }
            
            label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px; font-size: 13px;",
                input {
                    r#type: "checkbox",
                    checked: include_in_bundle,
                    onchange: move |evt| on_toggle_bundle.call(evt.checked()),
                }
                " Include in ZIP Bundle"
            }

            TextField {
                label: "Title",
                value: title,
                on_change: move |v| on_title.call(v),
            }
            CopyButton { html, status, copied_msg, label: copy_label }
        }
    }
}

#[component]
fn AboutBuilder(
    config: Signal<StaticPagesConfig>,
    status: Signal<String>,
) -> Element {
    let mut config = config;
    let html = generate_about_html(&config().about);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Profile & About Settings" }
            
            label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px; font-size: 13px;",
                input {
                    r#type: "checkbox",
                    checked: config().about.include_in_bundle,
                    onchange: move |evt| { let mut c = config(); c.about.include_in_bundle = evt.checked(); config.set(c); },
                }
                " Include in ZIP Bundle"
            }

            TextField {
                label: "Profile Image URL",
                value: config().about.profile_image_url,
                on_change: move |v| { let mut c = config(); c.about.profile_image_url = v; config.set(c); },
            }
            TextField {
                label: "Biography",
                value: config().about.bio_text,
                multiline: true,
                on_change: move |v| { let mut c = config(); c.about.bio_text = v; config.set(c); },
            }
            CopyButton {
                html, status,
                copied_msg: "About HTML copied to clipboard!",
                label: "Copy About HTML",
            }
        }
    }
}

#[component]
fn LmsBuilder(
    config: Signal<StaticPagesConfig>,
    status: Signal<String>,
) -> Element {
    let mut config = config;
    let catalog_html = generate_course_catalog_html(&config().lms);
    let syllabus_html = generate_syllabus_html(&config().lms);

    rsx! {
        div { class: "editor-field-group",
            h4 { "Learning Management System" }
            
            label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; margin-bottom: 6px; font-size: 13px;",
                input {
                    r#type: "checkbox",
                    checked: config().lms.include_catalog_in_bundle,
                    onchange: move |evt| { let mut c = config(); c.lms.include_catalog_in_bundle = evt.checked(); config.set(c); },
                }
                " Include Catalog in ZIP Bundle"
            }

            label { class: "editor-checkbox-label", style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px; font-size: 13px;",
                input {
                    r#type: "checkbox",
                    checked: config().lms.include_syllabus_in_bundle,
                    onchange: move |evt| { let mut c = config(); c.lms.include_syllabus_in_bundle = evt.checked(); config.set(c); },
                }
                " Include Syllabus in ZIP Bundle"
            }

            TextField {
                label: "Course Title",
                value: config().lms.course_title,
                on_change: move |v| { let mut c = config(); c.lms.course_title = v; config.set(c); },
            }
            div {
                style: "display: flex; gap: 12px; margin-top: 16px;",
                CopyButton {
                    html: catalog_html, status,
                    copied_msg: "Course Catalog HTML copied to clipboard!",
                    label: "Copy Master Catalog",
                }
                CopyButton {
                    html: syllabus_html, status,
                    copied_msg: "Course Syllabus HTML copied to clipboard!",
                    label: "Copy Syllabus Page",
                }
            }
        }
    }
}
/// Wraps static HTML in the master theme CSS and mocks offline Blogger feed calls.
pub fn inject_static_page(base_html: &str, static_html: &str) -> String {
    let mock_fetch = r##"<script>
    const _origFetch = window.fetch;
    window.fetch = async function(url, opts) {
        if (typeof url === 'string' && url.includes('/feeds/')) {
            return {
                ok: true,
                json: async () => ({
                    feed: {
                        openSearch$totalResults: { $t: "3" },
                        entry: [
                            {
                                title: { $t: "Archive Feed Intercepted" },
                                link: [{ rel: "alternate", href: "#" }],
                                summary: { $t: "Offline preview routing successful. Theme layout nominal." },
                                published: { $t: new Date().toISOString() },
                                category: [{ term: "System" }]
                            },
                            {
                                title: { $t: "Patch Notes v1.2" },
                                link: [{ rel: "alternate", href: "#" }],
                                summary: { $t: "Guild UI updated. Potions nerfed." },
                                published: { $t: "2025-06-03T10:00:00Z" },
                                category: [{ term: "Updates" }]
                            }
                        ]
                    }
                })
            };
        }

        return _origFetch(url, opts);
    };
    </script>"##;

    // Visually hide default blog content before the static page is mounted.
    let hide_css = "<style>main, .mor-main, #main, .main-section { display: none !important; }</style>";
    let head_injected = base_html.replace("<head>", &format!("<head>\n{}\n{}", mock_fetch, hide_css));

    let template_html = format!(
        r#"
    <template id="mor-static-injector">
        {}
    </template>
    <script>
    document.addEventListener('DOMContentLoaded', () => {{
        const target = document.querySelector('main, .mor-main, #main, .main-section, .mor-workspace');
        const template = document.getElementById('mor-static-injector');

        if (target && template) {{
            target.innerHTML = '';
            target.appendChild(template.content.cloneNode(true));
            target.style.display = 'block';

            // cloneNode does not execute script tags. Recreate them manually.
            target.querySelectorAll('script').forEach(oldScript => {{
                const newScript = document.createElement('script');
                Array.from(oldScript.attributes).forEach(attr => newScript.setAttribute(attr.name, attr.value));
                newScript.appendChild(document.createTextNode(oldScript.innerHTML));
                oldScript.parentNode.replaceChild(newScript, oldScript);
            }});
        }}
    }});
    </script>
    "#,
        static_html
    );

    head_injected.replace("</body>", &format!("{}\n</body>", template_html))
}
