//! Document workspace with integrated UI components.

use wolia_core::Document;
use wolia_edit::EditSession;
use wolia_format::{DocumentReader, DocumentWriter};
use wolia_layout::{LayoutEngine, LayoutTree};

use crate::sidebar::Sidebar;
use crate::statusbar::StatusBar;
use crate::toolbar::Toolbar;

/// A document workspace containing the document and editing state with UI components.
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
    /// Toolbar component.
    pub toolbar: Toolbar,
    /// Sidebar component.
    pub sidebar: Sidebar,
    /// Status bar component.
    pub statusbar: StatusBar,
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
            toolbar: Toolbar::new(),
            sidebar: Sidebar::new(),
            statusbar: StatusBar::new(),
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
        workspace.file_path = Some(path.to_path_buf());

        // Update UI with document info.
        workspace.update_ui_from_document();

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
        self.statusbar.mark_saved();

        Ok(())
    }

    /// Save document to a new path.
    pub fn save_to_path(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.file_path = Some(path.as_ref().to_path_buf());
        self.save()
    }

    /// Update the layout.
    pub fn update_layout(&mut self) {
        match self.layout_engine.layout(&self.document) {
            Ok(layout) => self.layout = Some(layout),
            Err(e) => tracing::error!("Layout failed: {}", e),
        }
    }

    /// Update UI components from document state.
    pub fn update_ui_from_document(&mut self) {
        // Update status bar with document statistics.
        if let Ok(content) = self.get_document_text() {
            self.statusbar.update_statistics(&content);
        }

        // Update status bar file path.
        if let Some(path) = &self.file_path {
            self.statusbar
                .set_file_path(Some(path.to_string_lossy().to_string()));
        }

        // Update sidebar with document structure.
        self.sidebar.update_outline(vec![]);
    }

    /// Mark document as modified.
    pub fn mark_modified(&mut self) {
        self.dirty = true;
        self.statusbar
            .set_status(crate::statusbar::StatusIndicator::Modified);
    }

    /// Get the document text content.
    fn get_document_text(&self) -> anyhow::Result<String> {
        // For now, return a placeholder. In a full implementation,
        // this would extract text from the document structure.
        Ok(String::new())
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

    /// Toggle toolbar visibility.
    pub fn toggle_toolbar(&mut self) {
        self.toolbar.toggle();
    }

    /// Toggle sidebar visibility.
    pub fn toggle_sidebar(&mut self) {
        self.sidebar.toggle();
    }

    /// Toggle status bar visibility.
    pub fn toggle_statusbar(&mut self) {
        self.statusbar.toggle();
    }
}
