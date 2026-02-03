//! Menu system.

/// Application menu.
pub struct Menu {
    /// Whether the menu bar is visible.
    pub visible: bool,
}

impl Menu {
    /// Create a new menu.
    pub fn new() -> Self {
        Self { visible: true }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

/// Menu item.
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Item label.
    pub label: String,
    /// Keyboard shortcut.
    pub shortcut: Option<String>,
    /// Whether the item is enabled.
    pub enabled: bool,
    /// Whether the item is checked (for toggles).
    pub checked: Option<bool>,
}

impl MenuItem {
    /// Create a new menu item.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            enabled: true,
            checked: None,
        }
    }

    /// Set the keyboard shortcut.
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set whether the item is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Make this a toggle item.
    pub fn toggle(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }
}
