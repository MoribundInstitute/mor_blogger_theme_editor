use crate::app::state::{DockPosition, LayoutState, SiteState, ThemeState};
use crate::ui::components::code_editor::CodeEditor;
use crate::ui::components::icons::{IconClose, IconDockLeft, IconDockRight, IconFloat};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct SubWindowMarker;

#[derive(Props, Clone, PartialEq)]
pub struct AssetEditorProps {
    pub title: &'static str,          // e.g., "CSS EDITOR" or "JS EDITOR"
    pub mode: &'static str,           // e.g., "css" or "javascript" (for syntect)
    pub default_file: &'static str,   // e.g., "preset_css.css" or "custom_js.js"
    pub available_files: Vec<String>, // Passed down based on context/preview
    pub dock_position: Signal<DockPosition>,
    pub content_signal: Signal<String>, // The actual VFS or fallback string to edit
    pub on_save: EventHandler<()>,      // e.g., move |_| fs_bridge::save_custom_css()
    pub on_close: EventHandler<()>,
    pub vfs_signal: Signal<std::collections::HashMap<String, String>>,
    pub is_native_window: bool,
}

#[component]
pub fn AssetEditorDock(props: AssetEditorProps) -> Element {
    let is_sub_window = try_use_context::<SubWindowMarker>().is_some();
    let mut layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();
    let mut vfs = props.vfs_signal;
    let mut active_tab = use_signal(|| props.default_file.to_string());
    let mut is_maximized = use_signal(|| false);
    let tx_opt = try_use_context::<tokio::sync::mpsc::UnboundedSender<EditorEvent>>();
    let site_state_opt = try_use_context::<SiteState>();

    #[cfg(not(target_arch = "wasm32"))]
    let mut is_window_open = use_signal(|| false);
    #[cfg(not(target_arch = "wasm32"))]
    let mut child_window_ref = use_signal(|| Option::<dioxus::desktop::WeakDesktopContext>::None);

    #[cfg(not(target_arch = "wasm32"))]
    let dock_position = props.dock_position;
    #[cfg(not(target_arch = "wasm32"))]
    let on_save = props.on_save.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let on_close = props.on_close.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let title_str = props.title.to_string();
    #[cfg(not(target_arch = "wasm32"))]
    let child_window_props = props.clone();

    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        if is_sub_window {
            return;
        }
        let current_pos = (dock_position)();
        if current_pos == DockPosition::Floating {
            if !is_window_open() {
                is_window_open.set(true);

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<EditorEvent>();

                let mut dp = dock_position;
                let on_save_cb = on_save.clone();
                let on_close_cb = on_close.clone();

                let title = title_str.clone();

                let child_props = EditorWindowProps {
                    dock_props: child_window_props.clone(),
                    render_state: use_context::<crate::app::state::RenderState>(),
                    layout_state: use_context::<crate::app::state::LayoutState>(),
                    theme_state: use_context::<crate::app::state::ThemeState>(),
                    site_state: use_context::<SiteState>(),
                    vfs: use_context::<crate::app::vfs::VfsDictionary>(),
                    tx: tx.clone(),
                };

                let dom = VirtualDom::new_with_props(IsolatedEditorWindow, child_props);
                let cfg = dioxus::desktop::Config::new()
                    .with_window(
                        dioxus::desktop::WindowBuilder::new()
                            .with_title(format!("MorBlogger - {}", title))
                            .with_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0))
                    );
                let pending_ctx = dioxus::desktop::window().new_window(dom, cfg);

                spawn(async move {
                    let desktop_context = pending_ctx.await;
                    let weak_ref = std::rc::Rc::downgrade(&desktop_context);
                    child_window_ref.set(Some(weak_ref));

                    while let Some(evt) = rx.recv().await {
                        match evt {
                            EditorEvent::Change { .. } => {}
                            EditorEvent::Save => {
                                on_save_cb.call(());
                            }
                            EditorEvent::DockLeft => {
                                dp.set(DockPosition::Left);
                                break;
                            }
                            EditorEvent::DockRight => {
                                dp.set(DockPosition::Right);
                                break;
                            }
                            EditorEvent::Close => {
                                on_close_cb.call(());
                                break;
                            }
                            EditorEvent::SelectFile(_) => {}
                        }
                    }
                    is_window_open.set(false);
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

    let pos = (props.dock_position)();
    if pos == DockPosition::Hidden {
        return rsx! {};
    }

    #[cfg(not(target_arch = "wasm32"))]
    if pos == DockPosition::Floating && !is_sub_window {
        return rsx! {};
    }

    let is_floating = pos == DockPosition::Floating;

    // Derived tab file selection
    let current_file = if props.available_files.contains(&active_tab()) {
        active_tab()
    } else {
        props
            .available_files
            .first()
            .cloned()
            .unwrap_or_else(|| props.default_file.to_string())
    };

    // Load actual content
    let raw_val = if current_file == props.default_file {
        if props.mode == "css" {
            theme.signals.preset_css.read().clone()
        } else {
            theme.signals.custom_js.read().clone()
        }
    } else {
        vfs.read().get(&current_file).cloned().unwrap_or_else(|| {
            if props.mode == "css" {
                mor_blogger_core::render::template_resolver::fetch_default_css(&current_file)
                    .to_string()
            } else {
                mor_blogger_core::render::template_resolver::fetch_js(&current_file).to_string()
            }
        })
    };

    let val = if props.mode == "css" {
        crate::utils::formatters::beautify_css(&raw_val)
    } else {
        raw_val
    };

    let mut last_file = use_signal(|| current_file.clone());
    let mut last_external_val = use_signal(|| val.clone());

    // Sync external changes (or tab changes) to props.content_signal
    let mut sig = props.content_signal;
    if last_file() != current_file {
        last_file.set(current_file.clone());
        sig.set(val.clone());
        last_external_val.set(val.clone());
    } else if last_external_val() != val {
        last_external_val.set(val.clone());
        sig.set(val.clone());
    } else if sig.read().is_empty() && !val.is_empty() {
        sig.set(val.clone());
    }

    // Live AI Bridge State Dropper
    {
        let active_file_signal = active_tab;
        let content_signal = props.content_signal;
        use_effect(move || {
            if let Some(site_state) = site_state_opt {
                if *site_state.enable_ai_bridge.read() {
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

                    // Write to the standard Linux temporary directory
                    let _ = std::fs::write("/tmp/mor_blogger_live_state.json", payload.to_string());
                } else {
                    // Clean up the file if they turn it off so the AI can't read stale data
                    let _ = std::fs::remove_file("/tmp/mor_blogger_live_state.json");
                }
            }
        });
    }

    let available_files_clone = props.available_files.clone();
    let default_file_clone = props.default_file;
    let onkeydown_handler = move |evt: Event<KeyboardData>| {
        let files = &available_files_clone;
        if files.is_empty() {
            return;
        }
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
        mode_cap = if props.mode == "css" { "Css" } else { "Js" }
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
        const panel = resizer.closest('.editor-left-panel, .editor-right-panel');
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
        mode_cap = if props.mode == "css" { "Css" } else { "Js" }
    );

    let editor_css = format!(
        r#"
    .editor-left-panel.{mode}-editor-mode,
    .editor-right-panel.{mode}-editor-mode {{
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
        background: var(--editor-bg);
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
        background: var(--editor-bg-deep);
        border-bottom: 1px solid var(--editor-border-soft);
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
        background: var(--editor-bg);
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
    .editor-left-panel:not(.is-floating) {{ width: var(--left-pane-width, 320px) !important; position: relative; }}
    .editor-right-panel:not(.is-floating) {{ width: var(--right-pane-width, 320px) !important; position: relative; }}
    .{mode}-pane-resizer {{ position: absolute; top: 0; bottom: 0; width: 6px; z-index: 999; cursor: ew-resize; background: transparent; transition: background 0.1s; }}
    .{mode}-pane-resizer:hover, .{mode}-pane-resizer:active {{ background: var(--editor-accent, rgba(255,255,255,0.2)); }}
    .pane-resizer-right {{ right: 0; }}
    .pane-resizer-left {{ left: 0; }}
    .floating-editor-window-bar {{
        display: flex !important;
        align-items: center !important;
        justify-content: space-between !important;
        padding: 0 12px !important;
        background: var(--bg-elevated) !important;
        border-bottom: 1px solid var(--editor-border-soft) !important;
        cursor: move !important;
        box-sizing: border-box !important;
        width: 100% !important;
        flex: 0 0 44px !important;
        min-height: 44px !important;
        overflow: hidden !important;
    }}
    .floating-editor-grip-group {{
        display: flex !important;
        align-items: center !important;
        gap: 8px !important;
        flex-shrink: 0 !important;
    }}
    .floating-editor-title {{
        margin: 0 !important;
        font-size: 0.8rem !important;
        font-weight: 600 !important;
        text-transform: uppercase !important;
        letter-spacing: 0.05em !important;
        white-space: nowrap !important;
    }}
    .floating-editor-window-actions {{
        display: flex !important;
        align-items: center !important;
        gap: 6px !important;
    }}
"#,
        mode = props.mode,
        default_x = if props.mode == "css" { 100 } else { 150 },
        default_y = if props.mode == "css" { 100 } else { 150 }
    );

    let target_id = if props.mode == "css" { "css" } else { "js" };

    let tx_opt_clone = tx_opt.clone();
    let header_actions = rsx! {
        div { class: "floating-editor-window-actions",
            button {
                class: "editor-mini-button",
                style: "padding: 2px 6px; font-size: 0.75rem; border-radius: 4px; font-weight: 600;",
                onclick: move |_| {
                    if let Some(tx) = &tx_opt_clone {
                        let _ = tx.send(EditorEvent::Save);
                    } else {
                        props.on_save.call(());
                    }
                },
                "Save"
            }
            if !props.is_native_window {
                if pos != DockPosition::Left {
                    button {
                        class: "editor-mini-button",
                        style: "display: flex; align-items: center; padding: 4px;",
                        title: "Dock Left",
                        onclick: move |_| {
                            layout.request_exclusive_dock(target_id, DockPosition::Left);
                        },
                        IconDockLeft {}
                    }
                }
                if pos != DockPosition::Right {
                    button {
                        class: "editor-mini-button",
                        style: "display: flex; align-items: center; padding: 4px;",
                        title: "Dock Right",
                        onclick: move |_| {
                            layout.request_exclusive_dock(target_id, DockPosition::Right);
                        },
                        IconDockRight {}
                    }
                }
                if pos != DockPosition::Floating {
                    button {
                        class: "editor-mini-button",
                        style: "display: flex; align-items: center; padding: 4px;",
                        title: "Float Window",
                        onclick: move |_| {
                            let mut dp = props.dock_position;
                            dp.set(DockPosition::Floating);
                        },
                        IconFloat {}
                    }
                }
                button {
                    class: "editor-mini-button",
                    style: "display: flex; align-items: center; padding: 4px;",
                    title: "Close",
                    onclick: move |_| {
                        props.on_close.call(());
                    },
                    IconClose {}
                }
            }
        }
    };

    let editor_body = rsx! {
        script { dangerous_inner_html: "{drag_js}" }
        script { dangerous_inner_html: "{resize_js}" }
        style { dangerous_inner_html: "{editor_css}" }

        if is_floating {
            div {
                class: "floating-editor-window-bar",
                ondoubleclick: move |_| is_maximized.set(!is_maximized()),
                div {
                    class: "floating-editor-grip-group",
                    span { class: "floating-editor-grip", style: "display: flex; align-items: center;", "⠿" }
                    span {
                        class: "floating-editor-title",
                        "{props.title}"
                    }
                }
                {header_actions}
            }
        } else {
            div { class: "editor-panel-header", style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-bottom: 1px solid var(--editor-border-soft); box-sizing: border-box;",
                h2 { class: "editor-panel-title", style: "margin: 0; font-size: 0.9rem; font-weight: 600;", "{props.title}" }
                {header_actions}
            }
        }

        div {
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; flex-grow: 1; min-height: 0;",
            div {
                class: "{props.mode}-tab-bar",
                style: "overflow-x: auto; scroll-behavior: smooth; white-space: nowrap;",
                for file in props.available_files {
                    {
                        let is_active = file == current_file;
                        let tab_style = if is_active {
                            "color: var(--fg-base); background: var(--bg-panel); border-bottom: 2px solid var(--accent); opacity: 1.0;"
                        } else {
                            "color: var(--fg-muted); background: transparent; border-bottom: 2px solid transparent; opacity: 0.7;"
                        };
                        rsx! {
                            button {
                                id: "tab-{file}",
                                class: if is_active { "{props.mode}-tab active" } else { "{props.mode}-tab" },
                                style: "{tab_style} padding: 8px 16px; cursor: pointer; transition: all 0.2s ease;",
                                onclick: {
                                    let f = file.to_string();
                                    move |_| active_tab.set(f.clone())
                                },
                                "{file}"
                            }
                        }
                    }
                }
            }

            div {
                style: "display: flex; flex-direction: column; flex-grow: 1; width: 100%; min-height: 0;",
                CodeEditor {
                    value: props.content_signal,
                    mode: props.mode.to_string(),
                    on_change: move |new_val: String| {
                        let file = current_file.clone();
                        if file == props.default_file {
                            if props.mode == "css" {
                                let mut sig = theme.signals.preset_css;
                                sig.set(new_val.clone());
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

    match pos {
        DockPosition::Left => {
            rsx! {
                aside {
                    class: "editor-left-panel {props.mode}-editor-mode",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: onkeydown_handler.clone(),
                    {editor_body}
                    div { class: "{props.mode}-pane-resizer pane-resizer-right" }
                }
            }
        }
        DockPosition::Right => {
            rsx! {
                aside {
                    class: "editor-right-panel {props.mode}-editor-mode",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: onkeydown_handler.clone(),
                    {editor_body}
                    div { class: "{props.mode}-pane-resizer pane-resizer-left" }
                }
            }
        }
        DockPosition::Floating => {
            let container_style = if props.is_native_window {
                "position: absolute; left: 0; top: 0; width: 100vw; height: 100vh; display: flex; flex-direction: column; background: var(--bg-base); border-radius: 0; box-shadow: none; resize: none; max-width: none; max-height: none;"
            } else if is_maximized() {
                "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 9999; border-radius: 0; resize: none;"
            } else {
                "resize: both; overflow: hidden; min-width: 400px; min-height: 300px; max-width: 95vw; max-height: 95vh;"
            };
            rsx! {
                div {
                    class: "floating-{props.mode}-editor",
                    style: "{container_style}",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: onkeydown_handler,
                    {editor_body}
                }
            }
        }
        DockPosition::Hidden => {
            rsx! {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub enum EditorEvent {
    Change { file: String, content: String },
    Save,
    DockLeft,
    DockRight,
    Close,
    SelectFile(String),
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Props, Clone)]
pub struct StandaloneEditorProps {
    pub title: String,
    pub mode: String,
    pub default_file: String,
    pub available_files: Vec<String>,
    pub vfs_signal: Signal<std::collections::HashMap<String, String>>,
    pub theme_preset_css: Signal<String>,
    pub theme_custom_js: Signal<String>,
    pub initial_active_tab: String,
    pub tx: tokio::sync::mpsc::UnboundedSender<EditorEvent>,
}

#[cfg(not(target_arch = "wasm32"))]
impl PartialEq for StandaloneEditorProps {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.mode == other.mode
            && self.default_file == other.default_file
            && self.available_files == other.available_files
            && self.vfs_signal == other.vfs_signal
            && self.theme_preset_css == other.theme_preset_css
            && self.theme_custom_js == other.theme_custom_js
            && self.initial_active_tab == other.initial_active_tab
    }
}

#[cfg(not(target_arch = "wasm32"))]
const EDITOR_UI_CSS: &str = include_str!("../../../editor_ui.css");

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn StandaloneEditor(props: StandaloneEditorProps) -> Element {
    let mode = props.mode.clone();
    let default_file = props.default_file.clone();
    let window = dioxus::desktop::use_window();

    let mut vfs = props.vfs_signal;
    let mut active_tab = use_signal(|| props.initial_active_tab.clone());
    let mut preset_css = props.theme_preset_css;
    let mut custom_js = props.theme_custom_js;

    let current_file = active_tab();
    let raw_current_val = if current_file == default_file {
        if mode == "css" {
            preset_css()
        } else {
            custom_js()
        }
    } else {
        vfs.read().get(&current_file).cloned().unwrap_or_default()
    };

    let current_val = if mode == "css" {
        crate::utils::formatters::beautify_css(&raw_current_val)
    } else {
        raw_current_val
    };

    let mut editor_val = use_signal(|| current_val.clone());

    use_effect(move || {
        let active = active_tab();
        let js = format!(
            "setTimeout(() => {{ let el = document.getElementById('tab-{}'); if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'nearest', inline: 'center' }}); }}, 20);",
            active
        );
        let _ = dioxus::document::eval(&js);
    });

    {
        let default_file = default_file.clone();
        let mode = mode.clone();
        use_effect(move || {
            let current_file = active_tab();
            let raw_val = if current_file == default_file {
                if mode == "css" {
                    preset_css()
                } else {
                    custom_js()
                }
            } else {
                vfs.read().get(&current_file).cloned().unwrap_or_default()
            };
            let val = if mode == "css" {
                crate::utils::formatters::beautify_css(&raw_val)
            } else {
                raw_val
            };
            editor_val.set(val);
        });
    }

    let on_change = {
        let tx = props.tx.clone();
        let default_file = default_file.clone();
        let mode = mode.clone();
        move |new_val: String| {
            let file = active_tab();
            if file == default_file {
                if mode == "css" {
                    preset_css.set(new_val.clone());
                } else {
                    custom_js.set(new_val.clone());
                }
            } else {
                vfs.write().insert(file.clone(), new_val.clone());
            }
            editor_val.set(new_val.clone());

            let _ = tx.send(EditorEvent::Change {
                file,
                content: new_val,
            });
        }
    };

    let on_save = {
        let tx = props.tx.clone();
        move |_| {
            let _ = tx.send(EditorEvent::Save);
        }
    };

    let on_dock_left = {
        let tx = props.tx.clone();
        let window = window.clone();
        move |_| {
            let _ = tx.send(EditorEvent::DockLeft);
            window.close();
        }
    };

    let on_dock_right = {
        let tx = props.tx.clone();
        let window = window.clone();
        move |_| {
            let _ = tx.send(EditorEvent::DockRight);
            window.close();
        }
    };

    let on_close = {
        let tx = props.tx.clone();
        let window = window.clone();
        move |_| {
            let _ = tx.send(EditorEvent::Close);
            window.close();
        }
    };

    let onkeydown_handler = {
        let files = props.available_files.clone();
        let tx = props.tx.clone();
        move |evt: Event<KeyboardData>| {
            if files.is_empty() {
                return;
            }
            let current_file = active_tab();
            let current_idx = files.iter().position(|f| f == &current_file).unwrap_or(0);

            if evt.modifiers().alt() {
                let key_str = evt.key().to_string();
                match key_str.as_str() {
                    "ArrowRight" | "k" | "K" => {
                        evt.prevent_default();
                        let next_idx = (current_idx + 1) % files.len();
                        let f = files[next_idx].clone();
                        active_tab.set(f.clone());
                        let _ = tx.send(EditorEvent::SelectFile(f));
                    }
                    "ArrowLeft" | "j" | "J" => {
                        evt.prevent_default();
                        let prev_idx = if current_idx == 0 { files.len() - 1 } else { current_idx - 1 };
                        let f = files[prev_idx].clone();
                        active_tab.set(f.clone());
                        let _ = tx.send(EditorEvent::SelectFile(f));
                    }
                    _ => {}
                }
            }
        }
    };

    let editor_css = format!(
        r#"
    .standalone-editor-container {{
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
        background: var(--editor-bg);
        color: var(--fg-base);
        overflow: hidden;
        font-family: var(--font-sans);
    }}
    .floating-editor-window-bar {{
        display: flex !important;
        align-items: center !important;
        justify-content: space-between !important;
        padding: 0 12px !important;
        background: var(--bg-elevated) !important;
        border-bottom: 1px solid var(--editor-border-soft) !important;
        box-sizing: border-box !important;
        width: 100% !important;
        flex: 0 0 44px !important;
        min-height: 44px !important;
        overflow: hidden !important;
    }}
    .floating-editor-grip-group {{
        display: flex !important;
        align-items: center !important;
        gap: 8px !important;
        flex-shrink: 0 !important;
    }}
    .floating-editor-title {{
        margin: 0 !important;
        font-size: 0.8rem !important;
        font-weight: 600 !important;
        text-transform: uppercase !important;
        letter-spacing: 0.05em !important;
        white-space: nowrap !important;
    }}
    .floating-editor-window-actions {{
        display: flex !important;
        align-items: center !important;
        gap: 6px !important;
    }}
    .{mode}-tab-bar {{
        display: flex;
        overflow-x: auto;
        background: var(--editor-bg-deep);
        border-bottom: 1px solid var(--editor-border-soft);
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
        background: var(--editor-bg);
        color: var(--editor-accent);
        border-bottom: 2px solid var(--editor-accent);
    }}
    "#,
        mode = mode
    );

    let is_native_window = true;

    rsx! {
        style { "{EDITOR_UI_CSS}" }
        style { "{editor_css}" }

        div {
            class: "standalone-editor-container",
            tabindex: "0",
            autofocus: true,
            onkeydown: onkeydown_handler,
            if !is_native_window {
                div { class: "floating-editor-window-bar",
                    div { class: "floating-editor-grip-group",
                        span { class: "floating-editor-title", "{props.title}" }
                    }
                    div { class: "floating-editor-window-actions",
                        button {
                            class: "editor-mini-button",
                            style: "padding: 2px 6px; font-size: 0.75rem; border-radius: 4px; font-weight: 600;",
                            onclick: on_save.clone(),
                            "Save"
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock Left",
                            onclick: on_dock_left.clone(),
                            IconDockLeft {}
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock Right",
                            onclick: on_dock_right.clone(),
                            IconDockRight {}
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Close",
                            onclick: on_close.clone(),
                            IconClose {}
                        }
                    }
                }
            }

            div { class: "{mode}-tab-bar", style: "display: flex; align-items: center; justify-content: space-between; width: 100%;",
                div { style: "display: flex; overflow-x: auto; flex-grow: 1; scroll-behavior: smooth; white-space: nowrap;",
                    for file in &props.available_files {
                        {
                            let is_active = current_file == *file;
                            let tab_style = if is_active {
                                "color: var(--fg-base); background: var(--bg-panel); border-bottom: 2px solid var(--accent); opacity: 1.0;"
                            } else {
                                "color: var(--fg-muted); background: transparent; border-bottom: 2px solid transparent; opacity: 0.7;"
                            };
                            rsx! {
                                button {
                                    id: "tab-{file}",
                                    class: if is_active { "{mode}-tab active" } else { "{mode}-tab" },
                                    style: "{tab_style} padding: 8px 16px; cursor: pointer; transition: all 0.2s ease;",
                                    onclick: {
                                        let f = file.to_string();
                                        let tx = props.tx.clone();
                                        move |_| {
                                            active_tab.set(f.clone());
                                            let _ = tx.send(EditorEvent::SelectFile(f.clone()));
                                        }
                                    },
                                    "{file}"
                                }
                            }
                        }
                    }
                }
                if is_native_window {
                    div { style: "display: flex; align-items: center; gap: 6px; padding: 0 12px; flex-shrink: 0;",
                        button {
                            class: "editor-mini-button",
                            style: "padding: 2px 6px; font-size: 0.75rem; border-radius: 4px; font-weight: 600;",
                            onclick: on_save,
                            "Save"
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock Left",
                            onclick: on_dock_left,
                            IconDockLeft {}
                        }
                        button {
                            class: "editor-mini-button",
                            style: "display: flex; align-items: center; padding: 4px;",
                            title: "Dock Right",
                            onclick: on_dock_right,
                            IconDockRight {}
                        }
                    }
                }
            }

            div {
                style: "display: flex; flex-direction: column; flex-grow: 1; width: 100%; min-height: 0;",
                CodeEditor {
                    value: Into::<ReadSignal<String>>::into(editor_val),
                    mode: props.mode.clone(),
                    on_change: on_change
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Props, Clone)]
pub struct EditorWindowProps {
    pub dock_props: AssetEditorProps,
    pub render_state: crate::app::state::RenderState,
    pub layout_state: crate::app::state::LayoutState,
    pub theme_state: crate::app::state::ThemeState,
    pub site_state: SiteState,
    pub vfs: crate::app::vfs::VfsDictionary,
    pub tx: tokio::sync::mpsc::UnboundedSender<EditorEvent>,
}

#[cfg(not(target_arch = "wasm32"))]
impl PartialEq for EditorWindowProps {
    fn eq(&self, other: &Self) -> bool {
        self.dock_props == other.dock_props
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn IsolatedEditorWindow(props: EditorWindowProps) -> Element {
    provide_context(props.render_state);
    provide_context(props.layout_state);
    provide_context(props.theme_state);
    provide_context(props.site_state);
    provide_context(props.vfs);
    provide_context(SubWindowMarker);
    provide_context(props.tx.clone());

    let _tx = use_signal(|| props.tx.clone());

    rsx! {
        style { "body {{ margin: 0; background-color: #16140f; color: #ece7da; overflow: hidden; }}" }
        AssetEditorDock {
            is_native_window: true,
            ..props.dock_props
        }
    }
}
