use super::hotswap::apply_hotswap_json;
use crate::app::state::{
    CenterView, ContextMenuPayload, DockPosition, LayoutState, RenderState, ThemeState,
};
use crate::ui::components::icon_context_menu::IconContextMenu;
use crate::ui::components::icons::{IconBug, IconCode, IconPalette, IconPlugin, IconPreset, IconSiteData, IconXml};
use crate::ui::layout::docks::{
    CssEditorPanel, JsEditorPanel, SiteDataDock, ThemePaletteDock,
    DiagnosticsDock, PluginManagerDock, CssBuilderDock, JsBuilderDock, TemplateModulesDock,
    CodeNavDock, StaticPagesDock, WidgetsDock,
};
use crate::ui::panels::quick_launch_bar::LaunchButton;
use crate::ui::panels::theme_palette::effects_panel_2::AdvancedGlowWindow;
use crate::ui::panels::theme_palette::presets::{PresetFloatingWindow, PresetsPanel};
use crate::ui::panels::theme_palette::static_pages_panel::StaticPagesFloatingWindow;
use crate::ui::workspace::blogger_workspace::BloggerWorkspace;
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;

#[derive(Clone, Copy)]
pub struct DockLocalSignals {
    pub show_preview: Signal<bool>,
    pub show_undocked_pages: Signal<bool>,
    pub tv_monitor: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ActivityBarButtonProps {
    pub dock_name: &'static str,
    pub dock_id: &'static str,
    pub pos_signal: Signal<DockPosition>,
    pub icon_kind: &'static str,
}

/// Render a dock's activity-bar icon from an optional override spec
/// ("emoji:…", "svg:name", "raw:<svg…>"), falling back to the built-in default.
fn render_dock_icon(spec: Option<String>, default_kind: &str) -> Element {
    // Override wins; else the dock's built-in default (which may itself be an
    // "emoji:"/"raw:"/"svg:" spec, or a bare built-in svg name).
    let chosen = spec
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(default_kind);
    render_icon_spec(chosen)
}

fn render_icon_spec(s: &str) -> Element {
    if let Some(e) = s.strip_prefix("emoji:") {
        return rsx! { span { class: "activity-emoji", "{e}" } };
    }
    if let Some(raw) = s.strip_prefix("raw:") {
        return rsx! { span { class: "activity-raw-icon", dangerous_inner_html: "{raw}" } };
    }
    let name = s.strip_prefix("svg:").unwrap_or(s);
    render_named_svg(name)
}

pub fn render_named_svg(kind: &str) -> Element {
    match kind {
        "palette" => rsx! { IconPalette {} },
        "site_data" => rsx! { IconSiteData {} },
        "xml" => rsx! { IconXml {} },
        "plugin" => rsx! { IconPlugin {} },
        "preset" => rsx! { IconPreset {} },
        "bug" => rsx! { IconBug {} },
        _ => rsx! { IconCode {} },
    }
}

#[component]
pub fn ActivityBarButton(props: ActivityBarButtonProps) -> Element {
    let mut layout = use_context::<LayoutState>();
    let current_pos = (props.pos_signal)();
    
    let is_open = current_pos != DockPosition::Hidden;
    let is_pinned = layout.is_dock_pinned(props.dock_id);
    let icon_override = layout
        .activity_icons
        .read()
        .get(&crate::app::state::normalize_dock_key(props.dock_id))
        .cloned();

    rsx! {
        div {
            class: "activity-btn-wrap",
            LaunchButton {
                is_active: is_open,
                is_visible: true,
                tooltip: props.dock_name.to_string(),
                onclick: move |_| {
                    let pos = (props.pos_signal)();
                    if pos == DockPosition::Hidden || pos == DockPosition::Floating {
                        // Closed → open it.
                        layout.request_exclusive_dock(props.dock_id, DockPosition::mor_panel_left);
                    } else if layout.is_dock_pinned(props.dock_id) {
                        // Open + pinned → toggle closed; the icon stays (it's pinned).
                        let mut sig = props.pos_signal;
                        sig.set(DockPosition::Hidden);
                    }
                    // Open + unpinned → no-op. Left-click never makes an icon vanish;
                    // close it from the dock's × or right-click → Unpin.
                },
                oncontextmenu: move |e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    let coords = e.client_coordinates();
                    layout.active_context_menu.set(Some(ContextMenuPayload {
                        x: coords.x,
                        y: coords.y,
                        kind: "dock".to_string(),
                        target_id: props.dock_id.to_string(),
                    }));
                },
                {render_dock_icon(icon_override, props.icon_kind)}
            }
            // Running indicator for an open dock that isn't pinned.
            if is_open && !is_pinned {
                span { class: "activity-running-dot", title: "Open — right-click to pin" }
            }
        }
    }
}

