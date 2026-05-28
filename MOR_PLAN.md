This is an excellent architectural pivot. Bringing the modular "block" philosophy from MorBlocks into the Theme Architect completely shifts the app from being a simple "color palette reskinner" to a genuine layout engine. It perfectly addresses the core issue: a GTK theme (or a retro MMORPG theme) relies on structural paradigms—headerbars, docks, and popovers—not just a set of hex codes.

Your immediate next step recommendation is completely correct. The most common pitfall when doing a refactor like this is breaking Blogger's extremely rigid b:section/b:widget XML parsing. By doing a "safe infrastructure pass" first (Phases 1 & 2), you guarantee the Rust plumbing works before you touch the file system or introduce new layout HTML.

Here is the exact Rust code and checklist to execute your safe infrastructure pass.

Phase 1: Add the Config Model
First, we define TemplatePackConfig and implement Default to mirror the variants you are currently using.

Using #[serde(default)] on the new field in ThemeConfig is the critical piece here. It ensures that any old user presets, built-in presets, or GTK imports saved to disk that lack the template_pack key will seamlessly deserialize and fall back to the terminal layout, rather than crashing the app.

src/models/config.rs (or wherever your config models live)

Rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplatePackConfig {
    pub header_variant: String,
    pub main_variant: String,
    pub left_sidebar_variant: String,
    pub right_sidebar_variant: String,
    pub script_variant: String,
    pub icon_pack: String,
}

// These defaults preserve the current layout structure
impl Default for TemplatePackConfig {
    fn default() -> Self {
        Self {
            header_variant: "terminal".to_string(),
            main_variant: "sidebars".to_string(),
            left_sidebar_variant: "blogger_widgets".to_string(),
            right_sidebar_variant: "toc".to_string(),
            script_variant: "terminal_panels".to_string(),
            icon_pack: "default".to_string(),
        }
    }
}

// Update your main ThemeConfig struct:
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    // ... [Your existing fields like colors, typography, etc.] ...
    
    #[serde(default)]
    pub template_pack: TemplatePackConfig,
}
Checklist action: Run a quick search for ThemeConfig { in your codebase to see if any handmade constructors need template_pack: TemplatePackConfig::default(), or ..Default::default() added to make cargo check happy.

Phase 2: Add the Resolver (Without Moving Files)
Create a new file for the resolver. In this phase, the resolver takes the config but deliberately ignores the variants. It just returns your existing files. This isolates the refactor to prove that passing file contents as &'static str fields works exactly like invoking include_str! locally inside the generator.

src/render/template_pack.rs (or similar)

Rust
use crate::models::ThemeConfig; // Adjust import based on your structure

pub struct TemplateParts {
    pub meta: &'static str,
    pub css: &'static str,
    pub header: &'static str,
    pub main: &'static str,
    pub sidebar_left: &'static str,
    pub sidebar_right: &'static str,
    pub javascript: &'static str,
}

pub fn resolve_template_parts(_config: &ThemeConfig) -> TemplateParts {
    // PHASE 2: Ignore _config for now to do a safe, behavior-preserving pass.
    // We return the exact same files to verify wiring without breaking Blogger XML.
    // Note: Adjust the relative paths below depending on where this module lives!
    
    TemplateParts {
        meta: include_str!("../../template_parts/meta.xml"),
        css: include_str!("../../template_parts/css.xml"),
        header: include_str!("../../template_parts/header.xml"),
        main: include_str!("../../template_parts/main.xml"),
        sidebar_left: include_str!("../../template_parts/sidebar_left.xml"),
        sidebar_right: include_str!("../../template_parts/sidebar_right.xml"),
        javascript: include_str!("../../template_parts/javascript_before_body_tag.xml"),
    }
}
Phase 2 (Continued): Wire xml_generator.rs
Now, go into your XML generation file, strip out the hardcoded include_str! macros at the top, and invoke the resolver instead.

src/render/xml_generator.rs

Rust
use crate::render::template_pack::resolve_template_parts;

pub fn generate_blogger_xml(config: &ThemeConfig) -> String {
    // 1. Resolve template parts based on config
    let parts = resolve_template_parts(config);
    
    // 2. Consume the resolved parts instead of calling include_str!() directly
    let meta_str = parts.meta;
    let css_str = parts.css;
    let header_str = parts.header;
    let main_str = parts.main;
    let sidebar_left_str = parts.sidebar_left;
    let sidebar_right_str = parts.sidebar_right;
    let javascript_str = parts.javascript;

    // ... Keep all of your existing replacement, string assembly, and 
    // token interpolation logic perfectly intact ...
}
The Validation Step (Phases 1 & 2 Checklist)
Once you've made these changes:

Run cargo check.

Open the Dioxus app and ensure old presets load without crashing.

Export a preset to an XML file.

The ultimate test: Compare the newly exported XML against a backup of an older export. It should be byte-for-byte identical.

Upload it to a test Blogger blog to completely confirm no widgets or b:includable blocks were corrupted.

Looking Ahead to Phase 3
Once this safe pass is verified, transitioning to Phase 3 is incredibly easy. You'll just move the files into their new subdirectories (headers/, mains/, etc.) and swap the dummy resolver logic out for clean match statements that fall back safely:

Rust
// Preview of Phase 3 implementation
let header = match config.template_pack.header_variant.as_str() {
    // "gtk_headerbar" => include_str!("../../template_parts/headers/gtk_headerbar.xml"), // Phase 4
    "terminal" | _  => include_str!("../../template_parts/headers/terminal.xml"),
};
Using the wildcard _ ensures that if a user deletes a layout, loads a corrupted preset, or inputs a missing template name, the compiler falls back gracefully to the safe terminal default rather than panicking the app.