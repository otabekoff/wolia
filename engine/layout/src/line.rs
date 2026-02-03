//! Line layout.

use wolia_math::Rect;

/// A laid-out line of text.
#[derive(Debug, Clone)]
pub struct Line {
    /// Bounding box of the line.
    pub bounds: Rect,
    /// Baseline y-coordinate relative to line top.
    pub baseline: f32,
    /// Fragments in this line.
    pub fragments: Vec<LineFragment>,
}

impl Line {
    /// Create a new line.
    pub fn new(bounds: Rect, baseline: f32) -> Self {
        Self {
            bounds,
            baseline,
            fragments: Vec::new(),
        }
    }
}

/// A fragment within a line (a run of text with uniform style).
#[derive(Debug, Clone)]
pub struct LineFragment {
    /// Bounding box of the fragment.
    pub bounds: Rect,
    /// Index into the source text (byte offset).
    pub text_start: usize,
    /// Length in bytes.
    pub text_len: usize,
    /// Glyph positions.
    pub glyphs: Vec<GlyphPosition>,
}

/// A positioned glyph.
#[derive(Debug, Clone, Copy)]
pub struct GlyphPosition {
    /// Glyph ID.
    pub glyph_id: u16,
    /// X offset from line start.
    pub x: f32,
    /// Advance width.
    pub advance: f32,
}
