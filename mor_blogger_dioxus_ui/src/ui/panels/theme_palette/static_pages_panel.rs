use dioxus::prelude::*;

use crate::app::theme_signals::ThemeSignals;
use crate::utils::clipboard::copy_to_clipboard;
use mor_blogger_core::config::pages::StaticPagesConfig;
use mor_blogger_core::render::pages::{
    generate_about_html, generate_archive_html, generate_categories_html,
    generate_course_catalog_html, generate_my_courses_html, generate_portfolio_html,
    generate_syllabus_html,
};

// (tab id, button label)
pub const TABS: &[(&str, &str)] = &[
    ("Archive", "Archive"),
    ("Directory", "Directory"),
    ("About", "About Me"),
    ("Portfolio", "Portfolio"),
    ("LMS", "Courses"),
    ("MyCourses", "My Courses"),
    ("Community", "Pasteboard"), // <-- NEW TAB ADDED HERE
];

fn preview_html_for_tab(id: &str, pages: &StaticPagesConfig) -> String {
    match id {
        "Archive" => generate_archive_html(&pages.archive),
        "Directory" => generate_categories_html(&pages.categories),
        "Portfolio" => generate_portfolio_html(&pages.portfolio),
        "About" => generate_about_html(&pages.about),
        "LMS" => generate_course_catalog_html(&pages.lms),
        "MyCourses" => generate_my_courses_html(&pages.lms),
        _ => String::new(),
    }
}

// Community hub: the app ships the templates above as defaults and links out
// here for layouts the community published. The Blogger JSON feed *is* the
// hub index — each published post is one static page (title + raw HTML body).
pub const COMMUNITY_HUB_URL: &str = "https://morpages.blogspot.com/";
const COMMUNITY_FEED_URL: &str =
    "https://morpages.blogspot.com/feeds/posts/default?alt=json&max-results=150";

/// One community-contributed static page pulled from the hub.
#[derive(Clone, PartialEq)]
pub struct CommunityPage {
    pub title: String,
    pub html: String,
}

#[derive(serde::Deserialize)]
struct FeedRoot {
    feed: FeedBody,
}
#[derive(serde::Deserialize)]
struct FeedBody {
    #[serde(default)]
    entry: Vec<FeedEntry>,
}
#[derive(serde::Deserialize)]
struct FeedEntry {
    title: FeedText,
    content: Option<FeedText>,
}
#[derive(serde::Deserialize)]
struct FeedText {
    #[serde(rename = "$t")]
    t: String,
}

/// Fetch the community static-page catalog from the Blogger JSON feed.
/// Empty Ok(vec) means the hub has nothing published yet.
pub async fn fetch_community_pages(url: &str) -> Result<Vec<CommunityPage>, String> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| format!("read failed: {e}"))?;
    let root: FeedRoot = serde_json::from_str(&body).map_err(|e| format!("bad feed: {e}"))?;
    Ok(root
        .feed
        .entry
        .into_iter()
        .filter_map(|e| {
            e.content.map(|c| CommunityPage {
                title: e.title.t,
                html: c.t,
            })
        })
        .collect())
}

