//! Application root.

use dioxus::prelude::*;

use crate::config::{MenuLink, ThemeConfig};
use crate::defaults::default_theme_config;
use crate::diagnostics::check_integrity;
use crate::render::{render_preview_html, render_theme};
use crate::ui::diagnostics_panel::DiagnosticsPanel;
use crate::ui::layout::{
    set_workbench_layout, PreviewTemplateMode, PreviewViewport, WorkbenchLayout,
};
use crate::ui::panels::{LeftPanel, RightPanel};
use crate::ui::presets_panel::ThemeSignals;

const TEMPLATE_SOURCE: &str = include_str!("template.xml");
const EDITOR_UI_CSS: &str = include_str!("editor_ui.css");

pub fn App() -> Element {
    let defaults = default_theme_config();

    // ---- Site identity ---------------------------------------------------
    let site_title = use_signal(|| defaults.site.site_title.clone());
    let site_subtitle = use_signal(|| defaults.site.site_subtitle.clone());
    let header_logo_url = use_signal(|| defaults.site.header_logo_url.clone());
    let home_url = use_signal(|| defaults.site.home_url.clone());

    // ---- Color palette ---------------------------------------------------
    let bg_base = use_signal(|| defaults.colors.bg_base.clone());
    let bg_panel = use_signal(|| defaults.colors.bg_panel.clone());
    let bg_elevated = use_signal(|| defaults.colors.bg_elevated.clone());
    let fg_base = use_signal(|| defaults.colors.fg_base.clone());
    let fg_muted = use_signal(|| defaults.colors.fg_muted.clone());
    let accent = use_signal(|| defaults.colors.accent.clone());
    let border = use_signal(|| defaults.colors.border.clone());

    // ---- Buttons ---------------------------------------------------------
    let btn_radius = use_signal(|| defaults.buttons.radius.clone());
    let btn_border_width = use_signal(|| defaults.buttons.border_width.clone());
    let btn_text_transform = use_signal(|| defaults.buttons.text_transform.clone());

    // ---- Typography ------------------------------------------------------
    let body_font_stack = use_signal(|| defaults.typography.body_font_stack.clone());
    let heading_font_stack = use_signal(|| defaults.typography.heading_font_stack.clone());
    let mono_font_stack = use_signal(|| defaults.typography.mono_font_stack.clone());
    let base_size = use_signal(|| defaults.typography.base_size.clone());
    let scale_ratio = use_signal(|| defaults.typography.scale_ratio.clone());
    let line_height = use_signal(|| defaults.typography.line_height.clone());
    let heading_weight = use_signal(|| defaults.typography.heading_weight.clone());

    // ---- Media/assets ----------------------------------------------------
    let background = use_signal(|| defaults.background.clone());
    let favicon_url = use_signal(|| defaults.assets.favicon_url.clone());
    let social_card_image_url = use_signal(|| defaults.assets.social_card_image_url.clone());

    // ---- SEO -------------------------------------------------------------
    let meta_description = use_signal(|| defaults.seo.meta_description.clone());
    let meta_keywords = use_signal(|| defaults.seo.meta_keywords.clone());
    let custom_robots = use_signal(|| defaults.seo.custom_robots.clone());
    let license_url = use_signal(|| defaults.seo.license_url.clone());
    let author_name = use_signal(|| defaults.seo.author_name.clone());

    // ---- Menu links ------------------------------------------------------
    let menu_1_label = use_signal(|| menu_label(&defaults, 0));
    let menu_1_url = use_signal(|| menu_url(&defaults, 0));
    let menu_2_label = use_signal(|| menu_label(&defaults, 1));
    let menu_2_url = use_signal(|| menu_url(&defaults, 1));
    let menu_3_label = use_signal(|| menu_label(&defaults, 2));
    let menu_3_url = use_signal(|| menu_url(&defaults, 2));
    let menu_4_label = use_signal(|| menu_label(&defaults, 3));
    let menu_4_url = use_signal(|| menu_url(&defaults, 3));

    // ---- Footer ----------------------------------------------------------
    let footer_text = use_signal(|| defaults.footer.footer_text.clone());
    let footer_license_label = use_signal(|| defaults.footer.footer_license_label.clone());
    let footer_license_url = use_signal(|| defaults.footer.footer_license_url.clone());

    // ---- Plugins ---------------------------------------------------------
    let custom_js = use_signal(|| defaults.plugins.custom_js.clone());

    // ---- Active preset CSS ----------------------------------------------
    let preset_css = use_signal(|| String::new());

    // ---- Preview / layout state -----------------------------------------
    let mut show_preview = use_signal(|| true);
    let active_preset = use_signal(|| None::<&'static str>);
    let mut left_panel_collapsed = use_signal(|| false);
    let is_dark_mode = use_signal(|| true);
    let mut workbench_layout = use_signal(|| WorkbenchLayout::Split);
    let mut layout_storage_ready = use_signal(|| false);

    // ---- Preview viewport state -----------------------------------------
    let preview_viewport = use_signal(|| PreviewViewport::Desktop);
    let preview_width = use_signal(|| 1200u32);
    let preview_template_mode = use_signal(|| PreviewTemplateMode::Sidebars);

    // ---- Global keyboard shortcuts + safer floating-panel drag via eval() ----
    use_effect(move || {
        let mut eval = eval(
            r#"
            const layoutKey = "mor_blogger_theme_editor.workbench_layout";
            const posKey    = "mor_blogger_theme_editor.floating_editor_position";

            // Restore persisted layout choice.
            const storedLayout = window.localStorage.getItem(layoutKey);
            setTimeout(function () {
                dioxus.send("restore_layout:" + (storedLayout || ""));
            }, 0);

            // Restore floating-panel position (apply when panel exists; retry briefly).
            function applyStoredFloatingPosition() {
                let raw = null;
                try { raw = window.localStorage.getItem(posKey); } catch (e) { return; }
                if (!raw) return;
                let pos;
                try { pos = JSON.parse(raw); } catch (e) { return; }
                if (!pos || typeof pos.x !== "number" || typeof pos.y !== "number") return;

                const panel = document.querySelector(".editor-left-panel");
                if (!panel) return;
                panel.style.setProperty("--floating-editor-x", pos.x + "px");
                panel.style.setProperty("--floating-editor-y", pos.y + "px");
            }

            // Try a few times because the panel may not be in the DOM on first tick.
            let restoreTries = 0;
            const restoreTimer = setInterval(function () {
                applyStoredFloatingPosition();
                restoreTries += 1;
                if (restoreTries >= 6) clearInterval(restoreTimer);
            }, 100);

            // ---- Safer floating panel drag --------------------------------
            // Rules:
            //   * Only [data-floating-window-bar="true"] starts a drag.
            //   * Clicks on buttons/inputs/links/textareas/selects pass through.
            //   * Position is saved on pointerup, NOT on every pointermove.
            //   * Panel stays inside the visible workbench bounds.

            let dragState = null;

            function clamp(v, lo, hi) {
                return Math.max(lo, Math.min(hi, v));
            }

            function getPanel() {
                return document.querySelector(".editor-layout-floating .editor-left-panel");
            }

            window.addEventListener("pointerdown", function (e) {
                if (e.button !== 0) return;
                if (!e.target || typeof e.target.closest !== "function") return;

                // Ignore drags that originate on interactive controls.
                if (e.target.closest("button, input, textarea, select, a, label")) return;

                const bar = e.target.closest('[data-floating-window-bar="true"]');
                if (!bar) return;

                const panel = getPanel();
                if (!panel) return;
                const rect = panel.getBoundingClientRect();
                dragState = {
                    panel: panel,
                    offsetX: e.clientX - rect.left,
                    offsetY: e.clientY - rect.top,
                    panelWidth: rect.width,
                    panelHeight: rect.height,
                };
                document.body.classList.add("editor-floating-dragging");
                // Do NOT preventDefault — let focus/blur work normally for any
                // text inside the bar. Buttons were already excluded above.
            }, true);

            window.addEventListener("pointermove", function (e) {
                if (!dragState) return;
                const margin = 4;
                const maxX = window.innerWidth  - dragState.panelWidth  - margin;
                const maxY = window.innerHeight - dragState.panelHeight - margin;
                const x = clamp(e.clientX - dragState.offsetX, margin, Math.max(margin, maxX));
                const y = clamp(e.clientY - dragState.offsetY, margin, Math.max(margin, maxY));

                dragState.panel.style.setProperty("--floating-editor-x", x + "px");
                dragState.panel.style.setProperty("--floating-editor-y", y + "px");
                dragState.lastX = x;
                dragState.lastY = y;
            });

            window.addEventListener("pointerup", function () {
                if (!dragState) return;
                if (typeof dragState.lastX === "number" && typeof dragState.lastY === "number") {
                    try {
                        window.localStorage.setItem(posKey, JSON.stringify({
                            x: dragState.lastX,
                            y: dragState.lastY,
                        }));
                    } catch (e) {}
                }
                document.body.classList.remove("editor-floating-dragging");
                dragState = null;
            });

            // Cancel a stuck drag if the pointer leaves the window.
            window.addEventListener("pointercancel", function () {
                if (!dragState) return;
                document.body.classList.remove("editor-floating-dragging");
                dragState = null;
            });

            // ---- Keyboard shortcuts ---------------------------------------
            window.addEventListener('keydown', function(e) {
                let k = e.key.toLowerCase();

                if (e.ctrlKey || e.metaKey) {
                    if (k === 'b') { e.preventDefault(); dioxus.send("toggle_left"); }
                    if (k === 'e') { e.preventDefault(); dioxus.send("toggle_right"); }
                }

                // Workbench layout shortcuts.
                // Alt+1..4 avoids the browser tab-switching behavior of Ctrl+1..4.
                if (e.altKey && !e.ctrlKey && !e.metaKey && !e.shiftKey) {
                    if (k === '1') { e.preventDefault(); dioxus.send("layout_split"); }
                    if (k === '2') { e.preventDefault(); dioxus.send("layout_wide"); }
                    if (k === '3') { e.preventDefault(); dioxus.send("layout_float"); }
                    if (k === '4') { e.preventDefault(); dioxus.send("layout_preview"); }
                }
            });
            "#,
        );

        spawn(async move {
            while let Ok(value) = eval.recv().await {
                if let Some(cmd) = value.as_str() {
                    match cmd {
                        "toggle_left" => left_panel_collapsed.set(!left_panel_collapsed()),
                        "toggle_right" => show_preview.set(!show_preview()),
                        "layout_split" => {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::Split);
                        }
                        "layout_wide" => {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::WideEditor);
                        }
                        "layout_float" => {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::FloatingEditor);
                        }
                        "layout_preview" => {
                            set_workbench_layout(workbench_layout, WorkbenchLayout::PreviewTakeover);
                        }
                        _ if cmd.starts_with("restore_layout:") => {
                            let value = cmd.trim_start_matches("restore_layout:");
                            if let Some(layout) = WorkbenchLayout::from_storage_value(value) {
                                workbench_layout.set(layout);
                            }
                            layout_storage_ready.set(true);
                        }
                        _ => {}
                    }
                }
            }
        });
    });


    // ---- Persist selected workbench layout -------------------------------
    use_effect(move || {
        let storage_ready = layout_storage_ready();
        let layout_value = workbench_layout().storage_value().to_string();

        if storage_ready {
            spawn(async move {
                let eval = eval(
                    r#"
                    let value = await dioxus.recv();
                    window.localStorage.setItem("mor_blogger_theme_editor.workbench_layout", value);
                    "#
                );

                let _ = eval.send(layout_value.into());
            });
        }
    });

    let signals = ThemeSignals {
        is_dark_mode,
        site_title,
        site_subtitle,
        header_logo_url,
        home_url,
        bg_base,
        bg_panel,
        bg_elevated,
        fg_base,
        fg_muted,
        accent,
        border,
        btn_radius,
        btn_border_width,
        btn_text_transform,
        body_font_stack,
        heading_font_stack,
        mono_font_stack,
        base_size,
        scale_ratio,
        line_height,
        heading_weight,
        background,
        favicon_url,
        social_card_image_url,
        meta_description,
        meta_keywords,
        custom_robots,
        license_url,
        author_name,
        menu_1_label,
        menu_1_url,
        menu_2_label,
        menu_2_url,
        menu_3_label,
        menu_3_url,
        menu_4_label,
        menu_4_url,
        footer_text,
        footer_license_label,
        footer_license_url,
        custom_js,
        preset_css,
    };

    let current_config = ThemeConfig {
        site: crate::config::SiteConfig {
            site_title: site_title(),
            site_subtitle: site_subtitle(),
            header_logo_url: header_logo_url(),
            home_url: home_url(),
        },
        colors: crate::config::ColorConfig {
            bg_base: bg_base(),
            bg_panel: bg_panel(),
            bg_elevated: bg_elevated(),
            fg_base: fg_base(),
            fg_muted: fg_muted(),
            accent: accent(),
            border: border(),
        },
        buttons: crate::config::ButtonConfig {
            radius: btn_radius(),
            border_width: btn_border_width(),
            text_transform: btn_text_transform(),
        },
        typography: crate::config::TypographyConfig {
            body_font_stack: body_font_stack(),
            heading_font_stack: heading_font_stack(),
            mono_font_stack: mono_font_stack(),
            base_size: base_size(),
            scale_ratio: scale_ratio(),
            line_height: line_height(),
            heading_weight: heading_weight(),
        },
        background: background(),
        assets: crate::config::AssetConfig {
            favicon_url: favicon_url(),
            social_card_image_url: social_card_image_url(),
        },
        seo: crate::config::SeoConfig {
            meta_description: meta_description(),
            meta_keywords: meta_keywords(),
            custom_robots: custom_robots(),
            license_url: license_url(),
            author_name: author_name(),
        },
        menu_links: vec![
            MenuLink {
                label: menu_1_label(),
                url: menu_1_url(),
            },
            MenuLink {
                label: menu_2_label(),
                url: menu_2_url(),
            },
            MenuLink {
                label: menu_3_label(),
                url: menu_3_url(),
            },
            MenuLink {
                label: menu_4_label(),
                url: menu_4_url(),
            },
        ],
        footer: crate::config::FooterConfig {
            footer_text: footer_text(),
            footer_license_label: footer_license_label(),
            footer_license_url: footer_license_url(),
        },
        plugins: crate::config::PluginConfig {
            custom_js: custom_js(),
        },
        preset_css: preset_css(),
    };

    let generated_xml = render_theme(&current_config);
    let preview_html = render_preview_html(&current_config, preview_template_mode());
    let diag = use_signal(|| check_integrity(TEMPLATE_SOURCE));
    let layout_class = format!("editor-main {}", workbench_layout().as_class());

    rsx! {
        style { "{EDITOR_UI_CSS}" }

        div {
            class: "editor-shell",

            div {
                class: "{layout_class}",

                LeftPanel {
                    is_collapsed: left_panel_collapsed,
                    workbench_layout,
                    signals,
                    active_preset,

                    site_title,
                    site_subtitle,
                    header_logo_url,
                    home_url,

                    bg_base,
                    bg_panel,
                    bg_elevated,
                    fg_base,
                    fg_muted,
                    accent,
                    border,

                    btn_radius,
                    btn_border_width,
                    btn_text_transform,

                    body_font_stack,
                    heading_font_stack,
                    mono_font_stack,
                    base_size,
                    scale_ratio,
                    line_height,
                    heading_weight,

                    background,
                    favicon_url,
                    social_card_image_url,

                    menu_1_label,
                    menu_1_url,
                    menu_2_label,
                    menu_2_url,
                    menu_3_label,
                    menu_3_url,
                    menu_4_label,
                    menu_4_url,

                    meta_description,
                    meta_keywords,
                    custom_robots,
                    author_name,
                    license_url,

                    footer_text,
                    footer_license_label,
                    footer_license_url,

                    custom_js,
                }

                RightPanel {
                    workbench_layout,
                    preview_viewport,
                    preview_width,
                    preview_template_mode,

                    generated_xml,
                    preview_html,
                    show_preview,
                    diag,

                    config_toml: toml::to_string_pretty(&current_config).unwrap_or_default(),

                    on_load_theme: move |toml_text: String| {
                        match toml::from_str::<ThemeConfig>(&toml_text) {
                            Ok(new_config) => apply_theme_config_to_signals(signals, new_config),
                            Err(err) => println!("Failed to load TOML theme: {}", err),
                        }
                    },

                    on_restore: move |new_config: ThemeConfig| {
                        println!("Restoring config from embedded Blogger XML: {:#?}", new_config);
                        apply_theme_config_to_signals(signals, new_config);
                    },
                }
            }

            footer {
                class: "editor-footer",
                DiagnosticsPanel { result: diag }
            }
        }
    }
}

