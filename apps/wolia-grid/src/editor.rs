//! Cell editing and input management for the grid.

use grid_engine::{Cell, CellRef, CellValue, GridView, Spreadsheet};

/// Grid editor state and operations.
pub struct GridEditor {
    /// Reference to the spreadsheet.
    pub spreadsheet: Spreadsheet,
    /// View management.
    pub view: GridView,
    /// Current mode (Normal or Edit).
    pub mode: EditMode,
    /// Clipboard content.
    pub clipboard: Option<Vec<Vec<CellValue>>>,
}

impl GridEditor {
    /// Create a new grid editor.
    pub fn new() -> Self {
        Self {
            spreadsheet: Spreadsheet::new(),
            view: GridView::new(),
            mode: EditMode::Normal,
            clipboard: None,
        }
    }

    /// Get the currently selected cell.
    pub fn selected_cell(&self) -> CellRef {
        self.view.selection.primary
    }

    /// Get the value of the selected cell.
    pub fn get_selected_value(&self) -> Option<CellValue> {
        let cell = self.selected_cell();
        self.get_cell_value(cell)
    }

    /// Get the value of a specific cell.
    pub fn get_cell_value(&self, cell: CellRef) -> Option<CellValue> {
        let sheet = self.spreadsheet.active();
        sheet.get(cell).map(|c| c.value.clone())
    }

    /// Set the value of a cell.
    pub fn set_cell_value(&mut self, cell: CellRef, value: CellValue) {
        let sheet = self.spreadsheet.active_mut();
        let cell_obj = Cell::with_value(value);
        sheet.set(cell, cell_obj);
    }

    /// Start editing the selected cell.
    pub fn start_editing(&mut self) {
        let cell = self.selected_cell();
        let value = self
            .get_cell_value(cell)
            .map(|v| v.to_display_string())
            .unwrap_or_default();

        self.view.start_edit(cell, value);
        self.mode = EditMode::Edit;
    }

    /// Finish editing and save the value.
    pub fn finish_editing(&mut self) {
        if let Some((cell, value)) = self.view.finish_edit() {
            // Parse the value based on its content
            let cell_value = if value.starts_with('=') {
                // Formula
                CellValue::Text(value) // TODO: Parse as formula
            } else if let Ok(n) = value.parse::<f64>() {
                // Number
                CellValue::Number(n)
            } else {
                // Text
                CellValue::Text(value)
            };

            self.set_cell_value(cell, cell_value);
        }

        self.mode = EditMode::Normal;
    }

    /// Cancel editing without saving.
    pub fn cancel_editing(&mut self) {
        self.view.cancel_edit();
        self.mode = EditMode::Normal;
    }

    /// Delete the content of the selected cell.
    pub fn delete_cell(&mut self) {
        let cell = self.selected_cell();
        self.set_cell_value(cell, CellValue::Empty);
    }

    /// Cut the selected cell(s).
    pub fn cut(&mut self) {
        self.copy();
        self.delete_cell();
    }

    /// Copy the selected cell(s).
    pub fn copy(&mut self) {
        let cell = self.selected_cell();
        if let Some(value) = self.get_cell_value(cell) {
            self.clipboard = Some(vec![vec![value]]);
        }
    }

    /// Paste clipboard content.
    pub fn paste(&mut self) {
        if let Some(clip) = &self.clipboard {
            if !clip.is_empty() && !clip[0].is_empty() {
                let cell = self.selected_cell();
                self.set_cell_value(cell, clip[0][0].clone());
            }
        }
    }

    /// Move cursor with arrow keys.
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        self.view.move_selection(dx, dy);
    }

    /// Move to a specific cell.
    pub fn goto_cell(&mut self, cell: CellRef) {
        self.view.selection.set(cell);
        self.view.scroll_to_cell(cell);
    }

    /// Get the current edit text (if editing).
    pub fn get_edit_text(&self) -> Option<&str> {
        if self.mode == EditMode::Edit {
            Some(&self.view.edit_buffer)
        } else {
            None
        }
    }

    /// Update edit buffer text.
    pub fn update_edit_text(&mut self, text: impl Into<String>) {
        self.view.edit_buffer = text.into();
    }

    /// Get the formula bar text (either cell value or edit text).
    pub fn get_formula_bar_text(&self) -> String {
        if let Some(edit_text) = self.get_edit_text() {
            edit_text.to_string()
        } else {
            self.get_selected_value()
                .map(|v| v.to_display_string())
                .unwrap_or_default()
        }
    }

    /// Check if currently editing.
    pub fn is_editing(&self) -> bool {
        self.mode == EditMode::Edit
    }
}

impl Default for GridEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Edit mode for the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditMode {
    /// Normal mode (cell selection).
    Normal,
    /// Editing mode (cell content editing).
    Edit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_editor_creation() {
        let editor = GridEditor::new();
        assert_eq!(editor.mode, EditMode::Normal);
        assert_eq!(editor.selected_cell(), CellRef::new(0, 0));
    }

    #[test]
    fn test_set_and_get_cell_value() {
        let mut editor = GridEditor::new();
        let cell = CellRef::new(0, 0);
        let value = CellValue::Number(42.0);

        editor.set_cell_value(cell, value.clone());
        assert_eq!(editor.get_cell_value(cell), Some(value));
    }

    #[test]
    fn test_copy_paste() {
        let mut editor = GridEditor::new();
        let cell = CellRef::new(0, 0);
        editor.set_cell_value(cell, CellValue::Text("Hello".to_string()));

        editor.copy();
        assert!(editor.clipboard.is_some());

        editor.goto_cell(CellRef::new(1, 1));
        editor.paste();
        assert_eq!(
            editor.get_cell_value(CellRef::new(1, 1)),
            Some(CellValue::Text("Hello".to_string()))
        );
    }

    #[test]
    fn test_cut() {
        let mut editor = GridEditor::new();
        let cell = CellRef::new(0, 0);
        editor.set_cell_value(cell, CellValue::Number(42.0));

        editor.cut();
        // After cut, the cell is cleared (removed from storage, returns None)
        assert!(editor.get_cell_value(cell).is_none());
        // Clipboard should contain the original value
        assert!(editor.clipboard.is_some());
        assert_eq!(
            editor.clipboard.as_ref().unwrap()[0][0],
            CellValue::Number(42.0)
        );
    }

    #[test]
    fn test_edit_mode() {
        let mut editor = GridEditor::new();
        assert!(!editor.is_editing());

        editor.start_editing();
        assert!(editor.is_editing());

        editor.cancel_editing();
        assert!(!editor.is_editing());
    }

    #[test]
    fn test_move_cursor() {
        let mut editor = GridEditor::new();
        editor.move_cursor(1, 1);
        assert_eq!(editor.selected_cell(), CellRef::new(1, 1));

        editor.move_cursor(-1, -1);
        assert_eq!(editor.selected_cell(), CellRef::new(0, 0));
    }

    #[test]
    fn test_formula_bar_text() {
        let mut editor = GridEditor::new();
        let cell = CellRef::new(0, 0);
        editor.set_cell_value(cell, CellValue::Text("Test".to_string()));

        let text = editor.get_formula_bar_text();
        assert_eq!(text, "Test");
    }
}
