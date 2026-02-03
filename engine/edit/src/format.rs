//! Text formatting system for rich text support.

use std::collections::HashMap;

/// Text style attributes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TextStyle {
    /// Bold text.
    Bold,
    /// Italic text.
    Italic,
    /// Underlined text.
    Underline,
    /// Strikethrough text.
    Strikethrough,
}

/// Color representation (RGBA).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255).
    pub red: u8,
    /// Green component (0-255).
    pub green: u8,
    /// Blue component (0-255).
    pub blue: u8,
    /// Alpha component (0-255).
    pub alpha: u8,
}

impl Color {
    /// Create a new color.
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Create a color from RGB (alpha = 255).
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(red, green, blue, 255)
    }

    /// Black color.
    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    /// White color.
    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    /// Convert to RGBA hex value.
    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 24)
            | ((self.green as u32) << 16)
            | ((self.blue as u32) << 8)
            | (self.alpha as u32)
    }

    /// Parse from hex color string (#RRGGBB or #RRGGBBAA).
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::new(r, g, b, a))
            }
            _ => None,
        }
    }
}

/// Text formatting properties.
#[derive(Debug, Clone)]
pub struct TextFormat {
    /// Active text styles (bold, italic, etc.).
    styles: std::collections::HashSet<TextStyle>,
    /// Font family name.
    font_family: String,
    /// Font size in points.
    font_size: f32,
    /// Text color.
    text_color: Color,
    /// Background color.
    background_color: Option<Color>,
}

impl TextFormat {
    /// Create a new text format with defaults.
    pub fn new() -> Self {
        Self {
            styles: std::collections::HashSet::new(),
            font_family: "Arial".to_string(),
            font_size: 12.0,
            text_color: Color::black(),
            background_color: None,
        }
    }

    /// Set font family.
    pub fn with_font_family(mut self, family: String) -> Self {
        self.font_family = family;
        self
    }

    /// Set font size in points.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size.max(1.0);
        self
    }

    /// Set text color.
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Set background color.
    pub fn with_background_color(mut self, color: Option<Color>) -> Self {
        self.background_color = color;
        self
    }

    /// Add a style.
    pub fn add_style(&mut self, style: TextStyle) {
        self.styles.insert(style);
    }

    /// Remove a style.
    pub fn remove_style(&mut self, style: TextStyle) {
        self.styles.remove(&style);
    }

    /// Toggle a style.
    pub fn toggle_style(&mut self, style: TextStyle) {
        if self.styles.contains(&style) {
            self.styles.remove(&style);
        } else {
            self.styles.insert(style);
        }
    }

    /// Check if a style is active.
    pub fn has_style(&self, style: TextStyle) -> bool {
        self.styles.contains(&style)
    }

    /// Check if text is bold.
    pub fn is_bold(&self) -> bool {
        self.has_style(TextStyle::Bold)
    }

    /// Check if text is italic.
    pub fn is_italic(&self) -> bool {
        self.has_style(TextStyle::Italic)
    }

    /// Check if text is underlined.
    pub fn is_underlined(&self) -> bool {
        self.has_style(TextStyle::Underline)
    }

    /// Get font family.
    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    /// Get font size.
    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    /// Get text color.
    pub fn text_color(&self) -> Color {
        self.text_color
    }

    /// Get background color.
    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    /// Get all active styles.
    pub fn styles(&self) -> Vec<TextStyle> {
        self.styles.iter().copied().collect()
    }
}

impl Default for TextFormat {
    fn default() -> Self {
        Self::new()
    }
}

/// Formatted text span.
#[derive(Debug, Clone)]
pub struct FormattedSpan {
    /// Text content.
    content: String,
    /// Formatting applied to this span.
    format: TextFormat,
}

impl FormattedSpan {
    /// Create a new formatted span.
    pub fn new(content: String, format: TextFormat) -> Self {
        Self { content, format }
    }

