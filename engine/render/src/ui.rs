//! UI rendering primitives.

use wolia_math::{Color, Rect};

/// A simple rectangle to render.
#[derive(Debug, Clone, Copy)]
pub struct RenderRect {
    /// The rectangle bounds.
    pub rect: Rect,
    /// The fill color.
    pub color: Color,
    /// Corner radius (0 for sharp corners).
    pub corner_radius: f32,
}

impl RenderRect {
    /// Create a new render rectangle.
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            color,
            corner_radius: 0.0,
        }
    }

    /// Set corner radius.
    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}

/// UI colors for the Wolia apps.
pub mod colors {
    use wolia_math::Color;

    // Toolbar colors
    pub const TOOLBAR_BG: Color = Color::rgba(0.96, 0.96, 0.96, 1.0);
    pub const TOOLBAR_BORDER: Color = Color::rgba(0.85, 0.85, 0.85, 1.0);

    // Document area colors
    pub const DOCUMENT_BG: Color = Color::rgba(0.88, 0.88, 0.88, 1.0);
    pub const PAPER_BG: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    pub const PAPER_SHADOW: Color = Color::rgba(0.0, 0.0, 0.0, 0.15);

    // Sidebar colors
    pub const SIDEBAR_BG: Color = Color::rgba(0.95, 0.95, 0.95, 1.0);
    pub const SIDEBAR_BORDER: Color = Color::rgba(0.85, 0.85, 0.85, 1.0);

    // Spreadsheet colors
    pub const CELL_BG: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    pub const CELL_BORDER: Color = Color::rgba(0.82, 0.82, 0.82, 1.0);
    pub const CELL_SELECTED: Color = Color::rgba(0.26, 0.52, 0.96, 0.2);
    pub const HEADER_BG: Color = Color::rgba(0.95, 0.95, 0.95, 1.0);

    // Presentation colors
    pub const SLIDE_BG: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    pub const SLIDE_PANEL_BG: Color = Color::rgba(0.12, 0.12, 0.14, 1.0);
    pub const SLIDE_THUMBNAIL_BG: Color = Color::rgba(0.2, 0.2, 0.22, 1.0);
    pub const CANVAS_BG: Color = Color::rgba(0.15, 0.15, 0.18, 1.0);

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::rgba(0.13, 0.13, 0.13, 1.0);
    pub const TEXT_SECONDARY: Color = Color::rgba(0.45, 0.45, 0.45, 1.0);
    pub const TEXT_LIGHT: Color = Color::rgba(0.95, 0.95, 0.95, 1.0);

    // Accent colors
    pub const ACCENT: Color = Color::rgba(0.26, 0.52, 0.96, 1.0);
    pub const ACCENT_HOVER: Color = Color::rgba(0.20, 0.45, 0.90, 1.0);
}

/// Standard UI dimensions.
pub mod dimensions {
    /// Standard toolbar height.
    pub const TOOLBAR_HEIGHT: f32 = 48.0;
    /// Standard sidebar width.
    pub const SIDEBAR_WIDTH: f32 = 250.0;
    /// Standard status bar height.
    pub const STATUS_BAR_HEIGHT: f32 = 24.0;
    /// Standard padding.
    pub const PADDING: f32 = 8.0;
    /// Standard margin.
    pub const MARGIN: f32 = 16.0;
    /// Standard border radius.
    pub const BORDER_RADIUS: f32 = 4.0;
    /// Standard icon size.
    pub const ICON_SIZE: f32 = 24.0;
    /// Standard button height.
    pub const BUTTON_HEIGHT: f32 = 32.0;
}
