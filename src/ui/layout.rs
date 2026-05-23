//! Workbench layout state for the Blogger Theme Architect editor.
//!
//! These layout enums let the app track whether the control panel is docked
//! normally, widened, floating above the preview, or hidden while the preview
//! takes over the workspace. They also track the responsive preview width
//! preset used by the preview canvas.

use dioxus::prelude::{Signal, Writable};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkbenchLayout {
    /// Normal split view: editor controls on the left, preview/export on the right.
    Split,

    /// Wider docked editor panel for dense forms and longer fields.
    WideEditor,

    /// Editor panel floats over the preview instead of consuming layout width.
    FloatingEditor,

    /// Preview gets the workspace; editor controls are hidden until reopened.
    PreviewTakeover,
}

impl WorkbenchLayout {
    /// CSS class suffix for the current workbench layout.
    pub fn as_class(self) -> &'static str {
        match self {
            Self::Split => "editor-layout-split",
            Self::WideEditor => "editor-layout-wide",
            Self::FloatingEditor => "editor-layout-floating",
            Self::PreviewTakeover => "editor-layout-preview-takeover",
        }
    }

    /// Whether the editor panel should be rendered in the main workspace.
    pub fn is_editor_visible(self) -> bool {
        !matches!(self, Self::PreviewTakeover)
    }

    /// Stable value used for localStorage persistence.
    pub fn storage_value(self) -> &'static str {
        match self {
            Self::Split => "split",
            Self::WideEditor => "wide",
            Self::FloatingEditor => "floating",
            Self::PreviewTakeover => "preview",
        }
    }

    /// Restore a persisted layout value from localStorage.
    pub fn from_storage_value(value: &str) -> Option<Self> {
        match value {
            "split" => Some(Self::Split),
            "wide" => Some(Self::WideEditor),
            "floating" => Some(Self::FloatingEditor),
            "preview" => Some(Self::PreviewTakeover),
            _ => None,
        }
    }
}

pub const WORKBENCH_LAYOUT_STORAGE_KEY: &str = "mor_blogger_theme_editor.workbench_layout";
pub const FLOATING_EDITOR_POSITION_STORAGE_KEY: &str = "mor_blogger_theme_editor.floating_editor_position";

/// Set the workbench layout from buttons/shortcuts.
///
/// Persistence is handled in `app.rs` by watching this signal and writing the
/// selected value to localStorage.
pub fn set_workbench_layout(mut signal: Signal<WorkbenchLayout>, layout: WorkbenchLayout) {
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

pub fn apply_preview_viewport(
    viewport: PreviewViewport,
    mut preview_width: Signal<u32>,
) {
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