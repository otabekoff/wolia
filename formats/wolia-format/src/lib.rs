//! # Native Wolia Format
//!
//! The native .wolia file format implementation.
//!
//! The format is a zstd-compressed JSON document with embedded binary assets.

use wolia_core::Document;

/// Read a document from .wolia format.
pub fn read(_data: &[u8]) -> Result<Document, Error> {
    // TODO: Implement
    Ok(Document::new())
}

/// Write a document to .wolia format.
pub fn write(_document: &Document) -> Result<Vec<u8>, Error> {
    // TODO: Implement
    Ok(Vec::new())
}

/// Format errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid format")]
    InvalidFormat,
}
