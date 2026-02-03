//! Window management.

use winit::window::WindowAttributes;
use wolia_math::Size;

/// Window configuration.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window title.
    pub title: String,
    /// Initial size.
    pub size: Size,
    /// Minimum size.
    pub min_size: Option<Size>,
    /// Maximum size.
    pub max_size: Option<Size>,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether the window is decorated (title bar, borders).
    pub decorated: bool,
    /// Whether the window is transparent.
    pub transparent: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Wolia".to_string(),
            size: Size::new(1280.0, 720.0),
            min_size: Some(Size::new(400.0, 300.0)),
            max_size: None,
            resizable: true,
            decorated: true,
            transparent: false,
        }
    }
}

impl WindowConfig {
    /// Create a new window configuration.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set the window size.
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }

    /// Convert to winit window attributes.
    pub fn to_window_attributes(&self) -> WindowAttributes {
        let mut attrs = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.size.width as f64,
                self.size.height as f64,
            ))
            .with_resizable(self.resizable)
            .with_decorations(self.decorated)
            .with_transparent(self.transparent);

        if let Some(min) = self.min_size {
            attrs = attrs.with_min_inner_size(winit::dpi::LogicalSize::new(
                min.width as f64,
                min.height as f64,
            ));
        }

        if let Some(max) = self.max_size {
            attrs = attrs.with_max_inner_size(winit::dpi::LogicalSize::new(
                max.width as f64,
                max.height as f64,
            ));
        }

        attrs
    }
}

/// A platform window.
pub struct Window {
    /// The winit window.
    pub inner: winit::window::Window,
}

impl Window {
    /// Get the window size.
    pub fn size(&self) -> Size {
        let size = self.inner.inner_size();
        Size::new(size.width as f32, size.height as f32)
    }

    /// Get the scale factor.
    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    /// Request a redraw.
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }

    /// Set the window title.
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }
}
