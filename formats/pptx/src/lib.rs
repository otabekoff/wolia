//! # PPTX Format
//!
//! Microsoft PowerPoint (.pptx) file format support.

use deck_engine::Presentation;

/// Read a presentation from .pptx format.
pub fn read(_data: &[u8]) -> Result<Presentation, Error> {
    // TODO: Implement OOXML parsing
    Ok(Presentation::new())
}

/// Write a presentation to .pptx format.
pub fn write(_presentation: &Presentation) -> Result<Vec<u8>, Error> {
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
