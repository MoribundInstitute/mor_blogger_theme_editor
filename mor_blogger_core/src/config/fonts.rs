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

/// Primary family name from a CSS font stack (text before the first comma).
pub fn primary_font_from_stack(stack: &str) -> &str {
    stack
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches('\'')
        .trim_matches('"')
}

/// True when the primary font does not require a Google Fonts stylesheet.
pub fn is_system_safe_font(name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() {
        return true;
    }

    if name.eq_ignore_ascii_case("serif")
        || name.eq_ignore_ascii_case("sans-serif")
        || name.eq_ignore_ascii_case("monospace")
        || name.eq_ignore_ascii_case("cursive")
        || name.eq_ignore_ascii_case("fantasy")
        || name.eq_ignore_ascii_case("system-ui")
        || name.eq_ignore_ascii_case("-apple-system")
        || name.eq_ignore_ascii_case("blinkmacsystemfont")
        || name.eq_ignore_ascii_case("segoe ui")
        || name.eq_ignore_ascii_case("helvetica")
        || name.eq_ignore_ascii_case("helvetica neue")
        || name.eq_ignore_ascii_case("arial")
        || name.eq_ignore_ascii_case("verdana")
        || name.eq_ignore_ascii_case("tahoma")
        || name.eq_ignore_ascii_case("trebuchet ms")
        || name.eq_ignore_ascii_case("georgia")
        || name.eq_ignore_ascii_case("times")
        || name.eq_ignore_ascii_case("times new roman")
        || name.eq_ignore_ascii_case("courier")
        || name.eq_ignore_ascii_case("courier new")
        || name.eq_ignore_ascii_case("lucida console")
        || name.eq_ignore_ascii_case("lucida sans unicode")
        || name.eq_ignore_ascii_case("comic sans ms")
        || name.eq_ignore_ascii_case("impact")
    {
        return true;
    }

    for font in FONT_REGISTRY.iter().chain(MONO_FONT_REGISTRY.iter()) {
        if font.name.eq_ignore_ascii_case(name) {
            return font.google_font_name.is_none();
        }
    }

    false
}

fn implies_serif(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("serif")
        || lower.contains("roman")
        || lower.contains("garamond")
        || lower.contains("georgia")
        || lower.contains("palatino")
        || lower.contains("times")
        || lower.contains("baskerville")
        || lower.contains("caslon")
        || lower.contains("bodoni")
}

pub fn resolve_font_stack_with_fallback(raw_input: &str, is_mono: bool) -> String {
    let trimmed = raw_input.trim();
    if trimmed.is_empty() {
        return if is_mono {
            "ui-monospace, 'Courier New', Courier, monospace".to_string()
        } else {
            "system-ui, -apple-system, sans-serif".to_string()
        };
    }

    let registry = if is_mono { MONO_FONT_REGISTRY } else { FONT_REGISTRY };

    for font in registry {
        if font.name.eq_ignore_ascii_case(trimmed) || font.css_stack.eq_ignore_ascii_case(trimmed) {
            return font.css_stack.to_string();
        }
    }

    if trimmed.contains(',') {
        return trimmed.to_string();
    }

    let generic = if is_mono {
        "monospace"
    } else if implies_serif(trimmed) {
        "serif"
    } else {
        "sans-serif"
    };

    if trimmed.contains(' ') {
        format!("\"{}\", {}", trimmed, generic)
    } else {
        format!("{}, {}", trimmed, generic)
    }
}

fn google_family_query(primary: &str) -> String {
    for font in FONT_REGISTRY.iter().chain(MONO_FONT_REGISTRY.iter()) {
        if font.name.eq_ignore_ascii_case(primary) {
            if let Some(google_name) = font.google_font_name {
                return format!(
                    "{}:wght@400;500;600;700",
                    google_name.replace(' ', "+")
                );
            }
            break;
        }
    }
    format!(
        "{}:wght@400;500;600;700",
        primary.replace(' ', "+")
    )
}

