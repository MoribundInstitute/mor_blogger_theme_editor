# MOR_PLAN.md

## Goal

Refactor the Blogger theme editor so it exports a valid, customizable Blogger XML theme from modular template parts instead of a brittle monolithic `src/template.xml`.

The app should remain a GUI editor. Users should be able to customize the visible theme shell, CSS, navigation, static page links, external links, footer content, colors, fonts, and optional scripts without accidentally breaking Blogger's widget engine.

## Core Principle

Separate the project into two layers:

```text
Customizable GUI layer:
  CSS variables
  theme colors
  fonts
  header branding
  menu links
  catalog links
  static page links
  external links
  footer text
  optional custom HTML/JS
  preview behavior

Frozen Blogger engine layer:
  Blog1 widget ID/type
  Label1 widget ID/type
  BlogArchive1 widget ID/type
  HTML1/widget IDs
  b:widget-settings
  b:includable blocks
  b:section IDs
  Blogger data bindings
```

Do not tokenize Blogger structural identifiers such as:

```text
Blog1
BlogArchive1
BlogArchive
Label1
Label
HTML1
BlogSearch1
BlogSearch
```

Only tokenize safe text, URLs, CSS values, labels, and configurable HTML/CSS/JS zones.

## Current Source Layout

Use the modular XML parts under:

```text
src/template_parts/
  meta.xml
  css.xml
  header.xml
  sidebar_left.xml
  main.xml
  sidebar_right.xml
  javascript_before_body_tag.xml
```

The old monolithic template should be archived only:

```text
docs/archive/template-old-monolith.xml
```

## File Responsibilities

### `src/template_parts/meta.xml`

Owns the XML declaration, doctype, Blogger namespaces, opening `<head>`, meta tags, Blogger `all-head-content`, favicons, social metadata, Google font link, and ad head script token.

Safe tokens include:

```text
{{SITE_TITLE}}
{{META_DESCRIPTION}}
{{META_KEYWORDS}}
{{AUTHOR_NAME}}
{{THEME_COLOR}}
{{FAVICON_URL}}
{{SOCIAL_CARD_IMAGE_URL}}
{{GOOGLE_FONTS_LINK}}
{{ADS_HEAD_SCRIPT}}
```

### `src/template_parts/css.xml`

Owns:

```text
<b:skin><![CDATA[
  theme CSS
]]></b:skin>
```

Safe tokens include:

```text
{{COLOR_BG_BASE}}
{{COLOR_BG_PANEL}}
{{COLOR_BG_ELEVATED}}
{{COLOR_FG_BASE}}
{{COLOR_FG_MUTED}}
{{COLOR_ACCENT}}
{{COLOR_BORDER}}
{{FONT_BODY}}
{{FONT_HEADING}}
{{FONT_MONO}}
{{BTN_RADIUS}}
{{BTN_BORDER_WIDTH}}
{{BACKGROUND_TILE_CSS}}
{{PRESET_CSS}}
```

Keep raw CSS inside CDATA.

### `src/template_parts/header.xml`

Owns the terminal header, panel toggle buttons, branding/logo, main nav, catalog dropdown, and search form.

Safe tokens include:

```text
{{SITE_TITLE}}
{{SITE_SUBTITLE}}
{{HEADER_LOGO_URL}}
{{SITE_HOME_URL}}
{{LEFT_PANEL_OPEN_LABEL}}
{{RIGHT_PANEL_OPEN_LABEL}}
{{NAV_HOME_LABEL}}
{{NAV_HOME_URL}}
{{NAV_ABOUT_LABEL}}
{{NAV_ABOUT_URL}}
{{NAV_PROJECTS_LABEL}}
{{NAV_PROJECTS_URL}}
{{NAV_CONTACT_LABEL}}
{{NAV_CONTACT_URL}}
{{SEARCH_ACTION_URL}}
{{SEARCH_PLACEHOLDER}}
{{SEARCH_BUTTON_LABEL}}
```

Future GUI feature: generate menu/catalog entries from config rather than hardcoding every slot.

### `src/template_parts/sidebar_left.xml`

Owns the left panel shell, `sidebar-left` section, `Label1`, and `BlogArchive1`.

Keep `Label1` and `BlogArchive1` full and Blogger-native.

