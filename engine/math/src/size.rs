//! Size type.

use serde::{Deserialize, Serialize};

/// A 2D size with width and height.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Size {
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

impl Size {
    /// Create a new size.
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Zero size.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Square size with equal width and height.
    pub const fn square(side: f32) -> Self {
        Self::new(side, side)
    }

    /// Check if the size is zero.
    pub fn is_zero(&self) -> bool {
        self.width == 0.0 && self.height == 0.0
    }

    /// Get the area.
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Get the aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0.0 {
            0.0
        } else {
            self.width / self.height
        }
    }

    /// Scale by a factor.
    pub fn scale(&self, factor: f32) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }

    /// Scale to fit within a maximum size while preserving aspect ratio.
    pub fn fit_within(&self, max: Size) -> Self {
        let scale_x = max.width / self.width;
        let scale_y = max.height / self.height;
        let scale = scale_x.min(scale_y);
        self.scale(scale)
    }

    /// Scale to fill a minimum size while preserving aspect ratio.
    pub fn fill(&self, min: Size) -> Self {
        let scale_x = min.width / self.width;
        let scale_y = min.height / self.height;
        let scale = scale_x.max(scale_y);
        self.scale(scale)
    }
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self::new(width, height)
    }
}

impl From<[f32; 2]> for Size {
    fn from([width, height]: [f32; 2]) -> Self {
        Self::new(width, height)
    }
}
