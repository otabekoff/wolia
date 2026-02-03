//! Document model.

use uuid::Uuid;

use crate::node::Node;
use crate::style::StyleSheet;

/// A Wolia document.
///
/// This is the root container for all document content, shared across
/// all Wolia applications (Write, Grid, Deck).
#[derive(Debug, Clone)]
pub struct Document {
    /// Unique identifier for this document.
    pub id: Uuid,
    /// Document metadata.
    pub metadata: Metadata,
    /// Root content node.
    pub root: Node,
    /// Document-level styles.
    pub styles: StyleSheet,
}

impl Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: Metadata::default(),
            root: Node::root(),
            styles: StyleSheet::default(),
        }
    }

    /// Create a document with a specific ID.
    pub fn with_id(id: Uuid) -> Self {
        Self { id, ..Self::new() }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// Document metadata.
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// Document title.
    pub title: Option<String>,
    /// Document author.
    pub author: Option<String>,
    /// Creation timestamp.
    pub created: Option<i64>,
    /// Last modified timestamp.
    pub modified: Option<i64>,
    /// Document description.
    pub description: Option<String>,
    /// Custom properties.
    pub properties: indexmap::IndexMap<String, String>,
}
