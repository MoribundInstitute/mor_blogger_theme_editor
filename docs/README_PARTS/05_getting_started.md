# 🛠️ Getting Started

MorBlogger Theme Editor is a Rust-powered desktop app for building, editing, validating, and packaging Blogger themes.

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

## Option A: Launch the Visual Editor

The visual workspace provides:

- Live preview workflows
- Hot reloading during development
- Theme configuration editing
- Visual design tooling
- Plugin and workspace integration (via Plugin Manager)

From the project root:

```bash
cd mor_blogger_dioxus_ui
dx serve
```

This opens the **native desktop window** (Dioxus desktop target — not a browser tab).

---

## Option B: Use the Command-Line Tool (mbt)

If you prefer a terminal workflow, use the `mbt` utility.

Build the release executable:

```bash
cargo build --release -p mor_blogger_cli
```

The binary will be located at:

```text
target/release/mbt
```

### Initialize a Project

Create a new theme workspace:

```bash
./target/release/mbt init
```

### Validate a Theme

Check XML syntax and catch Blogger import issues before deployment:

```bash
./target/release/mbt check
```

### Build a Blogger Theme

Generate the final Blogger-compatible XML output:

```bash
./target/release/mbt render -i workspace.toml
```

### Create a Distribution Bundle

Build the theme and package supporting assets:

```bash
./target/release/mbt bundle
```

---

## Typical Development Workflow

```bash
# 1. Create or open a workspace
mbt init

# 2. Edit visually
cd mor_blogger_dioxus_ui && dx serve

# 3. Validate theme output
mbt check

# 4. Build final XML
mbt render -i workspace.toml

# 5. Package for distribution
mbt bundle
```

![GUI and CLI development workflow](docs/diagrams/dev_workflow.drawio.png)

---

## Project Goals

MorBlogger Theme Editor aims to provide:

- A modern visual Blogger theme editor
- Rust-first architecture
- GUI and CLI workflows
- Validation before Blogger import
- Reusable theme components
- Extensible plugin support
- Static export and packaging tools

---

## Troubleshooting

### Dioxus CLI Not Found

Install or reinstall:

```bash
cargo install dioxus-cli
```

### Build Failures

Update Rust:

```bash
rustup update
```

Clean and rebuild:

```bash
cargo clean
cargo build
```

### Blogger Import Errors

Run:

```bash
mbt check
```

before importing generated XML into Blogger.

---

**Repository:** https://github.com/MoribundInstitute/mor_blogger_theme_editor