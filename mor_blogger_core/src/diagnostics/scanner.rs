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

    // 4. Catch skin CSS styling classes nothing on the page produces
    check_selector_drift(source, out);
}

/// A `.mor-*` class styled inside `<b:skin>` that appears nowhere in the rest of
/// the document (markup or scripts) is drift — usually a selector that survived a
/// module rename (the `.mor-catalog-dropdown` → `.mor-catalog-mega-dropdown`
/// lesson). Scoped to the project's `mor-` namespace, and a bare substring match
/// on the rest of the document, so dynamic/JS-built classes stay quiet: silence
/// over false alarms.
/// Class families rendered outside the theme document (generated static pages),
/// so their absence from the assembled theme source is expected, not drift.
/// ponytail: hardcoded prefix list; derive from render/pages if it grows.
const CROSS_DOCUMENT_PREFIXES: &[&str] = &["mor-analytics-"];

fn check_selector_drift(source: &str, out: &mut Vec<Warning>) {
    // Split the document into skin CSS vs everything else.
    let mut skin = String::new();
    let mut rest = String::with_capacity(source.len());
    let mut cur = 0;
    while let Some(open_rel) = source[cur..].find("<b:skin") {
        let open = cur + open_rel;
        let Some(close_rel) = source[open..].find("</b:skin>") else { break };
        let close = open + close_rel;
        rest.push_str(&source[cur..open]);
        skin.push_str(&source[open..close]);
        cur = close;
    }
    rest.push_str(&source[cur..]);
    if skin.is_empty() {
        return;
    }

    let mut styled = std::collections::BTreeSet::new();
    let mut i = 0;
    while let Some(rel) = skin[i..].find(".mor-") {
        let start = i + rel + 1; // past the dot
        let end = skin[start..]
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
            .map(|e| start + e)
            .unwrap_or(skin.len());
        styled.insert(&skin[start..end]);
        i = end;
    }

    for class in styled {
        if CROSS_DOCUMENT_PREFIXES.iter().any(|p| class.starts_with(p)) {
            continue;
        }
        if !rest.contains(class) {
            out.push(Warning::warn(
                "CSS_SELECTOR_DRIFT",
                format!(
                    ".{class} is styled in the skin CSS but never rendered by this template permutation's markup or scripts — stale selector, or its module isn't active."
                ),
            ));
        }
    }
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

        // A JS `<` operator (`a < b`) breaks XML; a `<` opening a live Blogger tag
        // (`<data:post.commentJso/>`, `<b:if …>`) is valid and must NOT be flagged.
        // Distinguish by the next char: XML tags start with a name char / `/` `!` `?`.
        // `&&` / `& ` catch bare ampersands while leaving entities (`&#39;`) alone.
        // ponytail: heuristic, not a full parser — `i<n` (no spaces) reads as a tag
        // and slips through; the spaced `i < n` style is caught. Ceiling: a real
        // JS+XML parser if that false-negative ever bites.
        if has_raw_lt(body) || body.contains("&&") || body.contains("& ") {
            out.push(Warning::error(
                "SCRIPT_NEEDS_CDATA",
                "An inline <script> contains a raw '<' or '&' but isn't wrapped in CDATA — Blogger's strict XML parser will reject the whole theme. Wrap the script body in `//<![CDATA[` … `//]]>` (see the 'Safe custom script' gadget for a working skeleton).",
            ));
            return; // one hit is enough guidance
        }
        i = body_start + close_rel + "</script>".len();
    }
}

/// True if `body` holds a `<` used as a JS operator rather than opening an XML tag.
/// A well-formed tag's `<` is followed by a name char (or `/` `!` `?`); anything else
/// (space, `=`, digit, `(`, EOF) means it's an operator that will break the parser.
fn has_raw_lt(body: &str) -> bool {
    let b = body.as_bytes();
    b.iter().enumerate().any(|(i, &c)| {
        c == b'<'
            && !matches!(b.get(i + 1), Some(n) if n.is_ascii_alphabetic() || matches!(n, b'/' | b'!' | b'?' | b'_'))
    })
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
    fn blogger_data_tags_in_script_are_clean() {
        // Standard Blogger comment scripts: <data:.../> are live XML tags and &#39; is
        // a valid entity — neither needs CDATA, so neither may be flagged.
        let src = "<script>BLOG_CMT_createIframe(&#39;<data:post.appRpcRelayPath/>&#39;);\
                   blogger.initThreadedComments(<data:post.commentJso/>);</script>";
        let mut out = Vec::new();
        run_text_checks(src, &mut out);
        assert!(!out.iter().any(|w| w.code == "SCRIPT_NEEDS_CDATA"));
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

    #[test]
    fn selector_drift_flags_unrendered_class() {
        let src = "<b:skin>.mor-ghost { color: red; } .mor-real { color: blue; }</b:skin><body class='mor-real'/>";
        let mut out = Vec::new();
        check_selector_drift(src, &mut out);
        let drifted: Vec<_> = out.iter().filter(|w| w.code == "CSS_SELECTOR_DRIFT").collect();
        assert_eq!(drifted.len(), 1);
        assert!(drifted[0].message.contains(".mor-ghost"));
    }

    #[test]
    fn selector_drift_quiet_for_script_built_classes_and_pages() {
        // Class named in a script string = produced at runtime; analytics classes
        // render on a separate generated page. Neither is drift.
        let src = "<b:skin>.mor-popup{} .mor-analytics-grid{}</b:skin><script>el.className = 'mor-popup';</script>";
        let mut out = Vec::new();
        check_selector_drift(src, &mut out);
        assert!(out.is_empty(), "{:?}", out.iter().map(|w| w.format_line()).collect::<Vec<_>>());
    }
}

fn unresolved_token_sample(source: &str) -> Option<String> {
    let start = source.find("{{")?;
    let after_start = &source[start..];
    let end_rel = after_start.find("}}")?;
    Some(after_start[..end_rel + 2].to_string())
}
