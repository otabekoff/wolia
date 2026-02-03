//! Page layout.

use wolia_math::{Rect, Size};

use crate::LayoutNode;

/// A laid-out page.
#[derive(Debug, Clone)]
pub struct Page {
    /// Page number (1-indexed).
    pub number: usize,
    /// Page size.
    pub size: Size,
    /// Content area.
    pub content_rect: Rect,
    /// Layout nodes on this page.
    pub nodes: Vec<LayoutNode>,
}

impl Page {
    /// Create a new page.
    pub fn new(number: usize, size: Size, content_rect: Rect) -> Self {
        Self {
            number,
            size,
            content_rect,
            nodes: Vec::new(),
        }
    }
}

/// Page layout configuration.
#[derive(Debug, Clone)]
pub struct PageLayout {
    /// Page size.
    pub size: Size,
    /// Content margins.
    pub margins: crate::Margins,
    /// Header height.
    pub header_height: f32,
    /// Footer height.
    pub footer_height: f32,
}

impl PageLayout {
    /// Create a new page layout.
    pub fn new(size: Size) -> Self {
        Self {
            size,
            margins: crate::Margins::default(),
            header_height: 0.0,
            footer_height: 0.0,
        }
    }

    /// A4 page layout.
    pub fn a4() -> Self {
        Self::new(Size::new(595.0, 842.0))
    }

    /// US Letter page layout.
    pub fn letter() -> Self {
        Self::new(Size::new(612.0, 792.0))
    }

    /// Get the main content area.
    pub fn content_rect(&self) -> Rect {
        let margins = self.margins;
        Rect::new(
            margins.left,
            margins.top + self.header_height,
            self.size.width - margins.left - margins.right,
            self.size.height
                - margins.top
                - margins.bottom
                - self.header_height
                - self.footer_height,
        )
    }
}

impl Default for PageLayout {
    fn default() -> Self {
        Self::a4()
    }
}
