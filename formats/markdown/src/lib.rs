//! # Markdown Format
//!
//! Markdown import/export for Wolia documents.

use wolia_core::Document;

/// Read a document from Markdown.
pub fn read(_data: &str) -> Result<Document, Error> {
    // TODO: Implement Markdown parsing
    Ok(Document::new())
}

/// Export a document to Markdown.
pub fn write(_document: &Document) -> Result<String, Error> {
    // TODO: Implement Markdown generation
    Ok(String::new())
}

/// Format errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(String),
}
