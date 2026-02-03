//! Native .wolia format.

use wolia_core::Document;

use crate::{Error, Result};

/// Magic bytes for the Wolia format.
const MAGIC: &[u8; 5] = b"WOLIA";

/// Current format version.
const VERSION: u8 = 1;

/// Read a document from the native format.
pub fn read(data: &[u8]) -> Result<Document> {
    // Check magic bytes
    if data.len() < 6 || &data[0..5] != MAGIC {
        return Err(Error::Parse("Invalid magic bytes".to_string()));
    }

    let version = data[5];
    if version > VERSION {
        return Err(Error::Parse(format!(
            "Unsupported format version: {}",
            version
        )));
    }

    // TODO: Implement full deserialization
    Ok(Document::new())
}

/// Write a document to the native format.
pub fn write(_document: &Document) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    // Write header
    data.extend_from_slice(MAGIC);
    data.push(VERSION);

    // TODO: Implement full serialization

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let doc = Document::new();
        let data = write(&doc).unwrap();
        let _ = read(&data).unwrap();
    }
}
