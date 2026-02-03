//! # Wolia Deck
//!
//! A modern presentation application built on the Wolia platform.

#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use tracing_subscriber::prelude::*;

mod app;
mod slides;
mod timeline;
mod transitions;
mod ui;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Wolia Deck");

    // Run the application
    app::run()
}
