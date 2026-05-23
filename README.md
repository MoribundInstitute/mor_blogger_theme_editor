# Blogger Theme Architect (`mor_blogger_theme_editor`)

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/UI-Dioxus-blue.svg)](https://dioxuslabs.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Blogger Theme Architect** is a reactive, Rust-based visual editor designed for modern, responsive Blogger template development. It allows you to build, customize, and export fully functional Blogger XML themes seamlessly without touching raw XML.

## ✨ Key Features

- **Live Visual Editor:** A fast, reactive UI built with Dioxus. Tweak colors, typography, and layouts, and watch the preview update instantly via an isolated iframe.

- **State Rehydration (Magic Import/Export):** Never lose your work. The editor serializes your configuration state into base64-encoded TOML and safely hides it inside the exported Blogger XML. To restore a previous session, simply paste your exported XML back into the editor.

- **Flexible Workspaces:** Customize your UI with hotkey-driven layouts:
  - `Alt + 1`: Split View (Standard)
  - `Alt + 2`: Wide Editor
  - `Alt + 3`: Floating, draggable editor window
  - `Alt + 4`: Preview Takeover

- **Deep Customization:**
  - **Typography & Colors:** Fine-tune font stacks, scaling ratios, line heights, and extensive color palettes.
  - **SEO & Metadata:** Easily configure meta descriptions, custom robots tags, and Open Graph social cards.
  - **Menu & Navigation:** Manage header menus, drop-downs, and catalog structures.

- **Plugin Injection:** A dedicated drag-and-drop zone to inject custom JavaScript files directly into your theme before export.

- **Built-in Diagnostics:** Real-time integrity checks ensure your template structure remains valid and ready for Blogger.

## 🚀 Getting Started

### Prerequisites

You will need the Rust toolchain and the Dioxus CLI installed on your machine.

1. **Install Rust:** [rustup.rs](https://rustup.rs/)
2. **Install Dioxus CLI:**

```bash
cargo install dioxus-cli
```

### Running the Editor Locally

Clone the repository and run the Dioxus development server:

```bash
git clone https://github.com/MoribundInstitute/mor_blogger_theme_editor.git
cd mor_blogger_theme_editor

# Serve with hot-reloading enabled
dx serve --hot-reload
```

The application will launch in your default web browser, usually at `http://localhost:8080`.

## 🏗️ Project Architecture

- `src/app.rs`: The Dioxus application root. Handles global state, layout hotkeys, and rendering the core UI shell.

- `src/render.rs`: The rendering engine. It takes the active `ThemeConfig` and merges it into the `template.xml` skeleton to generate both the live preview HTML and the final exported Blogger XML.

- `src/rehydration.rs`: Handles the encoding and decoding of the editor's TOML state, safely injecting it into HTML comments within the exported theme.

- `src/config.rs` and `src/defaults.rs`: Define the data structures for the theme and provide the standard "Modern Editorial" starter configuration.

- `src/ui/`: Contains the modular Dioxus components, including panels, inputs, and presets, that make up the visual editor.

## 🛠️ Usage Workflow

1. **Design:** Use the left-hand panel to adjust your site identity, palette, and typography.

2. **Preview:** Test different device viewports, including desktop, tablet, and mobile, in the right-hand canvas.

3. **Export:** Navigate to the Export panel, copy the generated XML, and paste it directly into your Blogger dashboard under **Theme > Edit HTML**.

4. **Restore:** To make changes later, paste that same XML back into the editor's "Restore Workspace" drawer to instantly recover your exact settings.

## 📄 License

This project is open-sourced under the MIT License.

