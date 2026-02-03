//! # DOCX Format
//!
//! Microsoft Word (.docx) file format support.

use wolia_core::Document;

/// Read a document from .docx format.
pub fn read(_data: &[u8]) -> Result<Document, Error> {
    // TODO: Implement OOXML parsing
    Ok(Document::new())
}

/// Write a document to .docx format.
pub fn write(_document: &Document) -> Result<Vec<u8>, Error> {
    // TODO: Implement OOXML generation
    Ok(Vec::new())
}

/// Format errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML error: {0}")]
    Xml(String),

    #[error("Invalid format")]
    InvalidFormat,
}
