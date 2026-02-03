//! # Deck Engine
//!
//! Presentation engine for Wolia Deck.
//!
//! Provides:
//! - Slide model
//! - Shape and object model
//! - Animations
//! - Transitions
//! - Speaker notes

pub mod animation;
pub mod presentation;
pub mod shape;
pub mod slide;

pub use animation::{Animation, AnimationEffect};
pub use presentation::Presentation;
pub use shape::{Shape, ShapeKind};
pub use slide::Slide;

/// Result type for deck operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Deck engine errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Slide not found: {0}")]
    SlideNotFound(usize),

    #[error("Shape not found: {0}")]
    ShapeNotFound(uuid::Uuid),

    #[error("Invalid animation: {0}")]
    InvalidAnimation(String),
}
