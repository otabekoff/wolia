//! Font loading and management.

use fontdb::{Database, ID};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::Path;

use crate::{AssetCache, AssetType, Error, Result};

/// Cached font data.
#[derive(Debug, Clone)]
pub struct CachedFont {
    /// Font ID from fontdb.
    pub id: ID,
    /// Font family name.
    pub family: String,
    /// Font data (if loaded from memory).
    pub data: Option<Vec<u8>>,
}

/// Font manager with caching.
pub struct FontManager {
    /// Font database.
    db: RwLock<Database>,
    /// Font cache.
    cache: AssetCache<CachedFont>,
    /// Family name to font ID mapping.
    family_map: RwLock<HashMap<String, ID>>,
}

impl FontManager {
    /// Create a new font manager.
    pub fn new() -> Self {
        let mut db = Database::new();
        db.load_system_fonts();

        Self {
            db: RwLock::new(db),
            cache: AssetCache::new(50 * 1024 * 1024), // 50 MB cache
            family_map: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new font manager with custom cache size.
    pub fn with_cache_size(cache_size: u64) -> Self {
        let mut db = Database::new();
        db.load_system_fonts();

        Self {
            db: RwLock::new(db),
            cache: AssetCache::new(cache_size),
            family_map: RwLock::new(HashMap::new()),
        }
    }

    /// Load system fonts.
    pub fn load_system_fonts(&self) {
        self.db.write().load_system_fonts();
    }

    /// Load fonts from a directory.
    pub fn load_fonts_dir(&self, path: impl AsRef<Path>) -> Result<()> {
        self.db.write().load_fonts_dir(path);
        Ok(())
    }

    /// Load a font from file.
    pub fn load_font_file(&self, path: impl AsRef<Path>) -> Result<()> {
        self.db.write().load_font_file(path)?;
        Ok(())
    }

    /// Load a font from bytes with caching.
    pub fn load_font_data(&self, family: String, data: Vec<u8>) -> Result<()> {
        let data_len = data.len() as u64;
        self.db.write().load_font_data(data.clone());

        // Cache the font data
        let cached = CachedFont {
            id: fontdb::ID::dummy(),
            family: family.clone(),
            data: Some(data),
        };

        let _ = self
            .cache
            .insert(family.clone(), AssetType::Font, cached, data_len);

        Ok(())
    }

    /// Query for a font.
    pub fn query(&self, query: &fontdb::Query) -> Option<ID> {
        self.db.read().query(query)
    }

    /// Query for a font by family name with caching.
    pub fn query_by_family(&self, family: &str) -> Option<ID> {
        // Check cache first
        if let Some(cached) = self.cache.get_by_path(family) {
            return Some(cached.id);
        }

        // Query database
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(family)],
            ..Default::default()
        };

        if let Some(id) = self.query(&query) {
            // Cache the result
            let cached = CachedFont {
                id,
                family: family.to_string(),
                data: None,
            };

            let _ = self.cache.insert(
                family.to_string(),
                AssetType::Font,
                cached,
                std::mem::size_of::<CachedFont>() as u64,
            );

            return Some(id);
        }

        None
    }

    /// Get the database.
    pub fn database(&self) -> parking_lot::RwLockReadGuard<'_, Database> {
        self.db.read()
    }

    /// Get cache statistics.
    pub fn cache_stats(&self) -> crate::CacheStats {
        self.cache.stats()
    }

    /// Clear the cache.
    pub fn clear_cache(&self) {
        self.cache.clear();
        self.family_map.write().clear();
    }

    /// Get the number of cached fonts.
    pub fn cached_fonts(&self) -> usize {
        self.cache.stats().total_entries
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_creation() {
        let manager = FontManager::new();
        let _ = manager.cache_stats();
    }

    #[test]
    fn test_font_manager_custom_cache() {
        let manager = FontManager::with_cache_size(100 * 1024 * 1024);
        assert_eq!(manager.cache_stats().max_size, 100 * 1024 * 1024);
    }

    #[test]
    fn test_font_cache_stats() {
        let manager = FontManager::new();
        let stats = manager.cache_stats();
        assert!(stats.max_size > 0);
        assert!(stats.usage_percent >= 0.0);
    }
}
