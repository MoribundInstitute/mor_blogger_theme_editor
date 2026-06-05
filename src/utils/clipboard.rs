// Cross-platform clipboard handler

#[cfg(not(target_arch = "wasm32"))]
pub fn copy_to_clipboard(text: String) {
    // Native desktop: arboard wraps the OS clipboard APIs (Win32 / NSPasteboard / X11 / Wayland).
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            let _ = clipboard.set_text(text);
        }
        Err(e) => log::error!("Failed to initialize native clipboard: {e}"),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn copy_to_clipboard(text: String) {
    // WebAssembly browser: navigator.clipboard.writeText (fire-and-forget).
    use wasm_bindgen::JsValue;
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