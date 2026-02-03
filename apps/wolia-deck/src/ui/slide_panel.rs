//! Slide panel (thumbnail view).

/// Slide panel for navigation.
pub struct SlidePanel {
    /// Panel width.
    pub width: f32,
    /// Is visible.
    pub visible: bool,
    /// Thumbnail size.
    pub thumbnail_width: f32,
}

impl SlidePanel {
    pub fn new() -> Self {
        Self {
            width: 200.0,
            visible: true,
            thumbnail_width: 180.0,
        }
    }
}

impl Default for SlidePanel {
    fn default() -> Self {
        Self::new()
    }
}
