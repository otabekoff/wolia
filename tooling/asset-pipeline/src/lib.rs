//! # Asset Pipeline
//!
//! Asset processing pipeline for Wolia builds.

/// Process assets for packaging.
pub fn process_assets(
    _input_dir: &std::path::Path,
    _output_dir: &std::path::Path,
) -> Result<(), Error> {
    // TODO: Implement asset processing
    Ok(())
}

/// Errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Processing error: {0}")]
    Processing(String),
}
