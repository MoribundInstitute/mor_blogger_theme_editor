# MOR_PLAN: Moribund Theme Architect

**Vision:** the definitive desktop tool for Blogger XML themes. A designer opens it and
never sees code; a developer opens it and gets a real editor with Blogger-aware
intelligence. Every visual control and every line of code compile through the same
`ThemeConfig` → modules → XML pipeline, so the two audiences never fork the theme.

**The four laws (already enforced — keep them):**
1. `ThemeConfig` is the brain. GUI panels and code editors are both just views of it.
2. Config-driven subsystems (buttons, scrollbars, cursors, palettes) are single sources
   of truth — presets and users never hand-write CSS the config can generate.
3. Ship no more than the page can use (JS behavior tree-shaking; hook attribution).
4. No `data-mor-edit` attribute, no direct editing (see Appendix).

---

## ✅ WHERE THE APP IS TODAY (v. 2026-07)

* **Modular compiler:** monolithic Blogger XML decomposed into a registry of Rust web
  components — headers, layouts, content feeds (`mor-magazine`, `mor-masonry`,
  `mor-minimal`), sidebars, footers, widgets, gadgets — with per-module manifests
  (CSS/JS deps) and `custom_<key>.xml` override persistence.
* **Dockable IDE shell:** 16 docks (theme palette, site data, CSS/JS/template editors,
  CSS/JS builders, diagnostics, plugins, widgets, code nav, static pages…), pinnable
  activity bar, floating/pop-out OS windows, full-viewport takeover, draggable dialogs,
  shortcut registry + modal.
* **Real code option:** CodeMirror 6 editors (XML/CSS/JS/TOML) with search, minimap,
  word-wrap, JS syntax lint (Lezer), theme-aware completions (`_MOR_CONFIG`, behavior
  DOM hooks), customized-file markers, TOML↔compiled-XML toggle.
* **JS workspace:** per-behavior Active/Wasted/Off analysis with hook→module
  attribution, one-click ship/stop-shipping, bundle byte budget, codeless behavior
  settings, cross-links into the JS editor.
* **Config-driven styling:** buttons, cursors, scrollbars, glow/frame targeting,
  logo, advanced color/typography dialogs; per-page layout/chrome overrides.
* **8 aesthetic presets** (rewritten 2026-07): dual light/dark palettes, registry
  fonts, validated against the compiled XML via `mbt build`.
* **Diagnostics:** module scanner incl. un-CDATA'd script detection, export safety CSS.
* **CLI (`mbt`):** deterministic TOML → Blogger XML builds for CI and verification.
* **Proven export:** compiled themes deploy onto live Blogger.

Phases 11–12 (shortcut wiring, UX pruning) and 14–15 (code viewing — exceeded: full
editors; CSS builder dock) from the previous plan are done or obsolete.

---

## ✅ PHASE 16 — Land & Harden (done 2026-07-02)

* **Task 1: Commit the in-flight branch.** ~44 modified files on
  `chore/ponytail-audit-cleanup` (audit cleanup + JS workspace/editor upgrades).
  Split into coherent commits; merge to `main`.
* **Task 2: Preset regression gate.** A test (or CI step) that runs every
  `theme_presets/*.toml` through the loader + `mbt build` and fails on parse errors,
  empty `preset_css`, or CDATA hazards — presets can never silently rot again.
* **Task 3: Selector-drift guard.** Extend diagnostics to flag preset/custom CSS
  selectors that match no class in the active template modules (the
  `.mor-catalog-dropdown` lesson, automated).
* **Task 4: Runtime smoke checklist.** One scripted pass (mbt + xdotool-optional)
  covering: preset load, module swap, JS workspace actions, export, editor lint.

## ✅ PHASE 17 — Keybinds & Editor Ergonomics (done 2026-07-02)

* **Task 1: Persist custom keybinds** in `editor_prefs.toml` (carried from old
  Phase 13 — still the only unfinished item from that plan).
* **Task 2: Rebind interceptor** in the Shortcuts modal: click a row, press keys,
  overwrite `ShortcutMeta.keys`, persist.
* **Task 3: Editor persistence parity:** word-wrap override per workspace (same
  mechanism as minimap), remembered active tab per editor dock.

## 📍 PHASE 18 — Blogger-Aware Code Intelligence (tasks 1–4 done 2026-07-02; importer remains)

The CM6 foundation exists; make it *Blogger-specialized* — the thing no generic
editor offers:

