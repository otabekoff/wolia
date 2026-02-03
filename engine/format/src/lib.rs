//! # Wolia Format
//!
//! Document serialization for the Wolia platform.
//!
//! This crate provides:
//! - Native .wolia format save/load
//! - Format detection
//! - Export interfaces

use wolia_core::Document;

pub mod detect;
pub mod native;

/// Result type for format operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during format operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Format capability trait.
pub trait Format {
    /// File extension for this format.
    fn extension(&self) -> &str;

    /// MIME type for this format.
    fn mime_type(&self) -> &str;

    /// Human-readable format name.
    fn name(&self) -> &str;
}

/// Document reader trait.
pub trait DocumentReader: Format {
    /// Read a document from bytes.
    fn read(&self, data: &[u8]) -> Result<Document>;
}

/// Document writer trait.
pub trait DocumentWriter: Format {
    /// Write a document to bytes.
    fn write(&self, document: &Document) -> Result<Vec<u8>>;
}

/// The native Wolia format.
pub struct WoliaFormat;

impl Format for WoliaFormat {
    fn extension(&self) -> &str {
        "wolia"
    }

    fn mime_type(&self) -> &str {
        "application/vnd.wolia"
    }

    fn name(&self) -> &str {
        "Wolia Document"
    }
}

impl DocumentReader for WoliaFormat {
    fn read(&self, data: &[u8]) -> Result<Document> {
        native::read(data)
    }
}

impl DocumentWriter for WoliaFormat {
    fn write(&self, document: &Document) -> Result<Vec<u8>> {
        native::write(document)
    }
}
