//! Spreadsheet model.

use crate::sheet::Sheet;

/// A spreadsheet workbook containing multiple sheets.
#[derive(Debug, Clone)]
pub struct Spreadsheet {
    /// Sheets in the workbook.
    sheets: Vec<Sheet>,
    /// Active sheet index.
    pub active_sheet: usize,
}

impl Spreadsheet {
    /// Create a new spreadsheet with one empty sheet.
    pub fn new() -> Self {
        Self {
            sheets: vec![Sheet::default()],
            active_sheet: 0,
        }
    }

    /// Get the number of sheets.
    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    /// Get a sheet by index.
    pub fn sheet(&self, index: usize) -> Option<&Sheet> {
        self.sheets.get(index)
    }

    /// Get a sheet mutably by index.
    pub fn sheet_mut(&mut self, index: usize) -> Option<&mut Sheet> {
        self.sheets.get_mut(index)
    }

    /// Get the active sheet.
    pub fn active(&self) -> &Sheet {
        &self.sheets[self.active_sheet]
    }

    /// Get the active sheet mutably.
    pub fn active_mut(&mut self) -> &mut Sheet {
        &mut self.sheets[self.active_sheet]
    }

    /// Add a new sheet.
    pub fn add_sheet(&mut self, name: impl Into<String>) -> usize {
        let index = self.sheets.len();
        self.sheets.push(Sheet::new(name));
        index
    }

    /// Remove a sheet.
    pub fn remove_sheet(&mut self, index: usize) -> Option<Sheet> {
        if self.sheets.len() <= 1 || index >= self.sheets.len() {
            return None;
        }

        let sheet = self.sheets.remove(index);
        if self.active_sheet >= self.sheets.len() {
            self.active_sheet = self.sheets.len() - 1;
        }
        Some(sheet)
    }

    /// Rename a sheet.
    pub fn rename_sheet(&mut self, index: usize, name: impl Into<String>) -> bool {
        if let Some(sheet) = self.sheets.get_mut(index) {
            sheet.name = name.into();
            true
        } else {
            false
        }
    }

    /// Get sheet names.
    pub fn sheet_names(&self) -> impl Iterator<Item = &str> {
        self.sheets.iter().map(|s| s.name.as_str())
    }
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self::new()
    }
}
