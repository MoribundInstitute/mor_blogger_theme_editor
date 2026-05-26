// Cross-platform clipboard handler

#[cfg(not(target_arch = "wasm32"))]
pub fn copy_to_clipboard(text: String) {
    // Native Desktop OS implementation
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text);
    } else {
        log::error!("Failed to initialize native clipboard");
    }
}

#[cfg(target_arch = "wasm32")]
pub fn copy_to_clipboard(text: String) {
    // WebAssembly Browser implementation
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::spawn_local;

    spawn_local(async move {
        if let Some(window) = web_sys::window() {
            if let Some(clipboard) = window.navigator().clipboard() {
                let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await;
            } else {
                web_sys::console::warn_1(&JsValue::from_str("Clipboard API not available"));
            }
        }
    });
}
