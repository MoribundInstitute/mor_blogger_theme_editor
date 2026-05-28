use super::super::ThemeConfig;

pub(crate) fn generate_gtk_preset_css(_config: &ThemeConfig, source_name: &str) -> String {
    format!(
        r#"/* ============================================================
   GTK-derived preset: {source_name}
   ============================================================ */

/* 1. Global Reset & Fonts */
html, body {{
  background-color: var(--bg-base) !important;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol" !important;
}}

* {{
  text-shadow: none !important; /* Strip the Mor terminal glows */
}}

.canvas-core {{
  background: var(--bg-base) !important;
}}

/* 2. GTK4 Headerbar */
.gtk-headerbar {{
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 46px; /* GNOME standard height */
  padding: 0 8px;
  background: var(--bg-panel) !important;
  border-bottom: 1px solid var(--border-color) !important;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.05);
}}

.headerbar-start, .headerbar-end {{
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 150px;
}}

.headerbar-end {{
  justify-content: flex-end;
}}

.gtk-window-title {{
  font-weight: 600;
  font-size: 0.95rem;
  color: var(--fg-base) !important;
}}

.headerbar-btn {{
  background: transparent !important;
  border: none !important;
  border-radius: 6px !important;
  width: 34px !important;
  height: 34px !important;
  color: var(--fg-base) !important;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
}}

.headerbar-btn:hover {{
  background: color-mix(in srgb, var(--fg-base) 10%, transparent) !important;
}}

.gtk-search-input {{
  background: var(--bg-elevated) !important;
  color: var(--fg-base) !important;
  border: 1px solid var(--border-color) !important;
  border-radius: 6px !important;
  padding: 4px 10px !important;
  height: 34px;
  width: 200px;
  font-size: 0.85rem;
  transition: all 0.2s;
}}

.gtk-search-input:focus {{
  border-color: var(--accent) !important;
  box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 30%, transparent) !important;
  outline: none;
}}

/* 3. GTK Sidebars & Docks */
.runelite-panel {{
  background: var(--bg-panel) !important;
  border-color: var(--border-color) !important;
}}

.panel-header {{
  background: transparent !important;
  border-bottom: 1px solid var(--border-color) !important;
}}

.widget-title {{
  color: var(--fg-muted) !important;
  font-size: 0.8rem !important;
  font-weight: 700 !important;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: none !important;
}}

/* 4. Post Cards (Libadwaita Style) */
.mor-post {{
  background: var(--bg-panel) !important;
  border: 1px solid var(--border-color) !important;
  border-radius: 12px !important;
  padding: 24px !important;
  margin-bottom: 24px !important;
  box-shadow: 0 2px 8px rgba(0,0,0,0.05);
}}

.post-title a {{
  color: var(--fg-base) !important;
  text-decoration: none !important;
  border: none !important;
}}

.post-title a:hover {{
  color: var(--accent) !important;
}}

.post-body a {{
  color: var(--accent) !important;
}}
"#
    )
}