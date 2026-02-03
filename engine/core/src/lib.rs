//! # Wolia Core
//!
//! The core document model for the Wolia platform.
//!
//! This crate provides the foundational types and structures that all Wolia
//! applications share:
//!
//! - Document structure and content model
//! - Text representation and attributes
//! - Style system
//! - Content nodes (paragraphs, tables, images, etc.)

pub mod content;
pub mod document;
pub mod node;
pub mod style;
pub mod text;

pub use content::*;
pub use document::Document;
pub use node::Node;
pub use style::Style;
pub use text::Text;

/// Result type for core operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in core operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid document structure: {0}")]
    InvalidStructure(String),

    #[error("Node not found: {0}")]
    NodeNotFound(uuid::Uuid),

    #[error("Style not found: {0}")]
    StyleNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