fn apply_theme_config_to_signals(signals: ThemeSignals, new_config: ThemeConfig) {
    let menu_1_label_value = menu_label(&new_config, 0);
    let menu_1_url_value = menu_url(&new_config, 0);
    let menu_2_label_value = menu_label(&new_config, 1);
    let menu_2_url_value = menu_url(&new_config, 1);
    let menu_3_label_value = menu_label(&new_config, 2);
    let menu_3_url_value = menu_url(&new_config, 2);
    let menu_4_label_value = menu_label(&new_config, 3);
    let menu_4_url_value = menu_url(&new_config, 3);

    signals.site_title.clone().set(new_config.site.site_title);
    signals.site_subtitle.clone().set(new_config.site.site_subtitle);
    signals.header_logo_url.clone().set(new_config.site.header_logo_url);
    signals.home_url.clone().set(new_config.site.home_url);

    signals.bg_base.clone().set(new_config.colors.bg_base);
    signals.bg_panel.clone().set(new_config.colors.bg_panel);
    signals.bg_elevated.clone().set(new_config.colors.bg_elevated);
    signals.fg_base.clone().set(new_config.colors.fg_base);
    signals.fg_muted.clone().set(new_config.colors.fg_muted);
    signals.accent.clone().set(new_config.colors.accent);
    signals.border.clone().set(new_config.colors.border);

    signals.btn_radius.clone().set(new_config.buttons.radius);
    signals.btn_border_width.clone().set(new_config.buttons.border_width);
    signals.btn_text_transform.clone().set(new_config.buttons.text_transform);

    signals
        .body_font_stack
        .clone()
        .set(new_config.typography.body_font_stack);
    signals
        .heading_font_stack
        .clone()
        .set(new_config.typography.heading_font_stack);
    signals
        .mono_font_stack
        .clone()
        .set(new_config.typography.mono_font_stack);
    signals.base_size.clone().set(new_config.typography.base_size);
    signals.scale_ratio.clone().set(new_config.typography.scale_ratio);
    signals.line_height.clone().set(new_config.typography.line_height);
    signals
        .heading_weight
        .clone()
        .set(new_config.typography.heading_weight);

    signals.background.clone().set(new_config.background);

    signals.favicon_url.clone().set(new_config.assets.favicon_url);
    signals
        .social_card_image_url
        .clone()
        .set(new_config.assets.social_card_image_url);

    signals
        .meta_description
        .clone()
        .set(new_config.seo.meta_description);
    signals.meta_keywords.clone().set(new_config.seo.meta_keywords);
    signals.custom_robots.clone().set(new_config.seo.custom_robots);
    signals.license_url.clone().set(new_config.seo.license_url);
    signals.author_name.clone().set(new_config.seo.author_name);

    signals.menu_1_label.clone().set(menu_1_label_value);
    signals.menu_1_url.clone().set(menu_1_url_value);
    signals.menu_2_label.clone().set(menu_2_label_value);
    signals.menu_2_url.clone().set(menu_2_url_value);
    signals.menu_3_label.clone().set(menu_3_label_value);
    signals.menu_3_url.clone().set(menu_3_url_value);
    signals.menu_4_label.clone().set(menu_4_label_value);
    signals.menu_4_url.clone().set(menu_4_url_value);

    signals.footer_text.clone().set(new_config.footer.footer_text);
    signals
        .footer_license_label
        .clone()
        .set(new_config.footer.footer_license_label);
    signals
        .footer_license_url
        .clone()
        .set(new_config.footer.footer_license_url);

    signals.custom_js.clone().set(new_config.plugins.custom_js);
    signals.preset_css.clone().set(new_config.preset_css);
}

fn menu_label(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|l| l.label.clone())
        .unwrap_or_default()
}

fn menu_url(config: &ThemeConfig, index: usize) -> String {
    config
        .menu_links
        .get(index)
        .map(|l| l.url.clone())
        .unwrap_or_default()
}