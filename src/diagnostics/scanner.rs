use super::Warning;

// Common HTML entities that cause strict XML parsers to crash.
const INVALID_ENTITIES: &[(&str, &str)] = &[
    ("&copy;", "&#169;"),
    ("&nbsp;", "&#160;"),
    ("&mdash;", "&#8212;"),
    ("&ndash;", "&#8211;"),
    ("&trade;", "&#8482;"),
    ("&reg;", "&#174;"),
    ("&hellip;", "&#8230;"),
];

pub fn run_text_checks(source: &str, out: &mut Vec<Warning>) {
    // 1. Catch HTML entities that crash the strict XML parser
    for (entity, fix) in INVALID_ENTITIES {
        if source.contains(entity) {
            out.push(Warning::error(
                "INVALID_XML_ENTITY",
                format!("Strict XML does not support the HTML entity '{entity}'. Use the numeric code '{fix}' instead."),
            ));
        }
    }

    // 2. Catch unresolved engine tokens
    if source.contains("{{") && source.contains("}}") {
        let sample = unresolved_token_sample(source)
            .unwrap_or_else(|| "unknown unresolved token".to_string());

        out.push(Warning::error(
            "UNRESOLVED_TOKEN",
            format!("Rendered XML contains unresolved template placeholder: {sample}"),
        ));
    }
}

fn unresolved_token_sample(source: &str) -> Option<String> {
    let start = source.find("{{")?;
    let after_start = &source[start..];
    let end_rel = after_start.find("}}")?;
    Some(after_start[..end_rel + 2].to_string())
}
