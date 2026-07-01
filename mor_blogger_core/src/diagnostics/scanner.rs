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

    // 3. Catch inline <script> blocks that will break Blogger's strict XML parser
    check_unwrapped_scripts(source, out);
}

/// Inline `<script>` blocks are spliced into the theme verbatim — only the global
/// custom-JS socket gets auto-wrapped. The moment a hand-written module script
/// contains a raw `<` or `&` (e.g. `if (a < b && c)`) outside a CDATA section,
/// Blogger's strict XML parser rejects the *entire* theme. Flag it with the fix
/// here, instead of letting it surface as the generic "not well-formed XML" error.
fn check_unwrapped_scripts(source: &str, out: &mut Vec<Warning>) {
    // Mask every CDATA section first: a script wholly inside one (the gadget/footer
    // convention) — or a `//<![CDATA[ … //]]>` guard inside the script body — is
    // already safe, so its raw chars must not register below.
    let masked = mask_cdata(source);
    let lower = masked.to_ascii_lowercase();

    let mut i = 0;
    while let Some(rel) = lower[i..].find("<script") {
        let open = i + rel;
        let Some(gt) = masked[open..].find('>') else { break };
        let body_start = open + gt + 1;
        let Some(close_rel) = lower[body_start..].find("</script>") else { break };
        let body = &masked[body_start..body_start + close_rel];

        // Raw `<` always breaks XML; `&&` / `& ` are the common bare-ampersand cases.
        // ponytail: heuristic, not a full entity parser — a literal "&amp;" typed in
        // JS won't false-positive, and any real `<` is caught regardless.
        if body.contains('<') || body.contains("&&") || body.contains("& ") {
            out.push(Warning::error(
                "SCRIPT_NEEDS_CDATA",
                "An inline <script> contains a raw '<' or '&' but isn't wrapped in CDATA — Blogger's strict XML parser will reject the whole theme. Wrap the script body in `//<![CDATA[` … `//]]>` (see the 'Safe custom script' gadget for a working skeleton).",
            ));
            return; // one hit is enough guidance
        }
        i = body_start + close_rel + "</script>".len();
    }
}

/// Blank out every `<![CDATA[ … ]]>` region (replace with spaces, keeping newlines)
/// so text inside it never registers as raw markup.
fn mask_cdata(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(start) = rest.find("<![CDATA[") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let end = after.find("]]>").map(|e| e + 3).unwrap_or(after.len());
        for ch in after[..end].chars() {
            out.push(if ch == '\n' { '\n' } else { ' ' });
        }
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_unwrapped_script() {
        let src = "<div><script>if (a < b && c) {}</script></div>";
        let mut out = Vec::new();
        run_text_checks(src, &mut out);
        assert!(out.iter().any(|w| w.code == "SCRIPT_NEEDS_CDATA"));
    }

    #[test]
    fn cdata_guarded_scripts_are_clean() {
        // `//<![CDATA[` guard inside the body — the safe convention.
        let guarded = "<script>\n//<![CDATA[\nif (a < b && c) {}\n//]]>\n</script>";
        let mut out = Vec::new();
        run_text_checks(guarded, &mut out);
        assert!(!out.iter().any(|w| w.code == "SCRIPT_NEEDS_CDATA"));

        // Whole body inside an outer CDATA (gadget/footer convention).
        let wrapped = "<div><![CDATA[<script>if (a < b) {}</script>]]></div>";
        let mut out2 = Vec::new();
        run_text_checks(wrapped, &mut out2);
        assert!(!out2.iter().any(|w| w.code == "SCRIPT_NEEDS_CDATA"));
    }

    #[test]
    fn plain_script_without_specials_is_clean() {
        let src = "<script>console.log('hi');</script>";
        let mut out = Vec::new();
        run_text_checks(src, &mut out);
        assert!(out.is_empty());
    }
}

fn unresolved_token_sample(source: &str) -> Option<String> {
    let start = source.find("{{")?;
    let after_start = &source[start..];
    let end_rel = after_start.find("}}")?;
    Some(after_start[..end_rel + 2].to_string())
}
