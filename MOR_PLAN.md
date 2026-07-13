# MOR_PLAN: Moribund Theme Architect

**Vision:** the definitive desktop tool for Blogger XML themes. A designer opens it and
never sees code; a developer opens it and gets a real editor with Blogger-aware
intelligence. Every visual control and every line of code compile through the same
`ThemeConfig` → modules → XML pipeline, so the two audiences never fork the theme.

**The four laws (already enforced — keep them):**
1. `ThemeConfig` is the brain. GUI panels and code editors are both just views of it.
2. Config-driven subsystems (buttons, scrollbars, palettes) are single sources of
   truth — presets and users never hand-write CSS the config can generate.
3. Ship no more than the page can use (JS behavior tree-shaking; hook attribution).
4. No `data-edit-target` / `data-field-path` marker, no direct editing (see Appendix).
5. *(new, learned the hard way)* Preview and export render from the SAME sources.
   A hand-built preview copy of anything the exporter also generates is a bug
   factory — it hid blank header icons and an empty Archive widget on live blogs.

---

## ✅ WHERE THE APP IS TODAY (v. 2026-07-13)

* **Modular compiler:** monolithic Blogger XML decomposed into a registry of Rust web
  components — headers, layouts, content feeds (`mor-magazine`, `mor-masonry`,
  `mor-minimal`), sidebars, footers, widgets, gadgets — with per-module manifests
  (CSS/JS deps) and `custom_<key>.xml` override persistence.
* **Dockable IDE shell:** 16 docks, pinnable activity bar, floating/pop-out OS
  windows, full-viewport takeover, draggable dialogs, shortcut registry + rebinding.
* **Real code option:** CodeMirror 6 editors (XML/CSS/JS/TOML) with search, minimap,
  JS lint, theme-aware completions, customized-file markers, diff-vs-bundled view.
* **Canvas editing (Phase 19, largely landed):** preview iframe carries
  `data-edit-target` / `data-field-path` / `data-block-id` markers end-to-end
  (export XML too — see `render/tracking.rs`); shift-click icon swapping, X-ray
  inspect chips, click-a-surface → the matching config panel focuses. Preview
  renders the REAL resolved header/sidebar/widget modules, guarded by tests.
* **Config-driven styling:** buttons, scrollbars, glow/frame targeting, logo,
  advanced color/typography dialogs; per-page layout/chrome overrides.
* **10 aesthetic presets** with dual light/dark palettes; compendium repo
  (`mor-blogger-theme-preset-compendium`) serves importable JSON presets.
* **Diagnostics:** module scanner incl. un-CDATA'd script detection, export safety
  CSS, plus drift guards in tests: default pack ids must exist in registries,
  UI dropdown ids must exist in core registries, widget blueprints using
  `data:this` must bind `var='this'`.
* **CLI (`mbt`):** `init` / `check` / `render` / `export` (reversible, embeds
  workspace state) / `restore` (rehydrate any exported XML) / `bundle` / `plugin`.
* **Packaged releases:** deb + rpm + Arch packages with declared runtime deps,
  built via `dx bundle` + `packaging/PKGBUILD` + `packaging/mor-blogger.spec`;
  published as GitHub prereleases (latest: v0.1.0-pre.2).
* **Proven export:** compiled themes deploy onto live Blogger (morbranding blog),
  including native Label/BlogArchive/HTML widgets rendering with data.

**Recently landed (2026-07-13):** repo-wide over-engineering cleanup (~6k lines,
−3 deps); standard `mask-*` fallbacks for sidebar icons; `url()` icon values render
as masked spans in header toggles; preview uses real header modules; BlogArchive
`var='this'` fix (was rendering empty on live blogs); stale default header id fix;
Linux packaging + prerelease pipeline.

---

## 📍 PHASE 18.5 — Theme Importer (the one Phase 18 leftover)

Open any FOREIGN Blogger theme XML and salvage it into workspace state.
`utils/rehydration.rs` round-trips only this app's own exports; foreign themes are
unhandled. v1 "salvage import" (separate from the existing import action): extract
`<b:skin>` CSS into `preset_css`, inventory sections/widgets into a report, flag
what can't map to the module registry. Full module decomposition is v2.

## 📍 PHASE 19.5 — Finish the Editor Canvas

What remains from the original Phase 19 design:
* **Inline text editing:** `contenteditable` on `data-field-path` text nodes
  (site title, menu labels, footer text, widget titles), blur/Enter → config
  update → rerender. Selection & markers already exist; this is the last mile.
* **Non-goal (unchanged):** free drag-and-drop layout. Module swaps + widget
  sockets cover structure; direct manipulation is for content and tokens.

## 📍 PHASE 20 — Publisher (editor → pipeline)

* **Task 1: `mor_blogger_api` crate:** OAuth 2.0 (Google), Blogger API v3.
* **Task 2: One-click deploy:** push the compiled theme to a chosen blog, gated on
  the diagnostics report, with automatic pre-deploy backup of the live theme
  (one-click rollback). Kills the manual download/upload loop that let stale
  exports linger on the live blog.
* **Task 3: Post & page manager:** fetch real posts so the preview renders the
  user's actual content instead of sample data.

## 📍 PHASE 21 — Ship v0.1.0 stable

* **CI gate (GitHub Actions):** `cargo test --workspace` + render every
  `theme_presets/*.toml` through `mbt render` and fail on errors — the preset
  regression gate, automated on every push.
* **Windows build:** `dx bundle --package-types msi` on Windows 11 (WebView2 is
  preinstalled); attach to the release.
* **Manual GUI pass** over the packaged build (docks, module swaps, import,
  export, restore), then promote the prerelease to v0.1.0.
* **Compendium refresh:** re-export bundled presets to the compendium repo so its
  JSON matches current export output; regenerate its demo site theme (currently a
  hand-written XML predating the icon system).

## 🚀 PHASE 22+ — Ecosystem & Polish

* Preset/theme sharing: export a workspace as a single portable pack.
* Budgets beyond JS: CSS byte budget and unused-selector report per export
  (the audit found ~21% dead selectors in the editor's own CSS — themes deserve
  the same lint).
* Accessibility pass over generated themes: contrast checks per palette, focus
  states, reduced-motion coverage.
* Burn down the `ponytail:` ledger (19 markers) as their ceilings are reached.

---

## Appendix — Editor Canvas design (updated to shipped naming)

**Shipped naming** (the old plan said `data-mor-edit`; the code went with):
* `data-edit-target="icons.label"` — shift-click editable glyph/surface, routes to
  the owning config panel.
* `data-field-path="site.site_title"` — text node bound to a config field.
* `data-block-id="Label1"` — module/widget block identity for X-ray + selection.

Markers are emitted by the renderer in BOTH preview and export
(`render/tracking.rs` tests pin this), so the canvas never edits DOM as truth —
the iframe emits JSON events, Rust updates the config signal, the renderer
regenerates. The preview is the eyes, the canvas bridge is the hands,
`ThemeConfig` stays the brain.

**Why not full Wix:** Wix edits a canvas model; this app edits a theme-compiler
model. Every editable thing needs a known path back to `ThemeConfig`.
