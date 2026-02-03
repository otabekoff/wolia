//! Toolbar component for formatting and editing operations.

use std::collections::HashMap;

/// Button state in the toolbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Button is normal (not pressed).
    Normal,
    /// Button is hovered over.
    Hovered,
    /// Button is pressed/active.
    Active,
    /// Button is disabled.
    Disabled,
}

/// Formatting button representation.
#[derive(Debug, Clone)]
pub struct FormatButton {
    /// Button identifier.
    pub id: String,
    /// Button display label.
    pub label: String,
    /// Button tooltip.
    pub tooltip: String,
    /// Icon name (Lucide icon, without .svg extension).
    pub icon: String,
    /// Current button state.
    pub state: ButtonState,
    /// Keyboard shortcut (e.g., "Ctrl+B").
    pub shortcut: Option<String>,
    /// X position in pixels.
    pub x: f32,
    /// Y position in pixels.
    pub y: f32,
    /// Button width in pixels.
    pub width: f32,
    /// Button height in pixels.
    pub height: f32,
}

impl FormatButton {
    /// Create a new format button.
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        tooltip: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tooltip: tooltip.into(),
            icon: icon.into(),
            state: ButtonState::Normal,
            shortcut: None,
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        }
    }

    /// Set the keyboard shortcut.
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set button dimensions and position.
    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self
    }

    /// Check if a point is inside the button.
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }
}

/// Toolbar state and actions.
pub struct Toolbar {
    /// Whether the toolbar is visible.
    pub visible: bool,
    /// Formatting buttons grouped by category.
    pub buttons: HashMap<String, Vec<FormatButton>>,
    /// Dropdown menus (font size, font family, colors).
    pub dropdowns: HashMap<String, DropdownMenu>,
    /// Currently selected font family.
    pub selected_font: String,
    /// Currently selected font size.
    pub selected_size: f32,
}

impl Toolbar {
    /// Create a new toolbar with all formatting buttons.
    pub fn new() -> Self {
        let mut toolbar = Self {
            visible: true,
            buttons: HashMap::new(),
            dropdowns: HashMap::new(),
            selected_font: "Arial".to_string(),
            selected_size: 12.0,
        };

        toolbar.init_buttons();
        toolbar.init_dropdowns();
        toolbar
    }

    /// Initialize all toolbar buttons.
    fn init_buttons(&mut self) {
        let mut file_buttons = vec![
            FormatButton::new("new", "New", "Create a new document", "file-plus")
                .with_shortcut("Ctrl+N"),
            FormatButton::new("open", "Open", "Open a document", "folder-open")
                .with_shortcut("Ctrl+O"),
            FormatButton::new("save", "Save", "Save the document", "save").with_shortcut("Ctrl+S"),
        ];

        let mut edit_buttons = vec![
            FormatButton::new("undo", "Undo", "Undo", "undo").with_shortcut("Ctrl+Z"),
            FormatButton::new("redo", "Redo", "Redo", "redo").with_shortcut("Ctrl+Y"),
            FormatButton::new("cut", "Cut", "Cut", "scissors").with_shortcut("Ctrl+X"),
            FormatButton::new("copy", "Copy", "Copy", "copy").with_shortcut("Ctrl+C"),
            FormatButton::new("paste", "Paste", "Paste", "clipboard-paste").with_shortcut("Ctrl+V"),
        ];

        let mut format_buttons = vec![
            FormatButton::new("bold", "Bold", "Bold", "bold").with_shortcut("Ctrl+B"),
            FormatButton::new("italic", "Italic", "Italic", "italic").with_shortcut("Ctrl+I"),
            FormatButton::new("underline", "Underline", "Underline", "underline")
                .with_shortcut("Ctrl+U"),
            FormatButton::new("strikethrough", "Strike", "Strikethrough", "strikethrough")
                .with_shortcut("Ctrl+Shift+X"),
        ];

        let mut align_buttons = vec![
            FormatButton::new("align_left", "Left", "Align left", "text-align-start")
                .with_shortcut("Ctrl+L"),
            FormatButton::new(
                "align_center",
                "Center",
                "Align center",
                "text-align-center",
            )
            .with_shortcut("Ctrl+E"),
            FormatButton::new("align_right", "Right", "Align right", "text-align-end")
                .with_shortcut("Ctrl+R"),
            FormatButton::new("align_justify", "Justify", "Justify", "text-align-justify")
                .with_shortcut("Ctrl+J"),
        ];

        let mut list_buttons = vec![
            FormatButton::new("bullet_list", "Bullets", "Bullet list", "list"),
            FormatButton::new("numbered_list", "Numbers", "Numbered list", "list-ordered"),
        ];

        let mut insert_buttons = vec![
            FormatButton::new("insert_image", "Image", "Insert image", "image"),
            FormatButton::new("insert_table", "Table", "Insert table", "table"),
            FormatButton::new("insert_link", "Link", "Insert link", "link").with_shortcut("Ctrl+K"),
        ];

        // Position file buttons
        let mut x = 8.0;
        for button in &mut file_buttons {
            button.x = x;
            button.y = 8.0;
            button.width = 32.0;
            button.height = 32.0;
            x += 40.0;
        }

        // Add separator space
        x += 8.0;

        // Position edit buttons
        for button in &mut edit_buttons {
            button.x = x;
            button.y = 8.0;
            button.width = 32.0;
            button.height = 32.0;
            x += 40.0;
        }

        // Add separator space
        x += 8.0;

        // Position format buttons
        for button in &mut format_buttons {
            button.x = x;
            button.y = 8.0;
            button.width = 32.0;
            button.height = 32.0;
            x += 40.0;
        }

        // Add separator space
        x += 8.0;

        // Position align buttons
        for button in &mut align_buttons {
            button.x = x;
            button.y = 8.0;
            button.width = 32.0;
            button.height = 32.0;
            x += 40.0;
        }

        // Add separator space
        x += 8.0;

        // Position list buttons
        for button in &mut list_buttons {
            button.x = x;
            button.y = 8.0;
            button.width = 32.0;
            button.height = 32.0;
            x += 40.0;
        }

        // Add separator space
        x += 8.0;

        // Position insert buttons
        for button in &mut insert_buttons {
            button.x = x;
            button.y = 8.0;
            button.width = 32.0;
            button.height = 32.0;
            x += 40.0;
        }

        self.buttons.insert("file".to_string(), file_buttons);
        self.buttons.insert("edit".to_string(), edit_buttons);
        self.buttons.insert("format".to_string(), format_buttons);
        self.buttons.insert("align".to_string(), align_buttons);
        self.buttons.insert("list".to_string(), list_buttons);
        self.buttons.insert("insert".to_string(), insert_buttons);
    }

