use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// Strict ASCII set that encodes dangerous CSS/XML characters but leaves standard text alone.
/// Includes Hash (#) to prevent browser from truncating image data inside url() blocks.
const SVG_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'#')
    .add(b'%')
    .add(b'{')
    .add(b'}')
    .add(b'|')
    .add(b'\\')
    .add(b'^')
    .add(b'~')
    .add(b'[')
    .add(b']')
    .add(b'`')
    .add(b'?')
    .add(b'\'');

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
    let encoded = utf8_percent_encode(&normalized, SVG_ENCODE_SET).to_string();
    // Single quotes mandatory. Prevents double-quote collision in standard HTML style attributes.
    format!("url('data:image/svg+xml,{}')", encoded)
}

pub fn is_svg(xml: &str) -> bool {
    xml.trim_start().starts_with("<svg")
}