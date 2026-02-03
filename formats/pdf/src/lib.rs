//! # PDF Format
//!
//! PDF export support for Wolia documents.

use wolia_core::Document;

/// Export a document to PDF.
pub fn export(_document: &Document) -> Result<Vec<u8>, Error> {
    // TODO: Implement PDF generation
    Ok(Vec::new())
}

/// Format errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("PDF generation error: {0}")]
    Generation(String),
}
