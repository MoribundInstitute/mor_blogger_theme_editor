//! Application root.

use dioxus::prelude::*;

use crate::config::{MenuLink, ThemeConfig};
use crate::defaults::default_theme_config;
use crate::diagnostics::check_integrity;
use crate::render::{render_preview_html, render_theme};
use crate::ui::diagnostics_panel::DiagnosticsPanel;
use crate::ui::layout::{PanelLayout, PreviewTemplateMode, PreviewViewport};
use crate::ui::panel_center_workspace::CenterWorkspacePanel;
use crate::ui::panel_left_visuals::LeftVisualsPanel;
use crate::ui::panel_right_data::RightDataPanel;
use crate::ui::presets_panel::ThemeSignals;

const EDITOR_UI_CSS: &str = include_str!("editor_ui.css");

// EXTRACTED TO CONST TO PREVENT MACRO PARSING CRASHES
const DRAG_JS: &str = r#"
document.addEventListener('pointerdown', (e) => {
    const bar = e.target.closest('.floating-editor-window-bar');
    if (!bar) return;
    
    // Don't drag if clicking a button inside the bar
    if (e.target.closest('button, input, textarea, select, a, label')) return;

    const panel = bar.closest('.editor-left-panel, .editor-right-panel');
    if (!panel) return;

    e.preventDefault();
    
    const isLeft = panel.classList.contains('editor-left-panel');
    const varX = isLeft ? '--floating-left-x' : '--floating-right-x';
    const varY = isLeft ? '--floating-left-y' : '--floating-right-y';

    const startX = e.clientX;
    const startY = e.clientY;
    const rect = panel.getBoundingClientRect();

    // Calculate starting positions based on CSS orientation
    let startPosX = isLeft ? rect.left : (window.innerWidth - rect.right);
    let startPosY = rect.top;

    document.body.classList.add('editor-floating-dragging');

    const onMove = (moveEvt) => {
        const dx = moveEvt.clientX - startX;
        const dy = moveEvt.clientY - startY;

        // Right panel moves opposite on the X axis because it's anchored right
        let newX = isLeft ? (startPosX + dx) : (startPosX - dx);
        let newY = startPosY + dy;

        // Bounds checking so it can't be dragged off screen
        newY = Math.max(0, Math.min(newY, window.innerHeight - 60));
        newX = Math.max(0, Math.min(newX, window.innerWidth - 100));

        document.documentElement.style.setProperty(varX, newX + 'px');
        document.documentElement.style.setProperty(varY, newY + 'px');
    };

    const onUp = () => {
        document.removeEventListener('pointermove', onMove);
        document.removeEventListener('pointerup', onUp);
        document.body.classList.remove('editor-floating-dragging');
    };

    document.addEventListener('pointermove', onMove);
    document.addEventListener('pointerup', onUp);
});
"#;

