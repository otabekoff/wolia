//! Text layout and measurement engine.
//!
//! This module provides text layout, measurement, and line breaking.

use wolia_core::style::{ParagraphStyle, TextStyle};

/// Text layout metrics.
#[derive(Debug, Clone, Copy)]
pub struct LayoutMetrics {
    /// Width available for text.
    pub width: f32,
    /// Height available for text.
    pub height: f32,
    /// Measured text width.
    pub measured_width: f32,
    /// Measured text height.
    pub measured_height: f32,
    /// Number of lines.
    pub line_count: usize,
    /// Baseline offset.
    pub baseline: f32,
}

impl LayoutMetrics {
    /// Create new layout metrics.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            measured_width: 0.0,
            measured_height: 0.0,
            line_count: 0,
            baseline: 0.0,
        }
    }
}

/// A line of text with position and metrics.
#[derive(Debug, Clone)]
pub struct TextLine {
    /// The text content of this line.
    pub text: String,
    /// Y offset of this line from the top.
    pub y_offset: f32,
    /// Height of this line.
    pub height: f32,
    /// Baseline offset within the line.
    pub baseline: f32,
    /// Width of this line.
    pub width: f32,
}

impl TextLine {
    /// Create a new text line.
    pub fn new(text: String, y_offset: f32, height: f32) -> Self {
        Self {
            text,
            y_offset,
            height,
            baseline: height * 0.8, // Approximate baseline
            width: 0.0,
        }
    }
}

/// Text layout engine.
#[allow(dead_code)]
pub struct TextLayout {
    /// Maximum width for wrapping.
    max_width: f32,
}

impl TextLayout {
    /// Create a new text layout engine.
    pub fn new(max_width: f32) -> Self {
        Self { max_width }
    }

    /// Layout text with the given constraints and styles.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to layout
    /// * `width` - Maximum width for wrapping
    /// * `text_style` - Text styling
    /// * `paragraph_style` - Paragraph styling
    ///
    /// # Returns
    ///
    /// Layout metrics and line information
    pub fn layout_text(
        &mut self,
        text: &str,
        width: f32,
        text_style: &TextStyle,
        paragraph_style: &ParagraphStyle,
    ) -> crate::Result<(LayoutMetrics, Vec<TextLine>)> {
        let font_size = text_style.font_size.unwrap_or(12.0);
        let line_height = font_size * paragraph_style.line_height.unwrap_or(1.2);

        // Simple line breaking algorithm (words)
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut total_height = 0.0f32;
        let mut max_width_found = 0.0f32;

        // Approximate character width based on font size
        let char_width = font_size * 0.5;

        for word in text.split_whitespace() {
            let word_with_space = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let estimated_width = word_with_space.len() as f32 * char_width;

            if estimated_width > width && !current_line.is_empty() {
                // Start new line
                let text_line = TextLine {
                    text: current_line.clone(),
                    y_offset: total_height,
                    height: line_height,
                    baseline: line_height * 0.8,
                    width: (current_line.len() as f32) * char_width,
                };
                lines.push(text_line);
                total_height += line_height;
                current_line = word.to_string();
                max_width_found = max_width_found.max((word.len() as f32) * char_width);
            } else {
                current_line = word_with_space;
                max_width_found = max_width_found.max(estimated_width);
            }
        }

        // Add last line
        if !current_line.is_empty() {
            let text_line = TextLine {
                text: current_line.clone(),
                y_offset: total_height,
                height: line_height,
                baseline: line_height * 0.8,
                width: (current_line.len() as f32) * char_width,
            };
            lines.push(text_line);
            total_height += line_height;
        }

        let mut layout_metrics = LayoutMetrics::new(width, total_height);
        layout_metrics.measured_width = max_width_found;
        layout_metrics.measured_height = total_height;
        layout_metrics.line_count = lines.len();
        layout_metrics.baseline = if !lines.is_empty() {
            lines[0].baseline
        } else {
            0.0
        };

        Ok((layout_metrics, lines))
    }

    /// Measure text without laying it out.
    ///
    /// Returns (width, height) of the text.
    pub fn measure_text(&mut self, text: &str, font_size: f32) -> crate::Result<(f32, f32)> {
        let char_width = font_size * 0.5;
        let _line_height = font_size * 1.2;

        let width = text
            .lines()
            .map(|line| (line.len() as f32) * char_width)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let height = (text.lines().count() as f32) * (font_size * 1.2);

        Ok((width, height))
    }

    /// Get cursor position (x, y) for a given character index.
    pub fn cursor_position(&self, char_index: usize, font_size: f32) -> Option<(f32, f32)> {
        let char_width = font_size * 0.5;
        let _line_height = font_size * 1.2;

        let x = (char_index as f32) * char_width;
        let y = 0.0; // Simplified: all on one line

        Some((x, y))
    }

    /// Find character index at the given position.
    pub fn hit_test(&self, x: f32, _y: f32, font_size: f32) -> Option<usize> {
        let char_width = font_size * 0.5;
        let char_index = (x / char_width).floor() as usize;
        Some(char_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_metrics() {
        let metrics = LayoutMetrics::new(100.0, 200.0);
        assert_eq!(metrics.width, 100.0);
        assert_eq!(metrics.height, 200.0);
        assert_eq!(metrics.line_count, 0);
    }

    #[test]
    fn test_text_line_creation() {
        let line = TextLine::new("Hello".to_string(), 10.0, 20.0);
        assert_eq!(line.text, "Hello");
        assert_eq!(line.y_offset, 10.0);
        assert_eq!(line.height, 20.0);
    }

    #[test]
    fn test_text_layout_creation() {
        let layout = TextLayout::new(100.0);
        assert_eq!(layout.max_width, 100.0);
    }
}