#[component]
pub fn ActivityBar() -> Element {
    let layout = use_context::<LayoutState>();

    // Global Dock Registry
    let docks = [
        ("Theme Palette", "theme_palette", layout.theme_palette_pos, "palette"),
        ("Site Data", "site_data", layout.site_data_pos, "emoji:📊"),
        ("Template Modules", "template_modules", layout.template_modules_pos, "emoji:🍱"),
        ("Widgets", "widgets", layout.widgets_pos, "emoji:🧩"),
        ("Code Nav", "code_nav", layout.code_nav_pos, "emoji:🧭"),
        ("Static Pages", "static_pages", layout.static_pages_pos, "emoji:📄"),
        ("CSS Editor", "css_editor", layout.css_editor_pos, "emoji:🖌️"),
        ("JS Editor", "js_editor", layout.js_editor_pos, "emoji:🔧"),
        ("Presets", "presets", layout.presets_pos, "preset"),
        ("Plugin Manager", "plugin_manager", layout.plugin_manager_pos, "plugin"),
        ("Diagnostics", "diagnostics", layout.diagnostics_pos, "bug"),
        ("CSS Builder", "css_builder", layout.css_builder_pos, "palette"),
        ("JS Builder", "js_builder", layout.js_builder_pos, "code"),
    ];

    // Taskbar model: show pinned docks (in pinned order), then any open-but-unpinned
    // docks as temporary "running" entries.
    let pinned = layout.pinned_docks.read().clone();
    let mut ordered: Vec<(&str, &str, Signal<DockPosition>, &str)> = Vec::new();
    for pid in &pinned {
        if let Some(d) = docks.iter().find(|d| d.1 == pid) {
            ordered.push(*d);
        }
    }
    for d in docks.iter() {
        let open = (d.2)() != DockPosition::Hidden;
        if open && !pinned.iter().any(|p| p == d.1) {
            ordered.push(*d);
        }
    }

    rsx! {
        aside {
            class: "mor-quick-launch-bar",
            style: "border-right: 1px solid var(--border-soft, #333); height: 100%; overflow-y: auto;",
            for (name, id, sig, icon) in ordered {
                ActivityBarButton {
                    key: "{id}",
                    dock_name: name,
                    dock_id: id,
                    pos_signal: sig,
                    icon_kind: icon,
                }
            }
        }
    }
}

