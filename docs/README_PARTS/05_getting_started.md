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

![GUI and CLI development workflow](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/dev_workflow.drawio.png)

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

# 🧩 BYOF Socket & Plug UI Pattern – Plain English Breakdown

This diagram describes a smart way to build software called the **"BYOF" (Bring Your Own Functionality) pattern**. 

Instead of building one big, messy program, the developers split the app into two main parts: a **Socket** (the skeleton) and a **Plug** (the brains). 

Here is what all the technical terms in the diagram actually mean:

---

## 1. The Socket (Left Side - Blue Box)
**Tagline:** *"The generic, reusable skeleton."*

This is the core engine of the program. It doesn't care *what* you are building; it just knows *how* to manage windows, tabs, and panels. 

**Jargon Breakdown:**

- **`MorLayoutChrome`** → **"The Main Window"** 
  - *Plain English:* This is the base application window that opens on your screen. It holds everything else together.

- **`DockZone + dock_chrome`** → **"The Docking System"** 
  - *Plain English:* This is the software that lets you organize panels, tabs, and sidebars. It decides how borders are drawn, how tabs stack up, and where grids sit on the page. 

- **`ActivityBar + FloatingWindowManager`** → **"The Navigation & Popup System"** 
  - *Plain English:* The ActivityBar is the skinny sidebar with buttons (like "Save" or "Settings"). The FloatingWindowManager handles windows that can be dragged anywhere on the screen (like a floating preview pane).

---

## 2. The Plug (Right Side - Green Box)
**Tagline:** *"The domain-specific brains."*

This is where the actual *content* lives. The Socket provides the empty rooms; the Plug decides what furniture goes inside them.

**Jargon Breakdown:**

- **`BloggerWorkspace (current)`** → **"The Blogger Editor"** 
  - *Plain English:* This is the currently installed plug. It contains all the specific code needed to edit Google Blogger themes.

- **`Preview · export · site data`** → **"Blogger-specific logic"** 
  - *Plain English:* These are the actual tools you use: generating live previews of your theme, exporting the final code, and pulling data from Blogger. 

- **`NeocitiesWorkspace (fork example)`** → **"A Future Use Case"** 
  - *Plain English:* The dotted outline shows a potential future plan. Someone could take the exact same **Socket** (blue box), swap out the Blogger logic, and create a theme editor for **Neocities** (a different website platform) without having to re-write the whole app.

---

## 3. The Yellow Box (Bottom Center)
**Tagline:** *"The core promise of this architecture."*

**Text:** *"Swap the plug, keep the socket — zero structural friction for forks."*

- **What "Structural Friction" means:** In regular programming, if you want to fork (copy and modify) an app, you usually have to dig through thousands of lines of messy code to change even a tiny thing. That is a painful, error-prone process (friction).
- **What this diagram promises:** Because the app is split into a "Socket" and a "Plug", a developer can copy the project, delete the green "Blogger" box, and write a brand-new green box for a different platform. The blue "Socket" doesn't change at all, meaning the new app gets a polished, fully-working interface for free. No friction!

---

## 4. The Connector: `"slots into"`

- **What it means:** The arrow shows that the right side (Plug) physically "plugs in" to the left side (Socket).
- **How it works:** The Socket leaves empty holes in its interface. The Plug provides specific tools to fill those holes. When you click a button on the Socket's activity bar, it triggers an action inside the Plug.

---

## 🚀 Why Build Software This Way?

1. **Less work for developers:** You only build the complex window engine (Socket) once.
2. **Easy to fork:** Anyone can copy this project to build a similar app for a different website (like Neocities or WordPress) without breaking the app.
3. **Bug-free updates:** Since the UI engine and the content logic are separate, fixing a bug in the Blogger code won't break the drag-and-drop window system.

---