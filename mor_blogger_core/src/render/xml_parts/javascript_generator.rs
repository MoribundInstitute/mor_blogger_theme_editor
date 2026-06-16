use crate::config::ThemeConfig;

const CUSTOM_BEFORE_BODY_JS_TOKEN: &str = "{{CUSTOM_BEFORE_BODY_JS}}";

fn escape_script_end_tags(javascript: &str) -> String {
    let mut escaped = String::with_capacity(javascript.len());
    let mut index = 0;

    while let Some(relative_start) = javascript[index..].find("</") {
        let start = index + relative_start;
        escaped.push_str(&javascript[index..start]);

        if let Some(tag) = javascript.get(start..start + 8) {
            if tag.eq_ignore_ascii_case("</script") {
                escaped.push_str("<\\/");
                escaped.push_str(&javascript[start + 2..start + 8]);
                index = start + 8;
                continue;
            }
        }

        escaped.push_str("</");
        index = start + 2;
    }

    escaped.push_str(&javascript[index..]);
    escaped
}

fn escape_javascript_cdata(javascript: &str) -> String {
    escape_script_end_tags(javascript).replace("]]>", "]]]]><![CDATA[>")
}

pub fn render_javascript_sockets(mut js_payload: String, config: &ThemeConfig) -> String {
    let custom_javascript = config.plugins.custom_js.trim();
    
    if js_payload.contains(CUSTOM_BEFORE_BODY_JS_TOKEN) {
        js_payload = js_payload.replace(CUSTOM_BEFORE_BODY_JS_TOKEN, custom_javascript);
    } else if !custom_javascript.is_empty() {
        if !js_payload.is_empty() {
            js_payload.push_str("\n\n");
        }
        js_payload.push_str(custom_javascript);
    }

    let trimmed = js_payload.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    format!(
        "<script type='text/javascript'>\n//<![CDATA[\n{}\n//]]>\n</script>",
        escape_javascript_cdata(trimmed)
    )
}