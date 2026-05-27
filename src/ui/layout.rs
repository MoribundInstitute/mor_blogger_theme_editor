//! Panel and preview layout state for the Blogger Theme Architect editor.
//!
//! `PanelLayout` controls how the editor panels behave independently from the
//! preview viewport. The preview enums below track the responsive preview width
//! preset and template mode used by the preview canvas.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PanelLayout {
    Split,
    Wide,
    Floating,
    Hidden,
}

pub fn set_panel_layout(signal: &mut Signal<PanelLayout>, layout: PanelLayout) {
    signal.set(layout);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewTemplateMode {
    Modern,
    Sidebars,
    StaticArchive,
    StaticCategories,
}

impl PreviewTemplateMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Modern => "Modern",
            Self::Sidebars => "Sidebars",
            Self::StaticArchive => "Static Archive",
            Self::StaticCategories => "Static Categories",
        }
    }
}

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
