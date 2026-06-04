// src/shortcut.rs
// Flat keyboard shortcut registry. Dynamic developer hooks. Zero-slop key handler.

use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct ShortcutRegistry {
    pub binds: HashMap<String, EventHandler<()>>,
}

pub fn init_shortcuts() {
    use_context_provider(|| Signal::new(ShortcutRegistry::default()));
}

// Public hook. Call anywhere to bind key to function.
pub fn use_shortcut(combo: Option<String>, handler: Option<EventHandler<()>>) {
    let registry_opt = try_consume_context::<Signal<ShortcutRegistry>>();
    
    // Connect wire on mount
    use_effect({
        let combo = combo.clone();
        let handler = handler.clone();
        move || {
            if let (Some(mut reg), Some(c), Some(h)) = (registry_opt, combo.clone(), handler.clone()) {
                reg.write().binds.insert(c.to_uppercase(), h);
            }
        }
    });

    // Cut wire on unmount. Prevents ghost firing.
    use_drop({
        let combo = combo.clone();
        move || {
            if let (Some(mut reg), Some(c)) = (registry_opt, combo) {
                reg.write().binds.remove(&c.to_uppercase());
            }
        }
    });
}

#[component]
pub fn MorShortcutRoot(children: Element) -> Element {
    init_shortcuts();
    let registry = use_context::<Signal<ShortcutRegistry>>();

    rsx! {
        div {
            class: "mor-shortcut-root mor-root",
            tabindex: "-1",
            autofocus: true,
            style: "outline: none; width: 100vw; height: 100vh; overflow: hidden;",
            onkeydown: move |evt| {
                // Preallocate capacity. Stop heap fragmentation on rapid typing.
                let mut key_combo = String::with_capacity(16);
                
                if evt.modifiers().ctrl() { key_combo.push_str("CTRL+"); }
                if evt.modifiers().shift() { key_combo.push_str("SHIFT+"); }
                if evt.modifiers().alt() { key_combo.push_str("ALT+"); }
                
                match evt.key() {
                    dioxus::html::Key::Character(c) => key_combo.push_str(&c.to_uppercase()),
                    other => key_combo.push_str(&other.to_string().to_uppercase()),
                }

                if let Some(handler) = registry.read().binds.get(&key_combo) {
                    handler.call(());
                    evt.stop_propagation();
                    evt.prevent_default();
                }
            },
            {children}
        }
    }
}