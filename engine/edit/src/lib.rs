//! # Wolia Edit
//!
//! Editing operations for the Wolia platform.
//!
//! This crate provides:
//! - Cursor and selection management
//! - Text editing operations
//! - Undo/redo history
//! - IME (Input Method Editor) support
//! - Clipboard integration

#![allow(dead_code, unused_imports, unused_variables)]

pub mod clipboard;
pub mod cursor;
pub mod editor;
pub mod format;
pub mod history;
pub mod ime;
pub mod input;
pub mod operation;
pub mod paragraph;

pub use cursor::{Cursor, Selection};
pub use editor::Editor;
pub use history::{History, UndoGroup};
pub use input::{InputHandler, Key, KeyModifiers, KeyboardEvent, MouseEvent};
pub use operation::Operation;

/// Result type for edit operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during editing.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid cursor position: {0}")]
    InvalidPosition(usize),

    #[error("Invalid selection")]
    InvalidSelection,

    #[error("Nothing to undo")]
    NothingToUndo,

    #[error("Nothing to redo")]
    NothingToRedo,

    #[error("Clipboard error: {0}")]
    Clipboard(String),
}

/// Edit session managing document state and editing.
pub struct EditSession {
    /// Current cursor state.
    pub cursor: Cursor,
    /// Edit history.
    pub history: History,
    /// IME composition state.
    pub ime: ime::ImeState,
}

impl EditSession {
    /// Create a new edit session.
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(),
            history: History::new(),
            ime: ime::ImeState::new(),
        }
    }

    /// Execute an operation.
    pub fn execute(&mut self, op: Operation) -> Result<()> {
        self.history.push(op);
        Ok(())
    }

    /// Undo the last operation.
    pub fn undo(&mut self) -> Result<()> {
        self.history.undo().ok_or(Error::NothingToUndo)?;
        Ok(())
    }

    /// Redo the last undone operation.
    pub fn redo(&mut self) -> Result<()> {
        self.history.redo().ok_or(Error::NothingToRedo)?;
        Ok(())
    }
}

impl Default for EditSession {
    fn default() -> Self {
        Self::new()
    }
}
