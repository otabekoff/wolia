//! Status bar.

/// Status bar state.
pub struct StatusBar {
    /// Whether the status bar is visible.
    pub visible: bool,
}

impl StatusBar {
    /// Create a new status bar.
    pub fn new() -> Self {
        Self { visible: true }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Status bar information.
#[derive(Debug, Clone, Default)]
pub struct StatusInfo {
    /// Current page number.
    pub page: usize,
    /// Total pages.
    pub total_pages: usize,
    /// Word count.
    pub word_count: usize,
    /// Character count.
    pub char_count: usize,
    /// Current line number.
    pub line: usize,
    /// Current column number.
    pub column: usize,
    /// Zoom percentage.
    pub zoom: f32,
    /// Document language.
    pub language: Option<String>,
}
