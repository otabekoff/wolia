//! # Wolia Write
//!
//! A modern word processor built on the Wolia platform.

#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use tracing_subscriber::prelude::*;

mod app;
mod automation;
mod editor;
mod sidebar;
mod statusbar;
mod toolbar;
mod ui;
mod workspace;

fn main() -> Result<()> {
    // Check for automation flag
    let args: Vec<String> = std::env::args().collect();
    let run_automation = args.contains(&"--test-scenario".to_string());

    // Initialize logging with explicit default if RUST_LOG is unset
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,wolia_write=debug"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Starting Wolia Write");
    if run_automation {
        tracing::info!("Running in Automation/Test Mode");
    }

    // Run the application
    app::run(run_automation)
}
