use crate::app::config_bridge::EditorPrefs;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// CodeMirror 6 bundle (exposes `window.morCM`). Inject once per webview root via
/// `script { dangerous_inner_html: "{CM6_BUNDLE_JS}" }` — done in the main shell and
/// in each isolated pop-out editor window, since those are separate webviews.
pub const CM6_BUNDLE_JS: &str = include_str!("../../../assets/js/cm6.bundle.js");

/// Global minimap default (Editor Settings), used by editors with no per-workspace
/// override. Provided as context so editors react live. Absent = treated as on.
#[derive(Clone, Copy)]
pub struct MinimapSetting(pub Signal<bool>);

/// Per-workspace minimap overrides keyed by `minimap_key`. Takes precedence over the
/// global default. Provided as context (seeded from prefs) so toggles apply live.
#[derive(Clone, Copy)]
pub struct MinimapOverrides(pub Signal<HashMap<String, bool>>);

/// Effective minimap state: per-workspace override > per-editor default > global.
/// Reads the signals, so call it inside a reactive scope (render or effect) to track.
fn resolve_minimap(
    global: Option<MinimapSetting>,
    overrides: Option<MinimapOverrides>,
    key: &str,
    default: Option<bool>,
) -> bool {
    let g = global.map(|s| (s.0)()).unwrap_or(true);
    let ov = overrides.and_then(|o| (o.0)().get(key).copied());
    ov.unwrap_or(default.unwrap_or(g))
}

#[derive(Props, Clone, PartialEq)]
pub struct CodeEditorProps {
    pub value: String,
    pub mode: String,
    pub on_change: EventHandler<String>,
    #[props(default = None)]
    pub id: Option<String>,
    #[props(default = false)]
    pub read_only: bool,
    /// Soft-wrap long lines instead of horizontal scrolling.
    #[props(default = false)]
    pub wrap: bool,
    /// Per-editor minimap default: `None` follows the global Editor Settings default;
    /// `Some(false)` means off by default for this workspace (e.g. module workbench).
    #[props(default = None)]
    pub minimap: Option<bool>,
    /// Stable key for persisting this editor's minimap on/off across sessions.
    /// Defaults to the (ephemeral) DOM id, so session-only without a key.
    #[props(default = None)]
    pub minimap_key: Option<String>,
}

