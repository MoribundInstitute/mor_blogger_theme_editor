use dioxus::prelude::*;
use crate::app::state::ThemeAppState;

#[component]
pub fn CssEditorPanel(
    #[props(default)] preset_css: Option<Signal<String>>,
) -> Element {
    let mut css_signal = match preset_css {
        Some(sig) => sig,
        None => {
            let state = consume_context::<ThemeAppState>();
            state.signals.preset_css
        }
    };

    let css_val = css_signal.read().clone();
    let char_count = css_val.len();

    rsx! {
        div {
            class: "export-viewport",
            style: "display: flex; flex-direction: column; height: 100%; border: 1px solid var(--editor-border); border-radius: var(--radius-md); overflow: hidden; background: var(--bg-panel);",

            div {
                style: "padding: 10px 16px; border-bottom: 1px solid var(--editor-border-soft); background: var(--bg-elevated); display: flex; align-items: center; justify-content: space-between; flex-shrink: 0;",
                span {
                    style: "font-size: 0.8rem; font-weight: 600; color: var(--editor-accent-warm); font-family: var(--font-mono);",
                    "preset_css.css"
                }
                span {
                    style: "font-size: 0.75rem; color: var(--fg-muted);",
                    "Size: {char_count} chars"
                }
            }

            textarea {
                id: "preset-css-textarea",
                class: "export-xml-textarea",
                style: "flex: 1; border: none; border-radius: 0; margin: 0; background: transparent; font-family: var(--font-mono); font-size: 0.85rem; line-height: 1.5; padding: 16px; resize: none; width: 100%; color: var(--fg-base); min-height: 0;",
                placeholder: "/* Write custom or preset CSS rules here... */",
                value: "{css_val}",
                oninput: move |evt| {
                    css_signal.set(evt.value());
                }
            }
        }
    }
}
