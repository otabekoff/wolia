//! Document editor with cursor, selection, and undo/redo support.

use wolia_core::{Document, Text};

use crate::cursor::{Cursor, Selection};
use crate::history::History;
use crate::input::{InputHandler, Key, KeyModifiers, KeyboardEvent};
use crate::operation::Operation;

/// A document editor that manages editing state and operations.
#[derive(Debug)]
pub struct Editor {
    /// The document being edited.
    pub document: Document,
    /// Cursor position.
    pub cursor: Cursor,
    /// Selection state.
    pub selection: Option<Selection>,
    /// Operation history for undo/redo.
    pub history: History,
    /// Input handler for keyboard/mouse events.
    pub input: InputHandler,
    /// Whether the document has unsaved changes.
    pub dirty: bool,
}

impl Editor {
    /// Create a new editor with a new document.
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: Cursor::new(),
            selection: None,
            history: History::new(),
            input: InputHandler::new(),
            dirty: false,
        }
    }

    /// Create an editor with an existing document.
    pub fn with_document(document: Document) -> Self {
        Self {
            document,
            cursor: Cursor::new(),
            selection: None,
            history: History::new(),
            input: InputHandler::new(),
            dirty: false,
        }
    }

    /// Insert text at the current cursor position.
    pub fn insert_text(&mut self, text: &str) -> crate::Result<()> {
        let position = self.cursor.position;

        let operation = Operation::InsertText {
            position,
            text: text.to_string(),
        };

        self.apply_operation(operation)?;
        self.cursor.position += text.len();
        self.dirty = true;

        Ok(())
    }

    /// Delete the character at the current cursor position.
    pub fn delete_char(&mut self) -> crate::Result<()> {
        let position = self.cursor.position;

        if position > 0 {
            let operation = Operation::DeleteText {
                start: position - 1,
                end: position,
                deleted: String::new(), // In a real implementation, get from document
            };

            self.apply_operation(operation)?;
            self.cursor.position = self.cursor.position.saturating_sub(1);
            self.dirty = true;
        }

        Ok(())
    }

    /// Delete the character after the current cursor position.
    pub fn delete_char_forward(&mut self) -> crate::Result<()> {
        let position = self.cursor.position;

        let operation = Operation::DeleteText {
            start: position,
            end: position + 1,
            deleted: String::new(), // In a real implementation, get from document
        };

        self.apply_operation(operation)?;
        self.dirty = true;

        Ok(())
    }

    /// Move cursor to the beginning of the line.
    pub fn cursor_line_start(&mut self) {
        // This would need document context to find actual line start
        // For now, just record the movement
        self.cursor.position = 0;
    }

    /// Move cursor to the end of the line.
    pub fn cursor_line_end(&mut self) {
        // This would need document context to find actual line end
        // For now, record the movement
        self.cursor.position = 1000;
    }

    /// Move cursor up by one line.
    pub fn cursor_up(&mut self) {
        // This requires layout information
    }

    /// Move cursor down by one line.
    pub fn cursor_down(&mut self) {
        // This requires layout information
    }

    /// Move cursor left by one character.
    pub fn cursor_left(&mut self) {
        let pos = self.cursor.position;
        if pos > 0 {
            self.cursor.position = pos - 1;
        }
    }

    /// Move cursor right by one character.
    pub fn cursor_right(&mut self) {
        self.cursor.position += 1;
    }

    /// Start a selection from the current cursor position.
    pub fn start_selection(&mut self) {
        self.selection = Some(Selection {
            start: self.cursor.position,
            end: self.cursor.position,
        });
    }

    /// Extend the current selection to the cursor position.
    pub fn extend_selection(&mut self) {
        if let Some(mut sel) = self.selection.take() {
            sel.end = self.cursor.position;
            self.selection = Some(sel);
        }
    }

    /// Clear the current selection.
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Get the selected text.
    pub fn selected_text(&self) -> Option<String> {
        self.selection.as_ref().map(|sel| {
            let start = sel.start.min(sel.end);
            let end = sel.start.max(sel.end);
            // In a real implementation, extract from document
            format!("[selected {} to {}]", start, end)
        })
    }

    /// Apply an operation to the document.
    pub fn apply_operation(&mut self, operation: Operation) -> crate::Result<()> {
        // Apply the operation to the document
        // In a real implementation, this would modify the document tree

        // Add to history
        self.history.push(operation);
        self.dirty = true;

        Ok(())
    }

    /// Undo the last operation.
    pub fn undo(&mut self) -> crate::Result<()> {
        if let Some(_operation) = self.history.undo() {
            // Apply the inverse operation
            // In a real implementation, reconstruct document state
            self.dirty = true;
        }
        Ok(())
    }

    /// Redo the last undone operation.
    pub fn redo(&mut self) -> crate::Result<()> {
        if let Some(_operation) = self.history.redo() {
            // Apply the operation
            // In a real implementation, reconstruct document state
            self.dirty = true;
        }
        Ok(())
    }

    /// Handle a keyboard event.
    pub fn handle_keyboard_event(&mut self, event: KeyboardEvent) -> crate::Result<()> {
        self.input.handle_keyboard(&event);

        match event.key {
            Key::ArrowLeft => {
                if event.modifiers.shift {
                    if self.selection.is_none() {
                        self.start_selection();
                    }
                } else {
                    self.clear_selection();
                }
                self.cursor_left();
                if event.modifiers.shift {
                    self.extend_selection();
                }
            }
            Key::ArrowRight => {
                if event.modifiers.shift {
                    if self.selection.is_none() {
                        self.start_selection();
                    }
                } else {
                    self.clear_selection();
                }
                self.cursor_right();
                if event.modifiers.shift {
                    self.extend_selection();
                }
            }
            Key::Home => self.cursor_line_start(),
            Key::End => self.cursor_line_end(),
            Key::Backspace if event.pressed => {
                self.delete_char()?;
            }
            Key::Delete if event.pressed => {
                self.delete_char_forward()?;
            }
            _ => {}
        }

        // Handle character input
        if event.pressed {
            if let Some(c) = event.char_code {
                if !event.modifiers.control && !event.modifiers.alt {
                    self.insert_text(&c.to_string())?;
                }
            }
        }

        Ok(())
    }

    /// Check if document has unsaved changes.
    pub fn has_unsaved_changes(&self) -> bool {
        self.dirty
    }

    /// Mark the document as saved.
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_creation() {
        let editor = Editor::new();
        assert!(!editor.has_unsaved_changes());
        assert_eq!(editor.cursor.position, 0);
        assert!(editor.selection.is_none());
    }

    #[test]
    fn test_cursor_movement() {
        let mut editor = Editor::new();
        editor.cursor_right();
        assert_eq!(editor.cursor.position, 1);

        editor.cursor_left();
        assert_eq!(editor.cursor.position, 0);
    }

    #[test]
    fn test_selection() {
        let mut editor = Editor::new();
        editor.start_selection();
        assert!(editor.selection.is_some());

        editor.cursor_right();
        editor.extend_selection();
        assert!(editor.selected_text().is_some());

        editor.clear_selection();
        assert!(editor.selection.is_none());
    }
}
