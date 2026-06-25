## Architecture: Bring Your Own Frontend (BYOF)

This project adheres to a strict "Bring Your Own Frontend" (BYOF) modular architecture. We believe the logic that generates a theme should not be permanently bolted to the interface used to design it.

![BYOF crate boundaries](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/byof_crates.drawio.png)

The workspace is divided into distinct crates with hard compile-time boundaries:

* **`mor_blogger_core` (The Headless Engine):** The pure logic heart of the compiler. It handles TOML parsing, XML template resolution, CSS generation, and strict structural validation. It has zero GUI, OS, or filesystem-dialog dependencies.
* **`mor_blogger_dioxus_ui` (The Visual Workspace):** A Dioxus-powered graphical interface. It features a true "Socket and Plug" design:
  * **The Socket (`MorLayoutChrome`):** A generic window shell that draws borders, dock zones, tabs, and layout boundaries.
  * **The Plug (`BloggerWorkspace`):** The Blogger-specific logic module that slots into the center workspace. If a developer forks this project for Neocities, they simply rip out the Blogger plug and insert their own. Zero structural friction.
* **`mor_blogger_cli` (`mbt`):** A lightweight, native terminal interface. Built for power users, it wraps the core engine in a fast, standard Unix command surface. Use it to scaffold projects, validate XML syntax, or integrate theme builds into automated CI/CD pipelines without ever opening a window.

![Socket and plug UI pattern](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/socket_plug_ui.drawio.png)

Deep dive: [Architecture Overview](docs/ARCHITECTURE.md)