Safe tokens include:

```text
{{LEFT_PANEL_TITLE}}
{{LEFT_PANEL_CLOSE_LABEL}}
{{LABEL_WIDGET_TITLE}}
{{ARCHIVE_WIDGET_TITLE}}
{{LABEL_SORTING}}
{{LABEL_DISPLAY}}
{{ARCHIVE_SHOW_STYLE}}
{{ARCHIVE_FREQUENCY}}
```

Never tokenize widget IDs or widget types.

### `src/template_parts/main.xml`

Owns the main canvas shell, main/content section, `Blog1`, post rendering includables, pager, comments, and sharing includables.

Keep `Blog1` full and Blogger-native.

Safe tokens include:

```text
{{BLOG_WIDGET_TITLE}}
{{BLOG_COMMENT_LABEL}}
{{BLOG_AUTHOR_LABEL}}
{{BLOG_TIMESTAMP_FORMAT}}
{{POST_TAGS_PREFIX}}
{{PAGER_NEWER_LABEL}}
{{PAGER_HOME_LABEL}}
{{PAGER_OLDER_LABEL}}
{{POST_METADATA_FALLBACK_IMAGE_URL}}
{{PUBLISHER_NAME}}
{{PUBLISHER_LOGO_URL}}
```

Do not replace the full Blog widget with a tiny generated fragment unless it has been proven in Blogger.

### `src/template_parts/sidebar_right.xml`

Owns the right panel shell, `sidebar-right` section, and `HTML1` table-of-contents widget.

Safe tokens include:

```text
{{RIGHT_PANEL_TITLE}}
{{RIGHT_PANEL_CLOSE_LABEL}}
{{RIGHT_WIDGET_TITLE}}
{{TOC_LOADING_MESSAGE}}
{{TOC_WAITING_MESSAGE}}
{{TOC_EMPTY_MESSAGE}}
{{TOC_HEADING_SELECTOR}}
```

### `src/template_parts/javascript_before_body_tag.xml`

Owns panel toggles, catalog behavior, keyboard shortcuts, back-to-top behavior, TOC helpers, and optional user JS insertion.

Safe token:

```text
{{CUSTOM_BEFORE_BODY_JS}}
```

Do not wrap `{{CUSTOM_BEFORE_BODY_JS}}` in another `<script>` if this file already lives inside a script block.

## Renderer Refactor

Edit:

```text
src/render/theme.rs
src/render/xml_generator.rs
src/render/util.rs
src/render/ads.rs
```

### `src/render/theme.rs`

Make it the stable public facade:

```rust
pub fn render_theme(config: &ThemeConfig) -> String {
    xml_generator::render_template(config)
}
```

It should no longer pass `BASE_TEMPLATE` from `src/template.xml`.

### `src/render/xml_generator.rs`

Responsibilities:

```text
include_str! each template part
assemble final XML
build token map
replace safe GUI tokens
run header socket generation if needed
return final XML
```

Desired constants:

```rust
const META: &str = include_str!("../template_parts/meta.xml");
const CSS: &str = include_str!("../template_parts/css.xml");
const HEADER: &str = include_str!("../template_parts/header.xml");
const SIDEBAR_LEFT: &str = include_str!("../template_parts/sidebar_left.xml");
const MAIN: &str = include_str!("../template_parts/main.xml");
const SIDEBAR_RIGHT: &str = include_str!("../template_parts/sidebar_right.xml");
const JS_BEFORE_BODY: &str = include_str!("../template_parts/javascript_before_body_tag.xml");
```

Assembly shape:

```rust
format!(
    "{meta}
{css}
<b:template-skin><![CDATA[]]></b:template-skin>
</head>
<body>
{header}
<div class='terminal-workspace'>
{left}
{main}
{right}
</div>
{ads_consent_banner}
{ads_runtime_script}
{js}
</body>
</html>"
)
```

Split the giant `.replace(...)` chain into grouped token builders:

```text
push_head_tokens
push_branding_tokens
push_nav_tokens
push_catalog_tokens
push_css_tokens
push_sidebar_tokens
push_blog_footer_tokens
push_plugin_tokens
```

Then apply:

