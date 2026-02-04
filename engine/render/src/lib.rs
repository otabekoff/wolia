//! # Wolia Render
//!
//! GPU rendering engine for the Wolia platform.
//!
//! This crate provides:
//! - GPU-accelerated rendering via wgpu
//! - Text rendering with cosmic-text
//! - Image rendering
//! - Shape and path rendering
//! - Compositing and effects

#![allow(dead_code, unused_imports, unused_variables)]

pub mod context;
pub mod icon;
pub mod pipeline;
pub mod quad;
pub mod text;
pub mod texture;
pub mod ui;

pub use icon::{IconRenderer, IconTexture, RasterizedIcon, TexturedVertex};
pub use quad::{Quad, QuadRenderer, Vertex};
pub use ui::{RenderRect, colors, dimensions};

use wolia_layout::LayoutTree;
use wolia_math::{Color, Rect, Size};

pub use context::RenderContext;
pub use pipeline::RenderPipeline;
pub use text::TextRenderer;
pub use texture::TextureAtlas;

/// Result type for render operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during rendering.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("GPU error: {0}")]
    Gpu(String),

    #[error("Surface error: {0}")]
    Surface(String),

    #[error("Texture error: {0}")]
    Texture(String),

    #[error("Font error: {0}")]
    Font(String),
}

/// The main renderer.
pub struct Renderer {
    /// Render context.
    context: RenderContext,
    /// Text renderer.
    text_renderer: TextRenderer,
    /// Clear color.
    clear_color: Color,
}

impl Renderer {
    /// Create a new renderer.
    pub async fn new() -> Result<Self> {
        let context = RenderContext::new().await?;
        let text_renderer = TextRenderer::new(&context)?;

        Ok(Self {
            context,
            text_renderer,
            clear_color: Color::WHITE,
        })
    }

    /// Set the clear color.
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    /// Render a layout tree.
    pub fn render(&mut self, _layout: &LayoutTree, _viewport: Rect) -> Result<()> {
        // TODO: Implement full rendering
        Ok(())
    }

    /// Resize the render surface.
    pub fn resize(&mut self, _size: Size) {
        // TODO: Handle resize
    }
}
