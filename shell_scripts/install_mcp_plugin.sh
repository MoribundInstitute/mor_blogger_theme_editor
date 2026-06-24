#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PLUGIN_DIR="${1:-$ROOT/plugins/mission5_echo_mcp}"

echo "➔ Installing local MCP plugin from: $PLUGIN_DIR"
cargo run --manifest-path "$ROOT/Cargo.toml" -p mor_blogger_cli -- plugin install "$PLUGIN_DIR"