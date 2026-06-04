# Theme Architect Template Pack Infrastructure Pass

This is an excellent architectural pivot. Bringing the modular “block” philosophy from MorBlocks into the Theme Architect completely shifts the app from being a simple “color palette reskinner” to a genuine layout engine. It perfectly addresses the core issue: a GTK theme, or a retro MMORPG theme, relies on structural paradigms—headerbars, docks, and popovers—not just a set of hex codes.

Your immediate next-step recommendation is correct. The most common pitfall when doing a refactor like this is breaking Blogger’s extremely rigid `b:section` / `b:widget` XML parsing. By doing a safe infrastructure pass first, with Phases 1 and 2, you guarantee the Rust plumbing works before you touch the file system or introduce new layout HTML.

Here is the exact Rust code and checklist to execute your safe infrastructure pass.

---

## Phase 1: Add the Config Model

First, define `TemplatePackConfig` and implement `Default` to mirror the variants you are currently using.

Using `#[serde(default)]` on the new field in `ThemeConfig` is the critical piece here. It ensures that any old user presets, built-in presets, or GTK imports saved to disk that lack the `template_pack` key will seamlessly deserialize and fall back to the terminal layout, rather than crashing the app.

### `src/models/config.rs`

Or wherever your config models currently live:

```rust
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

// These defaults preserve the current layout structure.
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
    // ... your existing fields like colors, typography, etc. ...

    #[serde(default)]
    pub template_pack: TemplatePackConfig,
}
```

### Checklist action

Run a quick search for this pattern in your codebase:

```rust
ThemeConfig {
```

Check whether any handmade constructors need either:

```rust
template_pack: TemplatePackConfig::default(),
```

or:

```rust
..Default::default()
```

added to make `cargo check` happy.

---

## Phase 2: Add the Resolver Without Moving Files

Create a new file for the resolver. In this phase, the resolver takes the config but deliberately ignores the variants. It just returns your existing files.

This isolates the refactor and proves that passing file contents as `&'static str` fields works exactly like invoking `include_str!` locally inside the generator.

### `src/render/template_pack.rs`

Or a similar module name:

```rust
use crate::models::ThemeConfig; // Adjust import based on your structure.

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
    // PHASE 2:
    // Ignore _config for now to do a safe, behavior-preserving pass.
    // We return the exact same files to verify wiring without breaking Blogger XML.
    //
    // Note:
    // Adjust the relative paths below depending on where this module lives.

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
```

---

## Phase 2 Continued: Wire `xml_generator.rs`

Now go into your XML generation file, strip out the hardcoded `include_str!` macros at the top, and invoke the resolver instead.

### `src/render/xml_generator.rs`

```rust
use crate::render::template_pack::resolve_template_parts;

pub fn generate_blogger_xml(config: &ThemeConfig) -> String {
    // 1. Resolve template parts based on config.
    let parts = resolve_template_parts(config);

    // 2. Consume the resolved parts instead of calling include_str!() directly.
    let meta_str = parts.meta;
    let css_str = parts.css;
    let header_str = parts.header;
    let main_str = parts.main;
    let sidebar_left_str = parts.sidebar_left;
    let sidebar_right_str = parts.sidebar_right;
    let javascript_str = parts.javascript;

    // ... keep all of your existing replacement, string assembly,
    // and token interpolation logic perfectly intact ...
}
```

---

## Validation Step: Phases 1 and 2 Checklist

Once you have made these changes:

1. Run `cargo check`.
2. Open the Dioxus app and ensure old presets load without crashing.
3. Export a preset to an XML file.
4. Compare the newly exported XML against a backup of an older export. It should be byte-for-byte identical.
5. Upload it to a test Blogger blog to completely confirm no widgets or `b:includable` blocks were corrupted.

---

## Looking Ahead to Phase 3

Once this safe pass is verified, transitioning to Phase 3 is easy.

You will move the files into their new subdirectories, such as:

```text
template_parts/
├── headers/
├── mains/
├── sidebars/
└── scripts/
```

Then swap the dummy resolver logic out for clean `match` statements that fall back safely.

### Preview of Phase 3 Implementation

```rust
let header = match config.template_pack.header_variant.as_str() {
    // "gtk_headerbar" => include_str!("../../template_parts/headers/gtk_headerbar.xml"), // Phase 4
    "terminal" | _ => include_str!("../../template_parts/headers/terminal.xml"),
};
```

Using the wildcard `_` ensures that if a user deletes a layout, loads a corrupted preset, or inputs a missing template name, the compiler falls back gracefully to the safe terminal default rather than panicking the app.

---

## Phase 4: Static Pages Offline Engine

### Goal

Build native SPA routing inside the offline preview sandbox. Mock Blogger APIs so static page generators render perfectly without a live server connection.

### Blueprint

#### 1. Upgrade Trojan Horse: Data Mocking

Expand the JavaScript inside `inject_static_page`.

Intercept:

```text
/feeds/comments/summary
/feeds/posts/summary?alt=json
```

Return mock JSON for:

- Analytics
- Directory

#### 2. Wire Missing UI

Add an `Analytics` tab to `static_pages_panel.rs`.

Build text and number inputs for manual fallback metrics.

#### 3. Flesh Out Generators

Write raw HTML for these modules:

- `portfolio.rs`
- `about.rs`
- `categories.rs`
- `lms`

Hook them into the master theme CSS tokens, such as:

```css
var(--bg-panel)
```

#### 4. Live State Binding

Ensure all inputs trigger Dioxus `use_effect` to repaint the monitor instantly.
