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

// ---------------------------------------------------------------------------
// Blogger Layouts v3 schema — drives lang-xml's native tag/attribute
// completion so the XML editor understands b: markup, not just generic XML.
// Curated, not exhaustive: the tags and attributes theme authors actually use.
const WIDGET_TYPES = [
  "Blog", "HTML", "Text", "Image", "Label", "PageList", "LinkList",
  "BlogSearch", "Header", "Profile", "Attribution", "PopularPosts",
  "BlogArchive", "FeaturedPost", "ContactForm", "Subscribe", "AdSense",
  "Stats", "Translate", "ReportAbuse",
];
const BLOGGER_ELEMENTS = [
  { name: "b:section", attributes: [{ name: "id" }, { name: "class" }, { name: "maxwidgets" }, { name: "showaddelement", values: ["yes", "no"] }, { name: "growth", values: ["horizontal", "vertical"] }], children: ["b:widget"] },
  { name: "b:widget", attributes: [{ name: "id" }, { name: "type", values: WIDGET_TYPES }, { name: "version", values: ["1", "2"] }, { name: "locked", values: ["true", "false"] }, { name: "visible", values: ["true", "false"] }, { name: "mobile", values: ["yes", "no", "only"] }, { name: "title" }] },
  { name: "b:includable", attributes: [{ name: "id" }, { name: "var" }] },
  { name: "b:include", attributes: [{ name: "name" }, { name: "data" }] },
  { name: "b:if", attributes: [{ name: "cond" }] },
  { name: "b:elseif", attributes: [{ name: "cond" }] },
  { name: "b:else", attributes: [] },
  { name: "b:loop", attributes: [{ name: "var" }, { name: "values" }, { name: "index" }, { name: "reverse", values: ["true", "false"] }] },
  { name: "b:switch", attributes: [{ name: "var" }] },
  { name: "b:case", attributes: [{ name: "value" }] },
  { name: "b:default", attributes: [] },
  { name: "b:eval", attributes: [{ name: "expr" }] },
  { name: "b:with", attributes: [{ name: "var" }, { name: "value" }] },
  { name: "b:tag", attributes: [{ name: "name" }, { name: "cond" }] },
  { name: "b:class", attributes: [{ name: "name" }, { name: "cond" }, { name: "expr:name" }] },
  { name: "b:attr", attributes: [{ name: "name" }, { name: "value" }, { name: "cond" }, { name: "expr:value" }] },
  { name: "b:comment", attributes: [] },
  { name: "b:skin", attributes: [] },
  { name: "b:template-skin", attributes: [] },
];
// Common on any HTML tag inside a Blogger template.
const BLOGGER_GLOBAL_ATTRS = [
  { name: "cond", global: true },
  ...["href", "src", "class", "id", "title", "alt", "value", "name", "content"]
    .map((n) => ({ name: `expr:${n}`, global: true })),
];

// XML-mode lint: Lezer parse errors (mismatched/unclosed tags) plus duplicate
// b:widget ids — Blogger rejects the whole theme upload on a duplicate id.
const xmlBloggerLinter = linter((view) => {
  const diagnostics = [];
  syntaxTree(view.state).cursor().iterate((node) => {
    if (!node.type.isError) return;
    diagnostics.push({
      from: node.from,
      to: Math.min(Math.max(node.to, node.from + 1), view.state.doc.length),
      severity: "error",
      message: "Malformed XML (unclosed or mismatched tag?)",
    });
  });
  const doc = view.state.doc.toString();
  const seen = new Map();
  const re = /<b:widget\b[^>]*?\bid=['"]([^'"]+)['"]/g;
  for (let m; (m = re.exec(doc)); ) {
    if (seen.has(m[1])) {
      diagnostics.push({
        from: m.index,
        to: m.index + m[0].length,
        severity: "error",
        message: `Duplicate widget id '${m[1]}' — Blogger rejects themes with repeated widget ids (first used earlier in this file).`,
      });
    } else {
      seen.set(m[1], m.index);
    }
  }
  return diagnostics;
});

function langExtension(mode) {
  switch (mode) {
    case "xml": return [
      xml({ elements: BLOGGER_ELEMENTS, attributes: BLOGGER_GLOBAL_ATTRS }),
      xmlBloggerLinter,
      lintGutter(),
    ];
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
