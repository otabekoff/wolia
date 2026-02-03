//! # XLSX Format
//!
//! Microsoft Excel (.xlsx) file format support.

use grid_engine::Spreadsheet;

/// Read a spreadsheet from .xlsx format.
pub fn read(_data: &[u8]) -> Result<Spreadsheet, Error> {
    // TODO: Implement OOXML parsing
    Ok(Spreadsheet::new())
}

/// Write a spreadsheet to .xlsx format.
pub fn write(_spreadsheet: &Spreadsheet) -> Result<Vec<u8>, Error> {
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
