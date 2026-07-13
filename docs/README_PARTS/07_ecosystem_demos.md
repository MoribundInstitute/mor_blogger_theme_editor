## 🌐 Ecosystem & Live Demos

MorBlogger sits in a larger toolchain of Rust UI kits, compendium repos, and optional MCP bridges:

![MorBlogger ecosystem map](docs/diagrams/ecosystem_map.drawio.png)

### Core Libraries
- [MOR UI Kit](https://github.com/MoribundInstitute/mor_rust_dioxus_ui_kit) — The standalone Dioxus UI toolkit powering the editor shell.
- [Theme Preset Compendium](https://github.com/MoribundInstitute/mor-blogger-theme-preset-compendium) — Community JSON color and typography presets.

![Compendium and plugin hub](docs/diagrams/compendium_plugin_hub.drawio.png)

Browseable galleries and matched GitHub sources are listed in **The Compendiums** (next section). Two additional live demos showcase features without their own compendium repo:

- [Custom Post Types](https://morcustomposttypes.blogspot.com/) — advanced Blogger data tags and post routing.
- [Static Pages](https://morpages.blogspot.com/) — standalone page layouts.

## ⚠️ Real-Time Diagnostics
- **Live Validation:** The engine actively detects structural inconsistencies, missing bindings, or broken toggles before you export.
- **Export Safety:** Prevents broken XML from ever reaching your clipboard.

![Real-time diagnostics and export safety](docs/diagrams/diagnostics_flow.drawio.png)