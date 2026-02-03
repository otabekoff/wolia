//! # LaTeX Plugin
//!
//! LaTeX/math equation support for Wolia.

use wolia_plugin::{Plugin, Result};

/// LaTeX plugin.
pub struct LatexPlugin {
    name: String,
    version: String,
}

impl LatexPlugin {
    pub fn new() -> Self {
        Self {
            name: "latex".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for LatexPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LatexPlugin {
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
