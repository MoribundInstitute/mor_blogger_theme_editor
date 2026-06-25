## Architecture: Bring Your Own Frontend (BYOF)

This project uses a "Bring Your Own Frontend" (BYOF) design. This simply means we split the software into two main parts: a **Socket** (the skeleton) and a **Plug** (the brains). 

We do this so the logic that builds a blog theme isn't permanently glued to the interface you use to design it.

![BYOF crate boundaries](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/byof_crates.drawio.png)

### The Three Main Pieces

* **1. The Engine (`mor_blogger_core`):** This is the pure logic heart of the software. It reads your settings, writes the CSS, and checks for errors. It has no buttons, no windows, and no interface at all.
* **2. The Visual Workspace (`mor_blogger_dioxus_ui`):** This is the graphical interface you actually click on and use. It uses a "Socket and Plug" design:
  * **The Socket (`MorLayoutChrome`):** The generic skeleton. It manages the main window, organizes your tabs, and draws the sidebars.
  * **The Plug (`BloggerWorkspace`):** The specific brains for Blogger. It contains the tools to preview and export Blogger themes. Because of this design, someone could easily copy this project, remove the Blogger plug, and insert a new plug to make a theme editor for a different website like Neocities—without having to rebuild the whole app from scratch.
* **3. The Command Line (`mor_blogger_cli` / `mbt`):** A lightweight terminal tool. For power users who prefer typing commands over clicking buttons, this lets you validate code or build themes directly from your computer's terminal.

![Socket and plug UI pattern](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/socket_plug_ui.drawio.png)

Deep dive: [Architecture Overview](docs/ARCHITECTURE.md)