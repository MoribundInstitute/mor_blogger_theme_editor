use std::collections::HashMap;

use crate::ui::components::code_editor::CodeEditor;
use dioxus::prelude::*;
use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::render::xml_parts::css_generator::render_css_sockets;

use mor_blogger_core::utils::fs_bridge;

use crate::app::state::{LayoutState, RenderState};
use crate::app::vfs::VfsDictionary;
use crate::ui::workspace::layout::{apply_preview_viewport, clamp_preview_width, PreviewViewport};
use crate::ui::workspace::preview_canvas::PreviewCanvas;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn esc_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn pick_xml(render: &RenderState, registry_type: &str, id: &str) -> String {
    render
        .get_manifest(registry_type, id)
        .or_else(|| {
            let fallback_id = match registry_type {
                "header" => "mor",
                "layout" => "sidebars",
                "content" => "blog_standard",
                "sidebar_left" => "blogger_left",
                "sidebar_right" => "toc_right",
                "footer" => "mega",
                _ => "",
            };
            render.get_manifest(registry_type, fallback_id)
        })
        .map(|c| c.xml_content.to_string())
        .unwrap_or_default()
}

fn resolve_module_xml(render: &RenderState, module_key: &str, config: &ThemeConfig) -> String {
    let pack = &config.template_pack;
    match module_key {
        "header_variant" => pick_xml(render, "header", &pack.header_variant),
        "main_variant" => pick_xml(render, "layout", &pack.main_variant),
        "content_variant" => pick_xml(render, "content", &pack.content_variant),
        "left_sidebar_variant" => pick_xml(render, "sidebar_left", &pack.left_sidebar_variant),
        "right_sidebar_variant" => pick_xml(render, "sidebar_right", &pack.right_sidebar_variant),
        "footer_variant" => pick_xml(render, "footer", &pack.footer_variant),
        _ => String::new(),
    }
}

