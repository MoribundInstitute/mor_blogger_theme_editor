use dioxus::prelude::*;
use crate::app::state::ThemeAppState; 

pub fn get_cursor_css(cursor_type: &str) -> &'static str {
    match cursor_type {
        "Bibata Amber" => "url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSIzMiIgdmlld0JveD0iMCAwIDMyIDMyIj48cGF0aCBmaWxsPSIjZmZiMzAwIiBzdHJva2U9IiNmZmYiIHN0cm9rZS13aWR0aD0iMiIgZD0iTTQgNGwxMCAyNCA0LTEwIDEwLTR6Ii8+PC9zdmc+'), auto",
        "Bibata Ice" => "url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSIzMiIgdmlld0JveD0iMCAwIDMyIDMyIj48cGF0aCBmaWxsPSIjZWNlZmY0IiBzdHJva2U9IiM0YzU2NmEiIHN0cm9rZS13aWR0aD0iMiIgZD0iTTQgNGwxMCAyNCA0LTEwIDEwLTR6Ii8+PC9zdmc+'), auto",
        _ => "auto", 
    }
}

#[component]
pub fn CursorPanel() -> Element {
    let mut state = consume_context::<ThemeAppState>();
    
    // Read current value from signal to keep UI in sync
    let current_cursor = state.signals.cursor_style.read().clone();

    rsx! {
        div { class: "panel-section",
            div { class: "setting-row",
                label { "Global Cursor" }
                select {
                    class: "mor-select",
                    value: "{current_cursor}",
                    onchange: move |evt| {
                        let selected = evt.value();
                        let css_value = get_cursor_css(&selected);
                        
                        // Update signal. Memory safe mutation. 
                        *state.signals.cursor_style.write() = css_value.to_string();
                    },
                    option { value: "Default", "System Default" }
                    option { value: "Bibata Amber", "Bibata Amber" }
                    option { value: "Bibata Ice", "Bibata Ice" }
                }
            }
        }
    }
}