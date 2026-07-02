use crate::app::state::{DockPosition, RenderState, ThemeState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::dock_chrome::DockChrome;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct SubWindowMarker;

#[derive(Props, Clone, PartialEq)]
pub struct AssetEditorProps {
    pub title: &'static str,          
    pub mode: &'static str,           
    pub default_file: &'static str,   
    pub available_files: Vec<String>, 
    pub dock_position: Signal<DockPosition>,
    pub content_signal: Signal<String>, 
    pub on_save: EventHandler<()>,      
    pub on_close: EventHandler<()>,
    pub vfs_signal: Signal<std::collections::HashMap<String, String>>,
    pub is_native_window: bool,
    /// One-shot "open this file" request (e.g. JS workspace behavior cards).
    /// The dock consumes it: selects the tab, then resets the signal to None.
    #[props(default = None)]
    pub open_file_request: Option<Signal<Option<String>>>,
}

static OPEN_WINDOWS: std::sync::Mutex<Option<std::collections::HashSet<String>>> =
    std::sync::Mutex::new(None);

fn is_window_already_open(title: &str) -> bool {
    let mut guard = OPEN_WINDOWS.lock().unwrap();
    let set = guard.get_or_insert_with(std::collections::HashSet::new);
    if set.contains(title) {
        true
    } else {
        set.insert(title.to_string());
        false
    }
}

fn mark_window_closed(title: &str) {
    let mut guard = OPEN_WINDOWS.lock().unwrap();
    if let Some(set) = guard.as_mut() {
        set.remove(title);
    }
}

#[derive(Clone)]
#[allow(dead_code)]
struct WindowCleanup {
    inner: std::sync::Arc<WindowCleanupInner>,
}

struct WindowCleanupInner {
    title: String,
}

impl Drop for WindowCleanupInner {
    fn drop(&mut self) {
        mark_window_closed(&self.title);
    }
}

/// Build the pop-out editor window's menu, replacing the default OS menu.
/// IDs are namespaced per editor mode (`xml-*`, `css-*`, `js-*`) so several
/// open windows don't cross-trigger on the global muda event channel.
fn build_editor_menu(mode: &str) -> dioxus::desktop::muda::Menu {
    use dioxus::desktop::muda::{
        accelerator::{Accelerator, Code, Modifiers},
        Menu, MenuItem, Submenu,
    };
    let acc = |m, c| Some(Accelerator::new(Some(m), c));
    let save = MenuItem::with_id(format!("{mode}-save"), "Save", true, acc(Modifiers::CONTROL, Code::KeyS));
    let close = MenuItem::with_id(format!("{mode}-close"), "Close", true, acc(Modifiers::CONTROL, Code::KeyW));
    let prev = MenuItem::with_id(format!("{mode}-prev"), "Previous File", true, acc(Modifiers::ALT, Code::ArrowLeft));
    let next = MenuItem::with_id(format!("{mode}-next"), "Next File", true, acc(Modifiers::ALT, Code::ArrowRight));
    let file = Submenu::with_items("File", true, &[&save, &close]).expect("file submenu");
    let go = Submenu::with_items("Go", true, &[&prev, &next]).expect("go submenu");
    Menu::with_items(&[&file, &go]).expect("editor menu")
}

#[component]
pub fn AssetEditorDock(props: AssetEditorProps) -> Element {
    let _cleanup = use_hook(|| WindowCleanup {
        inner: std::sync::Arc::new(WindowCleanupInner {
            title: props.title.to_string(),
        }),
    });
    let is_sub_window = try_use_context::<SubWindowMarker>().is_some();
    let theme = use_context::<ThemeState>();
    let mut vfs = props.vfs_signal;
    let mut active_tab = use_signal(|| props.default_file.to_string());
    // Full-viewport focused editing, mirroring the Module Workbench takeover.
    // Not offered in the pop-out OS window (it is already a standalone window).
    let mut is_takeover = use_signal(|| false);
    let tx_opt = try_use_context::<tokio::sync::mpsc::UnboundedSender<EditorEvent>>();
    let theme_state_opt = try_use_context::<ThemeState>();

    // Consume external open-file requests (one-shot: select the tab, clear the
    // request; the clearing write re-runs the effect once as a no-op).
    {
        let request = props.open_file_request;
        use_effect(move || {
            let Some(mut req) = request else { return };
            if let Some(file) = req() {
                active_tab.set(file);
                req.set(None);
            }
        });
    }

    let mut is_window_open = use_signal(|| false);
    let mut child_window_ref = use_signal(|| Option::<dioxus::desktop::WeakDesktopContext>::None);

    let dock_position = props.dock_position;
    let on_save = props.on_save.clone();
    let title_str = props.title.to_string();
    let child_window_props = props.clone();

    use_effect(move || {
        if is_sub_window {
            return;
        }
        let current_pos = (dock_position)();
        if current_pos == DockPosition::Floating {
            if !*is_window_open.peek() && !is_window_already_open(&title_str) {
                is_window_open.set(true);

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<EditorEvent>();
                let on_save_cb = on_save.clone();
                let title = title_str.clone();

                let child_props = EditorWindowProps {
                    dock_props: child_window_props.clone(),
                    render_state: use_context::<crate::app::state::RenderState>(),
                    layout_state: use_context::<crate::app::state::LayoutState>(),
                    theme_state: use_context::<crate::app::state::ThemeState>(),
                    vfs: use_context::<crate::app::vfs::VfsDictionary>(),
                    minimap_setting: try_consume_context::<
                        crate::ui::components::code_editor::MinimapSetting,
                    >()
                    .map(|m| m.0),
                    minimap_overrides: try_consume_context::<
                        crate::ui::components::code_editor::MinimapOverrides,
                    >()
                    .map(|m| m.0),
                    tx: tx.clone(),
                };

                let dom = VirtualDom::new_with_props(IsolatedEditorWindow, child_props);
                let cfg = dioxus::desktop::Config::new()
                    .with_menu(Some(build_editor_menu(child_window_props.mode)))
                    .with_window(
                        dioxus::desktop::WindowBuilder::new()
                            .with_title(format!("MorBlogger - {}", title))
                            // Roomy default for second-monitor editing.
                            .with_inner_size(dioxus::desktop::LogicalSize::new(1100.0, 820.0)),
                    );
                let pending_ctx = dioxus::desktop::window().new_window(dom, cfg);

                spawn(async move {
                    let desktop_context = pending_ctx.await;
                    let weak_ref = std::rc::Rc::downgrade(&desktop_context);
                    child_window_ref.set(Some(weak_ref));

                    while let Some(evt) = rx.recv().await {
                        if evt == EditorEvent::Save {
                            on_save_cb.call(());
                        }
                    }
                    is_window_open.set(false);
                    mark_window_closed(&title);
                    if dock_position() == DockPosition::Floating {
                        let mut dp = dock_position;
                        dp.set(DockPosition::Hidden);
                    }
                });
            }
        } else {
            if let Some(weak) = child_window_ref.write().take() {
                if let Some(desktop_context) = weak.upgrade() {
                    desktop_context.close();
                }
            }
            is_window_open.set(false);
            mark_window_closed(&title_str);
        }
    });

    use_effect(move || {
        let active = active_tab();
        let js = format!(
            "setTimeout(() => {{ let el = document.getElementById('tab-{}'); if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'nearest', inline: 'center' }}); }}, 20);",
            active
        );
        let _ = dioxus::document::eval(&js);
    });

    // Menu events arrive on a global channel and every mounted instance's
    // handler sees them, so act only in the native pop-out window and only on
    // this editor's own namespaced IDs.
    {
        let mode = props.mode;
        let is_native = props.is_native_window;
        let on_save = props.on_save;
        let on_close = props.on_close;
        let files = props.available_files.clone();
        let default_file = props.default_file;
        let mut active_tab = active_tab;
        dioxus::desktop::use_muda_event_handler(move |event| {
            if !is_native {
                return;
            }
            let id = event.id.0.as_str();
            if id == format!("{mode}-save") {
                on_save.call(());
            } else if id == format!("{mode}-close") {
                on_close.call(());
            } else if id == format!("{mode}-prev") || id == format!("{mode}-next") {
                if files.is_empty() {
                    return;
                }
                let cur = if files.contains(&active_tab()) {
                    active_tab()
                } else {
                    files.first().cloned().unwrap_or_else(|| default_file.to_string())
                };
                let idx = files.iter().position(|f| f == &cur).unwrap_or(0);
                let new_idx = if id.ends_with("-prev") {
                    if idx == 0 { files.len() - 1 } else { idx - 1 }
                } else {
                    (idx + 1) % files.len()
                };
                active_tab.set(files[new_idx].clone());
            }
        });
    }

    let pos = (props.dock_position)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    if pos == DockPosition::Floating && !is_sub_window {
        return rsx! {};
    }

    // Lift this editor's dock container above its sibling while taken over
    // (see the `mor-editor-takeover-*` CSS rules). Self-managing per side, so
    // a left and a right editor never clobber each other's class.
    {
        let dp = props.dock_position;
        use_effect(move || {
            let take = is_takeover();
            let side = match dp() {
                DockPosition::mor_panel_left => "left",
                DockPosition::mor_panel_right => "right",
                _ => "",
            };
            if !side.is_empty() {
                let action = if take { "add" } else { "remove" };
                let _ = dioxus::document::eval(&format!(
                    "document.documentElement.classList.{action}('mor-editor-takeover-{side}');"
                ));
            }
        });
    }


    let current_file = if props.available_files.contains(&active_tab()) {
        active_tab()
    } else {
        props.available_files.first().cloned().unwrap_or_else(|| props.default_file.to_string())
    };

    let raw_val = if current_file == props.default_file {
        if props.mode == "css" {
            theme.signals.preset_css.read().clone()
        } else if props.mode == "xml" {
            props.content_signal.read().clone()
        } else {
            theme.signals.custom_js.read().clone()
        }
    } else {
        vfs.read().get(&current_file).cloned().unwrap_or_else(|| {
            if props.mode == "css" {
                mor_blogger_core::render::template_resolver::fetch_default_css(&current_file).to_string()
            } else if props.mode == "xml" {
                props.content_signal.read().clone()
            } else {
                mor_blogger_core::render::template_resolver::fetch_js(&current_file).to_string()
            }
        })
    };

    // ponytail: store raw CSS only. `val` flows into content_signal/preset_css and
    // round-trips through on_change, so beautify must never touch it (it also shreds
    // {{TEMPLATE_TOKENS}}). Display raw; no separate display/state split needed.
    let val = raw_val;

    // Display the same per-file buffer that on_change writes (preset_css for the
    // default file, otherwise the file's VFS entry). Reading content_signal here
    // instead would show a single shared scalar that diverges from the edit
    // target once a non-preset file is selected, so edits never appear on screen.
    let editor_value = val.clone();

    let last_file = use_signal(|| current_file.clone());
    let mut last_external_val = use_signal(|| val.clone());
    let sig = props.content_signal;

    // Reconcile external content into content_signal from an effect (never during render),
    // keyed on the loaded file + its source value. peek() the bookkeeping signals: sig.read()
    // here would subscribe the effect to its own writes and loop, so the only reactive triggers
    // are current_file/val. on_change owns the buffer between loads.
    use_effect(use_reactive!(|current_file, val| {
        let mut last_file = last_file;
        let mut last_external_val = last_external_val;
        let mut sig = sig;
        if *last_file.peek() != current_file {
            last_file.set(current_file.clone());
            sig.set(val.clone());
            last_external_val.set(val.clone());
        } else if *last_external_val.peek() != val {
            last_external_val.set(val.clone());
            sig.set(val.clone());
        } else if sig.peek().is_empty() && !val.is_empty() {
            sig.set(val.clone());
        }
    }));

    {
        let active_file_signal = active_tab;
        let content_signal = props.content_signal;
        use_effect(move || {
            if let Some(theme_state) = theme_state_opt {
                if *theme_state.enable_ai_bridge.read() {
                    let active_file = active_file_signal.read().clone();
                    let current_content = content_signal.read().clone();
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs().to_string())
                        .unwrap_or_default();

                    let payload = serde_json::json!({
                        "active_file": active_file,
                        "unsaved_content": current_content,
                        "timestamp": timestamp
                    });
                    let _ = std::fs::write("/tmp/mor_blogger_live_state.json", payload.to_string());
                } else {
                    let _ = std::fs::remove_file("/tmp/mor_blogger_live_state.json");
                }
            }
        });
    }

    let available_files_clone = props.available_files.clone();
    let default_file_clone = props.default_file;
    let tx_opt_key = tx_opt.clone();
    let on_save_key = props.on_save;
    let is_native_kbd = props.is_native_window;
    let onkeydown_handler = move |evt: Event<KeyboardData>| {
        // In the pop-out window the muda menu owns these shortcuts; only handle
        // them here for the in-app docked panels (which have no menu).
        if is_native_kbd {
            return;
        }
        // Ctrl+S — save to disk via the same dispatch the Save button uses.
        let key_str = evt.key().to_string();
        if evt.modifiers().ctrl() && key_str.eq_ignore_ascii_case("s") {
            evt.prevent_default();
            evt.stop_propagation(); // ours; don't also trigger the global Save Project bind
            if let Some(tx) = &tx_opt_key {
                let _ = tx.send(EditorEvent::Save);
            } else {
                on_save_key.call(());
            }
            return;
        }
        // CodeMirror owns undo/redo while typing here — keep the global
        // theme-level Undo/Redo binds from also firing on the same keys.
        if evt.modifiers().ctrl() && (key_str.eq_ignore_ascii_case("z") || key_str.eq_ignore_ascii_case("y")) {
            evt.stop_propagation();
            return;
        }

        let files = &available_files_clone;
        if files.is_empty() { return; }
        let current_file = if files.contains(&active_tab()) {
            active_tab()
        } else {
            files.first().cloned().unwrap_or_else(|| default_file_clone.to_string())
        };
        let current_idx = files.iter().position(|f| f == &current_file).unwrap_or(0);

        if evt.modifiers().alt() {
            let key_str = evt.key().to_string();
            match key_str.as_str() {
                "ArrowRight" | "k" | "K" => {
                    evt.prevent_default();
                    let next_idx = (current_idx + 1) % files.len();
                    active_tab.set(files[next_idx].clone());
                }
                "ArrowLeft" | "j" | "J" => {
                    evt.prevent_default();
                    let prev_idx = if current_idx == 0 { files.len() - 1 } else { current_idx - 1 };
                    active_tab.set(files[prev_idx].clone());
                }
                _ => {}
            }
        }
    };

    let drag_js = format!(
        r#"
(function () {{
    if (window.__mor{mode_cap}DragInstalled) return;
    window.__mor{mode_cap}DragInstalled = true;

    document.addEventListener('pointerdown', function (e) {{
        const bar = e.target.closest('.floating-editor-window-bar');
        if (!bar) return;
        const panel = bar.closest('.floating-{mode}-editor');
        if (!panel) return;
        
        e.preventDefault();
        
        const rect = panel.getBoundingClientRect();
        const startX = e.clientX;
        const startY = e.clientY;
        const startLeft = rect.left;
        const startTop = rect.top;

        document.documentElement.style.setProperty('--{mode}-dock-x', startLeft + 'px');
        document.documentElement.style.setProperty('--{mode}-dock-y', startTop + 'px');

        const onMove = function (moveEvt) {{
            const dx = moveEvt.clientX - startX;
            const dy = moveEvt.clientY - startY;
            document.documentElement.style.setProperty('--{mode}-dock-x', Math.max(0, startLeft + dx) + 'px');
            document.documentElement.style.setProperty('--{mode}-dock-y', Math.max(0, startTop + dy) + 'px');
        }};

        const onUp = function () {{
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
        }};

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    }});
}})();
"#,
        mode = props.mode,
        mode_cap = if props.mode == "css" { "Css" } else if props.mode == "xml" { "Xml" } else { "Js" }
    );

    let resize_js = format!(
        r#"
(function() {{
    if (window.__mor{mode_cap}PaneResizeInstalled) return;
    window.__mor{mode_cap}PaneResizeInstalled = true;
    document.addEventListener('pointerdown', function(e) {{
        const resizer = e.target.closest('.{mode}-pane-resizer');
        if (!resizer) return;
        e.preventDefault();
        const isLeft = resizer.classList.contains('pane-resizer-right');
        const startX = e.clientX;
        const panel = resizer.closest('.mor_panel_left, .mor_panel_right');
        const startWidth = panel.getBoundingClientRect().width;
        
        const onMove = function(moveEvt) {{
            const dx = moveEvt.clientX - startX;
            const newWidth = isLeft ? startWidth + dx : startWidth - dx;
            const clamped = Math.max(200, Math.min(newWidth, window.innerWidth / 2.5));
            const varName = isLeft ? '--left-pane-width' : '--right-pane-width';
            document.documentElement.style.setProperty(varName, clamped + 'px');
        }};
        const onUp = function() {{
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
        }};
        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    }});
}})();
"#,
        mode = props.mode,
        mode_cap = if props.mode == "css" { "Css" } else if props.mode == "xml" { "Xml" } else { "Js" }
    );

    let editor_css = format!(
        r#"
    .mor_panel_left.{mode}-editor-mode,
    .mor_panel_right.{mode}-editor-mode {{
        padding: 0 !important;
        overflow: hidden !important;
        display: flex !important;
        flex-direction: column !important;
        width: 100% !important;
        height: 100% !important;
        box-sizing: border-box !important;
    }}
    .floating-{mode}-editor {{
        position: fixed;
        left: var(--{mode}-dock-x, {default_x}px);
        top: var(--{mode}-dock-y, {default_y}px);
        width: 600px;
        height: 450px;
        background: var(--editor-bg, #1e1e1e);
        border: 1px solid var(--editor-border);
        border-radius: 8px;
        box-shadow: 0 10px 40px rgba(0,0,0,0.5);
        z-index: 4000;
        display: flex;
        flex-direction: column;
        resize: both;
        overflow: hidden;
        min-width: 400px;
        min-height: 300px;
        max-width: 95vw;
        max-height: 95vh;
    }}
    .{mode}-tab-bar {{
        display: flex;
        overflow-x: auto;
        background: transparent;
    }}
    .{mode}-tab {{
        padding: 8px 16px;
        font-family: var(--font-mono);
        font-size: 0.75rem;
        background: transparent;
        border: none;
        border-right: 1px solid var(--editor-border-soft);
        color: var(--editor-text-muted);
        cursor: pointer;
        transition: background 0.1s;
    }}
    .{mode}-tab:hover {{ background: rgba(255,255,255,0.05); }}
    .{mode}-tab.active {{
        background: var(--editor-bg, #1e1e1e);
        color: var(--editor-accent);
        border-bottom: 2px solid var(--editor-accent);
    }}
    .{mode}-editor-textarea {{
        flex-grow: 1;
        width: 100%;
        background: var(--bg-panel);
        color: var(--fg-base);
        font-family: var(--font-mono);
        font-size: 0.85rem;
        line-height: 1.5;
        border: none;
        padding: 16px;
        resize: none;
    }}
    .mor_panel_left:not(.is-floating) {{ width: var(--left-pane-width, 320px) !important; position: relative; }}
    .mor_panel_right:not(.is-floating) {{ width: var(--right-pane-width, 320px) !important; position: relative; }}
    .{mode}-pane-resizer {{ position: absolute; top: 0; bottom: 0; width: 6px; z-index: 999; cursor: ew-resize; background: transparent; transition: background 0.1s; }}
    .{mode}-pane-resizer:hover, .{mode}-pane-resizer:active {{ background: var(--editor-accent, rgba(255,255,255,0.2)); }}
    .pane-resizer-right {{ right: 0; }}
    .pane-resizer-left {{ left: 0; }}
    .floating-editor-window-actions {{
        display: flex !important;
        align-items: center !important;
        gap: 6px !important;
    }}
    /* WebKitGTK hides scrollbars by default; child editor windows don't load the
       global editor CSS, so style the CodeMirror scroller's scrollbar here. */
    /* Scroll-past-end: let the last lines rise into comfortable view. */
    .cm-content {{ padding-bottom: 40vh; }}
    .cm-scroller::-webkit-scrollbar {{ width: 12px; height: 12px; }}
    .cm-scroller::-webkit-scrollbar-track {{ background: transparent; }}
    .cm-scroller::-webkit-scrollbar-thumb {{
        background: var(--editor-border, #555);
        border-radius: 6px;
        border: 2px solid transparent;
        background-clip: padding-box;
    }}
    .cm-scroller::-webkit-scrollbar-thumb:hover {{
        background: var(--editor-accent, #888);
        background-clip: padding-box;
    }}
    /* Each .panel-container is its own z-index:90 stacking context, so a fixed
       overlay inside one dock can't paint over the opposite dock. While an
       editor is taken over, lift its dock container above its sibling. */
    html.mor-editor-takeover-left .left-dock-container {{ z-index: 6000 !important; }}
    html.mor-editor-takeover-right .right-dock-container {{ z-index: 6000 !important; }}
"#,
        mode = props.mode,
        default_x = if props.mode == "css" { 100 } else { 150 },
        default_y = if props.mode == "css" { 100 } else { 150 }
    );

    let target_id = if props.mode == "css" { "css" } else if props.mode == "xml" { "xml" } else { "js" };

    let tx_opt_clone = tx_opt.clone();

    let editor_body = rsx! {
        script { dangerous_inner_html: "{drag_js}" }
        script { dangerous_inner_html: "{resize_js}" }
        script { dangerous_inner_html: crate::app::keyboard::EDITOR_KEY_GUARD_JS }
        style { dangerous_inner_html: "{editor_css}" }

        div {
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; flex-grow: 1; min-height: 0;",
            tabindex: "0",
            onkeydown: onkeydown_handler,

            // UNIFIED HEADER ROW: Tabs -> Actions
            div {
                class: "floating-editor-window-bar",
                style: "display: flex; align-items: center; background: var(--bg-elevated); border-bottom: 1px solid var(--editor-border-soft); flex-shrink: 0; min-height: 36px; gap: 8px; padding: 0 8px;",

                div {
                    class: "{props.mode}-tab-bar",
                    style: "display: flex; overflow-x: auto; scroll-behavior: smooth; white-space: nowrap; flex-grow: 1; min-width: 0;",
                    for file in props.available_files {
                        {
                            let is_active = file == current_file;
                            // Customized = VFS override that differs from the bundled
                            // default (edit-then-undo leaves an identical entry).
                            let is_edited = file != props.default_file
                                && vfs.read().get(&file).is_some_and(|c| match props.mode {
                                    "css" => c != mor_blogger_core::render::template_resolver::fetch_default_css(&file),
                                    "js" | "javascript" => c != mor_blogger_core::render::template_resolver::fetch_js(&file),
                                    // ponytail: xml has no single bundled default to diff against
                                    _ => true,
                                });
                            let tab_style = if is_active {
                                "color: var(--fg-base); background: var(--bg-panel); border-bottom: 2px solid var(--accent); opacity: 1.0;"
                            } else {
                                "color: var(--fg-muted); background: transparent; border-bottom: 2px solid transparent; opacity: 0.7;"
                            };
                            rsx! {
                                button {
                                    id: "tab-{file}",
                                    class: if is_active { "{props.mode}-tab active" } else { "{props.mode}-tab" },
                                    style: "{tab_style} padding: 6px 12px; cursor: pointer; transition: all 0.2s ease; font-size: 0.85rem; border: none;",
                                    onclick: {
                                        let f = file.to_string();
                                        move |_| active_tab.set(f.clone())
                                    },
                                    "{file}"
                                    if is_edited {
                                        span {
                                            style: "margin-left: 4px; color: var(--accent, #c2622a); font-size: 0.6rem; vertical-align: middle;",
                                            title: "Customized — differs from the bundled default",
                                            "●"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !props.is_native_window {
                    button {
                        class: "editor-mini-button",
                        style: "padding: 4px 10px; font-size: 0.75rem; border-radius: 4px; cursor: pointer;",
                        title: if is_takeover() { "Exit full-viewport editing" } else { "Expand the editor to a full-viewport focused stage" },
                        onclick: move |e| {
                            e.stop_propagation();
                            is_takeover.set(!is_takeover());
                        },
                        if is_takeover() { "Exit ×" } else { "Takeover" }
                    }
                }

                button {
                    class: "editor-mini-button",
                    style: "padding: 4px 10px; font-size: 0.75rem; border-radius: 4px; font-weight: 600; background: var(--accent); color: #111; border: none; cursor: pointer;",
                    onclick: move |e| {
                        e.stop_propagation();
                        if let Some(tx) = &tx_opt_clone {
                            let _ = tx.send(EditorEvent::Save);
                        } else {
                            props.on_save.call(());
                        }
                    },
                    "Save"
                }
            }

            div {
                style: "display: flex; flex-direction: column; flex-grow: 1; width: 100%; min-height: 0;",
                CodeEditor {
                    value: editor_value,
                    mode: props.mode.to_string(),
                    minimap_key: Some(format!("asset_{}", props.mode)),
                    on_change: move |new_val: String| {
                        let file = current_file.clone();
                        if file == props.default_file {
                            if props.mode == "css" {
                                let mut sig = theme.signals.preset_css;
                                sig.set(new_val.clone());
                            } else if props.mode == "xml" {
                                // XML mode writes directly to its own content_signal below
                            } else {
                                let mut sig = theme.signals.custom_js;
                                sig.set(new_val.clone());
                            }
                        } else {
                            vfs.write().insert(file, new_val.clone());
                        }
                        let mut sig = props.content_signal;
                        sig.set(new_val.clone());
                        last_external_val.set(new_val);
                    }
                }
            }
        }
    };

    // Full-viewport focused stage: same editor body, lifted out of the dock.
    // `editor_body` is moved here only when this diverging branch runs, so the
    // `match` below can still consume it on the normal path.
    if is_takeover() && !props.is_native_window {
        return rsx! {
            div {
                class: "editor-takeover-overlay",
                // Start below the 32px app menu bar so it stays usable and the
                // Exit button isn't hidden behind it (matches workbench takeover).
                style: "position: fixed; top: 32px; left: 0; right: 0; bottom: 0; z-index: 5000; background: var(--bg-base, #16140f); display: flex; flex-direction: column;",
                {editor_body}
            }
        };
    }

    match pos {
        DockPosition::mor_panel_left => {
            rsx! {
                aside {
                    class: "mor_panel_left {props.mode}-editor-mode",
                    tabindex: "0",
                    autofocus: true,
                    DockChrome {
                        title: props.title.to_string(),
                        dock_id: target_id.to_string(),
                        position: pos,
                        on_close: move |_| {
                            props.on_close.call(());
                        },
                        {editor_body}
                    }
                    div { class: "{props.mode}-pane-resizer pane-resizer-right" }
                }
            }
        }
        DockPosition::mor_panel_right => {
            rsx! {
                aside {
                    class: "mor_panel_right {props.mode}-editor-mode",
                    tabindex: "0",
                    autofocus: true,
                    DockChrome {
                        title: props.title.to_string(),
                        dock_id: target_id.to_string(),
                        position: pos,
                        on_close: move |_| {
                            props.on_close.call(());
                        },
                        {editor_body}
                    }
                    div { class: "{props.mode}-pane-resizer pane-resizer-left" }
                }
            }
        }
        DockPosition::Floating => {
            rsx! {
                if props.is_native_window {
                    {editor_body}
                } else {
                    DockChrome {
                        title: props.title.to_string(),
                        dock_id: target_id.to_string(),
                        position: pos,
                        on_close: move |_| {
                            props.on_close.call(());
                        },
                        {editor_body}
                    }
                }
            }
        }
        DockPosition::Hidden => {
            rsx! {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorEvent {
    Save,
}

#[derive(Props, Clone)]
pub struct EditorWindowProps {
    pub dock_props: AssetEditorProps,
    pub render_state: crate::app::state::RenderState,
    pub layout_state: crate::app::state::LayoutState,
    pub theme_state: crate::app::state::ThemeState,
    pub vfs: crate::app::vfs::VfsDictionary,
    pub minimap_setting: Option<Signal<bool>>,
    pub minimap_overrides: Option<Signal<std::collections::HashMap<String, bool>>>,
    pub tx: tokio::sync::mpsc::UnboundedSender<EditorEvent>,
}

impl PartialEq for EditorWindowProps {
    fn eq(&self, other: &Self) -> bool {
        self.dock_props == other.dock_props
    }
}

#[component]
pub fn IsolatedEditorWindow(props: EditorWindowProps) -> Element {
    provide_context(props.render_state);
    provide_context(props.layout_state);
    provide_context(props.theme_state);
    provide_context(props.vfs);
    provide_context(SubWindowMarker);
    provide_context(props.tx.clone());
    if let Some(sig) = props.minimap_setting {
        provide_context(crate::ui::components::code_editor::MinimapSetting(sig));
    }
    if let Some(sig) = props.minimap_overrides {
        provide_context(crate::ui::components::code_editor::MinimapOverrides(sig));
    }

    let _tx = use_signal(|| props.tx.clone());

    rsx! {
        // Separate webview: the main shell's bundle/CSS aren't here, so inject the
        // CodeMirror runtime this window's CodeEditor needs.
        script { dangerous_inner_html: "{crate::ui::components::code_editor::CM6_BUNDLE_JS}" }
        // Theme-aware JS completions, same as the main shell.
        script { dangerous_inner_html: "window.MOR_JS_HINTS = {mor_blogger_core::render::js_behaviors::editor_hints_json()};" }
        // Include the Dioxus mount root (#main/#root): this window doesn't load
        // editor_ui.css, so without height:100% here the root collapses to content
        // height and the editor's height:100% chain breaks (scroll cut off).
        style { "html, body, #main, #root {{ height: 100%; margin: 0; background-color: #16140f; color: #ece7da; overflow: hidden; }}" }
        AssetEditorDock {
            is_native_window: true,
            ..props.dock_props
        }
    }
}

pub fn resolve_workbench_dependencies(
    render: &RenderState,
    module_key: &str,
    ext: &str, 
) -> Vec<String> {
    let config = (render.current_config)();
    let pack = &config.template_pack;

    let registry_type = match module_key {
        "header_variant" => "header",
        "main_variant" => "layout",
        "content_variant" => "content",
        "left_sidebar_variant" => "sidebar_left",
        "right_sidebar_variant" => "sidebar_right",
        "footer_variant" => "footer",
        _ => return vec![],
    };

    let target_id = match module_key {
        "header_variant" => &pack.header_variant,
        "main_variant" => &pack.main_variant,
        "content_variant" => &pack.content_variant,
        "left_sidebar_variant" => &pack.left_sidebar_variant,
        "right_sidebar_variant" => &pack.right_sidebar_variant,
        "footer_variant" => &pack.footer_variant,
        _ => return vec![],
    };

    let mut deps: Vec<String> = render
        .get_manifest(registry_type, target_id)
        .map(|c| {
            if ext == "css" {
                c.css_deps.iter().map(|s| s.to_string()).collect()
            } else {
                c.js_deps.iter().map(|s| s.to_string()).collect()
            }
        })
        .unwrap_or_default();

    if deps.is_empty() {
        let fallback = match module_key {
            "header_variant" => "header",
            "main_variant" => "layout",
            "content_variant" => "content",
            "left_sidebar_variant" | "right_sidebar_variant" => "sidebar",
            "footer_variant" => "footer",
            _ => "",
        };
        if !fallback.is_empty() {
            deps.push(format!("{}.{}", fallback, ext));
        }
    }

    deps
}

pub fn resolve_theme_dependencies(
    render: &RenderState,
    ext: &str, 
) -> Vec<String> {
    let config = (render.current_config)();
    let pack = &config.template_pack;

    let targets = [
        ("header", &pack.header_variant),
        ("layout", &pack.main_variant),
        ("content", &pack.content_variant),
        ("sidebar_left", &pack.left_sidebar_variant),
        ("sidebar_right", &pack.right_sidebar_variant),
        ("footer", &pack.footer_variant),
    ];

    let mut all_deps = Vec::new();
    for (registry_type, target_id) in targets {
        if let Some(c) = render.get_manifest(registry_type, target_id) {
            let deps = if ext == "css" { c.css_deps } else { c.js_deps };
            all_deps.extend(deps.iter().map(|s| s.to_string()));
        }
    }
    all_deps
}