#!/usr/bin/env bash
# Install the .desktop entry + hicolor icon so Wayland maps the app_id to an
# icon (see the glib::set_prgname note in mor_blogger_dioxus_ui/src/main.rs).
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
APP_ID=com.moribundinstitute.morblogger-theme-editor
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}"
APPS_DIR="$DATA_DIR/applications"
ICON_DIR="$DATA_DIR/icons/hicolor/512x512/apps"

mkdir -p "$APPS_DIR" "$ICON_DIR"

# Point Exec at this checkout's release binary.
sed "s|^Exec=.*|Exec=$REPO_DIR/target/release/mor_blogger_dioxus_ui|" \
    "$REPO_DIR/packaging/$APP_ID.desktop" > "$APPS_DIR/$APP_ID.desktop"

cp "$REPO_DIR/mor_blogger_dioxus_ui/assets/icon.png" "$ICON_DIR/$APP_ID.png"

update-desktop-database "$APPS_DIR" 2>/dev/null || true
gtk-update-icon-cache -f "$DATA_DIR/icons/hicolor" 2>/dev/null || true

echo "Installed $APPS_DIR/$APP_ID.desktop and icon."
[ -x "$REPO_DIR/target/release/mor_blogger_dioxus_ui" ] \
    || echo "Note: release binary not built yet (cargo build --release)."
