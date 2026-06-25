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

    // TEMP DEBUG: byte length of the buffer CodeEditor actually reads, at mount.
    {
        let m = props.mode.clone();
        let n = props.value.len();
        let id = props.id.clone().unwrap_or_default();
        use_hook(move || eprintln!("[CodeEditor mount] mode={m} id={id} value_bytes={n}"));
    }

    let mut local_val = use_signal(|| props.value.clone());
    // TEMP DEBUG: confirm typing reaches this buffer (and what it held just before).
    let mut logged_first_input = use_signal(|| false);

    // The textarea binds its value to `seed`, NOT to the live `local_val`. Rewriting the
    // textarea value on every keystroke makes it a fully controlled input, which wipes the
    // webview's native undo stack and leaves Ctrl+Z nothing to undo. `seed` is re-derived
    // only on non-typing buffer changes (file load, format, hot-reload); our own keystroke
    // echo round-trips through props.value but never re-seeds, so the native undo/redo stack
    // survives. We tell the two apart with `last_emitted`: a non-typing change differs from
    // the last value our oninput sent, an echo matches. Browser undo/redo also emit input
    // events, so local_val / on_change stay in sync after an undo.
    let mut seed = use_signal(|| props.value.clone());
    let mut last_emitted = use_signal(|| props.value.clone());

    // Reconcile external content into the local buffer from an effect (never during render),
    // and only when props.value actually changes (a load) — use_reactive is the change-gate,
    // so typing via on_change is never clobbered mid-keystroke. on_change owns the buffer
    // between loads.
    let external_val = props.value.clone();
    use_effect(use_reactive!(|external_val| {
        local_val.set(external_val.clone());
        // Only re-seed the textarea for changes we did not type, so typing keeps the
        // native undo stack intact.
        if *last_emitted.peek() != external_val {
            seed.set(external_val);
        }
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
                value: "{seed}",
                readonly: props.read_only,
                spellcheck: "false", // FIX: Prevent native squiggly lines from breaking metrics
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; margin: 0; padding: 16px 16px 16px 48px; background: transparent; color: transparent; caret-color: #fff; font-family: monospace; font-size: 13px; line-height: 1.5; border: none; outline: none; resize: none; overflow: auto; white-space: pre; box-sizing: border-box;",
                oninput: move |evt| {
                    if !props.read_only {
                        let new_val = evt.value();
                        if !*logged_first_input.peek() {
                            eprintln!(
                                "[CodeEditor first input] mode={} prev_bytes={} new_bytes={}",
                                props.mode,
                                local_val.peek().len(),
                                new_val.len()
                            );
                            logged_first_input.set(true);
                        }
                        local_val.set(new_val.clone());
                        last_emitted.set(new_val.clone());
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
