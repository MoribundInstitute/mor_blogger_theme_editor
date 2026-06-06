# MOR_PLAN: Moribund Theme Architect

## ✅ RECENTLY COMPLETED
* **The Core Registry:** Modularized the monolithic Blogger XML into strict Rust Web Components (Headers, Layouts, Content, Sidebars, Footers).
* **Content Feeds:** Implemented `mor-magazine`, `mor-masonry`, and `mor-minimal` data loop architectures.
* **Dynamic CSS Engine:** Wired Dioxus UI signals to inject user-defined CSS cascade overrides into the `<b:skin>` block.
* **State Hotswapping:** Replaced manual layout mapping with zero-cost `serde_json` serialization for instant workspace saving/loading.
* **The Smoke Test:** Successfully compiled and deployed a modular layout onto Google's Blogger servers.
* **Shortcut Architecture:** Rewrote `shortcut.rs` from a basic `HashMap` into a `ShortcutMeta` registry capable of broadcasting keybinds, categories, and descriptions.

---

## 📍 IMMEDIATE NEXT STEPS (Phase 11-13)

### Phase 11: The Nemo Shortcut Integration
* **Task 1: Shell Wiring:** Inject the `show_shortcuts` state signal into `app/shell.rs` and pass it down to `menu_bar.rs` to trigger the modal drop-down.
* **Task 2: Dynamic Data Binding:** Refactor `src/ui/components/shortcut_modal.rs` to replace the hardcoded placeholder strings. It must consume the global `ShortcutRegistry`, group the `ShortcutMeta` structs into a `BTreeMap` by category, and dynamically render the `.mor-keycap` UI.

### Phase 12: UX Pruning & Button Sprawl Cleanup
* **Task 1: Master Canvas Purge:** Strip out redundant "Download Theme", "Restore Workspace", and Sample Content buttons from `src/ui/workspace/master_canvas.rs` to clean the center workspace.
* **Task 2: Menu Bar Finalization:** Ensure all global actions (Open Project, Save Project, Export XML) strictly fire from `src/ui/shell/menu_bar.rs` using the newly defined keyboard shortcuts.

### Phase 13: Persistent User Preferences
* **Task 1: Disk I/O for Keybinds:** Expand `editor_prefs.json` serialization to capture user-customized keybinds.
* **Task 2: The Interceptor:** Create a listener UI within the Shortcuts Modal that allows a user to click a shortcut row, press a new combination, and overwrite the default `ShortcutMeta.keys` string.

---

## 🚀 FUTURE MILESTONES (Phase 14+)

### Phase 14: The "View Code" Compromise
* To avoid the heavy dependency bloat of a full code editor, implement a read-only XML viewer. 
* Add a modal containing a basic `<textarea readonly>` so users can inspect the compiled output of `build_master_css` and `render_theme` without leaving the app.

### Phase 15: CSS Token Builder
* Wire up the "CSS Token Builder" tools menu item to allow users to define and save custom variable presets beyond the hardcoded core palettes.