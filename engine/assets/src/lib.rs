//! # Wolia Assets
//!
//! Asset loading for the Wolia platform.
//!
//! This crate provides:
//! - Font loading and management
//! - Image loading
//! - Resource caching

#![allow(dead_code, unused_imports, unused_variables)]

pub mod cache;
pub mod fonts;
pub mod icons;
pub mod images;
pub mod pipeline;

pub use cache::{AssetCache, AssetId, AssetMetadata, AssetType, CacheStats};
pub use fonts::FontManager;
pub use icons::IconManager;
pub use images::ImageLoader;
pub use pipeline::{AssetPipeline, PipelineConfig, PipelineStats};

/// Result type for asset operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during asset loading.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Font error: {0}")]
    Font(String),

    #[error("Image error: {0}")]
    Image(String),

    #[error("Asset not found: {0}")]
    NotFound(String),
}
