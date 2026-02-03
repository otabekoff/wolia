//! Image loading.

use image::{DynamicImage, ImageFormat};
use std::path::Path;

use crate::{Error, Result};

/// Image loader.
pub struct ImageLoader;

impl ImageLoader {
    /// Load an image from a file.
    pub fn load_file(path: impl AsRef<Path>) -> Result<DynamicImage> {
        image::open(path).map_err(|e| Error::Image(e.to_string()))
    }

    /// Load an image from bytes.
    pub fn load_bytes(data: &[u8]) -> Result<DynamicImage> {
        image::load_from_memory(data).map_err(|e| Error::Image(e.to_string()))
    }

    /// Load an image with a specific format.
    pub fn load_bytes_with_format(data: &[u8], format: ImageFormat) -> Result<DynamicImage> {
        image::load_from_memory_with_format(data, format).map_err(|e| Error::Image(e.to_string()))
    }

    /// Detect image format from bytes.
    pub fn detect_format(data: &[u8]) -> Option<ImageFormat> {
        image::guess_format(data).ok()
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
