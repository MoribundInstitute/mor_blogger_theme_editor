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
