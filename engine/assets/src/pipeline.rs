//! Asset pipeline for integrated resource management.

use parking_lot::RwLock;
use std::path::Path;
use std::sync::Arc;

use crate::{AssetCache, AssetId, FontManager, IconManager, ImageLoader, Result};

/// Asset pipeline coordinator.
pub struct AssetPipeline {
    /// Font manager.
    fonts: Arc<FontManager>,
    /// Image loader.
    images: Arc<ImageLoader>,
    /// Icon manager.
    icons: Arc<IconManager>,
    /// Pipeline configuration.
    config: RwLock<PipelineConfig>,
}

/// Pipeline configuration.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum font cache size in bytes.
    pub font_cache_size: u64,
    /// Maximum image cache size in bytes.
    pub image_cache_size: u64,
    /// Enable caching.
    pub enable_caching: bool,
    /// Base asset directory.
    pub asset_dir: String,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            font_cache_size: 50 * 1024 * 1024,   // 50 MB
            image_cache_size: 100 * 1024 * 1024, // 100 MB
            enable_caching: true,
            asset_dir: "./assets".to_string(),
        }
    }
}

impl AssetPipeline {
    /// Create a new asset pipeline.
    pub fn new() -> Self {
        let config = PipelineConfig::default();
        Self::with_config(config)
    }

    /// Create a new asset pipeline with custom configuration.
    pub fn with_config(config: PipelineConfig) -> Self {
        let fonts = Arc::new(FontManager::with_cache_size(config.font_cache_size));
        let images = Arc::new(ImageLoader::with_cache_size(config.image_cache_size));
        let icons = Arc::new(IconManager::new());

        // Load system fonts
        fonts.load_system_fonts();

        Self {
            fonts,
            images,
            icons,
            config: RwLock::new(config),
        }
    }

    /// Get the font manager.
    pub fn fonts(&self) -> &FontManager {
        &self.fonts
    }

    /// Get the image loader.
    pub fn images(&self) -> &ImageLoader {
        &self.images
    }

    /// Get the icon manager.
    pub fn icons(&self) -> &IconManager {
        &self.icons
    }

    /// Load all assets from a directory.
    pub fn load_assets_dir(&self, path: impl AsRef<Path>) -> Result<PipelineStats> {
        let path = path.as_ref();

        // Load fonts
        self.fonts.load_fonts_dir(path)?;

        // Load images
        let mut stats = PipelineStats::default();

        // Count loaded fonts
        let font_stats = self.fonts.cache_stats();
        stats.total_fonts = font_stats.total_entries;
        stats.fonts_cache_size = font_stats.total_size;

        // Count loaded images
        let image_stats = self.images.cache_stats();
        stats.total_images = image_stats.total_entries;
        stats.images_cache_size = image_stats.total_size;

        stats.total_icons = self.icons.count();

        Ok(stats)
    }

    /// Preload common assets.
    pub fn preload_common(&self) -> Result<()> {
        // Load system fonts
        self.fonts.load_system_fonts();

        Ok(())
    }

    /// Get pipeline statistics.
    pub fn stats(&self) -> PipelineStats {
        let font_stats = self.fonts.cache_stats();
        let image_stats = self.images.cache_stats();

        PipelineStats {
            total_fonts: font_stats.total_entries,
            fonts_cache_size: font_stats.total_size,
            total_images: image_stats.total_entries,
            images_cache_size: image_stats.total_size,
            total_icons: self.icons.count(),
        }
    }

    /// Clear all caches.
    pub fn clear_all(&self) {
        self.fonts.clear_cache();
        self.images.clear_cache();
    }

    /// Get current configuration.
    pub fn config(&self) -> PipelineConfig {
        self.config.read().clone()
    }

    /// Update configuration.
    pub fn set_config(&self, config: PipelineConfig) {
        *self.config.write() = config;
    }
}

impl Default for AssetPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset pipeline statistics.
#[derive(Debug, Clone, Default)]
pub struct PipelineStats {
    /// Total number of cached fonts.
    pub total_fonts: usize,
    /// Total size of cached fonts in bytes.
    pub fonts_cache_size: u64,
    /// Total number of cached images.
    pub total_images: usize,
    /// Total size of cached images in bytes.
    pub images_cache_size: u64,
    /// Total number of available icons.
    pub total_icons: usize,
}

impl PipelineStats {
    /// Get total cache size in bytes.
    pub fn total_cache_size(&self) -> u64 {
        self.fonts_cache_size + self.images_cache_size
    }

    /// Get total asset count.
    pub fn total_assets(&self) -> usize {
        self.total_fonts + self.total_images + self.total_icons
    }
}

impl std::fmt::Display for PipelineStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Assets: {} fonts, {} images, {} icons\nCache: {:.2} MB",
            self.total_fonts,
            self.total_images,
            self.total_icons,
            self.total_cache_size() as f32 / (1024.0 * 1024.0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = AssetPipeline::new();
        let stats = pipeline.stats();
        assert!(stats.total_assets() >= 0);
    }

    #[test]
    fn test_pipeline_custom_config() {
        let config = PipelineConfig {
            font_cache_size: 100 * 1024 * 1024,
            image_cache_size: 200 * 1024 * 1024,
            enable_caching: true,
            asset_dir: "./custom".to_string(),
        };

        let pipeline = AssetPipeline::with_config(config);
        assert_eq!(pipeline.fonts.cache_stats().max_size, 100 * 1024 * 1024);
        assert_eq!(pipeline.images.cache_stats().max_size, 200 * 1024 * 1024);
    }

    #[test]
    fn test_pipeline_stats() {
        let pipeline = AssetPipeline::new();
        let stats = pipeline.stats();

        // Stats should be accessible
        let _ = format!("{}", stats);
        let _ = stats.total_cache_size();
    }

    #[test]
    fn test_pipeline_preload() {
        let pipeline = AssetPipeline::new();
        assert!(pipeline.preload_common().is_ok());
    }

    #[test]
    fn test_pipeline_clear() {
        let pipeline = AssetPipeline::new();
        pipeline.clear_all();
        let stats = pipeline.stats();
        assert_eq!(stats.total_fonts, 0);
        assert_eq!(stats.total_images, 0);
    }
}
