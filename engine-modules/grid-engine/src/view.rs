//! Grid view management and rendering state.

use crate::cell::CellRef;
use crate::selection::Selection;

/// Grid view configuration and state.
#[derive(Debug, Clone)]
pub struct GridView {
    /// Width of row header column (in pixels).
    pub row_header_width: f32,
    /// Height of column header row (in pixels).
    pub column_header_height: f32,
    /// Default cell width (in pixels).
    pub cell_width: f32,
    /// Default cell height (in pixels).
    pub cell_height: f32,
    /// Top-left visible cell.
    pub scroll_position: CellRef,
    /// Current selection.
    pub selection: Selection,
    /// Currently editing cell (if any).
    pub editing_cell: Option<CellRef>,
    /// Current edit buffer.
    pub edit_buffer: String,
    /// Zoom level as percentage (100 = 100%).
    pub zoom: f32,
    /// Number of visible rows (computed based on height).
    pub visible_rows: usize,
    /// Number of visible columns (computed based on width).
    pub visible_cols: usize,
}

impl GridView {
    /// Create a new grid view.
    pub fn new() -> Self {
        Self {
            row_header_width: 50.0,
            column_header_height: 24.0,
            cell_width: 100.0,
            cell_height: 24.0,
            scroll_position: CellRef::new(0, 0),
            selection: Selection::default(),
            editing_cell: None,
            edit_buffer: String::new(),
            zoom: 100.0,
            visible_rows: 20,
            visible_cols: 10,
        }
    }

    /// Get the cell at the given pixel coordinates (relative to grid area).
    pub fn cell_at(&self, x: f32, y: f32) -> Option<CellRef> {
        let col_index = ((x - self.row_header_width) / self.cell_width).floor() as usize;
        let row_index = ((y - self.column_header_height) / self.cell_height).floor() as usize;

        let col = self.scroll_position.col + col_index;
        let row = self.scroll_position.row + row_index;

        Some(CellRef::new(row, col))
    }

    /// Get the pixel bounds of a cell (in grid coordinates).
    pub fn cell_bounds(&self, cell: CellRef) -> (f32, f32, f32, f32) {
        let col_offset = cell.col.saturating_sub(self.scroll_position.col) as f32;
        let row_offset = cell.row.saturating_sub(self.scroll_position.row) as f32;

        let x = self.row_header_width + col_offset * self.cell_width;
        let y = self.column_header_height + row_offset * self.cell_height;
        let width = self.cell_width;
        let height = self.cell_height;

        (x, y, width, height)
    }

    /// Scroll to make a cell visible.
    pub fn scroll_to_cell(&mut self, cell: CellRef) {
        // Handle vertical scrolling
        if cell.row < self.scroll_position.row {
            self.scroll_position.row = cell.row;
        } else if cell.row >= self.scroll_position.row + self.visible_rows {
            self.scroll_position.row = cell.row.saturating_sub(self.visible_rows - 1);
        }

        // Handle horizontal scrolling
        if cell.col < self.scroll_position.col {
            self.scroll_position.col = cell.col;
        } else if cell.col >= self.scroll_position.col + self.visible_cols {
            self.scroll_position.col = cell.col.saturating_sub(self.visible_cols - 1);
        }
    }

    /// Start editing a cell.
    pub fn start_edit(&mut self, cell: CellRef, initial_value: impl Into<String>) {
        self.editing_cell = Some(cell);
        self.edit_buffer = initial_value.into();
        self.scroll_to_cell(cell);
    }

    /// Finish editing and return the new value.
    pub fn finish_edit(&mut self) -> Option<(CellRef, String)> {
        if let Some(cell) = self.editing_cell {
            let value = self.edit_buffer.clone();
            self.editing_cell = None;
            self.edit_buffer.clear();
            Some((cell, value))
        } else {
            None
        }
    }

    /// Cancel editing without saving.
    pub fn cancel_edit(&mut self) {
        self.editing_cell = None;
        self.edit_buffer.clear();
    }

    /// Move selection (for arrow keys).
    pub fn move_selection(&mut self, dx: i32, dy: i32) {
        let current = self.selection.primary;
        let new_col = (current.col as i32 + dx).max(0) as usize;
        let new_row = (current.row as i32 + dy).max(0) as usize;

        let new_cell = CellRef::new(new_row, new_col);
        self.selection.set(new_cell);
        self.scroll_to_cell(new_cell);
    }

    /// Set zoom level (as percentage).
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(50.0, 200.0);
        // Update effective cell dimensions
        let scale = self.zoom / 100.0;
        self.cell_width = 100.0 * scale;
        self.cell_height = 24.0 * scale;
    }

    /// Get the pixel width needed for the spreadsheet area.
    pub fn content_width(&self) -> f32 {
        self.row_header_width + self.visible_cols as f32 * self.cell_width
    }

    /// Get the pixel height needed for the spreadsheet area.
    pub fn content_height(&self) -> f32 {
        self.column_header_height + self.visible_rows as f32 * self.cell_height
    }
}

impl Default for GridView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_at_coordinates() {
        let grid = GridView::new();
        let cell = grid.cell_at(150.0, 50.0).unwrap();
        assert_eq!(cell.col, 1); // Roughly at column 1
        assert_eq!(cell.row, 1); // Roughly at row 1
    }

    #[test]
    fn test_cell_bounds() {
        let grid = GridView::new();
        let cell = CellRef::new(0, 0);
        let (x, y, w, h) = grid.cell_bounds(cell);
        assert_eq!(x, 50.0); // row_header_width
        assert_eq!(y, 24.0); // column_header_height
        assert_eq!(w, 100.0);
        assert_eq!(h, 24.0);
    }

    #[test]
    fn test_scroll_to_cell() {
        let mut grid = GridView::new();
        grid.visible_rows = 5;
        grid.visible_cols = 5;

        grid.scroll_to_cell(CellRef::new(10, 10));
        assert!(grid.scroll_position.row <= 10);
        assert!(grid.scroll_position.col <= 10);
    }

    #[test]
    fn test_edit_operations() {
        let mut grid = GridView::new();
        let cell = CellRef::new(0, 0);

        grid.start_edit(cell, "hello");
        assert_eq!(grid.editing_cell, Some(cell));
        assert_eq!(grid.edit_buffer, "hello");

        let (ret_cell, ret_val) = grid.finish_edit().unwrap();
        assert_eq!(ret_cell, cell);
        assert_eq!(ret_val, "hello");
    }

    #[test]
    fn test_move_selection() {
        let mut grid = GridView::new();
        grid.move_selection(1, 1);
        assert_eq!(grid.selection.primary, CellRef::new(1, 1));
    }

    #[test]
    fn test_zoom() {
        let mut grid = GridView::new();
        grid.set_zoom(200.0);
        assert_eq!(grid.zoom, 200.0);
        assert_eq!(grid.cell_width, 200.0);
    }
}
