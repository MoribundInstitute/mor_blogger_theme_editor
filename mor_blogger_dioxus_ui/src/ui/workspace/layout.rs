//! Panel and preview layout state for the Blogger Theme Architect editor.
//!
//! `PanelLayout` controls how the editor panels behave independently from the
//! preview viewport. The preview enums below track the responsive preview width
//! preset and template mode used by the preview canvas.
//!
//! This module now contains only shared state types and helper functions for
//! the editor's layout system. The top-level shell composition lives in
//! `src/app.rs`, which owns the signals and renders the menu bar and panels
//! directly.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PanelLayout {
    Split,
    Wide,
    Floating,
    Hidden,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewViewport {
    Desktop,
    Laptop,
    Tablet,
    Phone,
    Fit,
    Custom,
}

impl PreviewViewport {
    pub fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Laptop => "Laptop",
            Self::Tablet => "Tablet",
            Self::Phone => "Phone",
            Self::Fit => "Fit",
            Self::Custom => "Custom",
        }
    }

    /// Returns the preview frame width for this preset.
    ///
    /// The editor does not use CSS transform scaling for the preview. The
    /// iframe renders naturally at this width, which keeps the preview readable.
    pub fn width(self) -> Option<u32> {
        match self {
            Self::Desktop => Some(1200),
            Self::Laptop => Some(1024),
            Self::Tablet => Some(768),
            Self::Phone => Some(390),
            // Fit is handled by CSS in the preview frame. The stored width is
            // still useful as a fallback if the user switches away from Fit.
            Self::Fit => Some(1200),
            Self::Custom => None,
        }
    }

    pub fn is_rotatable(self) -> bool {
        matches!(self, Self::Tablet | Self::Phone | Self::Custom)
    }
}

/// Clamp arbitrary custom widths to a sane website-preview range.
pub fn clamp_preview_width(width: u32) -> u32 {
    width.clamp(240, 2400)
}

pub fn apply_preview_viewport(viewport: PreviewViewport, mut preview_width: Signal<u32>) {
    if let Some(width) = viewport.width() {
        preview_width.set(width);
    }
}

// `PreviewTemplateMode` is owned by the core crate (mor_blogger_core::render::preview)
// and re-exported through `mor_blogger_core::render`. We re-export it from here as well
// so existing `crate::ui::workspace::layout::PreviewTemplateMode` references keep
// resolving — against the single canonical type the headless renderer expects, not a
// UI-local duplicate. The `label()` method now lives on the core type.
pub use mor_blogger_core::render::PreviewTemplateMode;

pub fn rotate_preview_width(viewport: PreviewViewport, current_width: u32) -> u32 {
    match viewport {
        PreviewViewport::Tablet => {
            if current_width <= 900 {
                1024
            } else {
                768
            }
        }
        PreviewViewport::Phone => {
            if current_width <= 600 {
                844
            } else {
                390
            }
        }
        PreviewViewport::Custom => {
            if current_width <= 700 {
                900
            } else {
                390
            }
        }
        _ => current_width,
    }
}
