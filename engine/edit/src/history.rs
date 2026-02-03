//! Undo/redo history.

use crate::Operation;

/// Edit history for undo/redo.
#[derive(Debug)]
pub struct History {
    /// Undo stack.
    undo_stack: Vec<UndoGroup>,
    /// Redo stack.
    redo_stack: Vec<UndoGroup>,
    /// Maximum history size.
    max_size: usize,
    /// Current group being built.
    current_group: Option<UndoGroup>,
}

impl History {
    /// Create a new history.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 1000,
            current_group: None,
        }
    }

    /// Push an operation to history.
    pub fn push(&mut self, op: Operation) {
        // Clear redo stack when new operations are added
        self.redo_stack.clear();

        if let Some(group) = &mut self.current_group {
            group.operations.push(op);
        } else {
            self.undo_stack.push(UndoGroup {
                operations: vec![op],
            });
        }

        // Trim history if too large
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Start a new undo group.
    pub fn begin_group(&mut self) {
        self.current_group = Some(UndoGroup {
            operations: Vec::new(),
        });
    }

    /// End the current undo group.
    pub fn end_group(&mut self) {
        if let Some(group) = self.current_group.take() {
            if !group.operations.is_empty() {
                self.undo_stack.push(group);
            }
        }
    }

    /// Undo the last operation group.
    pub fn undo(&mut self) -> Option<&UndoGroup> {
        let group = self.undo_stack.pop()?;
        self.redo_stack.push(group);
        self.redo_stack.last()
    }

    /// Redo the last undone operation group.
    pub fn redo(&mut self) -> Option<&UndoGroup> {
        let group = self.redo_stack.pop()?;
        self.undo_stack.push(group);
        self.undo_stack.last()
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_group = None;
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

/// A group of operations that are undone/redone together.
#[derive(Debug)]
pub struct UndoGroup {
    /// Operations in this group.
    pub operations: Vec<Operation>,
}
