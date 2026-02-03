//! # Wolia Write
//!
//! A modern word processor built on the Wolia platform.

#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use tracing_subscriber::prelude::*;

mod app;
mod editor;
mod sidebar;
mod statusbar;
mod toolbar;
mod ui;
mod workspace;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Wolia Write");

    // Run the application
    app::run()
}
