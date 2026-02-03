//! Sidebar component for document navigation and structure.

use std::collections::VecDeque;

/// Sidebar panel type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SidebarPanel {
    /// Document outline/navigation.
    Outline,
    /// Styles panel.
    Styles,
    /// Comments panel.
    Comments,
    /// Find & Replace panel.
    FindReplace,
}

impl SidebarPanel {
    /// Get the display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Outline => "Document Outline",
            Self::Styles => "Styles",
            Self::Comments => "Comments",
            Self::FindReplace => "Find & Replace",
        }
    }
}

/// Outline item representing a heading or section.
#[derive(Debug, Clone)]
pub struct OutlineItem {
    /// Item title/text.
    pub title: String,
    /// Heading level (1-6).
    pub level: usize,
    /// Line number in document.
    pub line: usize,
    /// Character offset in document.
    pub offset: usize,
    /// Child items for nested headings.
    pub children: Vec<OutlineItem>,
}

impl OutlineItem {
    /// Create a new outline item.
    pub fn new(title: impl Into<String>, level: usize, line: usize, offset: usize) -> Self {
        Self {
            title: title.into(),
            level,
            line,
            offset,
            children: Vec::new(),
        }
    }

    /// Add a child item.
    pub fn add_child(&mut self, child: OutlineItem) {
        self.children.push(child);
    }

    /// Get indentation level for display.
    pub fn indent_pixels(&self) -> f32 {
        ((self.level - 1) as f32) * 16.0
    }
}

/// Document outline/navigator.
#[derive(Debug, Clone)]
pub struct DocumentOutline {
    /// Root outline items.
    pub items: Vec<OutlineItem>,
    /// Currently selected item index.
    pub selected_index: Option<usize>,
    /// Expanded items (by their offset).
    pub expanded: std::collections::HashSet<usize>,
}

impl DocumentOutline {
    /// Create a new outline.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_index: None,
            expanded: std::collections::HashSet::new(),
        }
    }

    /// Add an item to the outline.
    pub fn add_item(&mut self, item: OutlineItem) {
        self.items.push(item);
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected_index = None;
        self.expanded.clear();
    }

    /// Select an item by index.
    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = Some(index);
        }
    }

    /// Toggle item expansion by offset.
    pub fn toggle_item(&mut self, offset: usize) {
        if self.expanded.contains(&offset) {
            self.expanded.remove(&offset);
        } else {
            self.expanded.insert(offset);
        }
    }

    /// Flatten outline to a list for rendering.
    pub fn flatten(&self) -> Vec<(&OutlineItem, f32)> {
        let mut result = Vec::new();
        let mut y = 24.0; // Start below header

        for item in &self.items {
            self.flatten_recursive(item, &mut result, &mut y);
        }

        result
    }

    fn flatten_recursive<'a>(
        &'a self,
        item: &'a OutlineItem,
        result: &mut Vec<(&'a OutlineItem, f32)>,
        y: &mut f32,
    ) {
        result.push((item, *y));
        *y += 24.0;

        if self.expanded.contains(&item.offset) {
            for child in &item.children {
                self.flatten_recursive(child, result, y);
            }
        }
    }
}

impl Default for DocumentOutline {
    fn default() -> Self {
        Self::new()
    }
}

/// Sidebar state and navigation.
pub struct Sidebar {
    /// Whether the sidebar is visible.
    pub visible: bool,
    /// Active panel.
    pub panel: SidebarPanel,
    /// Sidebar width in pixels.
    pub width: f32,
    /// Document outline/navigator.
    pub outline: DocumentOutline,
    /// Search history for find & replace.
    pub search_history: VecDeque<String>,
    /// Maximum search history items.
    pub max_history: usize,
}

impl Sidebar {
    /// Create a new sidebar.
    pub fn new() -> Self {
        Self {
            visible: true,
            panel: SidebarPanel::Outline,
            width: 250.0,
            outline: DocumentOutline::new(),
            search_history: VecDeque::new(),
            max_history: 20,
        }
    }

    /// Toggle sidebar visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Set the active panel.
    pub fn set_panel(&mut self, panel: SidebarPanel) {
        self.panel = panel;
        self.visible = true;
    }

    /// Add a search term to history.
    pub fn add_to_search_history(&mut self, term: String) {
        // Remove if already exists (to move to front).
        self.search_history.retain(|s| s != &term);

        // Add to front.
        self.search_history.push_front(term);

        // Limit history size.
        while self.search_history.len() > self.max_history {
            self.search_history.pop_back();
        }
    }

    /// Get search history.
    pub fn get_search_history(&self) -> Vec<&str> {
        self.search_history.iter().map(|s| s.as_str()).collect()
    }

    /// Update the document outline from current document structure.
    pub fn update_outline(&mut self, headings: Vec<OutlineItem>) {
        self.outline.clear();
        for item in headings {
            self.outline.add_item(item);
        }
    }

    /// Get pixel position for outline item at flattened index.
    pub fn get_outline_item_y(&self, index: usize) -> Option<f32> {
        let flat = self.outline.flatten();
        flat.get(index).map(|(_, y)| *y)
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}