```rust
fn apply_tokens(mut xml: String, tokens: TokenList) -> String {
    for (token, value) in tokens {
        xml = xml.replace(token, &value);
    }
    xml
}
```

## Diagnostics Refactor

Edit:

```text
src/diagnostics.rs
```

Diagnostics should validate the final rendered XML, not require template-source tokens to exist.

Correct final XML checks:

```text
XML parses
no unresolved {{TOKEN}} remains
terminal-workspace exists
panel-left exists
panel-right exists
canvas-core exists
b:section sidebar-left exists
b:section main or main-content exists
b:section sidebar-right exists
Blog1 exists
Label1 exists
BlogArchive1 exists
Blog1 has key includables
wrong compendium/gallery template markers are absent
```

Wrong-template markers to reject:

```text
MorThemeGallery
mor-theme-gallery
The MorBlogger Theme Compendium
compendium-header
mor-gallery-grid
```

## GUI Static Pages / External Links

Users should be able to add Blogger pages and external URLs through the GUI.

Edit:

```text
src/config/pages.rs
src/config/mod.rs
src/ui/static_pages_panel.rs
src/ui/menu_panel.rs
src/render/xml_parts/header_generator.rs
```

Suggested model:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticLink {
    pub label: String,
    pub url: String,
    pub kind: LinkKind,
    pub open_in_new_tab: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LinkKind {
    BloggerPage,
    BloggerLabel,
    BloggerSearch,
    ExternalSite,
    Custom,
}
```

Preview behavior:

```text
Blogger/internal links:
  render in preview or normal anchor behavior

External links:
  show Open in Browser
  open using the system default browser from the desktop app
```

Export behavior:

```xml
<a href='https://example.com' rel='noopener' target='_blank'>External Site</a>
```

## Safety Rules

Use:

```rust
escape_html()
escape_attr()
```

Rules:

```text
Element text -> escape_html
Attribute values -> escape_attr
Raw CSS in CDATA -> do not HTML-escape, but avoid breaking CDATA
Raw JS in CDATA -> do not double-wrap in script tags
```

Never tokenize structural Blogger names.

Bad:

```xml
<b:widget id='Blog{{ARCHIVE_WIDGET_TITLE}}1' type='Blog{{ARCHIVE_WIDGET_TITLE}}'>
```

Good:

```xml
<b:widget id='BlogArchive1' title='{{ARCHIVE_WIDGET_TITLE}}' type='BlogArchive'>
```

Keep the working `Blog1`, `Label1`, and `BlogArchive1` bodies intact unless testing proves a smaller body works inside Blogger.

## Migration Steps

1. Archive the monolith at `docs/archive/template-old-monolith.xml`.
2. Make `src/render/theme.rs` call `xml_generator::render_template(config)`.
3. Refactor `src/render/xml_generator.rs` to assemble the seven template parts.
4. Ensure every `{{TOKEN}}` in template parts has exactly one replacement.
5. Update diagnostics to check final rendered XML.
6. Smoke-test export and search for `terminal-workspace`, `canvas-core`, `panel-left`, `panel-right`, `Blog1`, `Label1`, `BlogArchive1`.
7. Search for wrong-template markers: `MorThemeGallery`, `mor-theme-gallery`, `compendium-header`, `mor-gallery-grid`. They must be absent.
8. Upload the exported XML to Blogger.
9. If Blogger accepts it but widgets render empty, inspect live source for `Blog1`, `blog-posts`, `post-title`, `Label1`, `BlogArchive1`, and `_WidgetManager`.

## Suggested Tests

Unit tests:

```text
hex_to_rgba handles valid hex
first_non_empty falls back on blank strings
rendered XML contains no {{TOKEN}}
rendered XML contains terminal-workspace
rendered XML contains Blog1, Label1, BlogArchive1
rendered XML does not contain MorThemeGallery
```

Manual tests:

```text
cargo fmt
cargo check
cargo run
export XML
search exported XML for unresolved tokens
upload to Blogger
view live source
confirm widgets render
```

## Anti-Loop Rule

Do not keep trying random widget fragments.

If widgets break, compare against a known-good Blogger export and copy the full widget body back into the matching template part.

The GUI editor customizes the shell and safe values. Blogger widgets are the haunted engine layer.

