//! Document workspace.

use wolia_core::Document;
use wolia_edit::EditSession;
use wolia_layout::{LayoutEngine, LayoutTree};

/// A document workspace containing the document and editing state.
pub struct Workspace {
    /// The document being edited.
    pub document: Document,
    /// Edit session (cursor, history, etc.).
    pub session: EditSession,
    /// Layout engine.
    pub layout_engine: LayoutEngine,
    /// Cached layout.
    pub layout: Option<LayoutTree>,
    /// Whether the document has unsaved changes.
    pub dirty: bool,
    /// File path (if saved).
    pub file_path: Option<std::path::PathBuf>,
}

impl Workspace {
    /// Create a new workspace with a document.
    pub fn new(document: Document) -> Self {
        Self {
            document,
            session: EditSession::new(),
            layout_engine: LayoutEngine::new(),
            layout: None,
            dirty: false,
            file_path: None,
        }
    }

    /// Create a workspace from a file.
    pub fn open(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let data = std::fs::read(path)?;

        // Detect format and read document
        let document = wolia_format::WoliaFormat
            .read(&data)
            .map_err(|e| anyhow::anyhow!("Failed to read document: {}", e))?;

        let mut workspace = Self::new(document);
        workspace.file_path = Some(path.to_owned());

        Ok(workspace)
    }

    /// Save the document.
    pub fn save(&mut self) -> anyhow::Result<()> {
        let path = self
            .file_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No file path set"))?;

        let data = wolia_format::WoliaFormat
            .write(&self.document)
            .map_err(|e| anyhow::anyhow!("Failed to write document: {}", e))?;

        std::fs::write(path, data)?;
        self.dirty = false;

        Ok(())
    }

    /// Update the layout.
    pub fn update_layout(&mut self) {
        match self.layout_engine.layout(&self.document) {
            Ok(layout) => self.layout = Some(layout),
            Err(e) => tracing::error!("Layout failed: {}", e),
        }
    }

    /// Get the document title.
    pub fn title(&self) -> String {
        let name = self
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");

        if self.dirty {
            format!("{}* - Wolia Write", name)
        } else {
            format!("{} - Wolia Write", name)
        }
    }
}

use wolia_format::DocumentReader;
use wolia_format::DocumentWriter;