pub fn App() -> Element {
    let defaults = default_theme_config();

    let site_title = use_signal(|| defaults.site.site_title.clone());
    let site_subtitle = use_signal(|| defaults.site.site_subtitle.clone());
    let header_logo_url = use_signal(|| defaults.site.header_logo_url.clone());
    let home_url = use_signal(|| defaults.site.home_url.clone());

    let bg_base = use_signal(|| defaults.colors.bg_base.clone());
    let bg_panel = use_signal(|| defaults.colors.bg_panel.clone());
    let bg_elevated = use_signal(|| defaults.colors.bg_elevated.clone());
    let fg_base = use_signal(|| defaults.colors.fg_base.clone());
    let fg_muted = use_signal(|| defaults.colors.fg_muted.clone());
    let accent = use_signal(|| defaults.colors.accent.clone());
    let border = use_signal(|| defaults.colors.border.clone());

    let btn_radius = use_signal(|| defaults.buttons.radius.clone());
    let btn_border_width = use_signal(|| defaults.buttons.border_width.clone());
    let btn_text_transform = use_signal(|| defaults.buttons.text_transform.clone());

    let body_font_stack = use_signal(|| defaults.typography.body_font_stack.clone());
    let heading_font_stack = use_signal(|| defaults.typography.heading_font_stack.clone());
    let mono_font_stack = use_signal(|| defaults.typography.mono_font_stack.clone());
    let base_size = use_signal(|| defaults.typography.base_size.clone());
    let scale_ratio = use_signal(|| defaults.typography.scale_ratio.clone());
    let line_height = use_signal(|| defaults.typography.line_height.clone());
    let heading_weight = use_signal(|| defaults.typography.heading_weight.clone());

    let background = use_signal(|| defaults.background.clone());
    let favicon_url = use_signal(|| defaults.assets.favicon_url.clone());
    let social_card_image_url = use_signal(|| defaults.assets.social_card_image_url.clone());

    let meta_description = use_signal(|| defaults.seo.meta_description.clone());
    let meta_keywords = use_signal(|| defaults.seo.meta_keywords.clone());
    let custom_robots = use_signal(|| defaults.seo.custom_robots.clone());
    let license_url = use_signal(|| defaults.seo.license_url.clone());
    let author_name = use_signal(|| defaults.seo.author_name.clone());

    let menu_1_label = use_signal(|| menu_label(&defaults, 0));
    let menu_1_url = use_signal(|| menu_url(&defaults, 0));
    let menu_2_label = use_signal(|| menu_label(&defaults, 1));
    let menu_2_url = use_signal(|| menu_url(&defaults, 1));
    let menu_3_label = use_signal(|| menu_label(&defaults, 2));
    let menu_3_url = use_signal(|| menu_url(&defaults, 2));
    let menu_4_label = use_signal(|| menu_label(&defaults, 3));
    let menu_4_url = use_signal(|| menu_url(&defaults, 3));

    let footer_text = use_signal(|| defaults.footer.footer_text.clone());
    let footer_license_label = use_signal(|| defaults.footer.footer_license_label.clone());
    let footer_license_url = use_signal(|| defaults.footer.footer_license_url.clone());

    let custom_js = use_signal(|| defaults.plugins.custom_js.clone());
    let static_pages = use_signal(|| defaults.static_pages.clone());
    let ads = use_signal(|| defaults.ads.clone());
    let preset_css = use_signal(String::new);

    let show_preview = use_signal(|| true);
    let active_preset = use_signal(|| None::<&'static str>);
    let is_dark_mode = use_signal(|| true);

    // Independent left/right panel layout states.
    let mut left_layout = use_signal(|| PanelLayout::Split);
    let mut right_layout = use_signal(|| PanelLayout::Split);

    let preview_viewport = use_signal(|| PreviewViewport::Desktop);
    let preview_width = use_signal(|| 1200u32);
    let preview_template_mode = use_signal(|| PreviewTemplateMode::Sidebars);

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
        static_pages,
        ads,
    };

    let current_config = use_memo(move || {
        ThemeConfig {
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
                MenuLink { label: menu_1_label(), url: menu_1_url() },
                MenuLink { label: menu_2_label(), url: menu_2_url() },
                MenuLink { label: menu_3_label(), url: menu_3_url() },
                MenuLink { label: menu_4_label(), url: menu_4_url() },
            ],
            footer: crate::config::FooterConfig {
                footer_text: footer_text(),
                footer_license_label: footer_license_label(),
                footer_license_url: footer_license_url(),
            },
            plugins: crate::config::PluginConfig {
                custom_js: custom_js(),
            },
            static_pages: static_pages(),
            ads: ads(),
            preset_css: preset_css(),
        }
    });

    let generated_xml = use_memo(move || render_theme(&current_config()));
    let preview_html = use_memo(move || render_preview_html(&current_config(), preview_template_mode()));
    
    let mut diag = use_signal(|| check_integrity(&generated_xml()));
    use_effect(move || {
        diag.set(check_integrity(&generated_xml()));
    });

    let layout_str = |layout: PanelLayout| match layout {
        PanelLayout::Split => "split",
        PanelLayout::Wide => "wide",
        PanelLayout::Floating => "floating",
        PanelLayout::Hidden => "hidden",
    };

    let active_left_tab = use_signal(|| "Presets");
    let active_right_tab = use_signal(|| "Site");

    // Global keyboard shortcuts for independent panel layouts.
    use_effect(move || {
        let mut eval = eval(
            r#"
            window.addEventListener('keydown', function(e) {
                let k = e.key.toLowerCase();

                if (e.ctrlKey || e.metaKey) {
                    if (k === 'b') { e.preventDefault(); dioxus.send("toggle_left"); }
                    if (k === 'e') { e.preventDefault(); dioxus.send("toggle_right"); }
                }

                if (e.altKey && !e.ctrlKey && !e.metaKey && !e.shiftKey) {
                    if (k === '1') { e.preventDefault(); dioxus.send("layout_split"); }
                    if (k === '2') { e.preventDefault(); dioxus.send("layout_wide"); }
                    if (k === '3') { e.preventDefault(); dioxus.send("layout_float"); }
                }
            });
            "#,
        );

        spawn(async move {
            while let Ok(value) = eval.recv().await {
                if let Some(cmd) = value.as_str() {
                    match cmd {
                        "toggle_left" => {
                            if left_layout() == PanelLayout::Hidden {
                                left_layout.set(PanelLayout::Split);
                            } else {
                                left_layout.set(PanelLayout::Hidden);
                            }
                        }
                        "toggle_right" => {
                            if right_layout() == PanelLayout::Hidden {
                                right_layout.set(PanelLayout::Split);
                            } else {
                                right_layout.set(PanelLayout::Hidden);
                            }
                        }
                        "layout_split" => {
                            left_layout.set(PanelLayout::Split);
                            right_layout.set(PanelLayout::Split);
                        }
                        "layout_wide" => {
                            left_layout.set(PanelLayout::Wide);
                            right_layout.set(PanelLayout::Wide);
                        }
                        "layout_float" => {
                            left_layout.set(PanelLayout::Floating);
                            right_layout.set(PanelLayout::Floating);
                        }
                        _ => {}
                    }
                }
            }
        });
    });

    rsx! {
        style { "{EDITOR_UI_CSS}" }
        div { class: "editor-shell",
            header { class: "editor-main-header",
                h1 { class: "editor-brand", "Moribund Institute Theme Architect" }
            }

            div {
                class: "editor-main",
                "data-left-layout": layout_str(left_layout()),
                "data-right-layout": layout_str(right_layout()),
                LeftVisualsPanel {
                    active_tab: active_left_tab,
                    layout: left_layout,
                    active_preset,
                    signals,
                    show_preview,
                }
                CenterWorkspacePanel {
                    preview_viewport,
                    preview_width,
                    preview_template_mode,
                    generated_xml: generated_xml(),
                    preview_html: preview_html(),
                    show_preview,
                    diag,
                    config_toml: toml::to_string_pretty(&current_config()).unwrap_or_default(),
                    on_load_theme: move |toml_text: String| {
                        if let Ok(new_config) = toml::from_str::<ThemeConfig>(&toml_text) {
                            signals.apply_config(&new_config);
                        }
                    },
                    on_restore: move |new_config: ThemeConfig| {
                        signals.apply_config(&new_config);
                    },
                    on_load_hotswap: move |json_text: String| {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_text) {
                            let apply_str = |val: &serde_json::Value, path: &[&str], sig: &mut Signal<String>| {
                                let mut current = val;
                                for p in path {
                                    if let Some(next) = current.get(p) { current = next; } else { return; }
                                }
                                if let Some(s_val) = current.as_str() { sig.set(s_val.to_string()); }
                            };

                            let mut s = signals;

                            apply_str(&val, &["site", "site_title"], &mut s.site_title);
                            apply_str(&val, &["site", "site_subtitle"], &mut s.site_subtitle);
                            apply_str(&val, &["site", "header_logo_url"], &mut s.header_logo_url);
                            apply_str(&val, &["site", "home_url"], &mut s.home_url);

                            apply_str(&val, &["assets", "favicon_url"], &mut s.favicon_url);
                            apply_str(&val, &["assets", "social_card_image_url"], &mut s.social_card_image_url);

                            if let Some(menus) = val.get("menu_links").and_then(|m| m.as_array()) {
                                if let Some(m) = menus.get(0) { apply_str(m, &["label"], &mut s.menu_1_label); apply_str(m, &["url"], &mut s.menu_1_url); }
                                if let Some(m) = menus.get(1) { apply_str(m, &["label"], &mut s.menu_2_label); apply_str(m, &["url"], &mut s.menu_2_url); }
                                if let Some(m) = menus.get(2) { apply_str(m, &["label"], &mut s.menu_3_label); apply_str(m, &["url"], &mut s.menu_3_url); }
                                if let Some(m) = menus.get(3) { apply_str(m, &["label"], &mut s.menu_4_label); apply_str(m, &["url"], &mut s.menu_4_url); }
                            }

                            apply_str(&val, &["seo", "meta_description"], &mut s.meta_description);
                            apply_str(&val, &["seo", "meta_keywords"], &mut s.meta_keywords);
                            apply_str(&val, &["seo", "custom_robots"], &mut s.custom_robots);
                            apply_str(&val, &["seo", "author_name"], &mut s.author_name);
                            apply_str(&val, &["seo", "license_url"], &mut s.license_url);

                            apply_str(&val, &["footer", "footer_text"], &mut s.footer_text);
                            apply_str(&val, &["footer", "footer_license_label"], &mut s.footer_license_label);
                            apply_str(&val, &["footer", "footer_license_url"], &mut s.footer_license_url);

                            apply_str(&val, &["plugins", "custom_js"], &mut s.custom_js);

                            if let Some(ads_val) = val.get("ads") {
                                if let Ok(ads_config) = serde_json::from_value(ads_val.clone()) {
                                    s.ads.set(ads_config);
                                }
                            }
                        }
                    },
                }
                RightDataPanel {
                    active_tab: active_right_tab,
                    layout: right_layout,
                    signals,
                }
            }
            
            footer { class: "editor-footer", DiagnosticsPanel { result: diag } }

            script {
                dangerous_inner_html: "{DRAG_JS}"
            }
        }
    }
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