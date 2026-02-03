//! # Wolia Layout
//!
//! Layout and pagination engine for the Wolia platform.
//!
//! This crate handles:
//! - Text wrapping and line breaking
//! - Paragraph layout
//! - Page layout and pagination
//! - Table layout
//! - Float positioning

pub mod line;
pub mod page;
pub mod paragraph;
pub mod text;
pub mod tree;

use wolia_core::Document;
use wolia_math::{Rect, Size};

pub use line::{Line, LineFragment};
pub use page::{Page, PageLayout};
pub use paragraph::ParagraphLayout;
pub use text::TextLayout;
pub use tree::{LayoutNode, LayoutTree};

/// Result type for layout operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during layout.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Layout overflow: content exceeds available space")]
    Overflow,

    #[error("Invalid constraint: {0}")]
    InvalidConstraint(String),

    #[error("Missing font: {0}")]
    MissingFont(String),
}

/// Layout constraints for a region.
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    /// Minimum size.
    pub min: Size,
    /// Maximum size.
    pub max: Size,
}

impl Constraints {
    /// Create new constraints.
    pub fn new(min: Size, max: Size) -> Self {
        Self { min, max }
    }

    /// Tight constraints (exact size).
    pub fn tight(size: Size) -> Self {
        Self {
            min: size,
            max: size,
        }
    }

    /// Loose constraints (any size up to max).
    pub fn loose(max: Size) -> Self {
        Self {
            min: Size::ZERO,
            max,
        }
    }

    /// Unbounded constraints.
    pub fn unbounded() -> Self {
        Self {
            min: Size::ZERO,
            max: Size::new(f32::INFINITY, f32::INFINITY),
        }
    }
}

/// The main layout engine.
pub struct LayoutEngine {
    /// Default page size.
    pub page_size: Size,
    /// Page margins.
    pub margins: Margins,
}

impl LayoutEngine {
    /// Create a new layout engine with A4 page size.
    pub fn new() -> Self {
        Self {
            page_size: Size::new(595.0, 842.0), // A4 in points
            margins: Margins::default(),
        }
    }

    /// Layout a document.
    pub fn layout(&self, _document: &Document) -> Result<LayoutTree> {
        // TODO: Implement full document layout
        Ok(LayoutTree::new(self.page_size))
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Page margins.
#[derive(Debug, Clone, Copy)]
pub struct Margins {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Margins {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn uniform(margin: f32) -> Self {
        Self::new(margin, margin, margin, margin)
    }

    /// Get the content rect for a page of the given size.
    pub fn content_rect(&self, page_size: Size) -> Rect {
        Rect::new(
            self.left,
            self.top,
            page_size.width - self.left - self.right,
            page_size.height - self.top - self.bottom,
        )
    }
}

impl Default for Margins {
    fn default() -> Self {
        Self::uniform(72.0) // 1 inch margins
    }
}
