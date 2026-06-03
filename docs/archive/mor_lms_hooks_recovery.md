# Mor LMS Hook Recovery Note

Status: archived idea, not active in the default Blogger XML export.

## Why this was removed

The active `blog_standard.xml` template was restored to a pre-LMS shape. The restored pre-LMS post wrapper should stay simple:

```xml
<article class='mor-post'>
```

During the LMS experiment, the template used a placeholder like this:

```xml
<article class='mor-post' {{MOR_LMS_HOOKS}}>
```

Then `src/render/xml_generator.rs` replaced `{{MOR_LMS_HOOKS}}` with Blogger expression attributes at export time.

That made the feature easy to inject, but it also made the core Blog widget skeleton more fragile. If the placeholder exists in the wrong version of the template, or the replacement does not match Blogger's XML rules, export integrity can fail.

## Original idea

The idea was to attach machine-readable metadata to each rendered Blogger post so a local-first/offline LMS tracker, browser extension, or study pipeline could identify posts and labels without scraping the visible UI.

Original replacement block:

```rust
.replace(
    "{{MOR_LMS_HOOKS}}",
    "expr:data-mor-id='data:post.id' expr:data-mor-labels='data:post.labels ? (data:post.labels map (l =&gt; l.name) join &quot;,&quot;) : &quot;&quot;'"
)
```

The intended generated article looked roughly like this:

```xml
<article
  class='mor-post'
  expr:data-mor-id='data:post.id'
  expr:data-mor-labels='data:post.labels ? (data:post.labels map (l =&gt; l.name) join &quot;,&quot;) : &quot;&quot;'>
```

## Safer future reimplementation

If this idea comes back, do not hardwire it into the default `blog_standard.xml`.

Better options:

1. Add a config flag, for example:

```rust
pub enable_lms_hooks: bool
```

2. Keep the normal template clean:

```xml
<article class='mor-post'>
```

3. Generate a separate LMS-enabled content part, for example:

```text
src/template_parts/content/blog_lms_enabled.xml
```

4. Gate the replacement in Rust:

```rust
if config.static_pages.lms.enable_tracking_hooks {
    rendered = rendered.replace("{{MOR_LMS_HOOKS}}", "...safe attributes...");
}
```

5. Add an integrity check that fails loudly if `{{MOR_LMS_HOOKS}}` appears in the final export.

## Possible tracker fields

Useful future attributes:

```xml
expr:data-mor-post-id='data:post.id'
expr:data-mor-post-url='data:post.url'
expr:data-mor-post-title='data:post.title'
expr:data-mor-labels='data:post.labels ? (data:post.labels map (l =&gt; l.name) join &quot;,&quot;) : &quot;&quot;'
```

## Recovery rule

Default theme export should remain boring and stable.

Experimental learning/tracking hooks should live in either:

```text
docs/archive/
src/template_parts/content/experimental/
```

or behind an explicit opt-in config toggle.
