//! Presenter view.

/// Presenter view for presentation mode.
pub struct PresenterView {
    /// Show speaker notes.
    pub show_notes: bool,
    /// Show timer.
    pub show_timer: bool,
    /// Show next slide preview.
    pub show_next_slide: bool,
    /// Elapsed time in seconds.
    pub elapsed: f32,
}

impl PresenterView {
    pub fn new() -> Self {
        Self {
            show_notes: true,
            show_timer: true,
            show_next_slide: true,
            elapsed: 0.0,
        }
    }
}

impl Default for PresenterView {
    fn default() -> Self {
        Self::new()
    }
}
