//! Slide model.

use uuid::Uuid;

use crate::animation::Animation;
use crate::shape::Shape;

/// A single slide.
#[derive(Debug, Clone)]
pub struct Slide {
    /// Unique ID.
    pub id: Uuid,
    /// Shapes on the slide.
    pub shapes: Vec<Shape>,
    /// Slide background.
    pub background: Background,
    /// Transition to this slide.
    pub transition: Option<Transition>,
    /// Animations.
    pub animations: Vec<Animation>,
    /// Speaker notes.
    pub notes: String,
    /// Slide layout name.
    pub layout: Option<String>,
}

impl Slide {
    /// Create a new empty slide.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            shapes: Vec::new(),
            background: Background::default(),
            transition: None,
            animations: Vec::new(),
            notes: String::new(),
            layout: None,
        }
    }

    /// Add a shape to the slide.
    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    /// Remove a shape by ID.
    pub fn remove_shape(&mut self, id: Uuid) -> Option<Shape> {
        let index = self.shapes.iter().position(|s| s.id == id)?;
        Some(self.shapes.remove(index))
    }

    /// Get a shape by ID.
    pub fn get_shape(&self, id: Uuid) -> Option<&Shape> {
        self.shapes.iter().find(|s| s.id == id)
    }

    /// Get a shape mutably by ID.
    pub fn get_shape_mut(&mut self, id: Uuid) -> Option<&mut Shape> {
        self.shapes.iter_mut().find(|s| s.id == id)
    }

    /// Move a shape to the front (top of z-order).
    pub fn bring_to_front(&mut self, id: Uuid) {
        if let Some(index) = self.shapes.iter().position(|s| s.id == id) {
            let shape = self.shapes.remove(index);
            self.shapes.push(shape);
        }
    }

    /// Move a shape to the back (bottom of z-order).
    pub fn send_to_back(&mut self, id: Uuid) {
        if let Some(index) = self.shapes.iter().position(|s| s.id == id) {
            let shape = self.shapes.remove(index);
            self.shapes.insert(0, shape);
        }
    }
}

impl Default for Slide {
    fn default() -> Self {
        Self::new()
    }
}

/// Slide background.
#[derive(Debug, Clone)]
pub enum Background {
    /// Solid color.
    Solid([u8; 4]),
    /// Gradient.
    Gradient {
        start: [u8; 4],
        end: [u8; 4],
        angle: f32,
    },
    /// Image.
    Image { src: String, fit: ImageFit },
}

impl Default for Background {
    fn default() -> Self {
        Self::Solid([255, 255, 255, 255])
    }
}

/// Image fitting mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    /// Stretch to fill.
    Fill,
    /// Fit within bounds.
    Contain,
    /// Cover bounds (may crop).
    Cover,
    /// Tile the image.
    Tile,
}

/// Slide transition.
#[derive(Debug, Clone)]
pub struct Transition {
    /// Transition type.
    pub kind: TransitionKind,
    /// Duration in seconds.
    pub duration: f32,
}

/// Transition types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionKind {
    None,
    Fade,
    Push,
    Wipe,
    Split,
    Reveal,
    Cover,
    Dissolve,
    Zoom,
}
