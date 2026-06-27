use crate::config::CategoriesPageConfig;
use crate::render::pages::escape_html;

/// Emits the Directory/Categories page as pure layout.
/// Inherits colors and fonts directly from the active theme's CSS variables.
pub fn generate_categories_html(config: &CategoriesPageConfig) -> String {
    let mut html = String::new();

    // 1. Structural CSS (No colors, just grid/flex layout)
    html.push_str(r#"
<style>
/* Hide the default Blogger page title to prevent duplication */
h3.post-title.entry-title, h1.post-title.entry-title { display: none !important; }

.mor-directory { max-width: 1000px; margin: 0 auto; padding: 20px 0; }

.mor-dir-intro { margin-bottom: 2rem; border-bottom: 1px solid var(--border-color); padding-bottom: 1rem; }
.mor-dir-kicker { font-size: 0.85rem; font-weight: 600; color: var(--accent); text-transform: uppercase; letter-spacing: 0.05em; }
.mor-dir-title { margin: 0.5rem 0; font-size: 2.5rem; color: var(--fg-base); }
.mor-dir-desc { color: var(--fg-muted); font-size: 1.1rem; line-height: 1.6; }

.mor-dir-section-title { 
    color: var(--accent); 
    border-bottom: 1px solid var(--border-color); 
    padding-bottom: 6px; 
    margin-top: 40px; 
    margin-bottom: 15px; 
}

.mor-dir-grid { 
    display: grid; 
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); 
    gap: 10px; 
    margin-top: 15px; 
}

.mor-dir-grid a { 
    display: block; 
    padding: 10px; 
    background: var(--bg-elevated); 
    color: var(--fg-base); 
    border: 1px solid var(--border-color); 
    border-radius: var(--radius-md, 6px); 
    text-align: center; 
    font-weight: 600; 
    text-decoration: none; 
    transition: transform 0.2s ease, border-color 0.2s ease, box-shadow 0.2s ease; 
}

.mor-dir-grid a:hover { 
    border-color: var(--accent); 
    transform: translateY(-2px); 
    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
}

.mor-dir-nav-buttons { 
    display: flex; 
    flex-wrap: wrap; 
    gap: 8px; 
    margin: 20px 0; 
    align-items: center; 
}

.mor-dir-nav-buttons span { 
    color: var(--fg-muted); 
    font-weight: 600; 
    margin-right: 8px; 
}

.mor-dir-nav-buttons a { 
    padding: 6px 12px; 
    background: var(--bg-panel); 
    color: var(--fg-base); 
    border: 1px solid var(--border-color); 
    border-radius: var(--radius-sm, 4px); 
    text-decoration: none; 
    transition: border-color 0.2s ease, background-color 0.2s ease; 
}

.mor-dir-nav-buttons a:hover { 
    border-color: var(--accent); 
    background: var(--bg-elevated);
}
</style>
"#);

    // 2. Intro Section
    html.push_str(&format!(
        r##"<div class="mor-directory">
  <section class="mor-dir-intro">
    <div class="mor-dir-kicker">{kicker}</div>
    <h2 class="mor-dir-title">{title}</h2>
    <p class="mor-dir-desc">{desc}</p>
  </section>
"##,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description),
    ));

    // 3. Global A-Z Bar
    html.push_str(r#"
  <h3 class="mor-dir-section-title">Global Index</h3>
  <div class="mor-dir-nav-buttons">
    <span>A–Z:</span>
    <a href="/search/label/A">A</a> <a href="/search/label/B">B</a> <a href="/search/label/C">C</a> <a href="/search/label/D">D</a>
    <a href="/search/label/E">E</a> <a href="/search/label/F">F</a> <a href="/search/label/G">G</a> <a href="/search/label/H">H</a>
    <a href="/search/label/I">I</a> <a href="/search/label/J">J</a> <a href="/search/label/K">K</a> <a href="/search/label/L">L</a>
    <a href="/search/label/M">M</a> <a href="/search/label/N">N</a> <a href="/search/label/O">O</a> <a href="/search/label/P">P</a>
    <a href="/search/label/Q">Q</a> <a href="/search/label/R">R</a> <a href="/search/label/S">S</a> <a href="/search/label/T">T</a>
    <a href="/search/label/U">U</a> <a href="/search/label/V">V</a> <a href="/search/label/W">W</a> <a href="/search/label/X">X</a>
    <a href="/search/label/Y">Y</a> <a href="/search/label/Z">Z</a>
  </div>
"#);

    // 4. Dynamic A-Z Sub-sections (Populated by JS)
    let dynamic_sections = [
        ("author-links", "By Author"),
        ("musician-links", "By Musical Artist"),
        ("painter-links", "By Painter"),
        ("actor-links", "By Actor"),
        ("anime-links", "By Anime"),
        ("kdrama-links", "By Korean Drama"),
        ("animal-links", "By Animal"),
    ];

    for (id, label) in dynamic_sections {
        html.push_str(&format!(
            r#"
  <h3 class="mor-dir-section-title">{label}</h3>
  <div class="mor-dir-nav-buttons" id="{id}">
    <span>A–Z:</span>
  </div>
"#
        ));
    }

    // 5. Subject Index (Dewey System) tied to User Config
    html.push_str(
        r#"
<h3 class="mor-dir-section-title">By Subject</h3>
<div class="mor-dir-nav-buttons">
<span>Jump to:</span>
"#,
    );

    for section in &config.enabled_sections {
        let anchor_id = section.split_whitespace().next().unwrap_or(section);

        // FIX: Upgraded to r##"..."## to protect href="# from terminating string
        html.push_str(&format!(
            r##" <a href="#subject-{id}">{name}</a> "##,
            id = escape_html(anchor_id),
            name = escape_html(section)
        ));
    }
    html.push_str("   </div>\n");

    for section in &config.enabled_sections {
        let anchor_id = section.split_whitespace().next().unwrap_or(section);
        html.push_str(&format!(
            r#"
<h3 id="subject-{id}" class="mor-dir-section-title">
 <a href="/search/label/{id}" style="color: inherit; text-decoration: none;">{name}</a>
 </h3>
 <div class="mor-dir-grid">
 <a href="/search/label/{id}">Browse All {id}</a>
 </div>
 "#,
            id = escape_html(anchor_id),
            name = escape_html(section)
        ));
    }
    html.push_str("</div>\n");

    // 6. Inject the user's Javascript logic to populate A-Z dynamically
    let js_template = include_str!("../../html_page_stencils/categories_script.js");
    html.push_str(js_template);

    format!("{}{}", crate::render::pages::page_chrome_overrides(&config.layout), html)
}
