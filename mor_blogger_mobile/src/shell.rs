#![allow(non_snake_case)]

use dioxus::prelude::*;
use mor_blogger_core::config::defaults::default_theme_config;
use mor_blogger_core::render::theme::render_theme;
use mor_blogger_core::schema::all_widget_schemas;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Widgets,
    Preview,
    Deploy,
}

const STYLE: &str = r#"
:root { color-scheme: dark; }
* { box-sizing: border-box; margin: 0; }
html, body, #main { height: 100%; overscroll-behavior: none; }
body { font-family: system-ui, sans-serif; background: #16132b; color: #e8e6f5; }
.app { display: flex; flex-direction: column; height: 100dvh; }
.view { flex: 1; overflow-y: auto; padding: 16px; padding-bottom: calc(16px + env(safe-area-inset-bottom)); }
.view h2 { margin-bottom: 12px; font-size: 1.1rem; }
.card { background: #221d45; border-radius: 12px; padding: 14px; margin-bottom: 10px; }
.card .kind { font-size: 0.8rem; opacity: 0.6; }
.preview-frame { width: 100%; height: 100%; border: 0; border-radius: 12px; background: #fff; }
.action { display: block; width: 100%; padding: 16px; margin-bottom: 12px; font-size: 1rem;
          border: 0; border-radius: 12px; background: #5b2c8a; color: #fff; }
.action:active { background: #6f3aa6; }
nav { display: flex; border-top: 1px solid #322a5e; background: #1a1636;
      padding-bottom: env(safe-area-inset-bottom); }
nav button { flex: 1; padding: 14px 0; font-size: 0.9rem; border: 0; background: none; color: #8f89b3; }
nav button.active { color: #fff; border-top: 2px solid #5b2c8a; }
"#;

pub fn MobileApp() -> Element {
    let mut tab = use_signal(|| Tab::Widgets);

    rsx! {
        style { {STYLE} }
        div { class: "app",
            match tab() {
                Tab::Widgets => rsx! { WidgetsView {} },
                Tab::Preview => rsx! { PreviewView {} },
                Tab::Deploy => rsx! { DeployView {} },
            }
            nav {
                for (t, label) in [(Tab::Widgets, "Widgets"), (Tab::Preview, "Preview"), (Tab::Deploy, "Deploy")] {
                    button {
                        class: if tab() == t { "active" } else { "" },
                        onclick: move |_| tab.set(t),
                        {label}
                    }
                }
            }
        }
    }
}

// ponytail: read-only schema list; swap in a mobile WidgetPropertyForm when editing lands.
fn WidgetsView() -> Element {
    rsx! {
        div { class: "view",
            h2 { "Widgets" }
            for schema in all_widget_schemas() {
                div { class: "card",
                    div { {schema.widget_type.clone()} }
                    div { class: "kind", "{schema.settings.len()} settings" }
                }
            }
        }
    }
}

fn PreviewView() -> Element {
    // ponytail: renders the default config each mount; wire shared theme state when the editor exists.
    let html = use_memo(|| render_theme(&default_theme_config(), &std::collections::HashMap::new()));
    rsx! {
        div { class: "view",
            iframe { class: "preview-frame", srcdoc: "{html}" }
        }
    }
}

fn DeployView() -> Element {
    rsx! {
        div { class: "view",
            h2 { "Deploy" }
            // ponytail: stubs — no file dialogs on Android; export goes through the
            // system share sheet / Downloads via a platform channel, added when needed.
            button { class: "action", "Export theme XML" }
            button { class: "action", "Copy to clipboard" }
        }
    }
}
