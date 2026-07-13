## 📦 The Compendiums

MorBlogger doesn't bake every preset, widget, and layout into the binary — that would bloat the app and bottleneck the whole ecosystem on one person's taste. Instead, the heavy stuff lives in community **compendiums**: open repositories that build themselves into browsable Blogger galleries ([RuneLite plugin-hub](https://github.com/runelite/plugin-hub) style). Browse the live gallery, copy a link or snippet, and paste it straight into the GUI.

Each compendium is a matched pair: a **GitHub repo** (the source of truth) and a **Blogger frontend** (the gallery you browse).

| Compendium | What it holds | Live gallery | Source |
| --- | --- | --- | --- |
| **Full Themes** | Complete, ready-to-upload Blogger XML themes | [mor-theme-compendium.blogspot.com](https://mor-theme-compendium.blogspot.com/) | [mor-blogger-full-theme-compendium](https://github.com/MoribundInstitute/mor-blogger-full-theme-compendium) |
| **Theme Presets** | JSON color &amp; typography presets — the *skin* you hot-swap | [morbloggerpresetcompendium.blogspot.com](https://morbloggerpresetcompendium.blogspot.com/) | [mor-blogger-theme-preset-compendium](https://github.com/MoribundInstitute/mor-blogger-theme-preset-compendium) |
| **Widgets** | Installable Blogger gadgets and their install-XML blueprints | [mor-widgets-compendium.blogspot.com](https://mor-widgets-compendium.blogspot.com/) | [mor-blogger-widget-compendium](https://github.com/MoribundInstitute/mor-blogger-widget-compendium) |
| **XML Structures** | Raw layout *bones* — headers, footers, grids — that adapt to your active theme variables (the *skeleton*) | [morxml.blogspot.com](https://morxml.blogspot.com/) | [mor-xml-compendium](https://github.com/MoribundInstitute/mor-xml-compendium) |

Think **skin vs. skeleton**: Theme Presets restyle what you already have, XML Structures swap the underlying bones, Widgets drop in self-contained gadgets, and Full Themes hand you the whole package at once.

### 🧪 Live demos (no standalone compendium)

These Blogger sites showcase specific capabilities rather than hosting a browsable collection:

- [Custom Post Types](https://morcustomposttypes.blogspot.com/) — advanced Blogger data tags and post routing.
- [Static Pages](https://morpages.blogspot.com/) — standalone page layouts.

> **Naming convention:** the `mor-` prefix is reserved for official Moribund Institute releases so we don't hog generic community names like "Modern Editorial." Folder slugs are path identifiers — display names are where you get creative.