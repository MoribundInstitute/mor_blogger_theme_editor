/// Structural data for font mapping. No heap strings allowed here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontPreset {
    pub name: &'static str,
    pub css_stack: &'static str,
    pub google_font_name: Option<&'static str>,
    pub category: &'static str,
}

/// Static catalog matching your preset aesthetics. Fast lookup, zero heap bloat.
pub const FONT_REGISTRY: &[FontPreset] = &[
    // --- Web 1.0 & System Safe ---
    FontPreset { name: "Times New Roman", css_stack: "'Times New Roman', Times, serif", google_font_name: None, category: "Web 1.0 / Classic" },
    FontPreset { name: "Arial", css_stack: "Arial, sans-serif", google_font_name: None, category: "Web 1.0 / Classic" },
    FontPreset { name: "Courier New", css_stack: "'Courier New', Courier, monospace", google_font_name: None, category: "Web 1.0 / Classic" },
    FontPreset { name: "Comic Sans", css_stack: "'Comic Sans MS', 'Comic Sans', cursive", google_font_name: None, category: "Web 1.0 / Classic" },
    FontPreset { name: "Impact", css_stack: "Impact, Haettenschweiler, 'Arial Narrow Bold', sans-serif", google_font_name: None, category: "Web 1.0 / Classic" },
    
    // --- Modern Clean Sans ---
    FontPreset { name: "Inter", css_stack: "'Inter', sans-serif", google_font_name: Some("Inter"), category: "Modern Sans" },
    FontPreset { name: "Roboto", css_stack: "'Roboto', sans-serif", google_font_name: Some("Roboto"), category: "Modern Sans" },
    FontPreset { name: "Montserrat", css_stack: "'Montserrat', sans-serif", google_font_name: Some("Montserrat"), category: "Modern Sans" },
    FontPreset { name: "Open Sans", css_stack: "'Open Sans', sans-serif", google_font_name: Some("Open Sans"), category: "Modern Sans" },
    FontPreset { name: "Poppins", css_stack: "'Poppins', sans-serif", google_font_name: Some("Poppins"), category: "Modern Sans" },

    // --- Elegant Serifs ---
    FontPreset { name: "Merriweather", css_stack: "'Merriweather', serif", google_font_name: Some("Merriweather"), category: "Serif" },
    FontPreset { name: "Playfair Display", css_stack: "'Playfair Display', serif", google_font_name: Some("Playfair Display"), category: "Serif" },
    FontPreset { name: "Lora", css_stack: "'Lora', serif", google_font_name: Some("Lora"), category: "Serif" },
    FontPreset { name: "IM Fell English", css_stack: "'IM Fell English', serif", google_font_name: Some("IM Fell English"), category: "Serif / Old World" },
    FontPreset { name: "Cinzel", css_stack: "'Cinzel', serif", google_font_name: Some("Cinzel"), category: "Serif / Display" },

    // --- Cyberpunk & Display ---
    FontPreset { name: "Orbitron", css_stack: "'Orbitron', sans-serif", google_font_name: Some("Orbitron"), category: "Display / Sci-Fi" },
    FontPreset { name: "Press Start 2P", css_stack: "'Press Start 2P', cursive", google_font_name: Some("Press Start 2P"), category: "Display / Retro" },
    FontPreset { name: "Righteous", css_stack: "'Righteous', cursive", google_font_name: Some("Righteous"), category: "Display / Chunky" },
    FontPreset { name: "Bebas Neue", css_stack: "'Bebas Neue', sans-serif", google_font_name: Some("Bebas Neue"), category: "Display / Tall" },
];

pub const MONO_FONT_REGISTRY: &[FontPreset] = &[
    FontPreset { name: "JetBrains Mono", css_stack: "'JetBrains Mono', monospace", google_font_name: Some("JetBrains Mono"), category: "Monospace" },
    FontPreset { name: "Fira Code", css_stack: "'Fira Code', monospace", google_font_name: Some("Fira Code"), category: "Monospace" },
    FontPreset { name: "Space Mono", css_stack: "'Space Mono', monospace", google_font_name: Some("Space Mono"), category: "Monospace" },
    FontPreset { name: "Inconsolata", css_stack: "'Inconsolata', monospace", google_font_name: Some("Inconsolata"), category: "Monospace" },
    FontPreset { name: "Source Code Pro", css_stack: "'Source Code Pro', monospace", google_font_name: Some("Source Code Pro"), category: "Monospace" },
    FontPreset { name: "Courier New", css_stack: "'Courier New', Courier, monospace", google_font_name: None, category: "System Safe" },
];

pub fn resolve_font_stack_with_fallback(raw_input: &str, is_mono: bool) -> String {
    let trimmed = raw_input.trim();
    if trimmed.is_empty() {
        return if is_mono { "monospace".to_string() } else { "sans-serif".to_string() };
    }

    let registry = if is_mono { MONO_FONT_REGISTRY } else { FONT_REGISTRY };
    
    for font in registry {
        if font.name.eq_ignore_ascii_case(trimmed) || font.css_stack.eq_ignore_ascii_case(trimmed) {
            return font.css_stack.to_string();
        }
    }

    if trimmed.contains(',') {
        trimmed.to_string()
    } else {
        format!("'{}', {}", trimmed, if is_mono { "monospace" } else { "sans-serif" })
    }
}

pub fn build_google_font_imports(font_stacks: &[&str]) -> String {
    let mut families = Vec::new();

    for stack in font_stacks {
        // FATAL BUG FIX: Isolate the primary font name before any commas
        let first_font = stack
            .split(',')
            .next()
            .unwrap_or("")
            .trim()
            .trim_matches('\'')
            .trim_matches('"');
            
        if first_font.is_empty() { continue; }

        let mut found = false;
        for font in FONT_REGISTRY.iter().chain(MONO_FONT_REGISTRY.iter()) {
            if font.name.eq_ignore_ascii_case(first_font) {
                if let Some(google_name) = font.google_font_name {
                    families.push(google_name.to_string());
                }
                found = true;
                break;
            }
        }

        if !found && !matches!(first_font.to_lowercase().as_str(), "serif" | "sans-serif" | "monospace" | "cursive" | "fantasy" | "system-ui") {
            let encoded = first_font.replace(' ', "+");
            families.push(format!("{}:wght@400;700", encoded));
        }
    }

    if families.is_empty() { return String::new(); }

    families.sort();
    families.dedup();

    let query = families.join("&amp;family=");
    format!("<link href=\"https://fonts.googleapis.com/css2?family={}&amp;display=swap\" rel=\"stylesheet\" />", query)
}