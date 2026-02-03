//! PDF format error types.

/// PDF format errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error during PDF generation or writing.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// PDF generation failed.
    #[error("PDF generation error: {0}")]
    Generation(String),

    /// Invalid document structure.
    #[error("Invalid document structure: {0}")]
    InvalidDocument(String),

    /// Encoding error.
    #[error("Encoding error: {0}")]
    Encoding(String),
}

impl Error {
    /// Create a new generation error.
    pub fn generation(msg: impl Into<String>) -> Self {
        Self::Generation(msg.into())
    }

    /// Create a new document error.
    pub fn invalid_document(msg: impl Into<String>) -> Self {
        Self::InvalidDocument(msg.into())
    }

    /// Create a new encoding error.
    pub fn encoding(msg: impl Into<String>) -> Self {
        Self::Encoding(msg.into())
    }
}
