//! Cursor and selection.

use wolia_math::Point;

/// A cursor position in a document.
#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    /// Position in the document (byte offset or node path).
    pub position: usize,
    /// Selection anchor (if different from position).
    pub anchor: Option<usize>,
    /// Preferred x-coordinate for vertical movement.
    pub preferred_x: Option<f32>,
}

impl Cursor {
    /// Create a new cursor at position 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a cursor at a specific position.
    pub fn at(position: usize) -> Self {
        Self {
            position,
            anchor: None,
            preferred_x: None,
        }
    }

    /// Check if there's an active selection.
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some_and(|a| a != self.position)
    }

    /// Get the selection range.
    pub fn selection(&self) -> Option<Selection> {
        self.anchor.map(|anchor| {
            let start = anchor.min(self.position);
            let end = anchor.max(self.position);
            Selection { start, end }
        })
    }

    /// Clear any selection.
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Set selection from current position.
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.position);
    }

    /// Move cursor to position.
    pub fn move_to(&mut self, position: usize, extend: bool) {
        if extend && self.anchor.is_none() {
            self.anchor = Some(self.position);
        } else if !extend {
            self.anchor = None;
        }
        self.position = position;
    }
}

/// A text selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Start position (inclusive).
    pub start: usize,
    /// End position (exclusive).
    pub end: usize,
}

impl Selection {
    /// Create a new selection.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start.min(end),
            end: start.max(end),
        }
    }

    /// Get the length of the selection.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the selection is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Check if a position is within the selection.
    pub fn contains(&self, position: usize) -> bool {
        position >= self.start && position < self.end
    }
}

/// Multiple cursors for multi-cursor editing.
#[derive(Debug, Clone, Default)]
pub struct MultiCursor {
    /// Primary cursor.
    pub primary: Cursor,
    /// Additional cursors.
    pub secondary: Vec<Cursor>,
}

impl MultiCursor {
    /// Create with a single cursor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a secondary cursor.
    pub fn add_cursor(&mut self, cursor: Cursor) {
        self.secondary.push(cursor);
    }

    /// Get all cursors.
    pub fn all(&self) -> impl Iterator<Item = &Cursor> {
        std::iter::once(&self.primary).chain(self.secondary.iter())
    }

    /// Get all cursors mutably.
    pub fn all_mut(&mut self) -> impl Iterator<Item = &mut Cursor> {
        std::iter::once(&mut self.primary).chain(self.secondary.iter_mut())
    }
}
