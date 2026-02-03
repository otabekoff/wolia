//! Text representation and spans.

use smallvec::SmallVec;

use crate::style::TextStyle;

/// Rich text content with formatting spans.
#[derive(Debug, Clone, Default)]
pub struct Text {
    /// The raw text content.
    pub content: String,
    /// Formatting spans applied to the text.
    pub spans: SmallVec<[Span; 4]>,
}

impl Text {
    /// Create new text with content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            spans: SmallVec::new(),
        }
    }

    /// Create empty text.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Add a formatting span.
    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Get the length of the text in bytes.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// A formatting span within text.
#[derive(Debug, Clone)]
pub struct Span {
    /// Start offset (byte index).
    pub start: usize,
    /// End offset (byte index, exclusive).
    pub end: usize,
    /// Style applied to this span.
    pub style: TextStyle,
}

impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize, style: TextStyle) -> Self {
        Self { start, end, style }
    }
}
