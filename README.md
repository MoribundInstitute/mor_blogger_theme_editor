<div align="center">

# 🏛️ MorBlogger GUI Theme Builder

A visual Blogger theme builder that handles the ugly XML machinery for you.<br>
Design your theme in a modern Rust-powered interface, then export clean Blogger XML, matching HTML pages, or a ready-to-upload ZIP without manually touching raw template code.

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-dca282.svg?logo=rust)](https://www.rust-lang.org/)
[![UI](https://img.shields.io/badge/UI-Dioxus_0.7-black.svg)](https://dioxuslabs.com/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

<img src="docs/screenshots/editor_preview.png" alt="Editor Preview" width="100%" style="border-radius: 8px; box-shadow: 0 4px 20px rgba(0,0,0,0.5);">

</div>


---


## 🌍 Why Do We Exist?
Blogger could literally become a free, highly customizable LMS for teachers worldwide. Imagine if schools, colleges, and educators could build their own free Khan Academies with built-in spaced repetition software (that is the ultimate end goal). 

Google could also foster a symbiotic relationship with Blogger by generating significant revenue through integrated ads. While the GUI editor includes several advertising options, they are left off by default, as traditional banner ads often degrade the user experience. Ideally, Google would introduce a Patreon-style monetization platform for Blogger, or perhaps an opt-in system for LLM training to support their platform.

##  The Problem
Editing a custom Blogger theme traditionally means wrestling with a monolithic, 3,000-line `template.xml` file. One missing CDATA tag or nested skin wrapper crashes the entire site. Iteration is slow, styling is dangerous, and modularity is non-existent.

## ✨ The Solution
The MorBlogger GUI Theme Builder replaces the monolith with a strict, component-driven pipeline. You work visually with structured modules in a highly responsive, desktop-class UI. When you are ready, the Rust engine safely compiles your palettes, typography, and modular CSS into a single, bulletproof XML file, matching HTML pages, or a ZIP archive containing the whole lot, ready for upload.


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
- **Modular UI Kit:** Powered by our custom `mor_rust_dioxus_ui_kit` running on Dioxus 0.7.
- **Adaptive Window Shell:** Features a custom Adwaita-inspired Client-Side Decoration (CSD) header bar. Seamlessly toggle between `frameless` with custom drag-and-drop borders, `native` with OS-drawn window chrome, or `tiling` with no custom window buttons for i3/Sway users.
- **Glassmorphic Environment:** Deep-space grid preview canvas with collapsible, frosted-glass control panels.
- **Hardware Accelerated:** Powered entirely by Rust and Dioxus for instantaneous hot-swapping and rendering.

---


## 🌐 Ecosystem & Live Demos
The Architect is part of a larger ecosystem of tools and live examples:

### Core Libraries
- [MOR UI Kit](https://github.com/MoribundInstitute/mor_rust_dioxus_ui_kit) — The standalone, zero-bloat Dioxus UI toolkit powering this editor.
- [Theme Compendium](https://github.com/MoribundInstitute/mor-blogger-theme-preset-compendium) — The open-source collection of community-driven Blogger templates.

### Live Production Sites
See the exported themes running live on Blogger's infrastructure:
- [Theme Gallery](https://mor-theme-compendium.blogspot.com/) — To keep the app lean, theme presets that didn't meet my arbitary cut to be hosted locally within the app are hosted externally on a companion Blogger site. Users can browse the collection there, copy a preset, and import it directly into the GUI Blogger theme maker.
- [XML Architecture](https://morxml.blogspot.com/) — It's a place to share XML bits like Blog post sections, custom sidebars, custom footers, etc.
- [Custom Post Types](https://morcustomposttypes.blogspot.com/) — Demonstrating advanced Blogger data tags and post routing.
- [Static Pages](https://morpages.blogspot.com/) — Demonstrating standalone page layouts.

## ⚠️ Real-Time Diagnostics
- **Live Validation:** The engine actively detects structural inconsistencies, missing bindings, or broken toggles before you export.
- **Export Safety:** Prevents broken XML from ever reaching your clipboard.


---


## 📚 Documentation & Deep Dives
The Architect is designed to be extensible. Whether you want to understand the reactive state engine or submit your own preset to the Compendium, our documentation hub has you covered:

- [Architecture Overview](docs/ARCHITECTURE.md) — How the Rust rendering engine and Dioxus state management interact.
- [The CSS Assembly Pipeline](docs/CSS_PIPELINE.md) — Understanding the `mor_` namespace and how modular CSS is stitched together.
- [Creating a Theme Preset](docs/THEME_CREATION.md) — A guide to defining tokens, palettes, and custom layouts for the Compendium.
- [GTK Theme Parsing](docs/GTK_PARSER.md) — How the engine translates Linux desktop themes into Blogger variables.


---


## 🛠️ Getting Started

### Prerequisites
- Rust toolchain → [rustup.rs](https://rustup.rs/)
- Dioxus CLI
  ```bash
  # Install the Dioxus CLI
  cargo install dioxus-cli


---


## 🤝 Contributing
The Moribund Institute welcomes contributions! If you have built a beautiful, robust theme preset using the Architect, we would love to add it to the default Compendium or feature it on the [Theme Gallery](https://mor-theme-compendium.blogspot.com/).

To leave naming space open for the community (so we don't hog generic names like "Modern Editorial" or "Web 2.0"), the Moribund Institute reserves the `mor-` prefix for our official theme releases. 

Whether you are submitting a PR to share your preset publicly or just building for yourself, please ensure your internal CSS and variables follow the `mor_` namespacing guidelines outlined in the [Theme Creation Guide](docs/THEME_CREATION.md).

## License
Published under the MIT License. 

The Moribund Institute doesn't strictly care about copyright (it's often an arbitrary barrier to the acceleration of ideas), but we do have egos, so attribution is always appreciated!

<div align="center">
  <br>
  <b>Developed by Murdoch</b><br>
  <i>The Moribund Institute</i>
</div>

