//! Slide editor view.

use wolia_math::Rect;

/// Slide editor.
pub struct SlideEditor {
    /// Viewport.
    pub viewport: Rect,
    /// Zoom level.
    pub zoom: f32,
    /// Show grid.
    pub show_grid: bool,
    /// Snap to grid.
    pub snap_to_grid: bool,
}

impl SlideEditor {
    pub fn new() -> Self {
        Self {
            viewport: Rect::ZERO,
            zoom: 1.0,
            show_grid: false,
            snap_to_grid: true,
        }
    }
}

impl Default for SlideEditor {
    fn default() -> Self {
        Self::new()
    }
}
