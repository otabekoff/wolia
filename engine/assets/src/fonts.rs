//! Font loading and management.

use fontdb::{Database, ID};
use parking_lot::RwLock;
use std::path::Path;

use crate::{Error, Result};

/// Font manager.
pub struct FontManager {
    /// Font database.
    db: RwLock<Database>,
}

impl FontManager {
    /// Create a new font manager.
    pub fn new() -> Self {
        let mut db = Database::new();
        db.load_system_fonts();

        Self {
            db: RwLock::new(db),
        }
    }

    /// Load system fonts.
    pub fn load_system_fonts(&self) {
        self.db.write().load_system_fonts();
    }

    /// Load fonts from a directory.
    pub fn load_fonts_dir(&self, path: impl AsRef<Path>) -> Result<()> {
        self.db.write().load_fonts_dir(path);
        Ok(())
    }

    /// Load a font from file.
    pub fn load_font_file(&self, path: impl AsRef<Path>) -> Result<()> {
        self.db.write().load_font_file(path)?;
        Ok(())
    }

    /// Load a font from bytes.
    pub fn load_font_data(&self, data: Vec<u8>) {
        self.db.write().load_font_data(data);
    }

    /// Query for a font.
    pub fn query(&self, query: &fontdb::Query) -> Option<ID> {
        self.db.read().query(query)
    }

    /// Get font data by ID.
    pub fn with_face_data<R>(&self, id: ID, f: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
        self.db.read().with_face_data(id, f)
    }

    /// Get the number of loaded fonts.
    pub fn len(&self) -> usize {
        self.db.read().len()
    }

    /// Check if the database is empty.
    pub fn is_empty(&self) -> bool {
        self.db.read().is_empty()
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}
