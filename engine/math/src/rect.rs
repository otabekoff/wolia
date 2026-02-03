//! Rectangle type.

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::Size;

/// An axis-aligned rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rect {
    /// X coordinate of the origin (top-left).
    pub x: f32,
    /// Y coordinate of the origin (top-left).
    pub y: f32,
    /// Width of the rectangle.
    pub width: f32,
    /// Height of the rectangle.
    pub height: f32,
}

impl Rect {
    /// Create a new rectangle.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create a rectangle from position and size.
    pub fn from_pos_size(pos: Vec2, size: Size) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Create a rectangle at the origin with the given size.
    pub fn from_size(size: Size) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }

    /// Zero-sized rectangle at origin.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Get the origin (top-left corner).
    pub fn origin(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Get the size.
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Get the center point.
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Get the right edge x-coordinate.
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Get the bottom edge y-coordinate.
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Check if a point is inside the rectangle.
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x < self.right() && point.y >= self.y && point.y < self.bottom()
    }

    /// Check if this rectangle intersects another.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Compute the intersection of two rectangles.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(Rect::new(x, y, right - x, bottom - y))
    }

    /// Compute the union (bounding box) of two rectangles.
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Rect::new(x, y, right - x, bottom - y)
    }

    /// Expand the rectangle by the given amount on all sides.
    pub fn expand(&self, amount: f32) -> Rect {
        Rect::new(
            self.x - amount,
            self.y - amount,
            self.width + amount * 2.0,
            self.height + amount * 2.0,
        )
    }

    /// Translate the rectangle by an offset.
    pub fn translate(&self, offset: Vec2) -> Rect {
        Rect::new(
            self.x + offset.x,
            self.y + offset.y,
            self.width,
            self.height,
        )
    }
}
