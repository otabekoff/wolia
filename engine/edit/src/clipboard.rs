//! Clipboard operations.

/// Clipboard content types.
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    /// Plain text.
    Text(String),
    /// Rich text (HTML).
    RichText(String),
    /// Wolia native format.
    Native(Vec<u8>),
}

/// Clipboard interface.
pub trait Clipboard {
    /// Get text from clipboard.
    fn get_text(&self) -> Option<String>;

    /// Set text to clipboard.
    fn set_text(&self, text: &str) -> crate::Result<()>;

    /// Get rich content from clipboard.
    fn get_content(&self) -> Option<ClipboardContent>;

    /// Set rich content to clipboard.
    fn set_content(&self, content: ClipboardContent) -> crate::Result<()>;
}
