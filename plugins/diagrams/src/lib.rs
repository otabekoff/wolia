//! # Diagrams Plugin
//!
//! Diagram support for Wolia (flowcharts, UML, etc.).

use wolia_plugin::{Plugin, Result};

/// Diagrams plugin.
pub struct DiagramsPlugin {
    name: String,
    version: String,
}

impl DiagramsPlugin {
    pub fn new() -> Self {
        Self {
            name: "diagrams".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for DiagramsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DiagramsPlugin {
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
