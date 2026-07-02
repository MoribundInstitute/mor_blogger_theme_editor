// CodeMirror 6 bundle for the MorBlogger code editors.
//
// Exposes a tiny imperative API on `window.morCM` so the Dioxus `CodeEditor`
// component can mount/update editors without knowing anything about CM internals.
// Built to ../cm6.bundle.js (IIFE) via `npm run build` and injected once in shell.rs.

import { EditorView, basicSetup } from "codemirror";
import { EditorState, Annotation, Compartment } from "@codemirror/state";
import { keymap } from "@codemirror/view";
import { indentWithTab } from "@codemirror/commands";
import { search, openSearchPanel } from "@codemirror/search";
import { StreamLanguage, syntaxTree } from "@codemirror/language";
import { linter, lintGutter } from "@codemirror/lint";
import { oneDark } from "@codemirror/theme-one-dark";

import { showMinimap } from "@replit/codemirror-minimap";

import { html } from "@codemirror/lang-html";
import { xml } from "@codemirror/lang-xml";
import { css } from "@codemirror/lang-css";
import { javascript, javascriptLanguage } from "@codemirror/lang-javascript";
import { json } from "@codemirror/lang-json";
import { toml } from "@codemirror/legacy-modes/mode/toml";

// Marks programmatic (Rust -> editor) edits so the change listener can ignore
// them and avoid an echo loop back into Rust.
const External = Annotation.define();

// Compartment lets the minimap be toggled live (settings change) without
// rebuilding the whole editor / losing cursor + undo history.
const minimapComp = new Compartment();
// Live word-wrap toggle without losing cursor/undo (same pattern as minimap).
const wrapComp = new Compartment();

function minimapExt() {
  return showMinimap.compute([], () => ({
    create: () => ({ dom: document.createElement("div") }),
    displayText: "blocks",
    showOverlay: "always",
  }));
}

// Squiggle Lezer parse-error nodes. JS-only: the CSS/XML buffers carry Blogger
// template syntax ($(var), <b:...>) their parsers would false-positive on.
const jsSyntaxLinter = linter((view) => {
  const diagnostics = [];
  syntaxTree(view.state).cursor().iterate((node) => {
    if (!node.type.isError) return;
    diagnostics.push({
      from: node.from,
      to: Math.min(Math.max(node.to, node.from + 1), view.state.doc.length),
      severity: "error",
      message: "Syntax error",
    });
  });
  return diagnostics;
});

// Theme-aware completions: _MOR_CONFIG keys + the DOM hooks the shipped
// behaviors query. The Rust shell injects `window.MOR_JS_HINTS`
// ([{label, type, detail}]) from the behavior catalog, so this never drifts
// from the real theme data.
const themeHints = javascriptLanguage.data.of({
  autocomplete: (ctx) => {
    const word = ctx.matchBefore(/[\w$.-]+/);
    if (!word || (word.from === word.to && !ctx.explicit)) return null;
    const hints = window.MOR_JS_HINTS || [];
    if (!hints.length) return null;
    return { from: word.from, options: hints, validFor: /^[\w$.-]*$/ };
  },
});

function langExtension(mode) {
  switch (mode) {
    case "xml": return xml();
    case "html": return html();
    case "css": return css();
    case "js":
    case "javascript": return [javascript(), themeHints, jsSyntaxLinter, lintGutter()];
    case "json": return json();
    case "toml":
    case "ini": return StreamLanguage.define(toml);
    default: return [];
  }
}

// Fill the host element and keep scrolling inside the editor.
const hostTheme = EditorView.theme({
  "&": { height: "100%", fontSize: "13px" },
  ".cm-scroller": { overflow: "auto", fontFamily: "monospace" },
});

const registry = new Map(); // id -> EditorView

function mount(id, opts) {
  const parent = document.getElementById(id);
  if (!parent) return;
  destroy(id); // idempotent: replace any existing view at this id

  const extensions = [
    basicSetup,
    search({ top: true }), // Ctrl+F find/replace panel anchored at the top
    keymap.of([indentWithTab]),
    langExtension(opts.lang),
    oneDark,
    hostTheme,
    EditorState.readOnly.of(!!opts.readOnly),
    EditorView.editable.of(!opts.readOnly),
  ];
  extensions.push(wrapComp.of(opts.wrap ? EditorView.lineWrapping : []));
  extensions.push(minimapComp.of(opts.minimap ? minimapExt() : []));
  if (typeof opts.onChange === "function") {
    extensions.push(
      EditorView.updateListener.of((u) => {
        if (!u.docChanged) return;
        if (u.transactions.some((tr) => tr.annotation(External))) return; // ours, skip
        opts.onChange(u.state.doc.toString());
      })
    );
  }

  const view = new EditorView({
    state: EditorState.create({ doc: opts.doc || "", extensions }),
    parent,
  });
  registry.set(id, view);
}

function setValue(id, text) {
  const view = registry.get(id);
  if (!view) return;
  if (view.state.doc.toString() === text) return;
  view.dispatch({
    changes: { from: 0, to: view.state.doc.length, insert: text },
    annotations: External.of(true),
  });
}

// Select + scroll to the first occurrence of `text` (replaces the old
// textarea setSelectionRange/scrollTop trick used by the TOML "jump to" rail).
function reveal(id, text) {
  const view = registry.get(id);
  if (!view) return;
  const idx = view.state.doc.toString().indexOf(text);
  if (idx === -1) return;
  view.focus();
  view.dispatch({
    selection: { anchor: idx, head: idx + text.length },
    effects: EditorView.scrollIntoView(idx, { y: "center" }),
  });
}

// Open the find/replace panel (same one Ctrl+F triggers) — lets the Rust side
// surface search from a visible button, not just the keyboard shortcut.
function openSearch(id) {
  const view = registry.get(id);
  if (!view) return;
  view.focus();
  openSearchPanel(view);
}

function setMinimap(id, on) {
  const view = registry.get(id);
  if (!view) return;
  view.dispatch({ effects: minimapComp.reconfigure(on ? minimapExt() : []) });
}

function setWrap(id, on) {
  const view = registry.get(id);
  if (!view) return;
  view.dispatch({ effects: wrapComp.reconfigure(on ? EditorView.lineWrapping : []) });
}

function destroy(id) {
  const view = registry.get(id);
  if (view) {
    view.destroy();
    registry.delete(id);
  }
}

window.morCM = { mount, setValue, reveal, openSearch, setMinimap, setWrap, destroy };
