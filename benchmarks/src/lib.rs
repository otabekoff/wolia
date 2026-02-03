//! Wolia benchmarks library.

pub mod utils {
    use wolia_core::Document;

    /// Create a test document with N paragraphs.
    pub fn create_test_document(_paragraphs: usize) -> Document {
        Document::new()
    }
}
