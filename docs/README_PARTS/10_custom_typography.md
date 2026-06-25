## Custom Fonts & Privacy CDNs

### The Blogger Limitation

Blogger's `<b:skin>` block is CSS-only. There is no asset host for binary font files. You cannot upload `.ttf`, `.woff`, or `.woff2` to Blogger and reference them with a local path. Any custom typeface must arrive via an external URL—either a CDN `@import` or a remote `@font-face` `src`.

![Blogger font constraints](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/blogger_font_constraints.drawio.png)

### The Privacy Alternative

[fonts.bunny.net](https://fonts.bunny.net) mirrors the Google Fonts catalog without tracking pixels, referrer logging, or IP retention. It is a drop-in, GDPR-compliant substitute for `fonts.googleapis.com`.

![Privacy-friendly font path via fonts.bunny.net](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/fonts_bunny_privacy_path.drawio.png)

1. Pick your family at [fonts.bunny.net](https://fonts.bunny.net).
2. Copy the generated `@import` rule.
3. Paste it into the **MorBlogger Custom CSS** panel.

```css
@import url('https://fonts.bunny.net/css?family=inter:400,700');

:root {
  --font-body: 'Inter', system-ui, sans-serif;
}
```

The MorBlogger export pipeline injects this block into the compiled `<b:skin>` stylesheet. No Google Fonts request hits your readers' browsers.

![Font normalization funnel](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/font_normalization_funnel.drawio.png)

All UI font inputs (typed names, drag-drop files, CDN rules) coerce to a font name string and pass through `resolve_font_stack()` in `mor_blogger_core` — see [DECISIONS.md](../DECISIONS.md).

### Self-Hosting

For fonts not on Bunny or Google—proprietary brand typefaces, niche libre fonts, or files you have licensed—you must host the `.ttf` (or `.woff2`) on a **CORS-enabled** static server. [GitHub Pages](https://pages.github.com/), Cloudflare R2, or any bucket with `Access-Control-Allow-Origin: *` works.

```css
@font-face {
  font-family: 'Brand Serif';
  src: url('https://youruser.github.io/fonts/BrandSerif-Regular.ttf') format('truetype');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}

:root {
  --font-heading: 'Brand Serif', Georgia, serif;
}
```

Paste the `@font-face` block and your `--font-*` overrides into the **MorBlogger Custom CSS** panel. Verify the URL returns `200` and the correct `Content-Type` (`font/ttf` or `font/woff2`) before exporting. A missing CORS header or wrong MIME type silently falls back to the system font stack in most browsers.