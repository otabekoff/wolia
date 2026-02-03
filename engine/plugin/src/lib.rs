//! # Wolia Plugin
//!
//! Plugin system for the Wolia platform.
//!
//! This crate provides:
//! - Plugin ABI definition
//! - Plugin loading and management
//! - Plugin API traits

pub mod api;
pub mod loader;
pub mod manifest;

pub use api::{Plugin, PluginApi};
pub use loader::PluginLoader;
pub use manifest::PluginManifest;

/// Result type for plugin operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in plugin operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Plugin load error: {0}")]
    Load(String),

    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),

    #[error("Plugin API version mismatch")]
    VersionMismatch,

    #[error("Plugin initialization failed: {0}")]
    InitFailed(String),
}

/// Plugin API version.
pub const API_VERSION: u32 = 1;
