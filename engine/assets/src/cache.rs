//! Asset caching system for efficient resource management.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::Result;

/// Unique asset identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(Uuid);

impl AssetId {
    /// Generate a new asset ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Get the UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for AssetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset metadata.
#[derive(Debug, Clone)]
pub struct AssetMetadata {
    /// Asset ID.
    pub id: AssetId,

    /// Asset path or name.
    pub path: String,

    /// Asset size in bytes.
    pub size: u64,

    /// Asset type.
    pub asset_type: AssetType,
}

/// Type of asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    /// Font asset.
    Font,
    /// Image asset.
    Image,
    /// Icon asset.
    Icon,
}

/// Cache entry with metadata and usage info.
#[derive(Debug, Clone)]
struct CacheEntry<T: Clone> {
    /// Asset metadata.
    metadata: AssetMetadata,
    /// Cached asset data.
    data: T,
    /// Number of references.
    ref_count: usize,
    /// Last access time (in arbitrary units).
    last_accessed: u64,
}

/// Generic asset cache with reference counting and LRU eviction.
pub struct AssetCache<T: Clone> {
    /// Cached entries.
    entries: RwLock<HashMap<AssetId, CacheEntry<T>>>,
    /// Path to ID mapping for quick lookup.
    path_map: RwLock<HashMap<String, AssetId>>,
    /// Maximum cache size in bytes.
    max_size: u64,
    /// Current cache size in bytes.
    current_size: RwLock<u64>,
    /// Access counter for LRU tracking.
    access_counter: RwLock<u64>,
}

impl<T: Clone> AssetCache<T> {
    /// Create a new asset cache with maximum size.
    pub fn new(max_size: u64) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            path_map: RwLock::new(HashMap::new()),
            max_size,
            current_size: RwLock::new(0),
            access_counter: RwLock::new(0),
        }
    }

    /// Insert an asset into the cache.
    pub fn insert(
        &self,
        path: String,
        asset_type: AssetType,
        data: T,
        size: u64,
    ) -> Result<AssetId> {
        let id = AssetId::new();
        let metadata = AssetMetadata {
            id,
            path: path.clone(),
            size,
            asset_type,
        };

        let mut entries = self.entries.write();
        let mut path_map = self.path_map.write();
        let mut current_size = self.current_size.write();

        // Evict entries if necessary
        while *current_size + size > self.max_size && !entries.is_empty() {
            self.evict_lru_unlocked(&mut entries, &mut path_map, &mut current_size);
        }

        // Insert new entry
        let entry = CacheEntry {
            metadata,
            data,
            ref_count: 1,
            last_accessed: *self.access_counter.read(),
        };

        *current_size += size;
        entries.insert(id, entry);
        path_map.insert(path, id);

        Ok(id)
    }

    /// Retrieve an asset from the cache.
    pub fn get(&self, id: AssetId) -> Option<T> {
        let mut entries = self.entries.write();
        let mut counter = self.access_counter.write();
        *counter += 1;

        if let Some(entry) = entries.get_mut(&id) {
            entry.last_accessed = *counter;
            entry.ref_count += 1;
            Some(entry.data.clone())
        } else {
            None
        }
    }

    /// Retrieve an asset by path.
    pub fn get_by_path(&self, path: &str) -> Option<T> {
        let path_map_guard = self.path_map.read();
        if let Some(id) = path_map_guard.get(path) {
            let id_copy = *id;
            drop(path_map_guard);
            self.get(id_copy)
        } else {
            None
        }
    }

    /// Release a reference to an asset.
    pub fn release(&self, id: AssetId) {
        let mut entries = self.entries.write();
        if let Some(entry) = entries.get_mut(&id) {
            if entry.ref_count > 0 {
                entry.ref_count -= 1;
            }
        }
    }

    /// Remove an asset from the cache.
    pub fn remove(&self, id: AssetId) -> Option<T> {
        let mut entries = self.entries.write();
        let mut path_map = self.path_map.write();
        let mut current_size = self.current_size.write();

        if let Some(entry) = entries.remove(&id) {
            path_map.remove(&entry.metadata.path);
            *current_size = current_size.saturating_sub(entry.metadata.size);
            Some(entry.data)
        } else {
            None
        }
    }

    /// Clear all entries from the cache.
    pub fn clear(&self) {
        self.entries.write().clear();
        self.path_map.write().clear();
        *self.current_size.write() = 0;
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        let entries = self.entries.read();
        let current_size = *self.current_size.read();

        CacheStats {
            total_entries: entries.len(),
            total_size: current_size,
            max_size: self.max_size,
            usage_percent: if self.max_size > 0 {
                (current_size as f32 / self.max_size as f32) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Evict the least recently used entry.
    fn evict_lru_unlocked(
        &self,
        entries: &mut HashMap<AssetId, CacheEntry<T>>,
        path_map: &mut HashMap<String, AssetId>,
        current_size: &mut u64,
    ) {
        if let Some((id, entry)) = entries
            .iter()
            .min_by_key(|(_, e)| (e.ref_count, e.last_accessed))
            .map(|(k, v)| (*k, v.clone()))
        {
            entries.remove(&id);
            path_map.remove(&entry.metadata.path);
            *current_size = current_size.saturating_sub(entry.metadata.size);
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total number of cached entries.
    pub total_entries: usize,
    /// Total size of cached entries in bytes.
    pub total_size: u64,
    /// Maximum cache size in bytes.
    pub max_size: u64,
    /// Cache usage as percentage.
    pub usage_percent: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let cache: AssetCache<Vec<u8>> = AssetCache::new(1000);
        let data = vec![1, 2, 3, 4, 5];
        let size = data.len() as u64;

        let id = cache
            .insert("test.png".to_string(), AssetType::Image, data.clone(), size)
            .unwrap();

        assert_eq!(cache.get(id), Some(data));
    }

    #[test]
    fn test_cache_get_by_path() {
        let cache: AssetCache<Vec<u8>> = AssetCache::new(1000);
        let data = vec![1, 2, 3];
        let path = "assets/image.png".to_string();

        cache
            .insert(path.clone(), AssetType::Image, data.clone(), 3)
            .unwrap();

        assert_eq!(cache.get_by_path(&path), Some(data));
    }

    #[test]
    fn test_cache_eviction() {
        let cache: AssetCache<Vec<u8>> = AssetCache::new(100);

        // Insert items that exceed cache size
        let _ = cache.insert("file1.png".to_string(), AssetType::Image, vec![0; 60], 60);
        let id2 = cache
            .insert("file2.png".to_string(), AssetType::Image, vec![0; 60], 60)
            .unwrap();

        // First item should be evicted
        let stats = cache.stats();
        assert!(stats.total_size <= 100);
        assert_eq!(cache.get(id2), Some(vec![0; 60]));
    }

    #[test]
    fn test_cache_stats() {
        let cache: AssetCache<Vec<u8>> = AssetCache::new(1000);
        cache
            .insert("test1.png".to_string(), AssetType::Image, vec![0; 300], 300)
            .unwrap();
        cache
            .insert("test2.png".to_string(), AssetType::Image, vec![0; 200], 200)
            .unwrap();

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.total_size, 500);
        assert_eq!(stats.max_size, 1000);
        assert!(stats.usage_percent > 0.0);
    }

    #[test]
    fn test_cache_clear() {
        let cache: AssetCache<Vec<u8>> = AssetCache::new(1000);
        cache
            .insert("test.png".to_string(), AssetType::Image, vec![1, 2, 3], 3)
            .unwrap();

        cache.clear();
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.total_size, 0);
    }
}
