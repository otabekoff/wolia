//! Content nodes.

use uuid::Uuid;

use crate::text::Text;

/// A node in the document tree.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique identifier.
    pub id: Uuid,
    /// Node type and content.
    pub kind: NodeKind,
    /// Child nodes.
    pub children: Vec<Node>,
}

impl Node {
    /// Create a root node.
    pub fn root() -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: NodeKind::Root,
            children: Vec::new(),
        }
    }

    /// Create a paragraph node.
    pub fn paragraph(text: Text) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: NodeKind::Paragraph(text),
            children: Vec::new(),
        }
    }

    /// Create a section node.
    pub fn section() -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: NodeKind::Section,
            children: Vec::new(),
        }
    }

    /// Add a child node.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

/// The type and content of a node.
#[derive(Debug, Clone)]
pub enum NodeKind {
    /// Document root.
    Root,
    /// A section/chapter.
    Section,
    /// A paragraph with text content.
    Paragraph(Text),
    /// A heading with level (1-6) and text.
    Heading { level: u8, text: Text },
    /// A list (ordered or unordered).
    List { ordered: bool },
    /// A list item.
    ListItem,
    /// A table.
    Table { rows: usize, cols: usize },
    /// A table row.
    TableRow,
    /// A table cell.
    TableCell,
    /// An image.
    Image { src: String, alt: Option<String> },
    /// A code block.
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    /// A horizontal rule.
    HorizontalRule,
    /// A page break.
    PageBreak,
    /// Custom/plugin content.
    Custom { kind: String, data: Vec<u8> },
}
