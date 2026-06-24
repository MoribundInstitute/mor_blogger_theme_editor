use dioxus::prelude::*;
use std::rc::Rc;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[derive(Props, Clone, PartialEq)]
pub struct CodeEditorProps {
    pub value: String,
    pub mode: String,
    pub on_change: EventHandler<String>,
    #[props(default = None)]
    pub id: Option<String>,
    #[props(default = false)]
    pub read_only: bool,
}

#[component]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
    let ss = use_hook(|| Rc::new(SyntaxSet::load_defaults_newlines()));
    let ts = use_hook(|| Rc::new(ThemeSet::load_defaults()));
    let theme = &ts.themes["base16-ocean.dark"];

    // FIX: Map internal string flags to exact Syntect definitions
    let effective_mode = match props.mode.as_str() {
        "toml" => "ini",
        "javascript" | "js" => "js",
        "xml" | "html" => "html",
        "css" => "css",
        other => other,
    };
    
    let syntax = ss
        .find_syntax_by_extension(effective_mode)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut local_val = use_signal(|| props.value.clone());

    // Reconcile external content into the local buffer from an effect (never during render),
    // and only when props.value actually changes (a load) — use_reactive is the change-gate,
    // so typing via on_change is never clobbered mid-keystroke. on_change owns the buffer
    // between loads.
    let external_val = props.value.clone();
    use_effect(use_reactive!(|external_val| {
        local_val.set(external_val);
    }));

    // FIX: Replace breaking space hack with a strict newline boundary
    let mut text_to_highlight = local_val().clone();
    if text_to_highlight.ends_with('\n') || text_to_highlight.is_empty() {
        text_to_highlight.push('\n'); 
    }

    let highlighted_html =
        highlighted_html_for_string(&text_to_highlight, &ss, syntax, theme)
            .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", text_to_highlight));

    let mut scroll_y = use_signal(|| 0.0);
    let mut scroll_x = use_signal(|| 0.0);

    let line_count = local_val().matches('\n').count() + 1;
    let line_numbers = (1..=line_count).map(|n| n.to_string()).collect::<Vec<_>>().join("\n");

    rsx! {
        div {
            class: "pure-rust-editor-container",
            style: "position: relative; width: 100%; height: 100%; overflow: hidden; background: #2b303b; display: flex; flex-direction: row;",
            
            // GUTTER LAYER
            div {
                class: "pure-rust-editor-gutter",
                style: "position: relative; width: 40px; background: #232831; color: #65737e; font-family: monospace; font-size: 13px; line-height: 1.5; padding: 16px 8px 16px 0; text-align: right; user-select: none; border-right: 1px solid #343d46; overflow: hidden; flex-shrink: 0; z-index: 10;",
                div {
                    style: "transform: translate3d(0, -{scroll_y}px, 0); white-space: pre;",
                    "{line_numbers}"
                }
            }

            // PAINT LAYER
            div {
                class: "pure-rust-editor-paint",
                style: "position: absolute; top: 0; left: 0; transform: translate3d(-{scroll_x}px, -{scroll_y}px, 0); pointer-events: none; margin: 0; padding: 16px 16px 16px 48px; min-width: 100%; font-family: monospace; font-size: 13px; line-height: 1.5; color: #c0c5ce;",
                dangerous_inner_html: "{highlighted_html}"
            }

            // GHOST LAYER
            textarea {
                class: "pure-rust-editor-ghost",
                id: props.id.clone().unwrap_or_default(),
                value: "{local_val}",
                readonly: props.read_only,
                spellcheck: "false", // FIX: Prevent native squiggly lines from breaking metrics
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; margin: 0; padding: 16px 16px 16px 48px; background: transparent; color: transparent; caret-color: #fff; font-family: monospace; font-size: 13px; line-height: 1.5; border: none; outline: none; resize: none; overflow: auto; white-space: pre; box-sizing: border-box;",
                oninput: move |evt| {
                    if !props.read_only {
                        let new_val = evt.value();
                        local_val.set(new_val.clone());
                        props.on_change.call(new_val);
                    }
                },
                onscroll: move |evt| {
                    scroll_y.set(evt.data().scroll_top() as f64);
                    scroll_x.set(evt.data().scroll_left() as f64);
                }
            }
        }
    }
}