/// Fetch a single raw HTML page from any URL (a community repo file, gist, etc.).
pub async fn fetch_raw_page(url: &str) -> Result<String, String> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.text().await.map_err(|e| format!("read failed: {e}"))
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
    let layout_state = use_context::<crate::app::state::LayoutState>();
    let mut pages = signals.static_pages;
    let status = use_signal(String::new);
    let mut active_tab = use_signal(|| "Archive");

    // State specifically for the Community Pasteboard
    let mut custom_html = use_signal(|| String::new());

    // Community hub catalog fetched from the Blogger feed (lazily loaded).
    let mut community_index = use_signal(Vec::<CommunityPage>::new);
    let mut fetch_status = use_signal(String::new);
    let mut remote_url = use_signal(String::new);

    // Wrap the selected static page inside the active generated theme preview.
    // This keeps the iframe CSS/fonts/colors intact and mocks Blogger feed calls offline.
    use_effect(move || {
        // Prevent auto-reloading on every keystroke if user is typing in the pasteboard
        if active_tab() == "Community" {
            return;
        }
        if *layout_state.center_view.read() == crate::app::state::CenterView::StaticPageEditor {
            return;
        }

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
                style: "display: flex; align-items: center; gap: 6px; margin: 20px 0; border-bottom: 1px solid var(--border-color); padding-bottom: 12px;",

                button {
                    class: "editor-mini-button",
                    style: "padding: 6px 4px; font-size: 10px;",
                    title: "Scroll Left",
                    onclick: move |_| {
                        let _ = dioxus::document::eval(
                            "document.getElementById('pages-scroll-tabs').scrollBy({ left: -150, behavior: 'smooth' });"
                        );
                    },
                    "◀"
                }

                div {
                    id: "pages-scroll-tabs",
                    style: "display: flex; gap: 8px; overflow-x: auto; flex: 1; padding-bottom: 4px;",

                    for (id, label) in TABS.iter().copied() {
                        button {
                            key: "{id}",
                            class: if active_tab() == id { "editor-button editor-button-active" } else { "editor-button" },
                            onclick: {
                                let id = id;
                                move |_| {
                                    active_tab.set(id);
                                    let mut ls = layout_state;
                                    ls.enter_workspace(crate::app::state::CenterView::StaticPageEditor);
                                    ls.active_static_page.set(Some(id.to_string()));

                                    let base = base_preview_html();
                                    let pages_snapshot = pages();
                                    let new_html = preview_html_for_tab(id, &pages_snapshot);
                                    preview_html.set(inject_static_page(&base, &new_html));
                                }
                            },
                            "{label}"
                        }
                    }
                }

                button {
                    class: "editor-mini-button",
                    style: "padding: 6px 4px; font-size: 10px;",
                    title: "Scroll Right",
                    onclick: move |_| {
                        let _ = dioxus::document::eval(
                            "document.getElementById('pages-scroll-tabs').scrollBy({ left: 150, behavior: 'smooth' });"
                        );
                    },
                    "▶"
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
                "MyCourses" => rsx! {
                    div { class: "editor-field-group",
                        h4 { "My Courses Dashboard" }
                        div { class: "editor-help-text",
                            "A local-first student dashboard. Progress and stats hydrate from the visitor's own localStorage on page load — no server required."
                        }
                        CopyButton {
                            html: generate_my_courses_html(&pages().lms),
                            status,
                            copied_msg: "My Courses dashboard HTML copied to clipboard!",
                            label: "Copy My Courses HTML",
                        }
                    }
                },
                "Community" => rsx! {
                    div { class: "editor-field-group",
                        h4 { "Community Hub" }
                        div { class: "editor-help-text",
                            "Import a static page from the hub, a URL, or by pasting raw HTML below. The engine wraps it in your active theme's CSS variables."
                        }

                        // Import any raw HTML page by URL (community repo file, gist, blog post).
                        div { class: "editor-field-group",
                            label { class: "editor-field-label", "Remote HTML URL" }
                            div { class: "editor-row-stretch",
                                input {
                                    class: "editor-field editor-flex-1", r#type: "text",
                                    placeholder: "https://.../page.html", value: "{remote_url}",
                                    oninput: move |evt| remote_url.set(evt.value()),
                                }
                                button {
                                    class: "editor-button",
                                    onclick: move |_| async move {
                                        // Reuse the theme importer's normalizer so a plain
                                        // github.com/.../blob/... link resolves to its raw file.
                                        let url = crate::ui::panels::theme_palette::presets::importers::normalize_preset_url(&remote_url());
                                        if url.is_empty() { fetch_status.set("Paste a URL first.".to_string()); return; }
                                        fetch_status.set("Importing…".to_string());
                                        match fetch_raw_page(&url).await {
                                            Ok(html) => { custom_html.set(html); fetch_status.set("Imported page from URL.".to_string()); }
                                            Err(e) => fetch_status.set(format!("Import failed: {e}")),
                                        }
                                    },
                                    "Import URL"
                                }
                            }
                        }

                        div { style: "display: flex; gap: 8px; margin-bottom: 10px; align-items: center; flex-wrap: wrap;",
                            button {
                                class: "editor-button editor-button-small",
                                onclick: move |_| async move {
                                    fetch_status.set("Loading…".to_string());
                                    match fetch_community_pages(COMMUNITY_FEED_URL).await {
                                        Ok(pages) if pages.is_empty() => {
                                            community_index.set(Vec::new());
                                            fetch_status.set("No community pages published yet.".to_string());
                                        }
                                        Ok(pages) => {
                                            fetch_status.set(format!("{} community page(s).", pages.len()));
                                            community_index.set(pages);
                                        }
                                        Err(e) => fetch_status.set(format!("Fetch failed: {e}")),
                                    }
                                },
                                "Load Community Pages"
                            }
                            button {
                                class: "editor-button editor-button-small",
                                title: "Open the static-page compendium (Blogger) in your browser",
                                onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMMUNITY_HUB_URL).spawn(); },
                                "Static Page Compendium (Blogger) ↗"
                            }
                        }

                        if !fetch_status().is_empty() {
                            div { class: "editor-help-text", style: "margin-bottom: 8px;", "{fetch_status}" }
                        }

                        for page in community_index() {
                            button {
                                key: "{page.title}",
                                class: "editor-button editor-button-small",
                                style: "display: block; width: 100%; text-align: left; margin-bottom: 4px;",
                                onclick: {
                                    let html = page.html.clone();
                                    move |_| custom_html.set(html.clone())
                                },
                                "{page.title}"
                            }
                        }

                        textarea {
                            class: "editor-textarea",
                            style: "min-height: 200px; font-family: monospace; font-size: 11px;",
                            placeholder: "",
                            value: "{custom_html}",
                            oninput: move |evt| custom_html.set(evt.value()),
                        }

                        div { style: "display: flex; gap: 12px; margin-top: 16px;",
                            button {
                                class: "editor-button editor-button-active",
                                onclick: move |_| {
                                    let base = base_preview_html();
                                    preview_html.set(inject_static_page(&base, &custom_html()));
                                },
                                "Test in Preview Monitor"
                            }
                            CopyButton {
                                html: custom_html(),
                                status,
                                copied_msg: "Community Template copied to clipboard!".to_string(),
                                label: "Copy Final HTML".to_string(),
                            }
                        }
                    }
                },
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
fn CopyButton(html: String, status: Signal<String>, copied_msg: String, label: String) -> Element {
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
fn AboutBuilder(config: Signal<StaticPagesConfig>, status: Signal<String>) -> Element {
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
fn LmsBuilder(config: Signal<StaticPagesConfig>, status: Signal<String>) -> Element {
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

    let head_injected = base_html.replace("<head>", &format!("<head>\n{}", mock_fetch));

    let template_html = format!(
        r#"
    <!-- Static-page mode: a Blogger Page renders as focused content, not the
         blog's sidebar layout. Collapse the workspace to a single full-width
         column so the page reads like a real static page. -->
    <style id="mor-static-page-mode">
        #panel-left, #panel-right {{ display: none !important; }}
        .mor-workspace {{ display: block !important; }}
        .canvas-core {{ width: 100% !important; max-width: 100% !important; }}
    </style>
    <template id="mor-static-injector">
        {}
    </template>
    <script>
    (function injectStaticPage() {{
        // The iframe reloads fresh HTML each time, but if DOMContentLoaded has
        // already fired by the time this runs, the listener never triggers.
        // Run now if the DOM is ready, otherwise wait for it.
        const run = () => {{
            const target = document.querySelector('.canvas-content');
            const template = document.getElementById('mor-static-injector');
            if (!target || !template) return;

            target.innerHTML = '';
            target.appendChild(template.content.cloneNode(true));

            // cloneNode does not execute script tags. Recreate them manually.
            target.querySelectorAll('script').forEach(oldScript => {{
                const newScript = document.createElement('script');
                Array.from(oldScript.attributes).forEach(attr => newScript.setAttribute(attr.name, attr.value));
                newScript.appendChild(document.createTextNode(oldScript.innerHTML));
                oldScript.parentNode.replaceChild(newScript, oldScript);
            }});
        }};
        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', run);
        }} else {{
            run();
        }}
    }})();
    </script>
    "#,
        static_html
    );

    head_injected.replace("</body>", &format!("{}\n</body>", template_html))
}

#[cfg(test)]
mod community_feed_tests {
    use super::*;

    fn parse(body: &str) -> Vec<CommunityPage> {
        let root: FeedRoot = serde_json::from_str(body).unwrap();
        root.feed
            .entry
            .into_iter()
            .filter_map(|e| e.content.map(|c| CommunityPage { title: e.title.t, html: c.t }))
            .collect()
    }

    #[test]
    fn empty_feed_has_no_entry_key() {
        // Blogger omits "entry" entirely when 0 posts — must not error.
        assert!(parse(r#"{"feed":{}}"#).is_empty());
    }

    #[test]
    fn inject_targets_real_content_container() {
        // The injector must target the element that actually exists in the
        // preview (.canvas-content), not the long-dead #mor-content-target id,
        // or the static page silently never renders.
        let out = inject_static_page("<html><head></head><body></body></html>", "<p>HELLO</p>");
        assert!(out.contains(".canvas-content"), "must target the real container");
        assert!(!out.contains("mor-content-target"), "dead target id must be gone");
        assert!(out.contains("<p>HELLO</p>"), "static html must be embedded");
    }

    #[test]
    fn populated_feed_maps_title_and_html() {
        let body = r#"{"feed":{"entry":[
            {"title":{"$t":"Gallery"},"content":{"$t":"<div>art</div>"}},
            {"title":{"$t":"NoBody"}}
        ]}}"#;
        let pages = parse(body);
        // entry without content is dropped
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title, "Gallery");
        assert_eq!(pages[0].html, "<div>art</div>");
    }
}
