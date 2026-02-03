//! Slide management.

use deck_engine::Presentation;

/// Slide workspace.
pub struct SlideWorkspace {
    /// The presentation.
    pub presentation: Presentation,
    /// Current slide index.
    pub current_slide: usize,
    /// Presentation mode.
    pub presenting: bool,
}

impl SlideWorkspace {
    /// Create a new workspace.
    pub fn new() -> Self {
        Self {
            presentation: Presentation::new(),
            current_slide: 0,
            presenting: false,
        }
    }

    /// Go to the next slide.
    pub fn next_slide(&mut self) {
        if self.current_slide < self.presentation.slide_count().saturating_sub(1) {
            self.current_slide += 1;
        }
    }

    /// Go to the previous slide.
    pub fn prev_slide(&mut self) {
        self.current_slide = self.current_slide.saturating_sub(1);
    }

    /// Start presentation mode.
    pub fn start_presentation(&mut self) {
        self.presenting = true;
        self.current_slide = 0;
    }

    /// Stop presentation mode.
    pub fn stop_presentation(&mut self) {
        self.presenting = false;
    }
}

impl Default for SlideWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
