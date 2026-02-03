//! Toolbar component.

/// Toolbar state and actions.
pub struct Toolbar {
    /// Whether the toolbar is visible.
    pub visible: bool,
}

impl Toolbar {
    /// Create a new toolbar.
    pub fn new() -> Self {
        Self { visible: true }
    }

    /// Toggle toolbar visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

/// Toolbar actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarAction {
    // File
    New,
    Open,
    Save,
    SaveAs,
    Export,
    Print,

    // Edit
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Find,
    Replace,

    // Format - Text
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Subscript,
    Superscript,

    // Format - Paragraph
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignJustify,

    // Lists
    BulletList,
    NumberedList,

    // Insert
    InsertImage,
    InsertTable,
    InsertLink,
    InsertPageBreak,
}
