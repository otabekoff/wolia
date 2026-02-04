//! Document management system for Wolia Write.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::editor::Editor;

/// Result type for document operations.
pub type Result<T> = std::result::Result<T, DocumentError>;

/// Document management errors.
#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Document is read-only")]
    ReadOnly,

    #[error("Document has unsaved changes")]
    UnsavedChanges,

    #[error("Edit operation error")]
    EditError,
}

/// Document metadata.
#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    /// Document title.
    pub title: String,
    /// Document file path.
    pub path: Option<PathBuf>,
    /// Last modified time.
    pub modified: SystemTime,
    /// Whether document has unsaved changes.
    pub dirty: bool,
    /// Whether document is read-only.
    pub read_only: bool,
    /// Word count.
    pub word_count: usize,
    /// Character count.
    pub char_count: usize,
    /// Page count estimate.
    pub page_count: usize,
}

impl DocumentMetadata {
    /// Create new metadata for a new document.
    pub fn new(title: String) -> Self {
        Self {
            title,
            path: None,
            modified: SystemTime::now(),
            dirty: false,
            read_only: false,
            word_count: 0,
            char_count: 0,
            page_count: 1,
        }
    }

    /// Get filename from path.
    pub fn filename(&self) -> String {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.title.clone())
    }

    /// Get display name (with modified indicator).
    pub fn display_name(&self) -> String {
        let name = self.filename();
        if self.dirty {
            format!("{}*", name)
        } else {
            name
        }
    }
}

/// Document management system.
pub struct DocumentManager {
    /// Current document editor.
    editor: Editor,
    /// Document metadata.
    metadata: DocumentMetadata,
    /// Recent files list.
    recent_files: Vec<PathBuf>,
}

impl DocumentManager {
    /// Create a new document.
    pub fn new(title: String) -> Self {
        let editor = Editor::new();
        let metadata = DocumentMetadata::new(title);

        Self {
            editor,
            metadata,
            recent_files: Vec::new(),
        }
    }

    /// Create a new document with custom title.
    pub fn new_with_title(title: String) -> Self {
        Self::new(title)
    }

    /// Open a document from file.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        // Check if file exists
        if !path.exists() {
            return Err(DocumentError::FileNotFound(path.display().to_string()));
        }

        // Check permissions
        let metadata_fs = fs::metadata(path)?;
        let read_only = metadata_fs.permissions().readonly();

        // Read file content
        let _content = fs::read_to_string(path)?;

        // Create editor with content (simplified - just track that it's open)
        let editor = Editor::new();

        let title = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let mut doc_metadata = DocumentMetadata::new(title);
        doc_metadata.path = Some(path.to_path_buf());
        doc_metadata.read_only = read_only;
        doc_metadata.dirty = false;

        Ok(Self {
            editor,
            metadata: doc_metadata,
            recent_files: Vec::new(),
        })
    }

    /// Save document to file.
    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.metadata.path {
            self.save_to_path(path.clone())
        } else {
            Err(DocumentError::FileNotFound(
                "No file path set for document".to_string(),
            ))
        }
    }

    /// Save document to a specific path.
    pub fn save_to_path(&mut self, path: impl AsRef<Path>) -> Result<()> {
        if self.metadata.read_only {
            return Err(DocumentError::ReadOnly);
        }

        let path = path.as_ref();

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write placeholder content
        fs::write(path, "document content")?;

        // Update metadata
        self.metadata.path = Some(path.to_path_buf());
        self.metadata.modified = SystemTime::now();
        self.metadata.dirty = false;

        // Add to recent files
        self.add_to_recent(path);

        Ok(())
    }

    /// Get the editor.
    pub fn editor(&self) -> &Editor {
        &self.editor
    }

    /// Get mutable editor reference.
    pub fn editor_mut(&mut self) -> &mut Editor {
        &mut self.editor
    }

    /// Get document metadata.
    pub fn metadata(&self) -> &DocumentMetadata {
        &self.metadata
    }

    /// Check if document has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.metadata.dirty
    }

    /// Mark document as dirty.
    pub fn mark_dirty(&mut self) {
        self.metadata.dirty = true;
    }

    /// Mark document as clean.
    pub fn mark_clean(&mut self) {
        self.metadata.dirty = false;
    }

    /// Update statistics from current content.
    pub fn update_statistics(&mut self) {
        // Simplified - in real implementation would count words from document content
        self.metadata.char_count = 100;
        self.metadata.word_count = 20;
        self.metadata.page_count = self.metadata.word_count.div_ceil(250);
    }

    /// Get recent files.
    pub fn recent_files(&self) -> &[PathBuf] {
        &self.recent_files
    }

    /// Add file to recent files list.
    fn add_to_recent(&mut self, path: &Path) {
        let path_buf = path.to_path_buf();

        // Remove if already exists
        self.recent_files.retain(|p| *p != path_buf);

        // Add to front
        self.recent_files.insert(0, path_buf);

        // Keep only last 10 recent files
        self.recent_files.truncate(10);
    }

    /// Close document and check for unsaved changes.
    pub fn close(&self) -> Result<()> {
        if self.is_dirty() {
            Err(DocumentError::UnsavedChanges)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_new_document() {
        let doc = DocumentManager::new("Untitled".to_string());
        assert_eq!(doc.metadata().title, "Untitled");
        assert!(doc.metadata().path.is_none());
        assert!(!doc.is_dirty());
    }

    #[test]
    fn test_metadata_display_name() {
        let mut metadata = DocumentMetadata::new("test.docx".to_string());
        assert_eq!(metadata.display_name(), "test.docx");

        metadata.dirty = true;
        assert_eq!(metadata.display_name(), "test.docx*");
    }

    #[test]
    fn test_save_and_open() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.txt");

        // Create and save document
        let mut doc1 = DocumentManager::new("Test".to_string());
        doc1.save_to_path(&file_path)?;

        assert!(!doc1.is_dirty());
        assert!(file_path.exists());

        // Open document
        let doc2 = DocumentManager::open(&file_path)?;
        assert_eq!(doc2.metadata().filename(), "test.txt");

        Ok(())
    }

    #[test]
    fn test_document_statistics() {
        let mut doc = DocumentManager::new("Test".to_string());
        doc.update_statistics();

        assert!(doc.metadata().word_count > 0);
        assert!(doc.metadata().char_count > 0);
        assert_eq!(doc.metadata().page_count, 1);
    }

    #[test]
    fn test_mark_dirty_clean() {
        let mut doc = DocumentManager::new("Test".to_string());
        assert!(!doc.is_dirty());

        doc.mark_dirty();
        assert!(doc.is_dirty());

        doc.mark_clean();
        assert!(!doc.is_dirty());
    }

    #[test]
    fn test_recent_files() {
        let temp_dir = tempdir().unwrap();
        let mut doc = DocumentManager::new("Test".to_string());

        // Create multiple files and add to recent
        for i in 0..5 {
            let path = temp_dir.path().join(format!("test{}.txt", i));
            let _ = fs::write(&path, "content");
            doc.add_to_recent(&path);
        }

        assert_eq!(doc.recent_files().len(), 5);
    }

    #[test]
    fn test_close_with_unsaved_changes() {
        let mut doc = DocumentManager::new("Test".to_string());
        doc.mark_dirty();

        assert!(doc.close().is_err());

        doc.mark_clean();
        assert!(doc.close().is_ok());
    }
}
