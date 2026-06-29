// Native desktop clipboard handler.

pub fn copy_to_clipboard(text: String) {
    // arboard wraps the OS clipboard APIs (Win32 / NSPasteboard / X11 / Wayland).
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            let _ = clipboard.set_text(text);
        }
        Err(e) => log::error!("Failed to initialize native clipboard: {e}"),
    }
}
