## 🛠️ Getting Started

### Prerequisites
- Rust toolchain → [rustup.rs](https://rustup.rs/)
- Dioxus CLI
  ```bash
  # Install the Dioxus CLI
  cargo install dioxus-cli
🖥️ Option A: The Visual Workspace
To launch the graphical GUI theme builder, use the Dioxus CLI from inside the UI crate. This provides hot-reloading for UI development:

Bash
cd mor_blogger_dioxus_ui
dx serve
💻 Option B: The Headless CLI (mbt)
For terminal users, you can compile and validate themes directly without the GUI.

First, build the optimized executable:

Bash
cargo build --release -p mor_blogger_cli
Then, you can use the command-line tool to manage your workspace:

Bash
# Scaffold a new project template
./target/release/mbt init

# Validate your XML syntax (catches Blogger import errors early)
./target/release/mbt check

# Build the final Blogger XML theme
./target/release/mbt build

# Build the theme AND package it into a zip with static HTML pages
./target/release/mbt bundle