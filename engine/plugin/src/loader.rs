//! Plugin loader.

use std::collections::HashMap;
use std::path::Path;

use crate::{Error, Plugin, PluginManifest, Result};

/// Plugin loader and manager.
pub struct PluginLoader {
    /// Loaded plugins.
    plugins: HashMap<String, Box<dyn Plugin>>,
    /// Search paths for plugins.
    search_paths: Vec<std::path::PathBuf>,
}

impl PluginLoader {
    /// Create a new plugin loader.
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            search_paths: Vec::new(),
        }
    }

    /// Add a search path for plugins.
    pub fn add_search_path(&mut self, path: impl AsRef<Path>) {
        self.search_paths.push(path.as_ref().to_owned());
    }

    /// Load a plugin by name.
    pub fn load(&mut self, name: &str) -> Result<()> {
        // Search for the plugin manifest
        for search_path in &self.search_paths {
            let manifest_path = search_path.join(name).join("plugin.json");
            if manifest_path.exists() {
                return self.load_from_manifest(&manifest_path);
            }
        }

        Err(Error::NotFound(name.to_string()))
    }

    /// Load a plugin from a manifest file.
    pub fn load_from_manifest(&mut self, path: &Path) -> Result<()> {
        let manifest_str = std::fs::read_to_string(path).map_err(|e| Error::Load(e.to_string()))?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_str)
            .map_err(|e| Error::InvalidManifest(e.to_string()))?;

        // Check API version
        if manifest.api_version != crate::API_VERSION {
            return Err(Error::VersionMismatch);
        }

        // TODO: Load the actual plugin library

        Ok(())
    }

    /// Get a loaded plugin.
    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Get a mutable reference to a loaded plugin.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(name)
    }

    /// Unload a plugin.
    pub fn unload(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if let Some(mut plugin) = self.plugins.remove(name) {
            plugin.shutdown();
            Some(plugin)
        } else {
            None
        }
    }

    /// Get all loaded plugin names.
    pub fn loaded_plugins(&self) -> impl Iterator<Item = &str> {
        self.plugins.keys().map(|s| s.as_str())
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}