#[component]
pub fn DockZone(position: DockPosition) -> Element {
    let layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();
    let render = use_context::<RenderState>();
    let local = use_context::<DockLocalSignals>();

    let active_tab = match position {
        DockPosition::mor_panel_left => layout.active_left_tab,
        _ => layout.active_right_tab,
    };

    let show_template_modules = (layout.template_modules_pos)() == position;
    let show_widgets = (layout.widgets_pos)() == position;
    let show_code_nav = (layout.code_nav_pos)() == position;
    let show_static_pages = (layout.static_pages_pos)() == position;
    let show_theme_palette = (layout.theme_palette_pos)() == position;
    let show_site_data = (layout.site_data_pos)() == position;
    let show_presets = (layout.presets_pos)() == position;
    let show_css_editor = (layout.css_editor_pos)() == position;
    let show_js_editor = (layout.js_editor_pos)() == position;
    let show_diagnostics = (layout.diagnostics_pos)() == position;
    let show_plugin_manager = (layout.plugin_manager_pos)() == position;
    let show_css_builder = (layout.css_builder_pos)() == position;
    let show_js_builder = (layout.js_builder_pos)() == position;

    // Workbench tool: wins the zone over palette/site-data while active.
    if show_template_modules {
        return rsx! {
            TemplateModulesDock {}
        };
    }

    if show_widgets {
        return rsx! {
            WidgetsDock {}
        };
    }

    if show_code_nav {
        return rsx! {
            CodeNavDock {}
        };
    }

    if show_static_pages {
        return rsx! {
            StaticPagesDock {
                signals: theme.signals,
                show_undocked_pages: local.show_undocked_pages,
                preview_html: local.tv_monitor,
                base_preview_html: render.preview_html,
            }
        };
    }

    if show_css_editor {
        return rsx! {
            CssEditorPanel {}
        };
    }

    if show_js_editor {
        return rsx! {
            JsEditorPanel {}
        };
    }

    if show_diagnostics {
        return rsx! {
            DiagnosticsDock {}
        };
    }

    if show_plugin_manager {
        return rsx! {
            PluginManagerDock {}
        };
    }

    if show_css_builder {
        return rsx! {
            CssBuilderDock {}
        };
    }

    if show_js_builder {
        return rsx! {
            JsBuilderDock {}
        };
    }

    if show_theme_palette {
        return rsx! {
            ThemePaletteDock {
                active_tab,
                active_preset: theme.active_preset,
                signals: theme.signals,
                show_preview: local.show_preview,
                current_config: (render.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    let mut theme = theme;
                    theme.signals.apply_config(&new_config);
                    theme.active_preset.set(None);
                    theme.commit();
                },
                show_undocked_presets: theme.show_undocked_presets,
                show_undocked_pages: local.show_undocked_pages,
                show_advanced_glow: theme.show_advanced_glow,
                preview_html: local.tv_monitor,
                base_preview_html: render.preview_html,
            }
        };
    }

    if show_site_data {
        return rsx! {
            SiteDataDock {
                active_tab,
                signals: theme.signals,
                current_config: (render.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    theme.signals.apply_config(&new_config);
                },
            }
        };
    }

    if show_presets {
        return rsx! {
            PresetsPanel {
                active_preset: theme.active_preset,
                signals: theme.signals,
                current_config: (render.current_config)(),
                on_apply_theme: move |new_config: ThemeConfig| {
                    let mut theme = theme;
                    theme.signals.apply_config(&new_config);
                    theme.active_preset.set(None);
                    theme.commit();
                },
                show_undocked_presets: theme.show_undocked_presets,
            }
        };
    }

    rsx! {}
}

#[derive(Props, Clone, PartialEq)]
pub struct MorLayoutChromeProps {
    pub show_preview: Signal<bool>,
    pub center_view: Signal<CenterView>,
    pub tv_monitor: Signal<String>,
    pub config_toml_signal: Memo<String>,
    pub active_preset: Signal<Option<&'static str>>,
    pub original_toml: Signal<String>,
}

