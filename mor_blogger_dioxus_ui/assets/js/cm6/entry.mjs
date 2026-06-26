// CodeMirror 6 bundle for the MorBlogger code editors.
//
// Exposes a tiny imperative API on `window.morCM` so the Dioxus `CodeEditor`
// component can mount/update editors without knowing anything about CM internals.
// Built to ../cm6.bundle.js (IIFE) via `npm run build` and injected once in shell.rs.

import { EditorView, basicSetup } from "codemirror";
import { EditorState, Annotation, Compartment } from "@codemirror/state";
import { keymap } from "@codemirror/view";
import { indentWithTab } from "@codemirror/commands";
import { StreamLanguage } from "@codemirror/language";
import { oneDark } from "@codemirror/theme-one-dark";

import { showMinimap } from "@replit/codemirror-minimap";

import { html } from "@codemirror/lang-html";
import { xml } from "@codemirror/lang-xml";
import { css } from "@codemirror/lang-css";
import { javascript } from "@codemirror/lang-javascript";
import { json } from "@codemirror/lang-json";
import { toml } from "@codemirror/legacy-modes/mode/toml";

// Marks programmatic (Rust -> editor) edits so the change listener can ignore
// them and avoid an echo loop back into Rust.
const External = Annotation.define();

// Compartment lets the minimap be toggled live (settings change) without
// rebuilding the whole editor / losing cursor + undo history.
const minimapComp = new Compartment();

function minimapExt() {
  return showMinimap.compute([], () => ({
    create: () => ({ dom: document.createElement("div") }),
    displayText: "blocks",
    showOverlay: "always",
  }));
}

function langExtension(mode) {
  switch (mode) {
    case "xml": return xml();
    case "html": return html();
    case "css": return css();
    case "js":
    case "javascript": return javascript();
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
    keymap.of([indentWithTab]),
    langExtension(opts.lang),
    oneDark,
    hostTheme,
    EditorState.readOnly.of(!!opts.readOnly),
    EditorView.editable.of(!opts.readOnly),
  ];
  if (opts.wrap) extensions.push(EditorView.lineWrapping);
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

function setMinimap(id, on) {
  const view = registry.get(id);
  if (!view) return;
  view.dispatch({ effects: minimapComp.reconfigure(on ? minimapExt() : []) });
}

function destroy(id) {
  const view = registry.get(id);
  if (view) {
    view.destroy();
    registry.delete(id);
  }
}

window.morCM = { mount, setValue, reveal, setMinimap, destroy };
