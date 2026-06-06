use super::super::ThemeConfig;

pub(crate) fn generate_gtk_preset_css(_config: &ThemeConfig, source_name: &str) -> String {
    format!(
        r#"/* ============================================================
   GTK-derived preset: {source_name}
   This is not raw GTK CSS. It is a Blogger-safe GTK translation layer.
   ============================================================ */

html, body {{
  background:
    radial-gradient(circle at top left, color-mix(in srgb, var(--accent) 14%, transparent), transparent 34rem),
    linear-gradient(180deg, var(--bg-panel), var(--bg-base)) !important;
  color: var(--fg-base) !important;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif !important;
}}

* {{
  text-shadow: none !important;
}}

.canvas-core {{
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--bg-elevated) 70%, transparent), var(--bg-base)) !important;
}}

.gtk-headerbar {{
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 46px;
  padding: 0 8px;
  background:
    linear-gradient(180deg, var(--bg-elevated), var(--bg-panel)) !important;
  border-bottom: 1px solid var(--border-color) !important;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.10),
    0 1px 2px rgba(0, 0, 0, 0.35);
}}

.headerbar-start,
.headerbar-end {{
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
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--fg-base) 9%, transparent), transparent) !important;
  border: 1px solid color-mix(in srgb, var(--border-color) 80%, transparent) !important;
  border-radius: 8px !important;
  width: 34px !important;
  height: 34px !important;
  color: var(--fg-base) !important;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s, transform 0.12s, border-color 0.2s;
  cursor: pointer;
}}

.headerbar-btn:hover {{
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--accent) 24%, transparent), color-mix(in srgb, var(--fg-base) 8%, transparent)) !important;
  border-color: color-mix(in srgb, var(--accent) 55%, var(--border-color)) !important;
  transform: translateY(-1px);
}}

.gtk-mask-icon {{
  display: inline-block;
  width: 16px;
  height: 16px;
  background-color: currentColor;
  -webkit-mask-size: contain;
  mask-size: contain;
  -webkit-mask-repeat: no-repeat;
  mask-repeat: no-repeat;
  -webkit-mask-position: center;
  mask-position: center;
}}

.gtk-icon-close {{
  -webkit-mask-image: var(--icon-panel-close);
  mask-image: var(--icon-panel-close);
}}

.gtk-icon-search {{
  -webkit-mask-image: var(--icon-search);
  mask-image: var(--icon-search);
}}

.gtk-icon-menu {{
  -webkit-mask-image: var(--icon-menu);
  mask-image: var(--icon-menu);
}}

.gtk-icon-sidebar-left {{
  -webkit-mask-image: var(--icon-sidebar-left);
  mask-image: var(--icon-sidebar-left);
}}

.gtk-icon-sidebar-right {{
  -webkit-mask-image: var(--icon-sidebar-right);
  mask-image: var(--icon-sidebar-right);
}}

.gtk-search-input {{
  background:
    linear-gradient(180deg, var(--bg-base), var(--bg-elevated)) !important;
  color: var(--fg-base) !important;
  border: 1px solid var(--border-color) !important;
  border-radius: 8px !important;
  padding: 4px 10px !important;
  height: 34px;
  width: 200px;
  font-size: 0.85rem;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.20);
  transition: all 0.2s;
}}

.gtk-search-input:focus {{
  border-color: var(--accent) !important;
  box-shadow:
    inset 0 1px 2px rgba(0, 0, 0, 0.20),
    0 0 0 2px color-mix(in srgb, var(--accent) 30%, transparent) !important;
  outline: none;
}}

.runelite-panel {{
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--bg-elevated) 88%, transparent), var(--bg-panel)) !important;
  border-color: var(--border-color) !important;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.06),
    0 12px 30px rgba(0, 0, 0, 0.25);
}}

.panel-header {{
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--fg-base) 6%, transparent), transparent) !important;
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

.mor-post {{
  background:
    linear-gradient(180deg, var(--bg-panel), var(--bg-elevated)) !important;
  border: 1px solid var(--border-color) !important;
  border-radius: 12px !important;
  padding: 24px !important;
  margin-bottom: 24px !important;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.06),
    0 2px 8px rgba(0, 0, 0, 0.18);
}}

.post-title a {{
  color: var(--fg-base) !important;
  text-decoration: none !important;
  border: none !important;
}}

.post-title a:hover,
.post-body a {{
  color: var(--accent) !important;
}}
"#
    )
}