/// Build Google Fonts `<link>` tags for non-system typography stacks.
pub fn build_google_font_imports(font_stacks: &[&str]) -> String {
    let mut families: Vec<String> = Vec::new();

    for stack in font_stacks {
        let primary = primary_font_from_stack(stack);
        if primary.is_empty() || is_system_safe_font(primary) {
            continue;
        }
        let query = google_family_query(primary);
        if !families.iter().any(|f| f == &query) {
            families.push(query);
        }
    }

    if families.is_empty() {
        return String::new();
    }

    let mut out =
        String::with_capacity(96 + families.iter().map(|f| f.len()).sum::<usize>());
    out.push_str(
        "<link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family=",
    );
    for (i, family) in families.iter().enumerate() {
        if i > 0 {
            out.push_str("&amp;family=");
        }
        out.push_str(family);
    }
    out.push_str("&amp;display=swap\"/>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ThemeConfig;

    #[test]
    fn skips_system_safe_fonts() {
        assert!(is_system_safe_font("Courier New"));
        assert!(is_system_safe_font("system-ui"));
        assert!(is_system_safe_font("serif"));
        assert!(!is_system_safe_font("Inter"));
    }

    #[test]
    fn builds_weighted_google_link() {
        let link = build_google_font_imports(&["Inter", "'Playfair Display', serif"]);
        assert!(link.contains("rel=\"stylesheet\""));
        assert!(link.contains("Inter:wght@400;500;600;700"));
        assert!(link.contains("Playfair+Display:wght@400;500;600;700"));
        assert!(link.contains("&amp;display=swap"));
    }

    #[test]
    fn system_stack_produces_no_link() {
        let link = build_google_font_imports(&[
            "system-ui, -apple-system, sans-serif",
            "'Courier New', Courier, monospace",
        ]);
        assert!(link.is_empty());
    }

    #[test]
    fn exported_theme_injects_font_link_in_head() {
        let mut config = ThemeConfig::default();
        config.typography.body_font_stack = "Inter".to_string();
        let xml = crate::render::render_theme(&config);
        assert!(xml.contains("fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700"));
    }

    #[test]
    fn empty_stacks_fall_back_to_native() {
        assert_eq!(resolve_font_stack_with_fallback("", false), "system-ui, -apple-system, sans-serif");
        assert_eq!(resolve_font_stack_with_fallback("   ", false), "system-ui, -apple-system, sans-serif");
        assert_eq!(resolve_font_stack_with_fallback("", true), "ui-monospace, 'Courier New', Courier, monospace");
    }

    #[test]
    fn custom_spaced_font_uses_double_quotes() {
        assert_eq!(resolve_font_stack_with_fallback("My Custom Font", false), "\"My Custom Font\", sans-serif");
        assert_eq!(resolve_font_stack_with_fallback("Old Garamond Text", false), "\"Old Garamond Text\", serif");
        assert_eq!(resolve_font_stack_with_fallback("Code Thing", true), "\"Code Thing\", monospace");
    }

    #[test]
    fn single_word_custom_font_no_quotes() {
        assert_eq!(resolve_font_stack_with_fallback("Raleway", false), "Raleway, sans-serif");
    }

    #[test]
    fn serif_name_detection() {
        assert_eq!(resolve_font_stack_with_fallback("MySerif", false), "MySerif, serif");
        assert_eq!(resolve_font_stack_with_fallback("BookRoman", false), "BookRoman, serif");
    }

    #[test]
    fn css_builder_maps_font_variables() {
        let mut config = ThemeConfig::default();
        config.typography.body_font_stack = "Inter".to_string();
        config.typography.heading_font_stack = "Playfair Display".to_string();
        config.typography.mono_font_stack = "Courier New".to_string();
        let css = crate::render::css_builder::build_master_css(&[], &config);
        assert!(css.contains("--font-body: 'Inter', sans-serif;"));
        assert!(css.contains("--font-heading: 'Playfair Display', serif;"));
        assert!(css.contains("--font-mono: 'Courier New', Courier, monospace;"));
        assert!(css.contains("--font-size-base: 16px;"));
        assert!(css.contains("--line-height-body: 1.6;"));
        assert!(css.contains("--heading-weight: 700;"));
    }
}