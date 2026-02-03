//! Presentation model.

use wolia_math::Size;

use crate::slide::Slide;

/// A presentation containing slides.
#[derive(Debug, Clone)]
pub struct Presentation {
    /// Slides.
    slides: Vec<Slide>,
    /// Slide size.
    pub slide_size: Size,
    /// Presentation metadata.
    pub metadata: PresentationMetadata,
}

impl Presentation {
    /// Create a new presentation.
    pub fn new() -> Self {
        Self {
            slides: vec![Slide::new()],
            slide_size: Size::new(1920.0, 1080.0), // 16:9 Full HD
            metadata: PresentationMetadata::default(),
        }
    }

    /// Create with a specific slide size.
    pub fn with_size(width: f32, height: f32) -> Self {
        Self {
            slides: vec![Slide::new()],
            slide_size: Size::new(width, height),
            metadata: PresentationMetadata::default(),
        }
    }

    /// Get the number of slides.
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Get a slide by index.
    pub fn slide(&self, index: usize) -> Option<&Slide> {
        self.slides.get(index)
    }

    /// Get a slide mutably.
    pub fn slide_mut(&mut self, index: usize) -> Option<&mut Slide> {
        self.slides.get_mut(index)
    }

    /// Add a new slide.
    pub fn add_slide(&mut self) -> usize {
        let index = self.slides.len();
        self.slides.push(Slide::new());
        index
    }

    /// Insert a slide at a position.
    pub fn insert_slide(&mut self, index: usize) {
        let index = index.min(self.slides.len());
        self.slides.insert(index, Slide::new());
    }

    /// Remove a slide.
    pub fn remove_slide(&mut self, index: usize) -> Option<Slide> {
        if self.slides.len() <= 1 || index >= self.slides.len() {
            return None;
        }
        Some(self.slides.remove(index))
    }

    /// Move a slide.
    pub fn move_slide(&mut self, from: usize, to: usize) {
        if from < self.slides.len() && to < self.slides.len() && from != to {
            let slide = self.slides.remove(from);
            self.slides.insert(to, slide);
        }
    }

    /// Duplicate a slide.
    pub fn duplicate_slide(&mut self, index: usize) -> Option<usize> {
        let slide = self.slides.get(index)?.clone();
        let new_index = index + 1;
        self.slides.insert(new_index, slide);
        Some(new_index)
    }
}

impl Default for Presentation {
    fn default() -> Self {
        Self::new()
    }
}

/// Presentation metadata.
#[derive(Debug, Clone, Default)]
pub struct PresentationMetadata {
    /// Title.
    pub title: Option<String>,
    /// Author.
    pub author: Option<String>,
    /// Subject.
    pub subject: Option<String>,
    /// Keywords.
    pub keywords: Vec<String>,
}
