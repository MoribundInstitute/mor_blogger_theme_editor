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

# 🌍 Why Do We Exist?

We want to make Blogger GUI editing software so easy that even 95-year-old, technologically illiterate grandmothers could use it. I'm thinking we should lean into [skeuomorphism](https://en.wikipedia.org/wiki/Skeuomorph) to maximize the software's intuitiveness. Blogger could be an ideal platform for older people because the content management system is already fairly intuitive, while we could help them make their blogs look good.

Those of us who are more technically inclined should be willing to sacrifice some of our time creating presets, widgets, & other tools (and post them on the various compendiums this GUI software will manage in a similiar fashion to [RuneLite plugins](https://github.com/runelite/plugin-hub)) so that people who may be technologically illiterate, but have other interesting hobbies, such as maintaining an orchard of [heritage apples](https://en.wikipedia.org/wiki/Lost_Apple_Project) or translating [Beowulf](https://en.wikipedia.org/wiki/Beowulf) into [Old Scots](https://en.wikipedia.org/wiki/Early_Scots), can more easily manage a blog without worrying about subscription fees and similar obstacles.

Blogger could also become a free, highly customizable learning management system (LMS) for teachers worldwide. Imagine schools, colleges, and independent educators being able to build their own free versions of Khan Academy, complete with built-in spaced repetition software. That's the ultimate end goal.

We're experimenting with several LMS options and identity verification methods: Syncthing, Web3, whatnot, OAuth 2.0, Rauthy, you name it. We also take loose inspiration from [rebane2001/xikipedia](https://github.com/rebane2001/xikipedia) because we really want educational content to become decentralized skinner boxes.

🔗 **Experimental vault:** [MoribundInstitute/mor_lms_vault](https://github.com/MoribundInstitute/mor_lms_vault)

Google could also foster a symbiotic relationship with Blogger by generating significant revenue through integrated ads. While the GUI editor includes several advertising options, they are left off by default, as traditional banner ads often degrade the user experience. Ideally, Google would introduce a Patreon-style monetization platform for Blogger, or perhaps an opt-in system for LLM training to support their platform.

![I know there is good in you - Google meme](mor_blogger_dioxus_ui/assets/images/memes/I_Know_Theres_Good_In_You_Google.jpg)

*Star Wars: Return of the Jedi © Lucasfilm Ltd. Google logo ™ Google LLC. Used here as parody/meme. Repo code is MIT licensed third-party image, not covered by MIT.*

## The Problem

Editing a custom Blogger theme means wrestling with a monolithic, 3,000-line `template.xml` file. 

## ✨ The Solution

The MorBlogger GUI Theme Builder replaces the monolith with a component-driven pipeline. You work visually with structured modules in a desktop UI. When you are ready, the Rust engine safely compiles your palettes, typography, and modular CSS into a single bulletproof XML file, matching HTML pages, or a ZIP archive containing the whole lot, ready for upload.

The theming is also hype because we have Rust code that converts GTK4 SVGs into Data URIs. Then, with a bit of behind-the-scenes CSS magic, we can make all sorts of SVG whatnot appear on your blog. I think we could get it to work for these later too: [GNOME-Look SVG Themes](https://www.gnome-look.org/browse?cat=277&ord=rating).

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
- **Smart Code Dock:** An interactive configuration editor that maps visual template modules (Headers, Sidebars, Footers) directly to raw TOML byte-offsets, centering exactly what you need to edit.
- **Hardware Accelerated:** Powered entirely by Rust and Dioxus for instantaneous hot-swapping and rendering.

---

## 🎨 How to Import Native GTK4 Linux Themes

Moribund Architect can steal colors, borders, and UI icons directly from native Linux GTK Desktop themes and convert them into Blogger templates.

1. Go to [GNOME-Look.org](https://www.gnome-look.org/browse/).
2. Download any GTK3/GTK4 theme archive (e.g., `Mojave-Dark-alt.tar.xz`).
3. Extract the archive on your computer.
4. Open Moribund Architect and click **Import GTK4**.
5. Select the **top-level extracted folder** (It should be the folder that contains `gtk-4.0`, `gnome-shell`, etc. inside it). 
6. The engine will instantly absorb the CSS and SVG data URIs. Click **Save Imported Theme as Preset** to keep it!

---

# 🛠️ Getting Started with Mor Blogger Theme Editor

Welcome to **Mor Blogger Theme Editor**, a Rust-powered visual environment for building, editing, validating, and packaging Blogger themes.

## Prerequisites

### 1. Install Rust

Install the Rust toolchain from:

- https://rustup.rs/

Verify installation:

```bash
rustc --version
cargo --version
```

### 2. Install the Dioxus CLI

The graphical editor is built with Dioxus.

```bash
cargo install dioxus-cli
```

Verify installation:

```bash
dx --version
```

---

# Option A: Launch the Visual Editor

The visual workspace provides:

- Live preview workflows
- Hot reloading during development
- Theme configuration editing
- Visual design tooling
- Future plugin and workspace integration

From the project root:

```bash
cd mor_blogger_dioxus_ui
dx serve
```

Then open the local address displayed by Dioxus in your browser.

---

# Option B: Use the Command-Line Tool (mbt)

If you prefer a terminal workflow, use the `mbt` utility.

Build the release executable:

```bash
cargo build --release -p mor_blogger_cli
```

The binary will be located at:

```text
target/release/mbt
```

## Initialize a Project

Create a new theme workspace:

```bash
./target/release/mbt init
```

## Validate a Theme

Check XML syntax and catch Blogger import issues before deployment:

```bash
./target/release/mbt check
```

## Build a Blogger Theme

Generate the final Blogger-compatible XML output:

```bash
./target/release/mbt build
```

## Create a Distribution Bundle

Build the theme and package supporting assets:

```bash
./target/release/mbt bundle
```

---

# Typical Development Workflow

```bash
# 1. Create or open a workspace
mbt init

# 2. Edit visually
dx serve

# 3. Validate theme output
mbt check

# 4. Build final XML
mbt build

# 5. Package for distribution
mbt bundle
```

---

# Project Goals

Mor Blogger Theme Editor aims to provide:

- A modern visual Blogger theme editor
- Rust-first architecture
- GUI and CLI workflows
- Validation before Blogger import
- Reusable theme components
- Extensible plugin support
- Static export and packaging tools

---

# Troubleshooting

## Dioxus CLI Not Found

Install or reinstall:

```bash
cargo install dioxus-cli
```

## Build Failures

Update Rust:

```bash
rustup update
```

Clean and rebuild:

```bash
cargo clean
cargo build
```

## Blogger Import Errors

Run:

```bash
mbt check
```

before importing generated XML into Blogger.

---

# Repository

https://github.com/MoribundInstitute/mor_blogger_theme_editor


---

## Architecture: Bring Your Own Frontend (BYOF)

This project adheres to a strict "Bring Your Own Frontend" (BYOF) modular architecture. We believe the logic that generates a theme should not be permanently bolted to the interface used to design it.

The workspace is divided into distinct crates with hard compile-time boundaries:

* **`mor_blogger_core` (The Headless Engine):** The pure logic heart of the compiler. It handles TOML parsing, XML template resolution, CSS generation, and strict structural validation. It has zero GUI, OS, or filesystem-dialog dependencies.
* **`mor_blogger_dioxus_ui` (The Visual Workspace):** A Dioxus-powered graphical interface. It features a true "Socket and Plug" design:
  * **The Socket (`MainDock`):** A generic, framework-agnostic window container that draws borders, tabs, and layout boundaries.
  * **The Plug (`BloggerWorkspace`):** The Blogger-specific logic module that slots into the dock. If a developer forks this project for Neocities, they simply rip out the Blogger plug and insert their own. Zero structural friction.
* **`mor_blogger_cli` (`mbt`):** A lightweight, native terminal interface. Built for power users, it wraps the core engine in a fast, standard Unix command surface. Use it to scaffold projects, validate XML syntax, or integrate theme builds into automated CI/CD pipelines without ever opening a window.

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

- [Architecture Overview](docs/ARCHITECTURE.md) : How the Rust rendering engine and Dioxus state management interact.
- [The CSS Assembly Pipeline](docs/CSS_PIPELINE.md) : Understanding the `mor_` namespace and how modular CSS is stitched together.
- [Creating a Theme Preset](docs/THEME_CREATION.md) : Guide to defining tokens, palettes, and custom layouts for the Compendium.
- [GTK Theme Parsing](docs/GTK_PARSER.md) : How the engine translates Linux desktop themes into Blogger variables.

## 🧰 Resources
Need assets or reference material for your theme? Use these external tools:

- **Icons:** [Google Material Symbols](https://fonts.google.com/icons) (Download as SVG, upload via MorBlogger UI)
- **Icons:** [Lucide](https://lucide.dev/) (Clean, neutral SVG icons)
- **Blogger API:** [Official XML Documentation](https://developers.google.com/blogger)
- **Asset Generation:** [halftone.tools](https://halftone.tools) (Free, browser-based print-effects workshop. Perfect for generating retro dithered background plates or custom vector SVGs for the theme dictionary).

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

---

## Custom Fonts & Privacy CDNs

### The Blogger Limitation

Blogger's `<b:skin>` block is CSS-only. There is no asset host for binary font files. You cannot upload `.ttf`, `.woff`, or `.woff2` to Blogger and reference them with a local path. Any custom typeface must arrive via an external URL—either a CDN `@import` or a remote `@font-face` `src`.

### The Privacy Alternative

[fonts.bunny.net](https://fonts.bunny.net) mirrors the Google Fonts catalog without tracking pixels, referrer logging, or IP retention. It is a drop-in, GDPR-compliant substitute for `fonts.googleapis.com`.

1. Pick your family at [fonts.bunny.net](https://fonts.bunny.net).
2. Copy the generated `@import` rule.
3. Paste it into the **MorBlogger Custom CSS** panel.

```css
@import url('https://fonts.bunny.net/css?family=inter:400,700');

:root {
  --font-body: 'Inter', system-ui, sans-serif;
}
```

The MorBlogger export pipeline injects this block into the compiled `<b:skin>` stylesheet. No Google Fonts request hits your readers' browsers.

### Self-Hosting

For fonts not on Bunny or Google—proprietary brand typefaces, niche libre fonts, or files you have licensed—you must host the `.ttf` (or `.woff2`) on a **CORS-enabled** static server. [GitHub Pages](https://pages.github.com/), Cloudflare R2, or any bucket with `Access-Control-Allow-Origin: *` works.

```css
@font-face {
  font-family: 'Brand Serif';
  src: url('https://youruser.github.io/fonts/BrandSerif-Regular.ttf') format('truetype');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}

:root {
  --font-heading: 'Brand Serif', Georgia, serif;
}
```

Paste the `@font-face` block and your `--font-*` overrides into the **MorBlogger Custom CSS** panel. Verify the URL returns `200` and the correct `Content-Type` (`font/ttf` or `font/woff2`) before exporting. A missing CORS header or wrong MIME type silently falls back to the system font stack in most browsers.

