# GTK Theme Parsing

MorBlogger can import native Linux GTK desktop themes and convert them into Blogger-compatible presets.

## Import Flow

![GTK import flow](diagrams/gtk_import_flow.drawio.png)

## Parser Location

Implementation lives in `mor_blogger_core/src/config/gtk_theme/`:

- **`parser.rs`** — Reads `gtk-4.0` theme folders; extracts colors, borders, and variables.
- **`assets.rs`** — Converts SVG assets into embedded CSS data URIs.
- **`generator.rs`** — Maps GTK variables into `MorTheme` / `ThemeConfig` fields.

## Usage

1. Download a theme from [GNOME-Look.org](https://www.gnome-look.org/browse/).
2. Extract the archive.
3. In the editor, click **Import GTK4** and select the top-level folder containing `gtk-4.0`.
4. Click **Save Imported Theme as Preset** to persist the result.

## Related

- [Architecture Overview](ARCHITECTURE.md)
- README section: [How to Import Native GTK4 Linux Themes](../README.md)