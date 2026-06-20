use dioxus::prelude::*;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;

const SCROLL_LOCK_JS: &str = r#"
    setTimeout(() => {
        document.querySelectorAll('.pure-rust-editor-ghost').forEach(ta => {
            if (ta.dataset.scrollBound) return;
            ta.dataset.scrollBound = 'true';
            ta.addEventListener('scroll', (e) => {
                if (e.target.previousElementSibling) {
                    e.target.previousElementSibling.scrollTop = e.target.scrollTop;
                    e.target.previousElementSibling.scrollLeft = e.target.scrollLeft;
                }
            });
        });
    }, 100);
"#;

#[derive(Props, Clone, PartialEq)]
pub struct CodeEditorProps {
    #[props(into)]
    pub value: ReadSignal<String>,
    pub mode: String,
    pub on_change: EventHandler<String>,
    #[props(default = None)]
    pub id: Option<String>,
}

#[component]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    
    let effective_mode = if props.mode == "toml" { "ini" } else { &props.mode };
    let syntax = ss.find_syntax_by_extension(effective_mode).unwrap_or_else(|| ss.find_syntax_plain_text());

    // Local mutable state initialized from the read-only prop value
    let mut local_val = use_signal(|| props.value.read().clone());
    
    // Sync external changes (like switching tabs or hot-reloading) to the local state
    let external_val = props.value.cloned();
    let mut last_external_val = use_signal(|| external_val.clone());
    if last_external_val() != external_val {
        last_external_val.set(external_val.clone());
        local_val.set(external_val);
    }

    let val_str = local_val.read().clone();
    let display_val = if val_str.ends_with('\n') {
        format!("{} ", val_str)
    } else {
        val_str
    };

    let highlighted_html = highlighted_html_for_string(&display_val, &ss, syntax, theme).unwrap_or_else(|_| display_val.clone());

    let on_change = props.on_change;
    let text_id = props.id.unwrap_or_default();

    rsx! {
        div {
            class: "pure-rust-editor-container",
            style: "position: relative; width: 100%; height: 100%; overflow: hidden; background: #2b303b; font-family: monospace; font-size: 14px; line-height: 1.5;",
            
            // The Paint Layer (Output)
            div {
                class: "pure-rust-editor-paint",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; padding: 16px; box-sizing: border-box; white-space: pre-wrap; word-wrap: break-word; overflow-y: auto; pointer-events: none;",
                dangerous_inner_html: highlighted_html
            }

            // The Ghost Layer (Input)
            textarea {
                id: "{text_id}",
                class: "pure-rust-editor-ghost",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; padding: 16px; box-sizing: border-box; background: transparent; color: transparent; caret-color: #c0c5ce; border: none; outline: none; resize: none; overflow-y: auto; white-space: pre-wrap; word-wrap: break-word;",
                value: "{local_val}",
                oninput: move |evt| {
                    let val = evt.value();
                    local_val.set(val.clone());
                    on_change.call(val);
                },
                onscroll: move |_evt| {
                    // In a real implementation, you will need a tiny JS bridge OR Dioxus node refs 
                    // to sync the scroll position of the textarea with the background div.
                }
            }

            script {
                dangerous_inner_html: SCROLL_LOCK_JS
            }
        }
    }
}
