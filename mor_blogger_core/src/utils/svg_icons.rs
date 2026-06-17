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
    let normalized = ensure_svg_xmlns(svg);
    // Strictly encode only the characters that break data: URI parsing inside CSS url() / mask-image.
    // Do NOT encode spaces (per requirements). This prevents truncation on # (colors) and " (attrs).
    let encoded = normalized
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('#', "%23")
        .replace('"', "%22");
    // Single quotes mandatory around the data URI value. Prevents " collision with HTML attrs.
    format!("url('data:image/svg+xml,{}')", encoded)
}

pub fn is_svg(xml: &str) -> bool {
    xml.trim_start().starts_with("<svg")
}