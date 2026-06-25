# Creating a Theme Preset

This guide covers authoring presets for the MorBlogger compendium and local use.

## Naming

- **`mor-` prefix** — Reserved for official Moribund Institute releases.
- **Community presets** — Use distinctive names; follow `mor_` CSS variable namespacing (see [CSS Pipeline](CSS_PIPELINE.md)).

## Authoring Flow

1. Design tokens (colors, typography, effects) in the **Theme Palette** dock.
2. Apply and refine via `ThemeSignals` — changes flow through `apply_preset()` / `apply_config()`.
3. Validate with live diagnostics before export.
4. Export XML or submit to the [Theme Compendium](https://github.com/MoribundInstitute/mor-blogger-theme-preset-compendium).

## Compile Pipeline

Presets feed into the same pipeline as hand-authored themes:

![Theme compile pipeline](diagrams/theme_compile_pipeline.drawio.png)

## Related

- [CSS Assembly Pipeline](CSS_PIPELINE.md)
- [Architecture Overview](ARCHITECTURE.md)