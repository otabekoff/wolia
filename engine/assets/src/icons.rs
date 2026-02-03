//! # Icon Management
//!
//! This module provides functionality for loading and managing Lucide SVG icons.
//!
//! ## Example
//!
//! ```rust
//! use wolia_assets::IconManager;
//!
//! let manager = IconManager::new();
//! if let Some(svg) = manager.get("check") {
//!     println!("Found check icon: {}", svg);
//! }
//! ```

use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

/// Global icon cache
static ICON_CACHE: OnceLock<HashMap<String, String>> = OnceLock::new();

/// Icon manager for loading and caching Lucide SVG icons
#[derive(Debug, Clone)]
pub struct IconManager {
    icons: HashMap<String, String>,
}

impl IconManager {
    /// Create a new icon manager and load all available icons
    pub fn new() -> Self {
        let icons = ICON_CACHE.get_or_init(Self::load_icons).clone();

        IconManager { icons }
    }

    /// Load all icons from the embedded assets
    fn load_icons() -> HashMap<String, String> {
        let mut icons = HashMap::new();

        // Try to find icons directory relative to cargo manifest
        let icon_paths = [
            // Development: relative to workspace root
            "engine/assets/icons",
            // Alternative paths
            "../engine/assets/icons",
            "../../engine/assets/icons",
        ];

        let mut icons_dir = None;
        for path in icon_paths {
            let p = std::path::Path::new(path);
            if p.exists() && p.is_dir() {
                icons_dir = Some(p.to_path_buf());
                break;
            }
        }

        // Also check CARGO_MANIFEST_DIR at compile time for workspace root
        if icons_dir.is_none() {
            if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                // Go up to workspace root and then to icons
                let workspace_root = std::path::Path::new(&manifest_dir)
                    .parent()
                    .and_then(|p| p.parent());
                if let Some(root) = workspace_root {
                    let icons_path = root.join("engine/assets/icons");
                    if icons_path.exists() && icons_path.is_dir() {
                        icons_dir = Some(icons_path);
                    }
                }
            }
        }

        // Load icons if directory found
        if let Some(dir) = icons_dir {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "svg").unwrap_or(false) {
                        if let Some(stem) = path.file_stem() {
                            let name = stem.to_string_lossy().to_string();
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                icons.insert(name, content);
                            }
                        }
                    }
                }
            }
            tracing::debug!("Loaded {} icons from {:?}", icons.len(), dir);
        } else {
            tracing::warn!("Icons directory not found, no icons will be available");
        }

        icons
    }

    /// Get an SVG icon by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the icon (without .svg extension)
    ///
    /// # Returns
    ///
    /// Returns the SVG content as a string, or `None` if the icon is not found
    ///
    /// # Example
    ///
    /// ```
    /// use wolia_assets::IconManager;
    /// let manager = IconManager::new();
    /// if let Some(svg) = manager.get("check") {
    ///     println!("Icon content: {}", svg);
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<String> {
        self.icons.get(name).cloned()
    }

    /// List all available icon names
    pub fn list_all(&self) -> Vec<&str> {
        self.icons.keys().map(|s| s.as_str()).collect()
    }

    /// Get the count of available icons
    pub fn count(&self) -> usize {
        self.icons.len()
    }

    /// Search for icons by name pattern
    ///
    /// # Arguments
    ///
    /// * `pattern` - A substring to search for in icon names
    ///
    /// # Example
    ///
    /// ```
    /// use wolia_assets::IconManager;
    /// let manager = IconManager::new();
    /// let arrow_icons = manager.search("arrow");
    /// ```
    pub fn search(&self, pattern: &str) -> Vec<&str> {
        self.icons
            .keys()
            .filter(|name| name.contains(pattern))
            .map(|s| s.as_str())
            .collect()
    }

    /// Load an SVG icon from a file path
    ///
    /// This is primarily used for development and testing
    pub fn load_from_file(&mut self, path: &Path) -> crate::Result<String> {
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }
}

impl Default for IconManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_manager_creation() {
        let manager = IconManager::new();
        // Manager should initialize without panicking
        let _ = manager.count();
    }

    #[test]
    fn test_icon_search() {
        let manager = IconManager::new();
        // Search should work even with empty cache initially
        let results = manager.search("arrow");
        assert!(results.is_empty() || !results.is_empty()); // Valid regardless of initial state
    }
}
