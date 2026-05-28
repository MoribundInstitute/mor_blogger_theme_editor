<div align="center">

# 🏛️ Moribund Blogger Theme Architect

**A low-level XML generator with a modern, reactive GUI built in Rust.**<br>
Design, customize, and export complete Blogger themes without ever touching raw XML.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-dca282.svg?logo=rust)](https://www.rust-lang.org/)
[![UI by Dioxus](https://img.shields.io/badge/UI-Dioxus-black.svg)](https://dioxuslabs.com/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#-contributing)

<img src="docs/screenshots/editor_preview.png" alt="Editor Preview" width="100%" style="border-radius: 8px; box-shadow: 0 4px 20px rgba(0,0,0,0.5);">

</div>

---

## ⚡ The Problem

Editing a custom Blogger theme traditionally means wrestling with a monolithic, 3,000-line `template.xml` file. One missing CDATA tag or nested skin wrapper crashes the entire site. Iteration is slow, styling is dangerous, and modularity is non-existent.

## ✨ The Solution

The **Moribund Theme Architect** replaces the monolith with a strict, component-driven pipeline. You work visually with structured modules in a highly responsive desktop-class UI. When you are ready, the Rust engine safely compiles your palettes, typography, and modular CSS into a single, bulletproof XML file ready for upload.

---

## 🚀 Core Capabilities

### 🧩 Modular XML Assembly

- **No More Monoliths:** Build themes from discrete, manageable template parts (`meta.xml`, `header.xml`, `sidebars.xml`).
- **Suckless CSS Pipeline:** The engine safely slices, sanitizes, and stitches dozens of individual CSS modules into a final layout without nesting errors.
- **Intelligent Injection:** Automatically wires up SEO metadata, typography scaling systems, and dynamic widget sockets.

### 🎨 GTK Desktop Integration

- **Native-Feeling Themes:** Import visual variables directly from legendary Linux themes like Adwaita, Nord, and WhiteSur.
- **Asset Compilation:** Automatically converts external SVG assets into lightweight, embedded CSS data URIs to eliminate external HTTP requests.

### 🖥️ Fluid Workspace UI

- **Custom Window Manager:** Features a custom-built, floating, drag-and-drop window system inside the app.
- **Glassmorphic Environment:** Deep-space grid preview canvas with collapsible, frosted-glass control panels.
- **Hardware Accelerated:** Powered entirely by Rust and Dioxus for instantaneous hot-swapping and rendering.

### ⚠️ Real-Time Diagnostics

- **Live Validation:** The engine actively detects structural inconsistencies, missing bindings, or broken toggles before you export.
- **Export Safety:** Prevents broken XML from ever reaching your clipboard.

---

## 📚 Documentation & Deep Dives

The Architect is designed to be extensible. Whether you want to understand the reactive state engine or submit your own preset to the Compendium, our documentation hub has you covered:

- [**Architecture Overview**](docs/ARCHITECTURE.md) — How the Rust rendering engine and Dioxus state management interact.
- [**The CSS Assembly Pipeline**](docs/CSS_PIPELINE.md) — Understanding the `mor_` namespace and how modular CSS is stitched together.
- [**Creating a Theme Preset**](docs/THEME_CREATION.md) — A guide to defining tokens, palettes, and custom layouts for the Compendium.
- [**GTK Theme Parsing**](docs/GTK_PARSER.md) — How the engine translates Linux desktop themes into Blogger variables.

---

## 🛠️ Getting Started

### Prerequisites

- Rust toolchain → [rustup.rs](https://rustup.rs/)
- Dioxus CLI

```bash
# Install the Dioxus CLI
cargo install dioxus-cli
```

### Installation & Launch

```bash
# Clone the repository
git clone https://github.com/MoribundInstitute/mor_blogger_theme_editor.git

# Navigate to the project directory
cd mor_blogger_theme_editor

# Launch the Architect with hot-reloading
dx serve --hot-reload
```

---

## 🔄 The Editorial Workflow

1. **Design:** Select a baseline preset from the Compendium, such as Mor Modern Editorial or Mor Retro MMORPG.
2. **Customize:** Tweak colors, typography, and widget layouts using the floating UI panels.
3. **Preview:** Test your structural layout instantly across simulated desktop, tablet, and mobile viewports.
4. **Export:** Click **Export Theme**. The Rust engine compiles your perfect `template.xml` directly to your clipboard.
5. **Restore:** Paste your exported XML back into the editor at any time to instantly rehydrate your complete workspace state.

---

## 🤝 Contributing

The Moribund Institute welcomes contributions! If you have built a beautiful, robust theme preset using the Architect, we would love to add it to the default Compendium.

Please ensure your preset follows the `mor_` namespacing guidelines outlined in the Theme Creation Guide before submitting a Pull Request.

---

## License

Published under the MIT License.

---

<div align="center">

Developed by **Murdoch**  
The Moribund Institute

</div>
