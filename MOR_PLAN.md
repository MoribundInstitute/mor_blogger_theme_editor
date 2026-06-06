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

It is **not too advanced**, but the key is this:

You should **not** make users edit the rendered HTML directly. That would be fragile because your preview is generated from `ThemeConfig -> XML/templates -> preview_html`. If someone edits the iframe DOM directly, that change disappears the next time the preview rerenders.

The Wix-style version should be:

```text
User clicks text/button/panel in preview
        ↓
Preview iframe sends “edit this field” event to Dioxus
        ↓
Dioxus updates ThemeConfig / signals
        ↓
Renderer regenerates preview_html
        ↓
Preview updates cleanly
```

Right now your preview canvas is mostly a **viewer**. It writes `preview_html` into an iframe, scales the device frame, intercepts internal links, and sends only href navigation events back to Rust.  The center workspace already treats it as a passive child component: it passes `preview_html`, viewport state, and an optional `on_navigate` handler into `PreviewCanvas`. 

So, yes, it could become an `EditorCanvas`, but it needs an **event protocol** first.

## Better naming

I would not call it `preview_editor`. I would use:

```text
PreviewCanvas
  passive preview shell

EditorCanvas
  interactive editing mode

CanvasBridge
  iframe/Rust communication layer
```

Or:

```text
src/ui/workspace/canvas/
├── mod.rs
├── preview_canvas.rs
├── editor_canvas.rs
├── device_frame.rs
├── bridge.rs
├── inspector.rs
├── selection_overlay.rs
└── events.rs
```

That keeps preview and editing separate. You can still render the same iframe, but switch behavior depending on mode.

## The important trick: editable bindings

Your generated preview HTML needs stable edit markers:

```html
<h1 data-mor-edit="site.site_title" data-mor-edit-kind="text">
  Your Website Title Here
</h1>

<p data-mor-edit="site.site_subtitle" data-mor-edit-kind="text">
  A blurb about your site here
</p>

<a data-mor-edit="menu.0.label" data-mor-edit-kind="text">
  Home
</a>
```

Then the iframe bridge can detect clicks on `[data-mor-edit]`.

Example event:

```json
{
  "type": "select",
  "field": "site.site_title",
  "kind": "text",
  "text": "Your Website Title Here"
}
```

Then if the user edits it:

```json
{
  "type": "update_text",
  "field": "site.site_title",
  "value": "Moribund XML Compendium"
}
```

Rust receives that and updates the real config signal. That is the safe version of Wix editing.

## What you should make editable first

Start with easy, high-value stuff:

```text
Site title
Site subtitle
Header logo URL
Menu labels
Menu URLs
Footer text
Panel titles
Button labels
Theme colors by clicking a surface
SVG icons by clicking visible icons
```

Do **not** start with arbitrary drag-and-drop layout editing. That is where the monster lives.

## Why Wix-style editing is harder for your app

Wix owns the whole page model. It can say, “this box is object #493, with x/y/width/color/text.” Your app is compiling a Blogger XML theme, and Blogger has its own widget rules. So your editor cannot safely let users freely mutate every rendered DOM node.

Plain version:

Wix edits a canvas model.
Your app edits a theme compiler model.

That is not bad. It just means every editable thing needs a known path back to `ThemeConfig`.

## Best first version

Add an **Edit Mode** toggle next to Modern/Sidebars:

```text
Browse | Inspect | Edit
```

Your existing `PreviewTemplateMode` already proves this area is meant to grow beyond a dumb preview. 

In Edit Mode:

1. Hovering editable elements outlines them.
2. Clicking an editable element selects it.
3. A small floating mini-toolbar appears.
4. Text fields become `contenteditable`.
5. On blur or Enter, the iframe sends an update event to Rust.
6. Rust updates `ThemeConfig`.
7. The preview rerenders.

That gives you the “Wix feeling” without building a whole Wix clone.

## The rule I would use

```text
No data-mor-edit attribute, no direct editing.
```

That keeps the app sane.

You can absolutely evolve this into an editor canvas. Just do it as **structured direct manipulation**, not raw DOM hacking. The preview canvas becomes the eyes, the editor canvas becomes the hands, and `ThemeConfig` stays the brain.
