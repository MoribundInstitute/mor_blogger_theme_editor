use dioxus::prelude::*;
use mor_blogger_core::utils::fs_bridge::{self, WidgetBlueprint};

use crate::app::shell::WorkbenchEditState;
use crate::app::state::{CenterView, DockPosition, LayoutState};
use crate::ui::components::dock_chrome::DockChrome;
use crate::ui::components::icons::gadget_icon;
use crate::ui::workspace::widget_layout;

// Seed XML for a freshly created blueprint (a blank HTML gadget).
const NEW_WIDGET_XML: &str = "<b:widget id='HTML1' type='HTML' title='New Widget' visible='true'>\n  <b:includable id='main'>\n    <![CDATA[<div class='mor-widget'>New widget content</div>]]>\n  </b:includable>\n</b:widget>\n";

// Online widget library: the public gallery to browse, and the GitHub repo (source / contribute).
const COMPENDIUM_SITE_URL: &str = "https://mor-widgets-compendium.blogspot.com/";
const COMPENDIUM_REPO_URL: &str = "https://github.com/MoribundInstitute/mor-blogger-widget-compendium";
const COMPENDIUM_INDEX_URL: &str =
    "https://cdn.jsdelivr.net/gh/MoribundInstitute/mor-blogger-widget-compendium@main/index.json";

/// One entry from the compendium's `index.json` (only the fields we install with).
#[derive(serde::Deserialize, Clone, PartialEq)]
struct RemoteWidget {
    name: String,
    #[serde(default)]
    group: String,
    #[serde(default)]
    slug: String,
    #[serde(default)]
    xml: String,
}