#[component]
pub fn MorLayoutChrome(props: MorLayoutChromeProps) -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let theme = use_context::<ThemeState>();
    let signals = theme.signals;

    let left_active = use_memo(move || {
        (layout.theme_palette_pos)() == DockPosition::mor_panel_left
            || (layout.site_data_pos)() == DockPosition::mor_panel_left
            || (layout.css_editor_pos)() == DockPosition::mor_panel_left
            || (layout.js_editor_pos)() == DockPosition::mor_panel_left
            || (layout.presets_pos)() == DockPosition::mor_panel_left
            || (layout.plugin_manager_pos)() == DockPosition::mor_panel_left
            || (layout.diagnostics_pos)() == DockPosition::mor_panel_left
            || (layout.css_builder_pos)() == DockPosition::mor_panel_left
            || (layout.js_builder_pos)() == DockPosition::mor_panel_left
            || (layout.template_modules_pos)() == DockPosition::mor_panel_left
            || (layout.widgets_pos)() == DockPosition::mor_panel_left
            || (layout.code_nav_pos)() == DockPosition::mor_panel_left
            || (layout.static_pages_pos)() == DockPosition::mor_panel_left
    });

    let right_active = use_memo(move || {
        (layout.theme_palette_pos)() == DockPosition::mor_panel_right
            || (layout.site_data_pos)() == DockPosition::mor_panel_right
            || (layout.css_editor_pos)() == DockPosition::mor_panel_right
            || (layout.js_editor_pos)() == DockPosition::mor_panel_right
            || (layout.presets_pos)() == DockPosition::mor_panel_right
            || (layout.plugin_manager_pos)() == DockPosition::mor_panel_right
            || (layout.diagnostics_pos)() == DockPosition::mor_panel_right
            || (layout.css_builder_pos)() == DockPosition::mor_panel_right
            || (layout.js_builder_pos)() == DockPosition::mor_panel_right
            || (layout.template_modules_pos)() == DockPosition::mor_panel_right
            || (layout.widgets_pos)() == DockPosition::mor_panel_right
            || (layout.code_nav_pos)() == DockPosition::mor_panel_right
            || (layout.static_pages_pos)() == DockPosition::mor_panel_right
    });

    let grid_style = use_memo(move || {
        let l_w = if left_active() { "var(--left-pane-width, 360px)" } else { "0px" };
        let r_w = if right_active() { "var(--right-pane-width, 360px)" } else { "0px" };
        // Cleaned up: Single Activity Bar on the far left. No right-side bar slot.
        format!("grid-template-columns: 48px {} 1fr {};", l_w, r_w)
    });

    let left_layout_attr = use_memo(move || if left_active() { "split" } else { "hidden" });
    let right_layout_attr = use_memo(move || if right_active() { "split" } else { "hidden" });
    let left_pinned_attr = use_memo(move || left_active().to_string());
    let right_pinned_attr = use_memo(move || right_active().to_string());

    rsx! {
        div {
            class: "editor-main",
            style: grid_style,
            "data-left-layout": left_layout_attr,
            "data-right-layout": right_layout_attr,
            "data-left-pinned": left_pinned_attr,
            "data-right-pinned": right_pinned_attr,

            // Single unified Activity Bar
            ActivityBar {}

            LeftPanelContainer {}

            BloggerWorkspace {
                preview_viewport: layout.preview_viewport,
                preview_width: layout.preview_width,
                preview_html: props.tv_monitor,
                show_preview: props.show_preview,
                center_view: props.center_view,
                diag: render.diag,
                config_toml: props.config_toml_signal,
                active_preset: props.active_preset,
                on_load_theme: {
                    let mut original_toml = props.original_toml;
                    move |toml_text: String| {
                        if let Ok(new_config) = toml::from_str::<ThemeConfig>(&toml_text) {
                            signals.apply_config(&new_config);
                        }
                        original_toml.set(toml_text);
                    }
                },
                on_restore: move |new_config: ThemeConfig| {
                    signals.apply_config(&new_config);
                },
                on_load_hotswap: move |json_text: String| {
                    apply_hotswap_json(signals, json_text);
                },
                on_navigate: {
                    let mut tv_monitor = props.tv_monitor;
                    move |href: String| {
                        let base = (render.preview_html)();
                        let pages = (signals.static_pages)();

                        let new_html = if href.contains("archive") {
                            mor_blogger_core::render::pages::generate_archive_html(&pages.archive)
                        } else if href.contains("categories") || href.contains("directory") {
                            mor_blogger_core::render::pages::generate_categories_html(&pages.categories)
                        } else if href.contains("about") {
                            mor_blogger_core::render::pages::generate_about_html(&pages.about)
                        } else if href.contains("portfolio") {
                            mor_blogger_core::render::pages::generate_portfolio_html(&pages.portfolio)
                        } else if href.contains("my-courses") || href.contains("dashboard") {
                            mor_blogger_core::render::pages::generate_my_courses_html(&pages.lms)
                        } else if href.contains("catalog") || href.contains("lessons") || href.contains("courses") {
                            mor_blogger_core::render::pages::generate_course_catalog_html(&pages.lms)
                        } else {
                            tv_monitor.set(base);
                            return;
                        };

                        tv_monitor.set(
                            crate::ui::panels::theme_palette::static_pages_panel::inject_static_page(&base, &new_html),
                        );
                    }
                },
                on_toggle_dark_mode: {
                    let theme_state = theme;
                    move |_| {
                        theme_state.perform_dark_mode_toggle();
                    }
                },
            }

            RightPanelContainer {}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FloatingWindowManagerProps {
    pub show_preview: Signal<bool>,
    pub show_undocked_pages: Signal<bool>,
    pub tv_monitor: Signal<String>,
}

#[component]
pub fn FloatingWindowManager(props: FloatingWindowManagerProps) -> Element {
    let layout = use_context::<LayoutState>();
    let theme = use_context::<ThemeState>();
    let render = use_context::<RenderState>();

    let signals = theme.signals;
    let active_preset = theme.active_preset;
    let show_undocked_presets = theme.show_undocked_presets;
    let show_advanced_glow = theme.show_advanced_glow;

    rsx! {
        div {
            class: "window-manager-layer",
            style: "position: absolute; top: 0; left: 0; width: 100vw; height: 100vh; pointer-events: none; z-index: 9999;",

            if (layout.theme_palette_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    ThemePaletteDock {
                        active_tab: layout.active_left_tab,
                        active_preset,
                        signals: theme.signals,
                        show_preview: props.show_preview,
                        current_config: (render.current_config)(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            let mut theme = theme;
                            theme.signals.apply_config(&new_config);
                            theme.active_preset.set(None);
                            theme.commit();
                        },
                        show_undocked_presets,
                        show_undocked_pages: props.show_undocked_pages,
                        show_advanced_glow,
                        preview_html: props.tv_monitor,
                        base_preview_html: render.preview_html,
                    }
                }
            }

            if (layout.site_data_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    SiteDataDock {
                        active_tab: layout.active_right_tab,
                        signals: theme.signals,
                        current_config: (render.current_config)(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            theme.signals.apply_config(&new_config);
                        },
                    }
                }
            }

            if (layout.static_pages_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    StaticPagesDock {
                        signals: theme.signals,
                        show_undocked_pages: props.show_undocked_pages,
                        preview_html: props.tv_monitor,
                        base_preview_html: render.preview_html,
                    }
                }
            }

            // GLOBAL TOOLS
            if (layout.css_editor_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    CssEditorPanel {}
                }
            }

            if (layout.js_editor_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    JsEditorPanel {}
                }
            }

            if (layout.diagnostics_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    DiagnosticsDock {}
                }
            }

            if (layout.plugin_manager_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    PluginManagerDock {}
                }
            }

            if (layout.css_builder_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    CssBuilderDock {}
                }
            }

            if (layout.js_builder_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    JsBuilderDock {}
                }
            }

            if (layout.widgets_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    WidgetsDock {}
                }
            }

            // ADD THIS BLOCK: Rescue Presets from the void
            if (layout.presets_pos)() == DockPosition::Floating {
                div { style: "pointer-events: auto;",
                    PresetsPanel {
                        active_preset,
                        signals,
                        current_config: (render.current_config)(),
                        on_apply_theme: move |new_config: ThemeConfig| {
                            let mut theme = theme;
                            theme.signals.apply_config(&new_config);
                            theme.active_preset.set(None);
                            theme.commit();
                        },
                        show_undocked_presets,
                    }
                }
            }

            if show_undocked_presets() {
                div { style: "pointer-events: auto;",
                    PresetFloatingWindow { signals, active_preset, show_undocked_presets }
                }
            }

            if show_advanced_glow() {
                div { style: "pointer-events: auto;",
                    AdvancedGlowWindow { show_advanced_glow, signals: signals.clone() }
                }
            }

            if (props.show_undocked_pages)() {
                div { style: "pointer-events: auto;",
                    StaticPagesFloatingWindow { signals, show_undocked_pages: props.show_undocked_pages, preview_html: props.tv_monitor, base_preview_html: render.preview_html }
                }
            }

            // Absolute context menu prevents structural identity DOM shredding
            div { 
                style: "position: absolute; top: 0; left: 0; pointer-events: none; z-index: 9999;",
                if let Some(payload) = (layout.active_context_menu)() {
                    div {
                        style: "pointer-events: auto;",
                        IconContextMenu { payload: payload.clone() }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LeftPanelContainer() -> Element {
    let layout = use_context::<LayoutState>();
    
    let has_left_dock = (layout.theme_palette_pos)() == DockPosition::mor_panel_left
        || (layout.site_data_pos)() == DockPosition::mor_panel_left
        || (layout.css_editor_pos)() == DockPosition::mor_panel_left
        || (layout.js_editor_pos)() == DockPosition::mor_panel_left
        || (layout.presets_pos)() == DockPosition::mor_panel_left
        || (layout.plugin_manager_pos)() == DockPosition::mor_panel_left
        || (layout.diagnostics_pos)() == DockPosition::mor_panel_left
        || (layout.css_builder_pos)() == DockPosition::mor_panel_left
        || (layout.js_builder_pos)() == DockPosition::mor_panel_left
        || (layout.template_modules_pos)() == DockPosition::mor_panel_left
        || (layout.widgets_pos)() == DockPosition::mor_panel_left
        || (layout.code_nav_pos)() == DockPosition::mor_panel_left
        || (layout.static_pages_pos)() == DockPosition::mor_panel_left;

    let display_style = if has_left_dock { "display: flex;" } else { "display: none;" };

    rsx! {
        div { class: "panel-container left left-dock-container", style: "{display_style}",
            if has_left_dock {
                DockZone { position: DockPosition::mor_panel_left }
            }
        }
    }
}

#[component]
pub fn RightPanelContainer() -> Element {
    let layout = use_context::<LayoutState>();

    let has_right_dock = (layout.theme_palette_pos)() == DockPosition::mor_panel_right
        || (layout.site_data_pos)() == DockPosition::mor_panel_right
        || (layout.css_editor_pos)() == DockPosition::mor_panel_right
        || (layout.js_editor_pos)() == DockPosition::mor_panel_right
        || (layout.presets_pos)() == DockPosition::mor_panel_right
        || (layout.plugin_manager_pos)() == DockPosition::mor_panel_right
        || (layout.diagnostics_pos)() == DockPosition::mor_panel_right
        || (layout.css_builder_pos)() == DockPosition::mor_panel_right
        || (layout.js_builder_pos)() == DockPosition::mor_panel_right
        || (layout.template_modules_pos)() == DockPosition::mor_panel_right
        || (layout.widgets_pos)() == DockPosition::mor_panel_right
        || (layout.code_nav_pos)() == DockPosition::mor_panel_right
        || (layout.static_pages_pos)() == DockPosition::mor_panel_right;

    let display_style = if has_right_dock { "display: flex;" } else { "display: none;" };

    rsx! {
        div { class: "panel-container right right-dock-container", style: "{display_style}",
            if has_right_dock {
                DockZone { position: DockPosition::mor_panel_right }
            }
        }
    }
}