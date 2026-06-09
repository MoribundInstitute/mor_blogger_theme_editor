## Architecture: Bring Your Own Frontend (BYOF)

This project adheres to a strict "Bring Your Own Frontend" (BYOF) modular architecture. We believe the logic that generates a theme should not be permanently bolted to the interface used to design it.

The workspace is divided into distinct crates with hard compile-time boundaries:

* **`mor_blogger_core` (The Headless Engine):** The pure logic heart of the compiler. It handles TOML parsing, XML template resolution, CSS generation, and strict structural validation. It has zero GUI, OS, or filesystem-dialog dependencies.
* **`mor_blogger_dioxus_ui` (The Visual Workspace):** A Dioxus-powered graphical interface. It provides a rich, hot-reloading layout environment and connects the core engine to native OS file dialogs and clipboard APIs.
* **`mor_blogger_cli` (`mbt`):** A lightweight, native terminal interface. Built for power users, it wraps the core engine in a fast, standard Unix command surface. Use it to scaffold projects, validate XML syntax, or integrate theme builds into automated CI/CD pipelines without ever opening a window.

Because the core engine is completely agnostic, you can easily wrap it in a new frontend—whether that is a custom web service, a different desktop framework, or a specialized automation script.
