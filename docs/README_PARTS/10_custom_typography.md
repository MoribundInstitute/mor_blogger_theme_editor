## Custom Fonts & Privacy CDNs

### The Blogger Limitation

Google Blogger only allows you to upload CSS code to your theme file. It does not allow you to upload raw font files (like `.ttf` or `.woff2`) directly to their servers. 

Because of this, if you want to use a custom font, your theme has to load it from an external website link.

![Blogger font constraints](docs/diagrams/blogger_font_constraints.drawio.png)

### The Privacy Alternative

Many people use Google Fonts, but those can track your readers. Instead, we recommend using [fonts.bunny.net](https://fonts.bunny.net). It has the exact same font catalog as Google, but it strips out the tracking pixels and IP logging, keeping your readers' privacy safe.

![Privacy-friendly font path via fonts.bunny.net](docs/diagrams/fonts_bunny_privacy_path.drawio.png)

1. Pick your font family at [fonts.bunny.net](https://fonts.bunny.net).
2. Copy the generated `@import` rule.
3. Paste it into the **MorBlogger Custom CSS** panel.

```css
@import url('https://fonts.bunny.net/css?family=inter:400,700');

:root {
  --font-body: 'Inter', system-ui, sans-serif;
}
```

When you export your theme, the app automatically drops this code into the final file. No Google tracking requests will ever hit your readers' browsers.

Custom font rules pass through the internal normalization pipeline before export — see [DECISIONS.md](DECISIONS.md).

![Font normalization funnel](docs/diagrams/font_normalization_funnel.drawio.png)

### Hosting Your Own Fonts

If you bought a specific font or are using a rare one not found on Bunny Fonts, you have to upload the actual font file (like `.ttf`) to a web host yourself.

**The Security Padlock Problem:** Some web hosts put a "security padlock" (called CORS) on their files to stop other websites from stealing their bandwidth. If you upload your font file to a strict host, it will silently fail to load on your blog, and your browser will just show a boring default font instead.

To avoid this, we recommend uploading your font files to a free GitHub Pages account, as they leave the padlock open by default.

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

Just paste the `@font-face` block and your typography overrides into the MorBlogger Custom CSS panel, and the app will handle the rest.