    /// Initialize dropdown menus.
    fn init_dropdowns(&mut self) {
        self.dropdowns.insert(
            "font".to_string(),
            DropdownMenu {
                id: "font".to_string(),
                label: "Font Family".to_string(),
                x: 400.0,
                y: 8.0,
                width: 120.0,
                height: 32.0,
                options: vec![
                    "Arial".to_string(),
                    "Times New Roman".to_string(),
                    "Courier New".to_string(),
                    "Verdana".to_string(),
                    "Georgia".to_string(),
                    "Calibri".to_string(),
                ],
                selected_index: 0,
                expanded: false,
            },
        );

        self.dropdowns.insert(
            "size".to_string(),
            DropdownMenu {
                id: "size".to_string(),
                label: "Size".to_string(),
                x: 530.0,
                y: 8.0,
                width: 60.0,
                height: 32.0,
                options: vec![
                    "8".to_string(),
                    "10".to_string(),
                    "12".to_string(),
                    "14".to_string(),
                    "16".to_string(),
                    "18".to_string(),
                    "20".to_string(),
                    "24".to_string(),
                    "28".to_string(),
                    "32".to_string(),
                ],
                selected_index: 2, // 12pt selected by default
                expanded: false,
            },
        );
    }

    /// Toggle toolbar visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Get all buttons.
    pub fn all_buttons(&self) -> Vec<&FormatButton> {
        self.buttons
            .values()
            .flat_map(|buttons| buttons.iter())
            .collect()
    }

    /// Get button by ID.
    pub fn get_button(&self, id: &str) -> Option<&FormatButton> {
        for buttons in self.buttons.values() {
            if let Some(button) = buttons.iter().find(|b| b.id == id) {
                return Some(button);
            }
        }
        None
    }

    /// Get mutable button by ID.
    pub fn get_button_mut(&mut self, id: &str) -> Option<&mut FormatButton> {
        for buttons in self.buttons.values_mut() {
            if let Some(button) = buttons.iter_mut().find(|b| b.id == id) {
                return Some(button);
            }
        }
        None
    }

    /// Update button state for the given ID.
    pub fn set_button_state(&mut self, id: &str, state: ButtonState) {
        if let Some(button) = self.get_button_mut(id) {
            button.state = state;
        }
    }

    /// Get active (pressed) buttons.
    pub fn active_buttons(&self) -> Vec<&FormatButton> {
        self.all_buttons()
            .into_iter()
            .filter(|b| b.state == ButtonState::Active)
            .collect()
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

/// Dropdown menu for toolbar selections.
#[derive(Debug, Clone)]
pub struct DropdownMenu {
    /// Menu identifier.
    pub id: String,
    /// Menu display label.
    pub label: String,
    /// X position in pixels.
    pub x: f32,
    /// Y position in pixels.
    pub y: f32,
    /// Menu width in pixels.
    pub width: f32,
    /// Menu height in pixels.
    pub height: f32,
    /// Available options.
    pub options: Vec<String>,
    /// Currently selected option index.
    pub selected_index: usize,
    /// Whether the menu is expanded.
    pub expanded: bool,
}

impl DropdownMenu {
    /// Get the selected option.
    pub fn selected_option(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(|s| s.as_str())
    }

    /// Select an option by index.
    pub fn select(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected_index = index;
        }
    }

    /// Toggle menu expansion.
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }
}

/// Toolbar actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarAction {
    // File
    New,
    Open,
    Save,
    SaveAs,
    Export,
    Print,

    // Edit
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Find,
    Replace,

    // Format - Text
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Subscript,
    Superscript,

    // Format - Paragraph
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignJustify,

    // Lists
    BulletList,
    NumberedList,

    // Insert
    InsertImage,
    InsertTable,
    InsertLink,
    InsertPageBreak,
}
