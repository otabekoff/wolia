//! Plugin manifest.

use serde::{Deserialize, Serialize};

/// Plugin manifest (plugin.json).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin name.
    pub name: String,
    /// Plugin version.
    pub version: String,
    /// Plugin API version.
    pub api_version: u32,
    /// Plugin description.
    pub description: Option<String>,
    /// Plugin author.
    pub author: Option<String>,
    /// Plugin license.
    pub license: Option<String>,
    /// Entry point (library name).
    pub entry: String,
    /// Plugin capabilities.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Plugin dependencies.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

/// Plugin dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency name.
    pub name: String,
    /// Version requirement.
    pub version: String,
}
