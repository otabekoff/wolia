//! # Wolia Platform
//!
//! Platform abstraction layer for the Wolia platform.
//!
//! This crate provides:
//! - Window management
//! - Event handling
//! - OS integration (file dialogs, notifications, etc.)
//! - System clipboard access

pub mod event;
pub mod window;

pub use event::{Event, KeyEvent, MouseEvent};
pub use window::Window;

/// Result type for platform operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in platform operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Window creation failed: {0}")]
    WindowCreation(String),

    #[error("Event loop error: {0}")]
    EventLoop(String),

    #[error("Platform not supported: {0}")]
    Unsupported(String),
}

/// Platform information.
#[derive(Debug)]
pub struct Platform {
    /// Operating system name.
    pub os: &'static str,
    /// OS version.
    pub os_version: Option<String>,
    /// Is this a desktop platform?
    pub is_desktop: bool,
}

impl Platform {
    /// Get information about the current platform.
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS,
            os_version: None, // TODO: Get actual version
            is_desktop: cfg!(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "linux"
            )),
        }
    }

    /// Check if running on Windows.
    pub fn is_windows(&self) -> bool {
        self.os == "windows"
    }

    /// Check if running on macOS.
    pub fn is_macos(&self) -> bool {
        self.os == "macos"
    }

    /// Check if running on Linux.
    pub fn is_linux(&self) -> bool {
        self.os == "linux"
    }
}
