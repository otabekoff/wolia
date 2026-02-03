//! Layout tree.

use uuid::Uuid;
use wolia_math::{Rect, Size};

use crate::page::Page;

/// The result of laying out a document.
#[derive(Debug)]
pub struct LayoutTree {
    /// Pages in the document.
    pub pages: Vec<Page>,
    /// Total content height (for scrolling).
    pub total_height: f32,
}

impl LayoutTree {
    /// Create a new layout tree.
    pub fn new(page_size: Size) -> Self {
        Self {
            pages: vec![Page::new(1, page_size, Rect::from_size(page_size))],
            total_height: page_size.height,
        }
    }

    /// Get the number of pages.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get a page by number (1-indexed).
    pub fn page(&self, number: usize) -> Option<&Page> {
        self.pages.get(number.saturating_sub(1))
    }
}

/// A node in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    /// Source node ID.
    pub source_id: Uuid,
    /// Bounding box in page coordinates.
    pub bounds: Rect,
    /// Node content.
    pub content: LayoutContent,
}

/// Content of a layout node.
#[derive(Debug, Clone)]
pub enum LayoutContent {
    /// Text paragraph.
    Paragraph(crate::ParagraphLayout),
    /// Image.
    Image { src: String },
    /// Table.
    Table { cells: Vec<LayoutNode> },
    /// Container for other nodes.
    Container { children: Vec<LayoutNode> },
}