fn render_module_preview(
    module_key: &str,
    config: &ThemeConfig,
    vfs: &HashMap<String, String>,
) -> String {
    let parts = mor_blogger_core::render::template_resolver::resolve_template_parts(config, vfs);
    let true_css = render_css_sockets(parts.css, config);

    let site_title = esc(&config.site.site_title);
    let site_subtitle = esc(&config.site.site_subtitle);
    let footer_text = esc(&config.footer.footer_text);
    let label_title = mor_blogger_core::render::tracking::widget_title_h2(
        "Label1",
        config.template_pack.widget_title("Label1", "Labels"),
    );
    let archive_title = mor_blogger_core::render::tracking::widget_title_h2(
        "BlogArchive1",
        config.template_pack.widget_title("BlogArchive1", "Archive"),
    );
    let toc_title = mor_blogger_core::render::tracking::widget_title_h2(
        "HTML1",
        config
            .template_pack
            .widget_title("HTML1", "Table of Contents"),
    );
    let site_widget_title = mor_blogger_core::render::tracking::widget_title_h2(
        "HTML2",
        config.template_pack.widget_title("HTML2", "Site"),
    );
    let menu_links: String = config
        .menu_links
        .iter()
        .enumerate()
        .filter(|(_, l)| !l.label.trim().is_empty())
        .map(|(i, l)| mor_blogger_core::render::tracking::menu_link_anchor(i, &l.url, &l.label))
        .collect::<Vec<_>>()
        .join("");

    let body = match module_key {
        "header_variant" => format!(
            r##"<header class="main-header">
  <div class="header-top-row">
    <div class="header-side-controls left-controls">
      <button class="panel-toggle header-panel-toggle header-panel-toggle-left">Browse</button>
    </div>
    <a class="branding branding-link"><span class="institute-title" data-field-path="site.site_title">{site_title}</span></a>
    <div class="header-side-controls right-controls">
      <button class="header-panel-toggle">Contents</button>
    </div>
  </div>
  <div class="header-bottom-row">
    <nav class="mor-nav">{menu_links}</nav>
    <div class="mor-search">
      <form><span class="prompt">root@moribund:~$</span><input type="text" placeholder="Search..."><button type="button" class="icon-search-btn" data-edit-target="buttons.radius" aria-label="Search"></button></form>
    </div>
  </div>
</header>"##
        ),

        "main_variant" => format!(r##"<div class="mor-workspace" style="height:100vh;">
  <aside class="mor-panel panel-left" style="display:flex!important;transform:none!important;position:relative!important;">
    <div class="panel-header"><span>Browse</span></div>
    <div class="panel-content sidebar-section">
      <div class="widget" data-block-id="Label1">{label_title}
        <div class="widget-content Label"><ul>
          <li><a href="#">Typography</a></li><li><a href="#">Design</a></li><li><a href="#">Dev</a></li>
        </ul></div>
      </div>
    </div>
  </aside>
  <main class="canvas-core">
    <div class="canvas-content" style="padding:24px;color:var(--fg-muted);">— canvas area —</div>
  </main>
  <aside class="mor-panel panel-right" style="display:flex!important;transform:none!important;position:relative!important;">
    <div class="panel-header"><span>Contents</span></div>
    <div class="panel-content sidebar-section">
      <div class="widget" data-block-id="HTML1">{toc_title}</div>
    </div>
  </aside>
</div>"##),

        "content_variant" => r##"<div style="max-width:800px;margin:0 auto;padding:24px;">
  <article class="mor-post">
    <h2 class="post-title"><a href="#">WYSIWYG: The Modular Stage</a></h2>
    <div class="post-meta">
      <span class="sys-date">[24 Oct, 2026]</span>
      <span class="sys-tags"> Tags: <a href="#">Preview</a>, <a href="#">Module</a></span>
    </div>
    <div class="post-body">
      <p>The content feed variant renders the post list in the selected layout. This isolated stage lets you validate the CSS and DOM structure independent of the full theme shell.</p>
      <blockquote>"One module at a time. No noise."</blockquote>
    </div>
    <div class="mor-pager"><button class="pager-btn">Read More</button></div>
  </article>
  <article class="mor-post" style="margin-top:16px;">
    <h2 class="post-title"><a href="#">A Second Post Entry</a></h2>
    <div class="post-meta"><span class="sys-date">[23 Oct, 2026]</span></div>
    <div class="post-body"><p>Another sample post for the content feed module.</p></div>
    <div class="mor-pager"><button class="pager-btn">Read More</button></div>
  </article>
</div>"##
            .to_string(),

        "left_sidebar_variant" => format!(
            r##"<div style="display:flex;height:100vh;">
  <aside class="mor-panel panel-left" style="display:flex!important;transform:none!important;position:relative!important;width:300px!important;">
    <div class="panel-header"><span>Browse</span><button class="panel-toggle">Close</button></div>
    <div class="panel-content sidebar-section">
      <div class="widget" data-block-id="Label1">{label_title}
        <div class="widget-content Label"><ul>
          <li><a href="#">Typography</a></li><li><a href="#">Design</a></li><li><a href="#">Dev</a></li>
        </ul></div>
      </div>
      <div class="widget" data-block-id="BlogArchive1">{archive_title}
        <div class="widget-content BlogArchive"><ul>
          <li><a href="#">October 2026 (5)</a></li><li><a href="#">September 2026 (8)</a></li>
        </ul></div>
      </div>
      <div class="widget" data-block-id="HTML2">{site_widget_title}<div class="widget-content" data-field-path="site.site_subtitle">{site_subtitle}</div></div>
    </div>
  </aside>
  <main style="flex:1;padding:24px;color:var(--fg-muted);">← left sidebar isolated preview</main>
</div>"##
        ),

        "right_sidebar_variant" => format!(r##"<div style="display:flex;height:100vh;">
  <main style="flex:1;padding:24px;color:var(--fg-muted);">right sidebar isolated preview →</main>
  <aside class="mor-panel panel-right" style="display:flex!important;transform:none!important;position:relative!important;width:300px!important;">
    <div class="panel-header"><span>Contents</span><button class="panel-toggle">Close</button></div>
    <div class="panel-content sidebar-section">
      <div class="widget" data-block-id="HTML1">{toc_title}
        <div class="widget-content"><ul>
          <li><a href="#">Introduction</a></li><li><a href="#">Architecture</a></li><li><a href="#">Usage</a></li>
        </ul></div>
      </div>
    </div>
  </aside>
</div>"##),

        "footer_variant" => {
            let cols: String = config
                .footer
                .columns
                .iter()
                .take(4)
                .map(|col| {
                    let links: String = col
                        .links
                        .iter()
                        .map(|l| {
                            format!(
                                "<li><a href='{}'>{}</a></li>",
                                esc_attr(&l.url),
                                esc(&l.label)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("");
                    format!(
                        "<div class='footer-column'><h4 class='footer-column-header'>{}</h4><ul>{}</ul></div>",
                        esc(&col.heading),
                        links
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            format!(
                r##"<footer class="mor-footer">
  <div class="footer-column-grid">{cols}</div>
  <div class="footer-sys-info">
    <p class="footer-copyright" data-field-path="footer.footer_text">{footer_text}</p>
    <div class="footer-legal-links"><a href="#">Privacy policy</a> | <a href="#">Terms of use</a></div>
    <button class="back-to-top-btn" type="button">Back to Top</button>
  </div>
</footer>"##
            )
        }

        _ => r#"<div style="padding:40px;text-align:center;color:var(--fg-muted);">Select a module.</div>"#
            .to_string(),
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style id="mor-true-css">
{true_css}
html, body {{ overflow: hidden; margin: 0; }}
</style>
</head>
<body>
{body}
</body>
</html>"#
    )
}

fn module_key_to_category(key: &str) -> &'static str {
    match key {
        "header_variant" => "headers",
        "main_variant" => "layouts",
        "content_variant" => "content",
        "left_sidebar_variant" | "right_sidebar_variant" => "sidebars",
        "footer_variant" => "footers",
        _ => "custom",
    }
}

#[component]
pub fn ModuleWorkbench(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
) -> Element {
    let layout = use_context::<LayoutState>();
    let render = use_context::<RenderState>();
    let mut edited_xml: Signal<String> = use_signal(String::new);
    let mut is_takeover_active = use_signal(|| false);
    let mut workbench_status: Signal<String> = use_signal(String::new);
    let vfs = use_context::<VfsDictionary>().0;

    // Reactively resolve the active module's XML fragment from the live config.
    let module_xml = use_memo(move || match (layout.active_workbench_module)() {
        Some(key) => {
            let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
            resolve_module_xml(&render, key, &config)
        }
        None => String::new(),
    });

    // Show user's scratchpad edits; fall back to the config-derived XML when pristine.
    let display_xml = use_memo(move || {
        let edited = edited_xml();
        if edited.is_empty() {
            module_xml()
        } else {
            edited
        }
    });

    // Reset the scratchpad when the active module changes, so switching modules
    // (now driven from the TemplateModulesDock) falls back to the new module's
    // config-derived XML. Keyed on the module, so it never clobbers live typing.
    let module_key = (layout.active_workbench_module)();
    use_effect(use_reactive!(|module_key| {
        let _ = module_key;
        edited_xml.set(String::new());
    }));

    // Generate the isolated module preview HTML from the live config.
    let module_preview_html = use_memo(move || match (layout.active_workbench_module)() {
        Some(key) => {
            let config = toml::from_str::<ThemeConfig>(&config_toml()).unwrap_or_default();
            render_module_preview(key, &config, &*vfs.read())
        }
        None => String::new(),
    });

    let mut preview_viewport: Signal<PreviewViewport> = use_signal(|| PreviewViewport::Fit);
    let mut preview_width: Signal<u32> = use_signal(|| 1200u32);

    rsx! {
        if is_takeover_active() {
            // ── Takeover Mode ─ Full-viewport isolated canvas stage ───────
            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",

                // Top navigation bar
                div {
                    style: "flex-shrink: 0; display: flex; align-items: center; gap: 6px; padding: 6px 12px; background: var(--bg-elevated); border-bottom: 1px solid var(--editor-border);",

                    button {
                        class: if preview_viewport() == PreviewViewport::Desktop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Desktop, preview_width); preview_viewport.set(PreviewViewport::Desktop); },
                        "Desktop"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Laptop { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Laptop, preview_width); preview_viewport.set(PreviewViewport::Laptop); },
                        "Laptop"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Tablet { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Tablet, preview_width); preview_viewport.set(PreviewViewport::Tablet); },
                        "Tablet"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Phone { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Phone, preview_width); preview_viewport.set(PreviewViewport::Phone); },
                        "Phone"
                    }
                    button {
                        class: if preview_viewport() == PreviewViewport::Fit { "editor-mini-button editor-mini-button-active" } else { "editor-mini-button" },
                        onclick: move |_| { apply_preview_viewport(PreviewViewport::Fit, preview_width); preview_viewport.set(PreviewViewport::Fit); },
                        "Fit"
                    }

                    div { style: "width: 1px; height: 16px; background: var(--editor-border-soft); margin: 0 2px;" }

                    label {
                        class: "preview-width-control",
                        span { class: "preview-width-label", "Width" }
                        input {
                            class: "preview-width-input",
                            r#type: "number", min: "240", max: "2400", step: "10",
                            value: "{preview_width()}",
                            oninput: move |evt| {
                                if let Ok(w) = evt.value().parse::<u32>() {
                                    preview_width.set(clamp_preview_width(w));
                                    preview_viewport.set(PreviewViewport::Custom);
                                }
                            }
                        }
                    }

                    // Spacer — pushes the exit button to the far right
                    div { style: "flex: 1;" }

                    button {
                        class: "editor-mini-button",
                        onclick: move |_| is_takeover_active.set(false),
                        "Editor ×"
                    }
                }

                // Canvas stage — transparent tiled grid background
                div {
                    style: "
                        flex: 1; min-height: 0; overflow: auto;
                        display: flex; align-items: flex-start; justify-content: center;
                        padding: 32px;
                        background-color: var(--bg-base);
                        background-image:
                            linear-gradient(45deg, rgba(128,128,128,0.07) 25%, transparent 25%),
                            linear-gradient(-45deg, rgba(128,128,128,0.07) 25%, transparent 25%),
                            linear-gradient(45deg, transparent 75%, rgba(128,128,128,0.07) 75%),
                            linear-gradient(-45deg, transparent 75%, rgba(128,128,128,0.07) 75%);
                        background-size: 16px 16px;
                        background-position: 0 0, 0 8px, 8px -8px, -8px 0px;
                    ",

                    if module_preview_html().is_empty() {
                        div {
                            style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: var(--fg-muted); font-family: var(--font-mono); font-size: 0.9rem;",
                            "← select a module to preview"
                        }
                    } else {
                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column;",
                            PreviewCanvas {
                                preview_viewport,
                                preview_width,
                                preview_html: module_preview_html(),
                            }
                        }
                    }
                }
            }
        } else {
            // ── 3-Pane Editor Mode ────────────────────────────────────────
            div {
                class: "export-viewport",
                style: "display: flex; flex-direction: row; flex: 1; min-height: 0; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel);",

                // ── Left Pane ─ XML Fragment Editor ──────────────────────
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0; border-right: 1px solid var(--editor-border);",
                    div {
                        class: "editor-pane",
                        style: "display: flex; flex-direction: column; width: 100%; height: 100%; background: var(--bg-base); border-radius: 6px; overflow: hidden; border: 1px solid var(--border-color);",
                        
                        div {
                            class: "editor-pane-header",
                            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: rgba(0,0,0,0.2); border-bottom: 1px solid var(--border-color); flex-shrink: 0;",
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span {
                                    style: "font-family: monospace; font-size: 0.85rem; font-weight: bold; color: var(--fg-base);",
                                    {(layout.active_workbench_module)().map(|k| format!("{k}.xml")).unwrap_or_else(|| "module_fragment.xml".to_string())}
                                }
                                span {
                                    style: "font-size: 0.7rem; font-weight: 600; color: var(--editor-accent); background: rgba(0,0,0,0.25); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--editor-border-soft);",
                                    "Blogger XML · Live"
                                }
                            }
                            button {
                                class: if (layout.active_workbench_module)().is_some() { "editor-mini-button" } else { "editor-mini-button editor-mini-button-disabled" },
                                title: "Save current XML buffer to the user templates folder",
                                onclick: move |_| {
                                    let Some(key) = (layout.active_workbench_module)() else { return; };
                                    let category = module_key_to_category(key);
                                    let name = format!("custom_{}", key);
                                    let content = display_xml();
                                    match fs_bridge::save_custom_module(category, &name, &content) {
                                        Ok(path) => workbench_status.set(format!("Saved → {}", path.display())),
                                        Err(e) => workbench_status.set(format!("Save failed: {}", e)),
                                    }
                                },
                                "Save Module"
                            }
                        }

                        if !workbench_status().is_empty() {
                            div {
                                style: "padding: 4px 12px; font-size: 0.75rem; color: var(--editor-accent-warm); background: rgba(0,0,0,0.15); font-family: var(--font-mono); border-bottom: 1px solid var(--editor-border-soft); display: flex; justify-content: space-between; align-items: center; flex-shrink: 0;",
                                span { "{workbench_status()}" }
                                button {
                                    style: "background: none; border: none; color: var(--fg-muted); cursor: pointer; font-size: 0.9rem; padding: 0 2px;",
                                    onclick: move |_| workbench_status.set(String::new()),
                                    "×"
                                }
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; flex: 1; min-height: 0;",
                            CodeEditor {
                                value: display_xml(),
                                mode: "xml".to_string(),
                                on_change: move |new_val| {
                                    edited_xml.set(new_val);
                                    on_load_theme.call(config_toml());
                                }
                            }
                        }
                    }
                }

                // ── Right Pane ─ Isolated Module Preview Stage ────────────
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",

                    div {
                        style: "padding: 8px 12px; border-bottom: 1px solid var(--editor-border-soft); background: rgba(0,0,0,0.2); display: flex; align-items: center; justify-content: space-between;",
                        span {
                            style: "font-size: 0.8rem; color: var(--accent); font-family: var(--font-mono);",
                            "Module Preview"
                        }
                        button {
                            class: "editor-mini-button",
                            onclick: move |_| is_takeover_active.set(true),
                            "Takeover"
                        }
                    }

                    if module_preview_html().is_empty() {
                        div {
                            style: "flex: 1; display: flex; align-items: center; justify-content: center; color: var(--fg-muted); font-size: 0.9rem; font-family: var(--font-mono);",
                            "← select a module to preview"
                        }
                    } else {
                        PreviewCanvas {
                            preview_viewport,
                            preview_width,
                            preview_html: module_preview_html(),
                        }
                    }
                }
            }
        }
    }
}
