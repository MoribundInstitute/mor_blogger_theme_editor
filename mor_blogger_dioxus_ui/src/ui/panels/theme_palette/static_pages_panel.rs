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
/// Source repo behind the static-page hub (browse / contribute layouts).
pub const COMMUNITY_REPO_URL: &str =
    "https://github.com/MoribundInstitute/mor-blogger-static-page-compendium";
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
    // Active page is the single source of truth (shared with the workspace
    // editor via LayoutState), so the panel tab, the Layout & Chrome controls,
    // and the preview always target the same page.
    let active_tab = use_memo(move || {
        layout_state
            .active_static_page
            .read()
            .as_deref()
            .and_then(|s| TABS.iter().find(|(id, _)| *id == s).map(|(id, _)| *id))
            .unwrap_or("Archive")
    });

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
                                    // Set the shared active page; active_tab (a memo) follows it.
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

            // Per-page Layout & Chrome overrides (skip the raw-paste Community page).
            match active_tab() {
                "Archive" => rsx! { LayoutControls { config: pages, page: "archive".to_string() } },
                "Directory" => rsx! { LayoutControls { config: pages, page: "categories".to_string() } },
                "Portfolio" => rsx! { LayoutControls { config: pages, page: "portfolio".to_string() } },
                "About" => rsx! { LayoutControls { config: pages, page: "about".to_string() } },
                "LMS" | "MyCourses" => rsx! { LayoutControls { config: pages, page: "lms".to_string() } },
                _ => rsx! {},
            }

            if !status().is_empty() {
                div {
                    class: "export-status",
                    style: "margin-top: 15px; color: #3fb950; font-weight: bold;",
                    "{status}"
                }
            }

            // Always-visible footer: local folder + the online compendium links.
            div {
                style: "margin-top: 16px; padding-top: 10px; border-top: 1px solid var(--border-color); display: flex; flex-direction: column; gap: 6px;",
                button {
                    class: "editor-button",
                    title: "Open the static-pages folder in your file manager",
                    onclick: move |_| {
                        match mor_blogger_core::utils::fs_bridge::open_pages_folder() {
                            Ok(()) => fetch_status.set("Static pages folder opened.".to_string()),
                            Err(e) => fetch_status.set(format!("Could not open folder: {e}")),
                        }
                    },
                    "Open Static Pages Folder"
                }
                button {
                    class: "editor-button editor-button-small",
                    title: "Browse the static-page compendium gallery",
                    onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMMUNITY_HUB_URL).spawn(); },
                    "Browse Static Page Compendium ↗"
                }
                button {
                    class: "editor-button editor-button-small",
                    title: "View the static-page compendium source on GitHub",
                    onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMMUNITY_REPO_URL).spawn(); },
                    "Source on GitHub ↗"
                }
            }
        }
    }
}

// ---------------------------------------------------------
// Per-page Layout & Chrome overrides
// ---------------------------------------------------------

fn read_layout(c: &StaticPagesConfig, page: &str) -> mor_blogger_core::config::pages::PageLayout {
    match page {
        "archive" => c.archive.layout.clone(),
        "categories" => c.categories.layout.clone(),
        "about" => c.about.layout.clone(),
        "portfolio" => c.portfolio.layout.clone(),
        "lms" => c.lms.layout.clone(),
        "analytics" => c.analytics.layout.clone(),
        _ => mor_blogger_core::config::pages::PageLayout::default(),
    }
}

fn write_layout(c: &mut StaticPagesConfig, page: &str, l: mor_blogger_core::config::pages::PageLayout) {
    match page {
        "archive" => c.archive.layout = l,
        "categories" => c.categories.layout = l,
        "about" => c.about.layout = l,
        "portfolio" => c.portfolio.layout = l,
        "lms" => c.lms.layout = l,
        "analytics" => c.analytics.layout = l,
        _ => {}
    }
}

/// Flip one boolean field of a page's layout. Signal is Copy, so we take it by
/// value (each handler gets its own copy) — avoids a shared FnMut closure.
fn apply_toggle(
    mut config: Signal<StaticPagesConfig>,
    page: &str,
    set: fn(&mut mor_blogger_core::config::pages::PageLayout, bool),
    v: bool,
) {
    let mut c = config();
    let mut layout = read_layout(&c, page);
    set(&mut layout, v);
    write_layout(&mut c, page, layout);
    config.set(c);
}

fn apply_width(mut config: Signal<StaticPagesConfig>, page: &str, w: String) {
    let mut c = config();
    let mut layout = read_layout(&c, page);
    layout.width = w;
    write_layout(&mut c, page, layout);
    config.set(c);
}

