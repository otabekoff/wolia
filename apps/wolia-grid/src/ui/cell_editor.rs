//! Cell editor overlay.

/// Cell editor overlay.
pub struct CellEditor {
    /// Is visible.
    pub visible: bool,
    /// Current text.
    pub text: String,
}

impl CellEditor {
    pub fn new() -> Self {
        Self {
            visible: false,
            text: String::new(),
        }
    }
}

impl Default for CellEditor {
    fn default() -> Self {
        Self::new()
    }
}
