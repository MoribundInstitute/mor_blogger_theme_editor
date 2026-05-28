//! CSS Builder module for sanitizing and concatenating base CSS files.

/// Cleans user-uploaded or modular CSS by stripping out existing Blogger XML wrappers
/// so we can safely concatenate and re-wrap it later without nesting errors.
pub fn clean_raw_css(input: &str) -> String {
    let mut cleaned = input.to_string();

    // Strip out any variations of the Blogger skin tags and CDATA wrappers
    cleaned = cleaned.replace("<b:skin>", "");
    cleaned = cleaned.replace("</b:skin>", "");
    cleaned = cleaned.replace("<![CDATA[", "");
    cleaned = cleaned.replace("]]>", "");
    
    // Trim excess whitespace from the top and bottom
    cleaned.trim().to_string()
}

/// Builds the master CSS baseline from modular chunks or user uploads,
/// ensuring it is perfectly wrapped for Blogger.
pub fn build_master_css(base_css_chunks: &[&str]) -> String {
    let mut combined_css = String::new();

    // Loop through each chunk, sanitize it, and append it to our master string
    for chunk in base_css_chunks {
        let cleaned = clean_raw_css(chunk);
        if !cleaned.is_empty() {
            combined_css.push_str(&cleaned);
            combined_css.push_str("\n\n");
        }
    }

    // Wrap the final massive string in a single set of Blogger's required tags,
    // and inject the preset override token at the very end.
    format!(
        "<b:skin><![CDATA[\n{}\n/* ===== Active Preset CSS ===== */\n{{{{PRESET_CSS}}}}\n]]></b:skin>",
        combined_css
    )
}
