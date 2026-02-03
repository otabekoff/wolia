//! # Font Processor
//!
//! Font processing tools for Wolia development.

/// Process and validate font files.
pub fn validate_font(_data: &[u8]) -> Result<FontInfo, Error> {
    // TODO: Implement font validation
    Err(Error::NotImplemented)
}

/// Font information.
pub struct FontInfo {
    pub family: String,
    pub style: String,
    pub num_glyphs: u16,
}

/// Errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid font data")]
    InvalidFont,

    #[error("Not implemented")]
    NotImplemented,
}
