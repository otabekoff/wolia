//! Cell selection and range management.

use crate::cell::CellRef;
use std::cmp::{max, min};
use std::collections::HashSet;

/// A range of cells from (start_row, start_col) to (end_row, end_col).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellRange {
    /// Start cell reference (top-left).
    pub start: CellRef,
    /// End cell reference (bottom-right).
    pub end: CellRef,
}

impl CellRange {
    /// Create a new cell range.
    pub fn new(start: CellRef, end: CellRef) -> Self {
        // Normalize the range so start is always top-left
        let (start, end) = if start.row <= end.row && start.col <= end.col {
            (start, end)
        } else {
            let min_row = min(start.row, end.row);
            let max_row = max(start.row, end.row);
            let min_col = min(start.col, end.col);
            let max_col = max(start.col, end.col);
            (
                CellRef::new(min_row, min_col),
                CellRef::new(max_row, max_col),
            )
        };

        Self { start, end }
    }

    /// Check if a cell is within this range.
    pub fn contains(&self, cell: CellRef) -> bool {
        cell.row >= self.start.row
            && cell.row <= self.end.row
            && cell.col >= self.start.col
            && cell.col <= self.end.col
    }

    /// Get the number of rows in this range.
    pub fn row_count(&self) -> usize {
        self.end.row - self.start.row + 1
    }

    /// Get the number of columns in this range.
    pub fn col_count(&self) -> usize {
        self.end.col - self.start.col + 1
    }

    /// Get all cells in this range.
    pub fn cells(&self) -> impl Iterator<Item = CellRef> {
        let start_row = self.start.row;
        let start_col = self.start.col;
        let end_row = self.end.row;
        let end_col = self.end.col;

        (start_row..=end_row)
            .flat_map(move |row| (start_col..=end_col).map(move |col| CellRef::new(row, col)))
    }

    /// Parse a range from a string (e.g., "A1:B10").
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        match parts.as_slice() {
            [start_str, end_str] => {
                let start = CellRef::parse(start_str)?;
                let end = CellRef::parse(end_str)?;
                Some(Self::new(start, end))
            }
            _ => None,
        }
    }

    /// Convert to string representation (e.g., "A1:B10").
    pub fn to_range_string(&self) -> String {
        format!("{}:{}", self.start.to_a1(), self.end.to_a1())
    }
}

/// Selection of cells (can be a single cell or multiple ranges).
#[derive(Debug, Clone)]
pub struct Selection {
    /// Primary selection (the last selected cell).
    pub primary: CellRef,
    /// All selected ranges (including the primary cell).
    ranges: Vec<CellRange>,
}

impl Selection {
    /// Create a new selection with a single cell.
    pub fn new(cell: CellRef) -> Self {
        let range = CellRange::new(cell, cell);
        Self {
            primary: cell,
            ranges: vec![range],
        }
    }

    /// Create a selection from a range.
    pub fn from_range(range: CellRange) -> Self {
        Self {
            primary: range.end,
            ranges: vec![range],
        }
    }

    /// Expand selection to include a range.
    pub fn extend_to(&mut self, end: CellRef) {
        let range = CellRange::new(self.primary, end);
        self.ranges = vec![range];
    }

    /// Add a range to the selection (for multi-select with Ctrl).
    pub fn add_range(&mut self, range: CellRange) {
        self.ranges.push(range);
        self.primary = range.end;
    }

    /// Clear and set to single cell.
    pub fn set(&mut self, cell: CellRef) {
        self.primary = cell;
        self.ranges = vec![CellRange::new(cell, cell)];
    }

    /// Check if a cell is selected.
    pub fn is_selected(&self, cell: CellRef) -> bool {
        self.ranges.iter().any(|r| r.contains(cell))
    }

    /// Get all selected cells.
    pub fn cells(&self) -> HashSet<CellRef> {
        self.ranges.iter().flat_map(|r| r.cells()).collect()
    }

    /// Get the selected range (first range if multiple).
    pub fn range(&self) -> CellRange {
        self.ranges[0]
    }

    /// Get all selected ranges.
    pub fn ranges(&self) -> &[CellRange] {
        &self.ranges
    }

    /// Get the total count of selected cells.
    pub fn cell_count(&self) -> usize {
        self.cells().len()
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new(CellRef::new(0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_range_creation() {
        let range = CellRange::new(CellRef::new(0, 0), CellRef::new(2, 2));
        assert_eq!(range.row_count(), 3);
        assert_eq!(range.col_count(), 3);
    }

    #[test]
    fn test_cell_range_contains() {
        let range = CellRange::new(CellRef::new(0, 0), CellRef::new(2, 2));
        assert!(range.contains(CellRef::new(1, 1)));
        assert!(!range.contains(CellRef::new(3, 3)));
    }

    #[test]
    fn test_cell_range_parse() {
        let range = CellRange::parse("A1:C5").unwrap();
        assert_eq!(range.start, CellRef::new(0, 0));
        assert_eq!(range.end, CellRef::new(4, 2));
        assert_eq!(range.to_range_string(), "A1:C5");
    }

    #[test]
    fn test_selection_extend() {
        let mut sel = Selection::new(CellRef::new(0, 0));
        sel.extend_to(CellRef::new(2, 2));
        assert_eq!(sel.cell_count(), 9);
        assert!(sel.is_selected(CellRef::new(1, 1)));
    }

    #[test]
    fn test_selection_multiselect() {
        let mut sel = Selection::new(CellRef::new(0, 0));
        sel.add_range(CellRange::new(CellRef::new(5, 5), CellRef::new(7, 7)));
        assert!(sel.is_selected(CellRef::new(0, 0)));
        assert!(sel.is_selected(CellRef::new(5, 5)));
        assert!(!sel.is_selected(CellRef::new(3, 3)));
    }
}
