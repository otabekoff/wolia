//! # Code Blocks Plugin
//!
//! Syntax-highlighted code blocks for Wolia.

use wolia_plugin::{Plugin, Result};

/// Code blocks plugin.
pub struct CodeBlocksPlugin {
    name: String,
    version: String,
}

impl CodeBlocksPlugin {
    pub fn new() -> Self {
        Self {
            name: "code-blocks".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for CodeBlocksPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for CodeBlocksPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) {
        // Cleanup
    }
}