/// Slug → friendly label (title-case on `-`/`_`).
fn pretty(slug: &str) -> String {
    slug.split(['-', '_'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().chain(c).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// First of `base`, `base-2`, `base-3`, … not already used within `group`.
fn unique_name(blueprints: &[WidgetBlueprint], group: &str, base: &str) -> String {
    let taken = |n: &str| blueprints.iter().any(|b| b.group == group && b.name == n);
    if !taken(base) {
        return base.to_string();
    }
    let mut i = 2;
    loop {
        let cand = format!("{base}-{i}");
        if !taken(&cand) {
            return cand;
        }
        i += 1;
    }
}

/// The widget blueprint library: browse, edit (routes into the workbench),
/// create, duplicate, and delete reusable `<b:widget>` blueprints.
#[component]
pub fn WidgetsDock() -> Element {
    let mut layout = use_context::<LayoutState>();
    let mut edit_state = use_context::<WorkbenchEditState>();
    let pos = (layout.widgets_pos)();

    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    let mut blueprints = use_signal(fs_bridge::load_widget_blueprints);
    let mut status = use_signal(String::new);
    // Remote widgets fetched from the compendium's index.json (lazily loaded).
    let mut compendium = use_signal(Vec::<RemoteWidget>::new);

    // Fold consecutive (already group/name-sorted) blueprints under one header.
    let mut groups: Vec<(String, Vec<WidgetBlueprint>)> = Vec::new();
    for bp in blueprints() {
        match groups.last_mut() {
            Some((g, v)) if *g == bp.group => v.push(bp),
            _ => groups.push((bp.group.clone(), vec![bp])),
        }
    }

    rsx! {
        crate::ui_kit::MorPanelWrapper {
            position: pos,
            default_position: DockPosition::mor_panel_left,
            DockChrome {
                title: "WIDGETS".to_string(),
                dock_id: "widgets".to_string(),
                position: pos,
                on_close: move |_| {
                    layout.widgets_pos.set(DockPosition::Hidden);
                },
                div {
                    class: "palette-content widgets-library",
                    style: "padding: 10px; height: calc(100% - 45px); overflow-y: auto; display: flex; flex-direction: column; gap: 6px; background: var(--bg-panel); color: var(--fg-base);",

                    button {
                        class: "editor-button",
                        style: "flex-shrink: 0;",
                        title: "Create a new blank widget and open it for editing",
                        onclick: move |_| {
                            let name = unique_name(&blueprints(), "custom", "new-widget");
                            match fs_bridge::save_widget_blueprint("custom", &name, NEW_WIDGET_XML) {
                                Ok(_) => {
                                    blueprints.set(fs_bridge::load_widget_blueprints());
                                    status.set(format!("Created {name}"));
                                    edit_state.edit_widget_request.set(Some(WidgetBlueprint {
                                        group: "custom".to_string(),
                                        name: name.clone(),
                                        xml: NEW_WIDGET_XML.to_string(),
                                    }));
                                    layout.center_view.set(CenterView::WidgetWorkbench);
                                }
                                Err(e) => status.set(format!("Create failed: {e}")),
                            }
                        },
                        "+ New Widget"
                    }

                    if !status().is_empty() {
                        div {
                            style: "font-size: 0.7rem; color: var(--editor-accent-warm); font-family: var(--font-mono); flex-shrink: 0;",
                            "{status()}"
                        }
                    }

                    // Where "Add" sends gadgets: a targeted slot, or the active module.
                    div {
                        style: "font-size: 0.66rem; color: var(--fg-muted); font-family: var(--font-mono); flex-shrink: 0; padding: 2px 4px;",
                        { match (edit_state.add_target)() {
                            Some(key) => format!("Add → slot: {key}"),
                            None => "Add → active module".to_string(),
                        } }
                    }

                    for (gi, (group, items)) in groups.into_iter().enumerate() {
                        div {
                            key: "wg-{gi}",
                            div {
                                style: "padding: 5px 4px 3px; color: var(--fg-muted); font-size: 0.64rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;",
                                "{pretty(&group)}"
                            }
                            for bp in items {
                                {
                                    let bp_add = bp.clone();
                                    let bp_edit = bp.clone();
                                    let bp_del = bp.clone();
                                    let label = pretty(&bp.name);
                                    let w_type = widget_layout::parse_slots(&bp.xml).first().map(|s| s.w_type.clone()).unwrap_or_default();
                                    let row_key = format!("{}-{}", bp.group, bp.name);
                                    rsx! {
                                        div {
                                            key: "{row_key}",
                                            style: "display: flex; align-items: center; gap: 6px; padding: 4px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                                            span { style: "color: var(--editor-accent); display: flex; align-items: center; flex-shrink: 0;", { gadget_icon(&w_type) } }
                                            span {
                                                style: "flex: 1; min-width: 0; font-size: 0.78rem; line-height: 1.2; word-break: break-word; overflow: hidden; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical;",
                                                title: "{bp.group}/{bp.name}",
                                                "{label}"
                                            }
                                            button {
                                                class: "editor-mini-button editor-mini-button-active",
                                                title: "Add this gadget to the current module/slot",
                                                onclick: move |_| {
                                                    edit_state.add_widget_request.set(Some(bp_add.clone()));
                                                    layout.center_view.set(CenterView::ModuleWorkbench);
                                                },
                                                "Add"
                                            }
                                            button {
                                                class: "editor-mini-button",
                                                title: "Edit this widget in the workbench",
                                                onclick: move |_| {
                                                    edit_state.edit_widget_request.set(Some(bp_edit.clone()));
                                                    layout.center_view.set(CenterView::WidgetWorkbench);
                                                },
                                                "Edit"
                                            }
                                            button {
                                                class: "editor-mini-button",
                                                title: "Delete this widget file",
                                                onclick: move |_| {
                                                    match fs_bridge::delete_widget_blueprint(&bp_del.group, &bp_del.name) {
                                                        Ok(()) => {
                                                            blueprints.set(fs_bridge::load_widget_blueprints());
                                                            status.set(format!("Deleted {}", bp_del.name));
                                                        }
                                                        Err(e) => status.set(format!("Delete failed: {e}")),
                                                    }
                                                },
                                                "Del"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Compendium install picker (appears once loaded) ──
                    if !compendium().is_empty() {
                        div {
                            style: "margin-top: 10px; padding-top: 8px; border-top: 1px solid var(--border-color); display: flex; flex-direction: column; gap: 4px;",
                            div {
                                style: "display: flex; align-items: center; gap: 6px;",
                                span { style: "flex: 1; color: var(--fg-muted); font-size: 0.64rem; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em;", "Compendium ({compendium().len()})" }
                                button {
                                    class: "editor-mini-button",
                                    title: "Hide the compendium list",
                                    onclick: move |_| compendium.set(Vec::new()),
                                    "×"
                                }
                            }
                            for rw in compendium() {
                                {
                                    let rw_install = rw.clone();
                                    rsx! {
                                        div {
                                            key: "{rw.group}-{rw.slug}-{rw.name}",
                                            style: "display: flex; align-items: center; gap: 6px; padding: 3px 4px; font-size: 0.76rem;",
                                            span { style: "flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", title: "{rw.group}/{rw.slug}", "{rw.name}" }
                                            button {
                                                class: "editor-mini-button editor-mini-button-active",
                                                title: "Download this widget into your library",
                                                onclick: move |_| {
                                                    let rw = rw_install.clone();
                                                    let mut bps = blueprints;
                                                    let mut st = status;
                                                    spawn(async move {
                                                        st.set(format!("Installing {}…", rw.name));
                                                        match crate::ui::panels::theme_palette::static_pages_panel::fetch_raw_page(&rw.xml).await {
                                                            Ok(xml) => {
                                                                let group = if rw.group.is_empty() { "custom".to_string() } else { rw.group.clone() };
                                                                let name = if rw.slug.is_empty() { rw.name.clone() } else { rw.slug.clone() };
                                                                match fs_bridge::save_widget_blueprint(&group, &name, &xml) {
                                                                    Ok(_) => { bps.set(fs_bridge::load_widget_blueprints()); st.set(format!("Installed {}", rw.name)); }
                                                                    Err(e) => st.set(format!("Save failed: {e}")),
                                                                }
                                                            }
                                                            Err(e) => st.set(format!("Fetch failed: {e}")),
                                                        }
                                                    });
                                                },
                                                "Install"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: "margin-top: auto; padding-top: 10px; border-top: 1px solid var(--border-color); display: flex; flex-direction: column; gap: 6px;",
                        button {
                            class: "editor-button",
                            title: "Open the widget blueprints directory in the system file manager",
                            onclick: move |_| {
                                match fs_bridge::open_widgets_folder() {
                                    Ok(()) => status.set("Widgets folder opened.".to_string()),
                                    Err(e) => status.set(format!("Could not open folder: {e}")),
                                }
                            },
                            "Open Widgets Folder"
                        }
                        button {
                            class: "editor-button editor-button-active",
                            title: "Load the online widget index so you can install widgets directly",
                            onclick: move |_| {
                                let mut comp = compendium;
                                let mut st = status;
                                spawn(async move {
                                    st.set("Loading compendium…".to_string());
                                    match crate::ui::panels::theme_palette::static_pages_panel::fetch_raw_page(COMPENDIUM_INDEX_URL).await {
                                        Ok(json) => match serde_json::from_str::<Vec<RemoteWidget>>(&json) {
                                            Ok(list) => { st.set(format!("{} widget(s) available.", list.len())); comp.set(list); }
                                            Err(e) => st.set(format!("Bad index.json: {e}")),
                                        },
                                        Err(e) => st.set(format!("Load failed: {e}")),
                                    }
                                });
                            },
                            "⬇ Install from Compendium"
                        }
                        button {
                            class: "editor-button editor-button-small",
                            title: "Browse the online Widget Compendium gallery",
                            onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMPENDIUM_SITE_URL).spawn(); },
                            "Browse Widget Compendium ↗"
                        }
                        button {
                            class: "editor-button editor-button-small",
                            title: "View the Widget Compendium source on GitHub",
                            onclick: move |_| { let _ = std::process::Command::new("xdg-open").arg(COMPENDIUM_REPO_URL).spawn(); },
                            "Source on GitHub ↗"
                        }
                    }
                }
            }
        }
    }
}
