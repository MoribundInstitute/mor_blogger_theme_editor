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
    FontPreset {
        name: "Times New Roman",
        css_stack: "'Times New Roman', Times, serif",
        google_font_name: None,
        category: "Web 1.0 / Classic",
    },
    FontPreset {
        name: "Arial",
        css_stack: "Arial, sans-serif",
        google_font_name: None,
        category: "Web 1.0 / Classic",
    },
    FontPreset {
        name: "Courier New",
        css_stack: "'Courier New', Courier, monospace",
        google_font_name: None,
        category: "Web 1.0 / Classic",
    },
    FontPreset {
        name: "Comic Sans",
        css_stack: "'Comic Sans MS', 'Comic Sans', cursive",
        google_font_name: None,
        category: "Web 1.0 / Classic",
    },
    FontPreset {
        name: "Impact",
        css_stack: "Impact, Haettenschweiler, 'Arial Narrow Bold', sans-serif",
        google_font_name: None,
        category: "Web 1.0 / Classic",
    },
    // --- Google Fonts Favorites ---
    FontPreset {
        name: "IM Fell English",
        css_stack: "'IM Fell English', Georgia, serif",
        google_font_name: Some("IM+Fell+English"),
        category: "Retro / Fantasy",
    },
    FontPreset {
        name: "Noto Serif JP",
        css_stack: "'Noto Serif JP', serif",
        google_font_name: Some("Noto+Serif+JP:wght@400;700"),
        category: "International Serif",
    },
    FontPreset {
        name: "Noto Serif KR",
        css_stack: "'Noto Serif KR', serif",
        google_font_name: Some("Noto+Serif+KR:wght@400;700"),
        category: "International Serif",
    },
    FontPreset {
        name: "Montserrat",
        css_stack: "Montserrat, sans-serif",
        google_font_name: Some("Montserrat:wght@400;700"),
        category: "Web 2.0 / Modern",
    },
];

/// Static catalog specifically for monospace fonts.
pub const MONO_FONT_REGISTRY: &[FontPreset] = &[
    FontPreset {
        name: "Courier New",
        css_stack: "'Courier New', Courier, monospace",
        google_font_name: None,
        category: "Web 1.0 / Classic",
    },
    FontPreset {
        name: "Fira Code",
        css_stack: "'Fira Code', monospace",
        google_font_name: Some("Fira+Code:wght@400;500;600"),
        category: "Modern Mono",
    },
    FontPreset {
        name: "JetBrains Mono",
        css_stack: "'JetBrains Mono', monospace",
        google_font_name: Some("JetBrains+Mono:wght@400;700"),
        category: "Modern Mono",
    },
    FontPreset {
        name: "Source Code Pro",
        css_stack: "'Source Code Pro', monospace",
        google_font_name: Some("Source+Code+Pro:wght@400;700"),
        category: "Modern Mono",
    },
    FontPreset {
        name: "Roboto Mono",
        css_stack: "'Roboto Mono', monospace",
        google_font_name: Some("Roboto+Mono:wght@400;700"),
        category: "Modern Mono",
    },
];

/// Resolves any arbitrary string into a functional CSS stack string.
/// If match found in registry, uses complete fallbacks. Else passes raw name.
pub fn resolve_font_stack(input: &str) -> String {
    resolve_font_stack_with_fallback(input, "serif")
}

/// Resolves any arbitrary string into a functional CSS stack string with a specific fallback.
pub fn resolve_font_stack_with_fallback(input: &str, fallback: &str) -> String {
    let trimmed = input.trim();
    
    // Check both registries for a match
    for font in FONT_REGISTRY.iter().chain(MONO_FONT_REGISTRY.iter()) {
        if font.name.eq_ignore_ascii_case(trimmed) {
            return font.css_stack.to_string();
        }
    }
    
    // Fallback for custom entries typed manually by user
    if trimmed.contains(',') {
        trimmed.to_string()
    } else if trimmed.is_empty() {
        fallback.to_string()
    } else {
        format!("'{}', {}", trimmed, fallback)
    }
}

/// Builds a Google Fonts `<link>` tag for the provided font stacks.
pub fn build_google_font_imports(font_stacks: &[&str]) -> String {
    let mut families = Vec::new();
    
    for stack in font_stacks {
        let trimmed = stack.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Check if it's in our registry
        let mut found = false;
        for font in FONT_REGISTRY.iter().chain(MONO_FONT_REGISTRY.iter()) {
            if font.name.eq_ignore_ascii_case(trimmed) {
                if let Some(google_name) = font.google_font_name {
                    families.push(google_name.to_string());
                }
                found = true;
                break;
            }
        }
        
        // If not in registry, but looks like a custom font name (no commas, not a generic family)
        if !found && !trimmed.contains(',') && !matches!(trimmed.to_lowercase().as_str(), "serif" | "sans-serif" | "monospace" | "cursive" | "fantasy" | "system-ui") {
            let encoded = trimmed.replace(' ', "+");
            families.push(format!("{}:wght@400;700", encoded));
        }
    }
    
    if families.is_empty() {
        return String::new();
    }
    
    // Remove duplicates and sort for consistent output
    families.sort();
    families.dedup();
    
    let query = families.join("&family=");
    format!("<link href=\"https://fonts.googleapis.com/css2?family={}&display=swap\" rel=\"stylesheet\">", query)
}