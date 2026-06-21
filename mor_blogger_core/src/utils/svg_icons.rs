pub fn ensure_svg_xmlns(svg: &str) -> String {
    let trimmed = svg.trim();

    if trimmed.contains("xmlns=") {
        return trimmed.to_string();
    }

    if let Some(pos) = trimmed.find("<svg") {
        if let Some(end) = trimmed[pos..].find('>') {
            let insert_at = pos + end;

            let mut out = String::new();
            out.push_str(&trimmed[..insert_at]);
            out.push_str(r#" xmlns="http://www.w3.org/2000/svg""#);
            out.push_str(&trimmed[insert_at..]);

            return out;
        }
    }

    trimmed.to_string()
}

pub fn svg_to_data_uri(svg: &str) -> String {
    // Drop a leading UTF-8 BOM if present. `trim()` / `trim_start()` do NOT
    // remove it (U+FEFF is not whitespace), and it would otherwise ride into
    // the data URI as raw bytes and break parsing.
    let svg = svg.trim_start_matches('\u{FEFF}');

    let normalized = ensure_svg_xmlns(svg);

    // Collapse all whitespace (newlines, tabs, runs of spaces) down to single
    // spaces. A raw newline inside the `url('...')` string below is a CSS parse
    // error that drops the rule; because icons live in the shared theme
    // stylesheet, a dropped rule can take neighbouring rules down with it.
    // Collapsing is harmless for icon-shaped SVGs.
    let collapsed: String = normalized.split_whitespace().collect::<Vec<_>>().join(" ");

    // Percent-encode every character that is significant inside a data: URI used
    // in a CSS url(). NOTE: '%' MUST be encoded first, or it double-encodes the
    // escapes added afterwards. The value is wrapped in single quotes, so '\''
    // must be encoded too (many real .svg files use single-quoted attributes),
    // and '\\' is the CSS escape character so it is encoded as well.
    let encoded = collapsed
        .replace('%', "%25")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('#', "%23")
        .replace('"', "%22")
        .replace('\'', "%27")
        .replace('\\', "%5C");

    format!("url('data:image/svg+xml,{}')", encoded)
}

/// Returns true if `xml` contains an `<svg>` root element.
///
/// This tolerates what real exported `.svg` files actually start with — a
/// UTF-8 BOM, an `<?xml ...?>` prolog, a `<!DOCTYPE>`, or leading comments —
/// instead of requiring the literal first characters to be `<svg`. That strict
/// check was rejecting most files coming in from the OS picker, the paste box,
/// and drag-drop.
pub fn is_svg(xml: &str) -> bool {
    let s = xml.trim_start_matches('\u{FEFF}');
    match s.find("<svg") {
        Some(i) => {
            // Make sure "<svg" begins a tag (reject substrings like "<svgx").
            let after = &s[i + 4..];
            after.starts_with(|c: char| c.is_whitespace() || c == '>' || c == '/')
        }
        None => false,
    }
}