/// Toggles that hide theme chrome (sidebars/header/footer/search) and set the
/// content width for THIS page only — written into the page's generated HTML.
#[component]
fn LayoutControls(config: Signal<StaticPagesConfig>, page: String) -> Element {
    let config = config;
    let l = read_layout(&config(), &page);

    let cb_style = "display: flex; align-items: center; gap: 8px; font-size: 12px; margin: 4px 0;";

    rsx! {
        div { class: "editor-field-group", style: "margin-top: 14px; border-top: 1px solid var(--editor-border-soft); padding-top: 12px;",
            h4 { style: "margin: 0 0 8px;", "Layout & Chrome" }
            div { class: "editor-help-text", style: "margin-bottom: 8px;",
                "Hide site chrome on this page only (e.g. drop sidebars on About). Applied via scoped CSS in the page's HTML."
            }

            label { style: "{cb_style}",
                input { r#type: "checkbox", checked: l.hide_left_sidebar,
                    onchange: { let p = page.clone(); move |e: Event<FormData>| apply_toggle(config, &p, |x, v| x.hide_left_sidebar = v, e.checked()) } }
                " Hide left sidebar (and its toggle)"
            }
            label { style: "{cb_style}",
                input { r#type: "checkbox", checked: l.hide_right_sidebar,
                    onchange: { let p = page.clone(); move |e: Event<FormData>| apply_toggle(config, &p, |x, v| x.hide_right_sidebar = v, e.checked()) } }
                " Hide right sidebar (and its toggle)"
            }
            label { style: "{cb_style}",
                input { r#type: "checkbox", checked: l.hide_header,
                    onchange: { let p = page.clone(); move |e: Event<FormData>| apply_toggle(config, &p, |x, v| x.hide_header = v, e.checked()) } }
                " Hide header"
            }
            label { style: "{cb_style}",
                input { r#type: "checkbox", checked: l.hide_footer,
                    onchange: { let p = page.clone(); move |e: Event<FormData>| apply_toggle(config, &p, |x, v| x.hide_footer = v, e.checked()) } }
                " Hide footer"
            }
            label { style: "{cb_style}",
                input { r#type: "checkbox", checked: l.hide_search,
                    onchange: { let p = page.clone(); move |e: Event<FormData>| apply_toggle(config, &p, |x, v| x.hide_search = v, e.checked()) } }
                " Hide search"
            }

            div { class: "editor-field-group", style: "margin-top: 8px;",
                label { class: "editor-field-label", "Content width" }
                select {
                    class: "editor-select",
                    value: if l.width.is_empty() { "default".to_string() } else { l.width.clone() },
                    onchange: {
                        let p = page.clone();
                        move |e: Event<FormData>| apply_width(config, &p, e.value())
                    },
                    option { value: "default", selected: l.width.is_empty() || l.width == "default", "Default (theme)" }
                    option { value: "full", selected: l.width == "full", "Full width" }
                    option { value: "centered", selected: l.width == "centered", "Centered article" }
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

    // Bake the static page directly into `.canvas-content` (server-side) instead
    // of injecting it client-side via a <template> + script. The preview iframe
    // morphs in-place for incremental updates (e.g. a dark/light toggle that
    // re-renders the base): a client-side injection gets clobbered by that morph
    // because the script only re-runs on a full reload. Baking it into the
    // source means the page survives both morph and reload, and chrome overrides
    // (sidebars/header/width) ride along with it. Scripts in the page execute on
    // reload (document.write), which is when the feed-mocking pages need them.
    const OPEN: &str = "<div class=\"canvas-content\">";
    let head_anchor = head_injected.find(OPEN);
    let footer_anchor = head_injected.find("<footer class=\"mor-footer\"");
    if let (Some(start), Some(footer)) = (head_anchor, footer_anchor) {
        let open_end = start + OPEN.len();
        // canvas-content closes with the last </div> before the footer.
        if let Some(close_rel) = head_injected[open_end..footer].rfind("</div>") {
            let close_abs = open_end + close_rel;
            let mut out = String::with_capacity(head_injected.len() + static_html.len());
            out.push_str(&head_injected[..open_end]);
            out.push('\n');
            out.push_str(static_html);
            out.push('\n');
            out.push_str(&head_injected[close_abs..]);
            return out;
        }
    }
    // ponytail: structure not found (unexpected) -> return base unchanged rather
    // than silently dropping the page into nowhere.
    head_injected
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
    fn inject_bakes_page_into_canvas_content() {
        // The page must be baked directly inside .canvas-content (server-side) so
        // it survives the preview's in-place morph on a dark/light re-render — a
        // client-side <template>/script injection got clobbered by that morph.
        let base = "<html><head></head><body><main class=\"canvas-core\"><div class=\"canvas-content\">OLD</div><footer class=\"mor-footer\">f</footer></main></body></html>";
        let out = inject_static_page(base, "<p>HELLO</p>");
        assert!(out.contains("<p>HELLO</p>"), "static html must be embedded");
        assert!(!out.contains("OLD"), "base content must be replaced");
        assert!(!out.contains("<template"), "no client-side template injection");
        // The page sits inside .canvas-content, before the footer.
        let content_pos = out.find("<p>HELLO</p>").unwrap();
        let footer_pos = out.find("<footer class=\"mor-footer\"").unwrap();
        assert!(content_pos < footer_pos, "page must be inside canvas-content");
    }

    #[test]
    fn inject_returns_base_when_no_canvas_content() {
        // Defensive: unknown structure -> base unchanged, page not dropped into
        // a nonexistent container.
        let out = inject_static_page("<html><head></head><body></body></html>", "<p>X</p>");
        assert!(!out.contains("<p>X</p>"));
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
