//! Style system.

use serde::{Deserialize, Serialize};

/// A named style in the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    /// Style name.
    pub name: String,
    /// Parent style (for inheritance).
    pub parent: Option<String>,
    /// Text formatting.
    pub text: TextStyle,
    /// Paragraph formatting.
    pub paragraph: ParagraphStyle,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            name: "Normal".to_string(),
            parent: None,
            text: TextStyle::default(),
            paragraph: ParagraphStyle::default(),
        }
    }
}

/// Text-level formatting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextStyle {
    /// Font family name.
    pub font_family: Option<String>,
    /// Font size in points.
    pub font_size: Option<f32>,
    /// Font weight (100-900).
    pub font_weight: Option<u16>,
    /// Italic.
    pub italic: Option<bool>,
    /// Underline.
    pub underline: Option<bool>,
    /// Strikethrough.
    pub strikethrough: Option<bool>,
    /// Text color (RGBA).
    pub color: Option<[u8; 4]>,
    /// Background/highlight color (RGBA).
    pub background: Option<[u8; 4]>,
    /// Superscript.
    pub superscript: Option<bool>,
    /// Subscript.
    pub subscript: Option<bool>,
    /// Small caps.
    pub small_caps: Option<bool>,
    /// Letter spacing in ems.
    pub letter_spacing: Option<f32>,
}

/// Paragraph-level formatting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParagraphStyle {
    /// Text alignment.
    pub alignment: Option<Alignment>,
    /// Line height multiplier.
    pub line_height: Option<f32>,
    /// Space before paragraph in points.
    pub space_before: Option<f32>,
    /// Space after paragraph in points.
    pub space_after: Option<f32>,
    /// First line indent in points.
    pub first_line_indent: Option<f32>,
    /// Left margin in points.
    pub margin_left: Option<f32>,
    /// Right margin in points.
    pub margin_right: Option<f32>,
    /// Tab stops.
    pub tab_stops: Option<Vec<TabStop>>,
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

/// A tab stop definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabStop {
    /// Position in points from left margin.
    pub position: f32,
    /// Tab stop alignment.
    pub alignment: TabAlignment,
    /// Leader character.
    pub leader: Option<char>,
}

/// Tab stop alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabAlignment {
    Left,
    Center,
    Right,
    Decimal,
}

/// A collection of styles.
#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    /// Named styles.
    pub styles: indexmap::IndexMap<String, Style>,
}

impl StyleSheet {
    /// Create a new empty stylesheet.
    pub fn new() -> Self {
        let mut sheet = Self::default();
        sheet.styles.insert("Normal".to_string(), Style::default());
        sheet
    }

    /// Get a style by name.
    pub fn get(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }

    /// Add or update a style.
    pub fn insert(&mut self, style: Style) {
        self.styles.insert(style.name.clone(), style);
    }
}
