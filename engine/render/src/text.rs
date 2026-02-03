//! Text rendering.

use cosmic_text::{FontSystem, SwashCache};
use parking_lot::Mutex;

use crate::context::RenderContext;
use crate::{Error, Result};

/// Text renderer using cosmic-text.
pub struct TextRenderer {
    /// Font system.
    font_system: Mutex<FontSystem>,
    /// Glyph cache.
    swash_cache: Mutex<SwashCache>,
}

impl TextRenderer {
    /// Create a new text renderer.
    pub fn new(_context: &RenderContext) -> Result<Self> {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();

        Ok(Self {
            font_system: Mutex::new(font_system),
            swash_cache: Mutex::new(swash_cache),
        })
    }

    /// Get mutable access to the font system.
    pub fn font_system(&self) -> parking_lot::MutexGuard<'_, FontSystem> {
        self.font_system.lock()
    }

    /// Load a font from bytes.
    pub fn load_font(&self, data: Vec<u8>) -> Result<()> {
        self.font_system.lock().db_mut().load_font_data(data);
        Ok(())
    }
}
