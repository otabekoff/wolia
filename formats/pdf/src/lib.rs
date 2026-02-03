//! # PDF Format
//!
//! PDF export support for Wolia documents.

use std::io::Write;
use wolia_core::Document;

pub use self::error::Error;
pub use self::generator::PdfGenerator;

mod error;
mod generator;

/// Export a document to PDF format.
///
/// # Arguments
///
/// * `document` - The Wolia document to export
///
/// # Returns
///
/// A Result containing the PDF bytes on success, or an Error on failure.
///
/// # Examples
///
/// ```ignore
/// let doc = Document::new();
/// let pdf_bytes = export(&doc)?;
/// std::fs::write("document.pdf", &pdf_bytes)?;
/// ```
pub fn export(document: &Document) -> Result<Vec<u8>, Error> {
    let mut generator = PdfGenerator::new();
    generator.generate(document)
}

/// Export a document to PDF and write to a file.
pub fn export_to_file(document: &Document, path: impl AsRef<std::path::Path>) -> Result<(), Error> {
    let bytes = export(document)?;
    let mut file = std::fs::File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_empty_document() {
        let doc = Document::new();
        let result = export(&doc);
        assert!(result.is_ok());
    }
}
