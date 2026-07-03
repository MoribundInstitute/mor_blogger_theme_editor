// src/shortcut.rs
// Upgraded Keyboard Shortcut Registry (Nemo-ready)

use dioxus::prelude::*;
use std::collections::HashMap;

/// Stores the metadata needed to dynamically generate the UI cheat sheet
#[derive(Clone)]
#[allow(dead_code)] // category/action/keys consumed once shortcut modal binds to ShortcutRegistry
pub struct ShortcutMeta {
    pub category: String,
    pub action: String,
    pub keys: String,
    pub handler: EventHandler<()>,
}

#[derive(Clone, Default)]
pub struct ShortcutRegistry {
    // Keyed by the actual key combo (e.g., "CTRL+S") for O(1) lookup during keypress
    pub binds: HashMap<String, ShortcutMeta>,
}

pub fn init_shortcuts() {
    use_context_provider(|| Signal::new(ShortcutRegistry::default()));
}

/// Canonical form for a key combo, shared by every producer and consumer of
/// combos (registry keys, the keyboard.rs JS map, prefs strings, the rebind
/// capture). Uppercase, modifiers in CTRL/SHIFT/ALT order, `Arrow` prefix
/// stripped ("Shift+Ctrl+e" / "Ctrl+Shift+ArrowLeft" → "CTRL+SHIFT+E" /
/// "CTRL+SHIFT+LEFT"). Without one canonical form the two dispatch layers
/// silently disagree about the same prefs string.
pub fn normalize_combo(combo: &str) -> String {
    let mut mods = String::new();
    let mut key = String::new();
    for part in combo.split('+').map(str::trim).filter(|p| !p.is_empty()) {
        match part.to_ascii_uppercase().as_str() {
            "CTRL" | "CONTROL" | "CMD" | "META" => {}
            "SHIFT" => {}
            "ALT" => {}
            other => key = other.strip_prefix("ARROW").unwrap_or(other).to_string(),
        }
    }
    let upper = combo.to_ascii_uppercase();
    if upper.contains("CTRL") || upper.contains("CONTROL") || upper.contains("CMD") || upper.contains("META") {
        mods.push_str("CTRL+");
    }
    if upper.contains("SHIFT") {
        mods.push_str("SHIFT+");
    }
    if upper.contains("ALT") {
        mods.push_str("ALT+");
    }
    format!("{mods}{key}")
}

// =========================================================================
// REGISTRATION HOOKS
// =========================================================================

/// Legacy hook. Keeps older components compiling without throwing errors.
pub fn use_shortcut(combo: Option<String>, handler: Option<EventHandler<()>>) {
    use_registered_shortcut("Uncategorized", "Global Action", combo, handler);
}

/// New detailed registration for the Nemo-inspired UI
pub fn use_registered_shortcut(
    category: &str,
    action: &str,
    combo: Option<String>,
    handler: Option<EventHandler<()>>,
) {
    let registry_opt = try_consume_context::<Signal<ShortcutRegistry>>();

    // Connect wire on mount
    use_effect({
        let combo = combo.clone();
        let handler = handler.clone();
        let cat = category.to_string();
        let act = action.to_string();

        move || {
            if let (Some(mut reg), Some(c), Some(h)) =
                (registry_opt, combo.clone(), handler.clone())
            {
                reg.write().binds.insert(
                    normalize_combo(&c),
                    ShortcutMeta {
                        category: cat.clone(),
                        action: act.clone(),
                        keys: c.clone(),
                        handler: h,
                    },
                );
            }
        }
    });

    // Cut wire on unmount. Prevents ghost firing.
    use_drop({
        let combo = combo.clone();
        move || {
            if let (Some(mut reg), Some(c)) = (registry_opt, combo) {
                reg.write().binds.remove(&normalize_combo(&c));
            }
        }
    });
}

// =========================================================================
// EVENT LISTENER ROOT
// =========================================================================

#[cfg(test)]
mod tests {
    use super::normalize_combo;

    #[test]
    fn combos_normalize_to_one_canonical_form() {
        // Same binding written three ways must collide on one registry key.
        assert_eq!(normalize_combo("Shift+Ctrl+E"), "CTRL+SHIFT+E");
        assert_eq!(normalize_combo("ctrl+shift+e"), "CTRL+SHIFT+E");
        assert_eq!(normalize_combo("CTRL+SHIFT+E"), "CTRL+SHIFT+E");
        // Arrow naming: prefs say "Left", the DOM says "ArrowLeft".
        assert_eq!(normalize_combo("Ctrl+Shift+Left"), "CTRL+SHIFT+LEFT");
        assert_eq!(normalize_combo("Ctrl+Shift+ArrowLeft"), "CTRL+SHIFT+LEFT");
        // Unmodified keys and meta aliasing.
        assert_eq!(normalize_combo("F9"), "F9");
        assert_eq!(normalize_combo("Cmd+S"), "CTRL+S");
    }
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
                let key_combo = normalize_combo(&key_combo);

                if let Some(meta) = registry.read().binds.get(&key_combo) {
                    meta.handler.call(());
                    evt.stop_propagation();
                    evt.prevent_default();
                }
            },
            {children}
        }
    }
}
