//! Status bar component for displaying document statistics and status.

use std::time::SystemTime;

/// Document statistics displayed in the status bar.
#[derive(Debug, Clone, Copy)]
pub struct DocumentStats {
    /// Total number of words in the document.
    pub word_count: usize,
    /// Total number of characters in the document.
    pub character_count: usize,
    /// Total number of characters (including spaces).
    pub character_count_with_spaces: usize,
    /// Number of lines in the document.
    pub line_count: usize,
    /// Number of paragraphs in the document.
    pub paragraph_count: usize,
    /// Estimated page count (based on 250 words per page).
    pub page_count: f32,
}

impl DocumentStats {
    /// Create new document statistics.
    pub fn new() -> Self {
        Self {
            word_count: 0,
            character_count: 0,
            character_count_with_spaces: 0,
            line_count: 0,
            paragraph_count: 0,
            page_count: 0.0,
        }
    }

    /// Update statistics from document content.
    pub fn update(&mut self, content: &str) {
        // Count characters (excluding spaces).
        self.character_count = content.chars().filter(|c| !c.is_whitespace()).count();

        // Count characters (including spaces).
        self.character_count_with_spaces = content.len();

        // Count words.
        self.word_count = content.split_whitespace().filter(|w| !w.is_empty()).count();

        // Count lines.
        self.line_count = content.lines().count().max(1);

        // Count paragraphs (separated by blank lines).
        self.paragraph_count = content
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .count()
            .max(1);

        // Calculate estimated pages (250 words per page is standard).
        self.page_count = (self.word_count as f32) / 250.0;
        if self.page_count < 1.0 {
            self.page_count = 1.0;
        }
    }
}

impl Default for DocumentStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Status indicator type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIndicator {
    /// Document has been modified but not saved.
    Modified,
    /// Document is being saved.
    Saving,
    /// Document has been saved successfully.
    Saved,
    /// An error occurred.
    Error,
    /// Normal status.
    Ready,
}

impl StatusIndicator {
    /// Get display text for the indicator.
    pub fn text(&self) -> &'static str {
        match self {
            Self::Modified => "Modified",
            Self::Saving => "Saving...",
            Self::Saved => "Saved",
            Self::Error => "Error",
            Self::Ready => "Ready",
        }
    }

    /// Get color RGB for the indicator.
    pub fn color_rgb(&self) -> (f32, f32, f32) {
        match self {
            Self::Modified => (0.93, 0.68, 0.04), // Orange
            Self::Saving => (0.27, 0.51, 0.71),   // Blue
            Self::Saved => (0.29, 0.69, 0.31),    // Green
            Self::Error => (0.96, 0.26, 0.21),    // Red
            Self::Ready => (0.50, 0.50, 0.50),    // Gray
        }
    }
}

/// Status bar component for displaying document information.
pub struct StatusBar {
    /// Whether the status bar is visible.
    pub visible: bool,
    /// Height of the status bar in pixels.
    pub height: f32,
    /// Current status indicator.
    pub status: StatusIndicator,
    /// Document statistics.
    pub stats: DocumentStats,
    /// Current cursor position (line, column).
    pub cursor_position: (usize, usize),
    /// File path being edited (if any).
    pub file_path: Option<String>,
    /// Last save time.
    pub last_save_time: Option<SystemTime>,
    /// Whether the document is read-only.
    pub read_only: bool,
    /// Current zoom level (as percentage).
    pub zoom_level: u32,
}

impl StatusBar {
    /// Create a new status bar.
    pub fn new() -> Self {
        Self {
            visible: true,
            height: 24.0,
            status: StatusIndicator::Ready,
            stats: DocumentStats::new(),
            cursor_position: (1, 1),
            file_path: None,
            last_save_time: None,
            read_only: false,
            zoom_level: 100,
        }
    }

    /// Update cursor position.
    pub fn set_cursor_position(&mut self, line: usize, column: usize) {
        self.cursor_position = (line.max(1), column.max(1));
    }

    /// Update status indicator.
    pub fn set_status(&mut self, status: StatusIndicator) {
        self.status = status;
    }

    /// Update document statistics.
    pub fn update_statistics(&mut self, content: &str) {
        self.stats.update(content);
    }

    /// Set file path.
    pub fn set_file_path(&mut self, path: Option<String>) {
        self.file_path = path;
    }

    /// Update last save time to now.
    pub fn mark_saved(&mut self) {
        self.last_save_time = Some(SystemTime::now());
        self.status = StatusIndicator::Saved;
    }

    /// Toggle read-only mode.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Set zoom level (as percentage).
    pub fn set_zoom_level(&mut self, level: u32) {
        self.zoom_level = level.clamp(50, 200);
    }

    /// Get formatted status text for display.
    pub fn format_status_text(&self) -> String {
        let mut text = format!("{} | ", self.status.text());

        // Add cursor position.
        let (line, col) = self.cursor_position;
        text.push_str(&format!("Line {}, Column {} | ", line, col));

        // Add word count.
        text.push_str(&format!("Words: {} | ", self.stats.word_count));

        // Add character count.
        text.push_str(&format!(
            "Characters: {} | ",
            self.stats.character_count_with_spaces
        ));

        // Add page count.
        text.push_str(&format!("Pages: {:.1} | ", self.stats.page_count));

        // Add read-only indicator.
        if self.read_only {
            text.push_str("Read-Only | ");
        }

        // Add zoom level.
        text.push_str(&format!("Zoom: {}%", self.zoom_level));

        text
    }

    /// Toggle status bar visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_stats_update() {
        let mut stats = DocumentStats::new();
        let content = "Hello world\nThis is a test";

        stats.update(content);

        assert_eq!(stats.word_count, 5);
        assert_eq!(stats.character_count, 22); // No spaces
        assert_eq!(stats.character_count_with_spaces, 27); // With spaces and newline
        assert_eq!(stats.line_count, 2);
    }

    #[test]
    fn test_status_indicator_color() {
        assert_eq!(StatusIndicator::Error.color_rgb(), (0.96, 0.26, 0.21));
        assert_eq!(StatusIndicator::Saved.color_rgb(), (0.29, 0.69, 0.31));
    }

    #[test]
    fn test_status_bar_format_text() {
        let mut statusbar = StatusBar::new();
        statusbar.set_cursor_position(5, 10);
        statusbar.set_status(StatusIndicator::Ready);
        statusbar.update_statistics("Hello world");

        let text = statusbar.format_status_text();
        assert!(text.contains("Line 5"));
        assert!(text.contains("Column 10"));
        assert!(text.contains("Words: 2"));
    }

    #[test]
    fn test_zoom_level_clamp() {
        let mut statusbar = StatusBar::new();
        statusbar.set_zoom_level(300);
        assert_eq!(statusbar.zoom_level, 200);

        statusbar.set_zoom_level(25);
        assert_eq!(statusbar.zoom_level, 50);
    }
}
