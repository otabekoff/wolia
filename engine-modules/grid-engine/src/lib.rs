//! # Grid Engine
//!
//! Spreadsheet engine for Wolia Grid.
//!
//! Provides:
//! - Cell model and storage
//! - Formula parsing and evaluation
//! - Cell references and ranges
//! - Data validation
//! - Sorting and filtering

pub mod cell;
pub mod evaluator;
pub mod formula;
pub mod selection;
pub mod sheet;
pub mod spreadsheet;
pub mod view;

pub use cell::{Cell, CellRef, CellValue};
pub use evaluator::{Evaluator, Function};
pub use formula::{Formula, FormulaContext, FormulaError};
pub use selection::{CellRange, Selection};
pub use sheet::Sheet;
pub use spreadsheet::Spreadsheet;
pub use view::GridView;

/// Result type for grid operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Grid engine errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid cell reference: {0}")]
    InvalidCellRef(String),

    #[error("Formula error: {0}")]
    Formula(#[from] FormulaError),

    #[error("Circular reference detected")]
    CircularReference,

    #[error("Invalid range: {0}")]
    InvalidRange(String),
}
