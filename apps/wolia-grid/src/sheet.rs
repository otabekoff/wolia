//! Spreadsheet sheet model.

use grid_engine::{CellRef, Spreadsheet};

/// A spreadsheet workspace.
pub struct SheetWorkspace {
    /// The spreadsheet.
    pub spreadsheet: Spreadsheet,
    /// Active sheet index.
    pub active_sheet: usize,
    /// Selected cell.
    pub selection: CellRef,
}

impl SheetWorkspace {
    /// Create a new workspace.
    pub fn new() -> Self {
        Self {
            spreadsheet: Spreadsheet::new(),
            active_sheet: 0,
            selection: CellRef::new(0, 0),
        }
    }
}

impl Default for SheetWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
