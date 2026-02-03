//! Paragraph layout.

use wolia_core::text::Text;
use wolia_math::Rect;

use crate::Constraints;
use crate::line::Line;

/// A laid-out paragraph.
#[derive(Debug, Clone)]
pub struct ParagraphLayout {
    /// Bounding box.
    pub bounds: Rect,
    /// Lines in this paragraph.
    pub lines: Vec<Line>,
}

impl ParagraphLayout {
    /// Create a new paragraph layout.
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            lines: Vec::new(),
        }
    }

    /// Layout text into a paragraph.
    pub fn layout(_text: &Text, _constraints: Constraints) -> Self {
        // TODO: Implement text shaping and line breaking
        Self::new(Rect::ZERO)
    }

    /// Get the total height.
    pub fn height(&self) -> f32 {
        self.bounds.height
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}
