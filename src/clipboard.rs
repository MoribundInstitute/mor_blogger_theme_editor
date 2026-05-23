use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub fn copy_to_clipboard(text_to_copy: String) {
    spawn_local(async move {
        match copy_to_clipboard_inner(text_to_copy).await {
            Ok(_) => {
                web_sys::console::log_1(&"Generated Blogger XML copied to clipboard.".into());
            }
            Err(err) => {
                web_sys::console::error_1(&err);
            }
        }
    });
}

async fn copy_to_clipboard_inner(text_to_copy: String) -> Result<(), JsValue> {
    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("No browser window available."))?;

    let navigator = window.navigator();

    // Older pinned web-sys versions may not expose Navigator::clipboard()
    // as a typed Rust method, so we access navigator.clipboard dynamically.
    let clipboard = js_sys::Reflect::get(navigator.as_ref(), &JsValue::from_str("clipboard"))?;

    if clipboard.is_undefined() || clipboard.is_null() {
        return Err(JsValue::from_str(
            "Clipboard API is unavailable. Try running from localhost or HTTPS.",
        ));
    }

    let write_text = js_sys::Reflect::get(&clipboard, &JsValue::from_str("writeText"))?
        .dyn_into::<js_sys::Function>()?;

    let promise_value = write_text.call1(&clipboard, &JsValue::from_str(&text_to_copy))?;

    let promise = promise_value.dyn_into::<js_sys::Promise>()?;

    JsFuture::from(promise).await?;

    Ok(())
}
