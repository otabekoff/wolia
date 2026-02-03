//! Image loading with caching.

use image::{DynamicImage, GenericImageView, ImageFormat, Rgba, RgbaImage};
use std::path::Path;

use crate::{AssetCache, AssetId, AssetType, Error, Result};

/// Cached image data.
#[derive(Debug, Clone)]
pub struct CachedImage {
    /// Image dimensions (width, height).
    pub dimensions: (u32, u32),
    /// Color type (simplified to RGBA8).
    pub color_type: &'static str,
    /// Image buffer (RGBA8).
    pub buffer: Vec<u8>,
}

impl CachedImage {
    /// Create from a DynamicImage.
    pub fn from_dynamic(img: &DynamicImage) -> Self {
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        Self {
            dimensions: (width, height),
            color_type: "rgba8",
            buffer: rgba.into_raw(),
        }
    }

    /// Get as DynamicImage.
    pub fn to_dynamic(&self) -> DynamicImage {
        let img = RgbaImage::from_raw(self.dimensions.0, self.dimensions.1, self.buffer.clone())
            .expect("Invalid image buffer");
        DynamicImage::ImageRgba8(img)
    }
}

/// Image loader with caching.
pub struct ImageLoader {
    /// Image cache.
    cache: AssetCache<CachedImage>,
}

impl ImageLoader {
    /// Create a new image loader.
    pub fn new() -> Self {
        Self {
            cache: AssetCache::new(100 * 1024 * 1024), // 100 MB cache
        }
    }

    /// Create a new image loader with custom cache size.
    pub fn with_cache_size(cache_size: u64) -> Self {
        Self {
            cache: AssetCache::new(cache_size),
        }
    }

    /// Load an image from a file with caching.
    pub fn load_file(&self, path: impl AsRef<Path>) -> Result<AssetId> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        // Check cache first
        if let Some(_cached) = self.cache.get_by_path(&path_str) {
            if let Ok(entries) = self.get_cached_paths() {
                if let Some(id) = entries.get(&path_str) {
                    return Ok(*id);
                }
            }
        }

        // Load from file
        let img = image::open(&path).map_err(|e| Error::Image(e.to_string()))?;
        let cached = CachedImage::from_dynamic(&img);
        let size = cached.buffer.len() as u64;

        let id = self
            .cache
            .insert(path_str, AssetType::Image, cached, size)?;

        Ok(id)
    }

    /// Load an image from bytes with caching.
    pub fn load_bytes(&self, name: String, data: &[u8]) -> Result<AssetId> {
        // Check cache first
        if let Some(_cached) = self.cache.get_by_path(&name) {
            if let Ok(entries) = self.get_cached_paths() {
                if let Some(id) = entries.get(&name) {
                    return Ok(*id);
                }
            }
        }

        // Load from memory
        let img = image::load_from_memory(data).map_err(|e| Error::Image(e.to_string()))?;
        let cached = CachedImage::from_dynamic(&img);
        let size = cached.buffer.len() as u64;

        let id = self.cache.insert(name, AssetType::Image, cached, size)?;

        Ok(id)
    }

    /// Load an image with a specific format.
    pub fn load_bytes_with_format(
        &self,
        name: String,
        data: &[u8],
        format: ImageFormat,
    ) -> Result<AssetId> {
        let img = image::load_from_memory_with_format(data, format)
            .map_err(|e| Error::Image(e.to_string()))?;
        let cached = CachedImage::from_dynamic(&img);
        let size = cached.buffer.len() as u64;

        let id = self.cache.insert(name, AssetType::Image, cached, size)?;

        Ok(id)
    }

    /// Detect image format from bytes.
    pub fn detect_format(data: &[u8]) -> Option<ImageFormat> {
        image::guess_format(data).ok()
    }

    /// Get a cached image by ID.
    pub fn get_cached(&self, id: AssetId) -> Option<CachedImage> {
        self.cache.get(id)
    }

    /// Get all cached image paths (for testing/debugging).
    pub fn get_cached_paths(&self) -> Result<std::collections::HashMap<String, AssetId>> {
        // This is a simplified implementation
        // In production, you'd track paths separately
        Ok(std::collections::HashMap::new())
    }

    /// Clear the cache.
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Get cache statistics.
    pub fn cache_stats(&self) -> crate::CacheStats {
        self.cache.stats()
    }

    /// Get the number of cached images.
    pub fn cached_images(&self) -> usize {
        self.cache.stats().total_entries
    }
}

impl Default for ImageLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Supported image formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Bmp,
    Ico,
    Tiff,
}

impl SupportedFormat {
    /// Get the file extension.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Gif => "gif",
            Self::WebP => "webp",
            Self::Bmp => "bmp",
            Self::Ico => "ico",
            Self::Tiff => "tiff",
        }
    }

    /// Get the MIME type.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::WebP => "image/webp",
            Self::Bmp => "image/bmp",
            Self::Ico => "image/x-icon",
            Self::Tiff => "image/tiff",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_loader_creation() {
        let loader = ImageLoader::new();
        assert_eq!(loader.cached_images(), 0);
    }

    #[test]
    fn test_image_loader_custom_cache() {
        let loader = ImageLoader::with_cache_size(200 * 1024 * 1024);
        assert_eq!(loader.cache_stats().max_size, 200 * 1024 * 1024);
    }

    #[test]
    fn test_cached_image_conversion() {
        let img = image::RgbaImage::new(100, 100);
        let dynamic = DynamicImage::ImageRgba8(img);
        let cached = CachedImage::from_dynamic(&dynamic);

        assert_eq!(cached.dimensions, (100, 100));
        assert_eq!(cached.buffer.len(), 40000); // 100 * 100 * 4 bytes
    }
}
