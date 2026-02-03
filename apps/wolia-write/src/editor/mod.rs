//! Text editor component.

use wolia_core::text::Text;
use wolia_edit::{Cursor, Selection};
use wolia_layout::ParagraphLayout;
use wolia_math::Rect;

/// The document editor view.
pub struct Editor {
    /// Viewport rectangle.
    pub viewport: Rect,
    /// Scroll offset.
    pub scroll_y: f32,
    /// Zoom level (1.0 = 100%).
    pub zoom: f32,
    /// Show page boundaries.
    pub show_pages: bool,
    /// Show ruler.
    pub show_ruler: bool,
}

impl Editor {
    /// Create a new editor.
    pub fn new() -> Self {
        Self {
            viewport: Rect::ZERO,
            scroll_y: 0.0,
            zoom: 1.0,
            show_pages: true,
            show_ruler: true,
        }
    }

    /// Set the viewport size.
    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    /// Scroll by a delta.
    pub fn scroll(&mut self, delta: f32) {
        self.scroll_y = (self.scroll_y + delta).max(0.0);
    }

    /// Set zoom level.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.25, 4.0);
    }

    /// Zoom in.
    pub fn zoom_in(&mut self) {
        self.set_zoom(self.zoom * 1.1);
    }

    /// Zoom out.
    pub fn zoom_out(&mut self) {
        self.set_zoom(self.zoom / 1.1);
    }

    /// Reset zoom to 100%.
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
