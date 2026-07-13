use crate::config::ArchivePageConfig;
use crate::render::util::escape_attr as escape_html;

/// Emits the Archive page as pure layout. This stencil inherits both its tokens
/// and its layout classes (`.archive-year`, `.post-grid`, ...) from the Mor
/// theme skin it is pasted inside, so it emits no `<style>` block of its own.
/// See the portability caveat in the README if you ship this as a standalone
/// compendium stencil.
pub fn generate_archive_html(config: &ArchivePageConfig) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r##"<div class="mor-archive-section">
  <section class="mor-archive-intro">
    <div class="mor-archive-kicker">{kicker}</div>
    <h2 class="mor-archive-title">{title}</h2>
    <p class="mor-archive-desc">{desc}</p>
  </section>

  <div id="archive-container">
    <div class="mor-archive-status">Loading archive...</div>
  </div>
</div>
"##,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description)
    ));

    // Script fragment lives next to the other stencils; `{{MAX_RESULTS}}` is the
    // only data-driven slot, injected at render time.
    let js_template = include_str!("../../html_page_stencils/archive_script.js");
    html.push_str(&js_template.replace("{{MAX_RESULTS}}", &config.max_results.to_string()));

    format!("{}{}", crate::render::pages::page_chrome_overrides(&config.layout), html)
}
