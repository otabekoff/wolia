//! Edit operations.

/// An atomic editing operation.
#[derive(Debug, Clone)]
pub enum Operation {
    /// Insert text at position.
    InsertText { position: usize, text: String },
    /// Delete text range.
    DeleteText {
        start: usize,
        end: usize,
        /// The deleted text (for undo).
        deleted: String,
    },
    /// Replace text range.
    ReplaceText {
        start: usize,
        end: usize,
        old_text: String,
        new_text: String,
    },
    /// Apply formatting to a range.
    Format {
        start: usize,
        end: usize,
        style_changes: Vec<StyleChange>,
    },
    /// Insert a node.
    InsertNode {
        parent_id: uuid::Uuid,
        index: usize,
        node_data: Vec<u8>,
    },
    /// Delete a node.
    DeleteNode {
        node_id: uuid::Uuid,
        node_data: Vec<u8>,
    },
    /// Move a node.
    MoveNode {
        node_id: uuid::Uuid,
        old_parent: uuid::Uuid,
        old_index: usize,
        new_parent: uuid::Uuid,
        new_index: usize,
    },
}

impl Operation {
    /// Get the inverse operation (for undo).
    pub fn inverse(&self) -> Operation {
        match self {
            Operation::InsertText { position, text } => Operation::DeleteText {
                start: *position,
                end: position + text.len(),
                deleted: text.clone(),
            },
            Operation::DeleteText { start, deleted, .. } => Operation::InsertText {
                position: *start,
                text: deleted.clone(),
            },
            Operation::ReplaceText {
                start,
                end,
                old_text,
                new_text,
            } => Operation::ReplaceText {
                start: *start,
                end: start + new_text.len(),
                old_text: new_text.clone(),
                new_text: old_text.clone(),
            },
            Operation::Format { .. } => {
                // TODO: Store original formatting for proper undo
                self.clone()
            }
            Operation::InsertNode {
                parent_id,
                node_data,
                ..
            } => Operation::DeleteNode {
                node_id: *parent_id, // TODO: Get actual node ID
                node_data: node_data.clone(),
            },
            Operation::DeleteNode { node_id, node_data } => Operation::InsertNode {
                parent_id: *node_id, // TODO: Store original parent
                index: 0,
                node_data: node_data.clone(),
            },
            Operation::MoveNode {
                node_id,
                old_parent,
                old_index,
                new_parent,
                new_index,
            } => Operation::MoveNode {
                node_id: *node_id,
                old_parent: *new_parent,
                old_index: *new_index,
                new_parent: *old_parent,
                new_index: *old_index,
            },
        }
    }
}

/// A style change.
#[derive(Debug, Clone)]
pub struct StyleChange {
    pub property: String,
    pub value: Option<String>,
}