/// Code editor backed by CodeMirror 6 (bundle injected once in `shell.rs`).
///
/// The Rust side owns the buffer via `value`/`on_change`; the JS `window.morCM`
/// API owns rendering, highlighting, undo/redo, search, etc. Edits flow JS -> Rust
/// through the eval channel; external buffer changes (file load, format, hot-reload)
/// flow Rust -> JS via `setValue`, tagged so they don't echo back.
#[component]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
    // Stable DOM id: caller-provided (some docs target it by id) or auto-generated.
    let host_id = use_hook(|| {
        static N: AtomicU64 = AtomicU64::new(0);
        props
            .id
            .clone()
            .unwrap_or_else(|| format!("cm-editor-{}", N.fetch_add(1, Ordering::Relaxed)))
    });
    // Persistence key for this editor's minimap toggle.
    let minimap_key = use_hook(|| props.minimap_key.clone().unwrap_or_else(|| host_id.clone()));

    // Last value we know JS holds — gates the Rust->JS push so we never re-send an
    // edit that originated in the editor.
    let mut last_synced = use_signal(|| props.value.clone());

    // Effective minimap: per-workspace override > per-editor default > global default.
    let minimap_default = props.minimap;
    let global = try_consume_context::<MinimapSetting>();
    let overrides = try_consume_context::<MinimapOverrides>();
    let minimap_on = resolve_minimap(global, overrides, &minimap_key, minimap_default);

    // Apply minimap changes live (toggle / settings change) without remounting.
    let minimap_fx_id = host_id.clone();
    let minimap_fx_key = minimap_key.clone();
    use_effect(move || {
        let on = resolve_minimap(global, overrides, &minimap_fx_key, minimap_default);
        let id = minimap_fx_id.clone();
        spawn(async move {
            let eval = dioxus::document::eval(
                "const c = await dioxus.recv(); if (window.morCM) window.morCM.setMinimap(c.id, c.on);",
            );
            let _ = eval.send(serde_json::json!({ "id": id, "on": on }));
        });
    });

    // Push external buffer changes into the editor (skips our own echoed edits).
    let external_val = props.value.clone();
    let effect_id = host_id.clone();
    use_effect(use_reactive!(|external_val| {
        if *last_synced.peek() != external_val {
            last_synced.set(external_val.clone());
            let id = effect_id.clone();
            let v = external_val.clone();
            spawn(async move {
                let eval = dioxus::document::eval(
                    "const c = await dioxus.recv(); if (window.morCM) window.morCM.setValue(c.id, c.v);",
                );
                let _ = eval.send(serde_json::json!({ "id": id, "v": v }));
            });
        }
    }));

    // Tear down the CM view when this component unmounts (dock closed).
    {
        let id = host_id.clone();
        use_drop(move || {
            let id = id.clone();
            spawn(async move {
                let eval = dioxus::document::eval(
                    "const c = await dioxus.recv(); if (window.morCM) window.morCM.destroy(c.id);",
                );
                let _ = eval.send(serde_json::json!({ "id": id }));
            });
        });
    }

    // Toggle handler: flip this editor's effective state and persist it by key.
    let toggle_key = minimap_key.clone();
    let toggle = move |_| {
        let new_state = !minimap_on;
        if let Some(o) = overrides {
            let mut sig = o.0;
            sig.write().insert(toggle_key.clone(), new_state);
        }
        EditorPrefs::update_minimap_override(toggle_key.clone(), new_state);
    };

    let mount_id = host_id.clone();
    let mount_doc = props.value.clone();
    let mount_mode = props.mode.clone();
    let read_only = props.read_only;
    let wrap = props.wrap;
    let minimap = minimap_on;
    let on_change = props.on_change;

    rsx! {
        div {
            // Wrapper owns the toggle button; the inner `cm-host` is left child-free
            // so Dioxus never reconciles (and clobbers) CodeMirror's appended DOM.
            class: "cm-host-wrap",
            style: "position: relative; width: 100%; height: 100%; overflow: hidden;",

            // Per-workspace minimap toggle, top-right above the editor.
            button {
                class: "cm-minimap-toggle",
                title: if minimap_on { "Hide minimap" } else { "Show minimap" },
                onclick: toggle,
                if minimap_on { "▦" } else { "▢" }
            }

            div {
            class: "cm-host",
            id: "{host_id}",
            style: "width: 100%; height: 100%; overflow: hidden;",
            onmounted: move |_| {
                let cfg = serde_json::json!({
                    "id": mount_id.clone(),
                    "doc": mount_doc.clone(),
                    "lang": mount_mode.clone(),
                    "readOnly": read_only,
                    "wrap": wrap,
                    "minimap": minimap,
                });
                spawn(async move {
                    let mut eval = dioxus::document::eval(
                        r#"
                        const cfg = await dioxus.recv();
                        function ready(cb) { window.morCM ? cb() : setTimeout(() => ready(cb), 30); }
                        ready(() => window.morCM.mount(cfg.id, {
                            doc: cfg.doc, lang: cfg.lang, readOnly: cfg.readOnly, wrap: cfg.wrap,
                            minimap: cfg.minimap,
                            onChange: (v) => dioxus.send(v),
                        }));
                        "#,
                    );
                    let _ = eval.send(cfg);
                    while let Ok(v) = eval.recv::<String>().await {
                        last_synced.set(v.clone());
                        on_change.call(v);
                    }
                });
            },
            }
        }
    }
}