    /// Get the text content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the formatting.
    pub fn format(&self) -> &TextFormat {
        &self.format
    }

    /// Get mutable formatting.
    pub fn format_mut(&mut self) -> &mut TextFormat {
        &mut self.format
    }

    /// Get text length.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if span is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Split span at character position.
    pub fn split_at(&self, pos: usize) -> Option<(FormattedSpan, FormattedSpan)> {
        if pos >= self.content.len() {
            return None;
        }

        let (left, right) = self.content.split_at(pos);
        let left_span = FormattedSpan::new(left.to_string(), self.format.clone());
        let right_span = FormattedSpan::new(right.to_string(), self.format.clone());

        Some((left_span, right_span))
    }
}

/// Formatted line with multiple styled spans.
#[derive(Debug, Clone)]
pub struct FormattedLine {
    /// Spans that make up this line.
    spans: Vec<FormattedSpan>,
}

impl FormattedLine {
    /// Create a new formatted line.
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    /// Add a span to the line.
    pub fn add_span(&mut self, span: FormattedSpan) {
        if !span.is_empty() {
            self.spans.push(span);
        }
    }

    /// Get all spans.
    pub fn spans(&self) -> &[FormattedSpan] {
        &self.spans
    }

    /// Get total text content.
    pub fn text(&self) -> String {
        self.spans.iter().map(|s| s.content()).collect()
    }

    /// Get total character count.
    pub fn len(&self) -> usize {
        self.spans
            .iter()
            .map(|s| s.len())
            .collect::<Vec<_>>()
            .iter()
            .sum()
    }

    /// Check if line is empty.
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.text().is_empty()
    }

    /// Clear all spans.
    pub fn clear(&mut self) {
        self.spans.clear();
    }
}

impl Default for FormattedLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_color_hex_conversion() {
        let color = Color::rgb(255, 128, 64);
        let hex = format!("{:08x}", color.to_hex());
        assert_eq!(hex, "ff8040ff");
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_text_format_styles() {
        let mut format = TextFormat::new();

        format.add_style(TextStyle::Bold);
        assert!(format.is_bold());

        format.add_style(TextStyle::Italic);
        assert!(format.is_italic());

        format.remove_style(TextStyle::Bold);
        assert!(!format.is_bold());
    }

    #[test]
    fn test_text_format_toggle_style() {
        let mut format = TextFormat::new();

        assert!(!format.is_bold());
        format.toggle_style(TextStyle::Bold);
        assert!(format.is_bold());
        format.toggle_style(TextStyle::Bold);
        assert!(!format.is_bold());
    }

    #[test]
    fn test_formatted_span_split() {
        let format = TextFormat::new();
        let span = FormattedSpan::new("Hello World".to_string(), format);

        let (left, right) = span.split_at(5).unwrap();
        assert_eq!(left.content(), "Hello");
        assert_eq!(right.content(), " World");
    }

    #[test]
    fn test_formatted_line() {
        let mut line = FormattedLine::new();

        let format1 = TextFormat::new();
        line.add_span(FormattedSpan::new("Hello ".to_string(), format1));

        let mut format2 = TextFormat::new();
        format2.add_style(TextStyle::Bold);
        line.add_span(FormattedSpan::new("World".to_string(), format2));

        assert_eq!(line.text(), "Hello World");
        assert_eq!(line.len(), 11);
        assert!(!line.is_empty());
    }

    #[test]
    fn test_formatted_line_multiple_styles() {
        let mut line = FormattedLine::new();

        let mut format = TextFormat::new();
        format.add_style(TextStyle::Bold);
        format.add_style(TextStyle::Italic);
        format.add_style(TextStyle::Underline);

        let span = FormattedSpan::new("Formatted".to_string(), format);
        line.add_span(span);

        let spans = line.spans();
        assert_eq!(spans.len(), 1);
        assert!(spans[0].format().is_bold());
        assert!(spans[0].format().is_italic());
        assert!(spans[0].format().is_underlined());
    }
}
