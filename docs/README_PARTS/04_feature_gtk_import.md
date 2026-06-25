## 🎨 How to Import Native GTK4 Linux Themes

Moribund Architect can steal colors, borders, and UI icons directly from native Linux GTK Desktop themes and convert them into Blogger templates.

![GTK theme import flow](docs/diagrams/gtk_import_flow.drawio.png)

See also: [GTK Theme Parsing](docs/GTK_PARSER.md)

1. Go to [GNOME-Look.org](https://www.gnome-look.org/browse/).
2. Download any GTK3/GTK4 theme archive (e.g., `Mojave-Dark-alt.tar.xz`).
3. Extract the archive on your computer.
4. Open Moribund Architect and click **Import GTK4**.
5. Select the **top-level extracted folder** (It should be the folder that contains `gtk-4.0`, `gnome-shell`, etc. inside it). 
6. The engine will instantly absorb the CSS and SVG data URIs. Click **Save Imported Theme as Preset** to keep it!