//! # Test Generator
//!
//! Generate test documents for Wolia testing.

use wolia_core::Document;

/// Generate a test document with various content types.
pub fn generate_test_document() -> Document {
    Document::new()
}

/// Generate a stress-test document.
pub fn generate_stress_document(_paragraphs: usize) -> Document {
    Document::new()
}
