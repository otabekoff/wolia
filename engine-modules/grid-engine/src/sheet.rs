//! Sheet model.

use indexmap::IndexMap;

use crate::cell::{Cell, CellRef};

/// A single sheet in a spreadsheet.
#[derive(Debug, Clone)]
pub struct Sheet {
    /// Sheet name.
    pub name: String,
    /// Cells (sparse storage).
    cells: IndexMap<CellRef, Cell>,
    /// Column widths (in points).
    pub col_widths: IndexMap<usize, f32>,
    /// Row heights (in points).
    pub row_heights: IndexMap<usize, f32>,
    /// Default column width.
    pub default_col_width: f32,
    /// Default row height.
    pub default_row_height: f32,
    /// Frozen rows.
    pub frozen_rows: usize,
    /// Frozen columns.
    pub frozen_cols: usize,
}

impl Sheet {
    /// Create a new sheet.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cells: IndexMap::new(),
            col_widths: IndexMap::new(),
            row_heights: IndexMap::new(),
            default_col_width: 100.0,
            default_row_height: 24.0,
            frozen_rows: 0,
            frozen_cols: 0,
        }
    }

    /// Get a cell.
    pub fn get(&self, cell_ref: CellRef) -> Option<&Cell> {
        self.cells.get(&cell_ref)
    }

    /// Get a cell mutably.
    pub fn get_mut(&mut self, cell_ref: CellRef) -> Option<&mut Cell> {
        self.cells.get_mut(&cell_ref)
    }

    /// Set a cell.
    pub fn set(&mut self, cell_ref: CellRef, cell: Cell) {
        if cell.value.is_empty() && cell.formula.is_none() {
            self.cells.shift_remove(&cell_ref);
        } else {
            self.cells.insert(cell_ref, cell);
        }
    }

    /// Clear a cell.
    pub fn clear(&mut self, cell_ref: CellRef) {
        self.cells.shift_remove(&cell_ref);
    }

    /// Get column width.
    pub fn col_width(&self, col: usize) -> f32 {
        self.col_widths
            .get(&col)
            .copied()
            .unwrap_or(self.default_col_width)
    }

    /// Set column width.
    pub fn set_col_width(&mut self, col: usize, width: f32) {
        self.col_widths.insert(col, width);
    }

    /// Get row height.
    pub fn row_height(&self, row: usize) -> f32 {
        self.row_heights
            .get(&row)
            .copied()
            .unwrap_or(self.default_row_height)
    }

    /// Set row height.
    pub fn set_row_height(&mut self, row: usize, height: f32) {
        self.row_heights.insert(row, height);
    }

    /// Get all non-empty cells.
    pub fn cells(&self) -> impl Iterator<Item = (&CellRef, &Cell)> {
        self.cells.iter()
    }

    /// Get the used range (bounding box of all non-empty cells).
    pub fn used_range(&self) -> Option<(CellRef, CellRef)> {
        if self.cells.is_empty() {
            return None;
        }

        let mut min_row = usize::MAX;
        let mut min_col = usize::MAX;
        let mut max_row = 0;
        let mut max_col = 0;

        for cell_ref in self.cells.keys() {
            min_row = min_row.min(cell_ref.row);
            min_col = min_col.min(cell_ref.col);
            max_row = max_row.max(cell_ref.row);
            max_col = max_col.max(cell_ref.col);
        }

        Some((
            CellRef::new(min_row, min_col),
            CellRef::new(max_row, max_col),
        ))
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new("Sheet1")
    }
}
