//! Sidebar component.

/// Sidebar panel type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Sidebar state.
pub struct Sidebar {
    /// Whether the sidebar is visible.
    pub visible: bool,
    /// Active panel.
    pub panel: SidebarPanel,
    /// Sidebar width.
    pub width: f32,
}

impl Sidebar {
    /// Create a new sidebar.
    pub fn new() -> Self {
        Self {
            visible: true,
            panel: SidebarPanel::Outline,
            width: 250.0,
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
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}