* **Task 1: `b:` tag completions** in XML mode — `b:section`, `b:widget`, `b:if`,
  `b:loop`, `data:` expressions, `expr:` attributes — sourced from a curated schema of
  Blogger Layouts v3, same injection pattern as `MOR_JS_HINTS`.
* **Task 2: Blogger lint:** unclosed `b:` tags, widgets outside sections, duplicate
  widget ids, unescaped CDATA hazards — reusing the existing diagnostics analyzers as
  editor squiggles instead of a separate dock-only report.
* **Task 3: Per-module custom JS** (`custom_<key>.js`, mirroring `custom_<key>.xml`)
  so hand-written JS can follow its module through swaps and exports.
* **Task 4: Diff & reset:** side-by-side diff of an edited file vs its bundled
  default + one-click reset (the "customized ●" marker already knows).
* **Task 5: Theme importer (remaining).** Open any existing Blogger theme XML and
  decompose it into workspace state. Note: `utils/rehydration.rs` only round-trips
  *this app's own* exports (embedded workspace payload) — foreign themes are
  unhandled. Suggested v1 ("salvage import", separate from the existing import
  action): extract the `<b:skin>` CSS into `preset_css`, inventory sections and
  widgets into a report, and flag what can't map to the module registry — useful
  restyling on-ramp without pretending full decomposition. Full module
  decomposition is v2.

## 📍 PHASE 19 — The Editor Canvas (structured direct manipulation)

The Wix-feeling, done safely — full design in the Appendix. Build order:

* **Task 1: CanvasBridge event protocol** (iframe → Dioxus JSON events; the preview
  canvas already sends navigation events, extend that channel).
* **Task 2: `data-mor-edit` markers** emitted by the renderer for the easy,
  high-value fields: site title/subtitle, logo URL, menu labels/URLs, footer text,
  panel titles.
* **Task 3: Edit Mode toggle** (`Browse | Inspect | Edit`) with hover outlines,
  selection, `contenteditable` text, blur/Enter → config update → rerender.
* **Task 4: Click-a-surface color editing** (select a panel → theme palette focuses
  the matching config color).
* **Non-goal:** free drag-and-drop layout. Module swaps + widget sockets already
  cover structure; direct manipulation is for *content and tokens*.

## 📍 PHASE 20 — Publisher (editor → pipeline)

* **Task 1: `mor_blogger_api` crate:** OAuth 2.0 (Google), Blogger API v3.
* **Task 2: One-click deploy:** push the compiled theme to a chosen blog, with the
  pre-flight diagnostics report as the gate and an automatic pre-deploy XML backup
  of the current live theme (one-click rollback).
* **Task 3: Post & page manager:** fetch/update posts so the preview can render the
  user's *real* content instead of sample data.

## 🚀 PHASE 21+ — Ecosystem & Polish

* Preset/theme sharing: export a workspace as a single portable pack; import others'.
* Template pack growth: grid gallery + text-minimalist layout presets (ideas.md).
* Budgets beyond JS: CSS byte budget and unused-selector report per export.
* Accessibility pass over generated themes: contrast checks per palette, focus
  states, reduced-motion coverage (presets already lead here — make it enforced).
* Burn down the `ponytail:` ledger (17 markers) as their ceilings are reached.

---

## Appendix — Editor Canvas design (kept from prior plan; still the blueprint)

**Naming:** `PreviewCanvas` (passive shell) / `EditorCanvas` (interactive mode) /
`CanvasBridge` (iframe↔Rust events), living in `src/ui/workspace/canvas/`
(`preview_canvas.rs`, `editor_canvas.rs`, `device_frame.rs`, `bridge.rs`,
`inspector.rs`, `selection_overlay.rs`, `events.rs`).

**The trick — editable bindings.** Generated preview HTML carries stable markers:

```html
<h1 data-mor-edit="site.site_title" data-mor-edit-kind="text">
  Your Website Title Here
</h1>
```

The bridge detects clicks on `[data-mor-edit]` and emits:

```json
{ "type": "select", "field": "site.site_title", "kind": "text", "text": "…" }
{ "type": "update_text", "field": "site.site_title", "value": "Moribund XML Compendium" }
```

Rust updates the real config signal; the renderer regenerates the preview. Never edit
the iframe DOM as truth — it is discarded on every rerender.

**Why not full Wix:** Wix edits a canvas model; this app edits a theme-compiler
model. Every editable thing needs a known path back to `ThemeConfig`.

**The rule:** no `data-mor-edit` attribute, no direct editing. The preview canvas is
the eyes, the editor canvas is the hands, `ThemeConfig` stays the brain.
