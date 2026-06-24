pub fn beautify_css(raw: &str) -> String {
    // Tokenize: {{...}} spans pass through verbatim; only text outside them is formatted.
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(start) = rest.find("{{") {
        let (before, from_token) = rest.split_at(start);
        out.push_str(&format_outside(before));
        match from_token.find("}}") {
            Some(end) => {
                let (token, after) = from_token.split_at(end + 2);
                out.push_str(token); // verbatim
                rest = after;
            }
            None => {
                out.push_str(from_token); // unterminated token: leave as-is
                return out;
            }
        }
    }
    out.push_str(&format_outside(rest));
    out
}

fn format_outside(raw: &str) -> String {
    raw.replace("{", "{\n    ")
        .replace(";", ";\n    ")
        .replace("}", "\n}\n\n")
        .replace("    \n", "\n") // Clean up trailing spaces on empty lines
        .replace("    }", "}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beautify_keeps_template_tokens() {
        let out = beautify_css("--bg-base: {{DARK_BG_BASE}};");
        assert!(out.contains("{{DARK_BG_BASE}}"), "token shredded: {out}");
    }
}
