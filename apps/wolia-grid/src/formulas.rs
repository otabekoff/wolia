//! Formula editor.

/// Formula editor state.
pub struct FormulaEditor {
    /// Current formula text.
    pub text: String,
    /// Cursor position.
    pub cursor: usize,
    /// Is the editor active.
    pub active: bool,
}

impl FormulaEditor {
    /// Create a new formula editor.
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            active: false,
        }
    }

    /// Start editing a formula.
    pub fn start(&mut self, initial: &str) {
        self.text = initial.to_string();
        self.cursor = self.text.len();
        self.active = true;
    }

    /// Cancel editing.
    pub fn cancel(&mut self) {
        self.text.clear();
        self.cursor = 0;
        self.active = false;
    }

    /// Confirm the formula.
    pub fn confirm(&mut self) -> String {
        self.active = false;
        std::mem::take(&mut self.text)
    }
}

impl Default for FormulaEditor {
    fn default() -> Self {
        Self::new()
    }
}
