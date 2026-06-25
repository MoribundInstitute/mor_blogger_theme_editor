## 📚 Documentation & Deep Dives
The Architect is designed to be extensible. Whether you want to understand the reactive state engine or submit your own preset to the Compendium, our documentation hub has you covered:

- [Architecture Overview](docs/ARCHITECTURE.md) — Rust rendering engine, Dioxus state, docks, and ThemeSignals (with diagrams).
- [The CSS Assembly Pipeline](docs/CSS_PIPELINE.md) — The `mor_` namespace and how modular CSS is stitched together.
- [Creating a Theme Preset](docs/THEME_CREATION.md) — Tokens, palettes, and compendium submission.
- [GTK Theme Parsing](docs/GTK_PARSER.md) — How Linux desktop themes become Blogger variables.

Editable diagram sources live in the atlas [`diagrams/shared/`](https://github.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/tree/main/diagrams/shared) folder (canonical copy).

### Codebase atlas (contributors)

For a guided tour of this repo — crate boundaries, Dioxus bootstrap, folder maps, and per-source file cards — see the companion [**mor_blogger_theme_editor_atlas**](https://github.com/MoribundMurdoch/mor_blogger_theme_editor_atlas):

- [Guides index](https://github.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/blob/main/atlas/guides/README.md) — recommended reading order
- [01 — Crate map (BYOF)](https://github.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/blob/main/atlas/guides/01-crate-map.md)
- [07 — Repo folder map](https://github.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/blob/main/atlas/guides/07-repo-folder-map.md)
- [Master index (305 file cards)](https://github.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/blob/main/atlas/INDEX.md)

## 🧰 Resources
Need assets or reference material for your theme? Use these external tools:

- **Icons:** [Google Material Symbols](https://fonts.google.com/icons) (Download as SVG, upload via MorBlogger UI)
- **Icons:** [Lucide](https://lucide.dev/) (Clean, neutral SVG icons)
- **Blogger API:** [Official XML Documentation](https://developers.google.com/blogger)
- **Asset Generation:** [halftone.tools](https://halftone.tools) (Free, browser-based print-effects workshop. Perfect for generating retro dithered background plates or custom vector SVGs for the theme dictionary).