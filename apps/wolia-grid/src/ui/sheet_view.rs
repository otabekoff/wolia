//! Sheet view.

use wolia_math::Rect;

/// Sheet view configuration.
pub struct SheetView {
    /// Viewport.
    pub viewport: Rect,
    /// Default column width.
    pub default_col_width: f32,
    /// Default row height.
    pub default_row_height: f32,
    /// Scroll position.
    pub scroll_x: f32,
    pub scroll_y: f32,
    /// Frozen rows.
    pub frozen_rows: usize,
    /// Frozen columns.
    pub frozen_cols: usize,
}

impl SheetView {
    pub fn new() -> Self {
        Self {
            viewport: Rect::ZERO,
            default_col_width: 100.0,
            default_row_height: 24.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            frozen_rows: 0,
            frozen_cols: 0,
        }
    }
}

impl Default for SheetView {
    fn default() -> Self {
        Self::new()
    }
}
