use dioxus::prelude::*;

#[component]
pub fn SmartCodeDock(
    config_toml: ReadSignal<String>,
    on_load_theme: EventHandler<String>,
) -> Element {
    let jump_to = move |target: &str| {
        let target_str = target.to_string();
        spawn(async move {
            // FIX: Removed 'mut' promise.
            let eval = dioxus::document::eval(r#"
                let target = await dioxus.recv();
                let el = document.getElementById("toml-editor-textarea");
                if (el) {
                    let idx = el.value.indexOf(target);
                    if (idx !== -1) {
                        el.focus();
                        el.setSelectionRange(idx, idx + target.length);
                        
                        // Calculate lines to scroll
                        const linesBefore = el.value.substring(0, idx).split('\n').length;
                        const lineHeight = parseFloat(window.getComputedStyle(el).lineHeight) || 19;
                        
                        // Vertically center the highlighted term
                        el.scrollTop = Math.max(0, (linesBefore * lineHeight) - (el.clientHeight / 2));
                    }
                }
            "#);
            let _ = eval.send(target_str);
        });
    };

    rsx! {
        div {
            class: "export-viewport",
            style: "display: flex; flex-direction: row; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel);",
            
            div {
                style: "width: 220px; border-right: 1px solid var(--editor-border); display: flex; flex-direction: column;",
                div {
                    style: "padding: 12px; border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                    span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Template Layouts" }
                }
                div {
                    style: "padding: 12px; display: flex; flex-direction: column; gap: 8px; overflow-y: auto;",
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("header_variant"), "Header Variant" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("main_variant"), "Main Canvas Variant" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("content_variant"), "Content Feed" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("left_sidebar_variant"), "Left Sidebar" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("right_sidebar_variant"), "Right Sidebar" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("footer_variant"), "Footer Grid" }
                }
                div {
                    style: "padding: 12px; border-top: 1px solid var(--editor-border-soft); border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated);",
                    span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Core Identity" }
                }
                div {
                    style: "padding: 12px; display: flex; flex-direction: column; gap: 8px; overflow-y: auto;",
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("[site]"), "Site Information" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("[colors]"), "Color Palette" }
                    button { class: "editor-button", style: "text-align: left; font-size: 0.85rem;", onclick: move |_| jump_to("[typography]"), "Typography" }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",
                div {
                    style: "padding: 8px 12px; border-bottom: 1px solid var(--editor-border-soft); background: rgba(0,0,0,0.2); display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 0.8rem; color: var(--editor-accent-warm); font-family: var(--font-mono);", "theme_config.toml" }
                    span { style: "font-size: 0.75rem; color: var(--fg-muted);", "Live Reload Active" }
                }
                textarea {
                    id: "toml-editor-textarea",
                    class: "export-xml-textarea",
                    style: "flex: 1; border: none; border-radius: 0; margin: 0; background: transparent;",
                    value: "{config_toml()}",
                    oninput: move |evt| {
                        on_load_theme.call(evt.value());
                    }
                }
            }
        }
    }
}