//! Paragraph formatting for document structure.

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlignment {
    /// Left aligned.
    #[default]
    Left,
    /// Center aligned.
    Center,
    /// Right aligned.
    Right,
    /// Justified.
    Justify,
}

impl TextAlignment {
    /// Get alignment as CSS value.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
        }
    }
}

/// Heading level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadingLevel {
    /// H1 heading.
    H1 = 1,
    /// H2 heading.
    H2 = 2,
    /// H3 heading.
    H3 = 3,
    /// H4 heading.
    H4 = 4,
    /// H5 heading.
    H5 = 5,
    /// H6 heading.
    H6 = 6,
}

impl HeadingLevel {
    /// Get heading font size multiplier.
    pub fn font_size_multiplier(&self) -> f32 {
        match self {
            Self::H1 => 2.0,
            Self::H2 => 1.75,
            Self::H3 => 1.5,
            Self::H4 => 1.25,
            Self::H5 => 1.1,
            Self::H6 => 1.0,
        }
    }

    /// Get heading name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::H1 => "Heading 1",
            Self::H2 => "Heading 2",
            Self::H3 => "Heading 3",
            Self::H4 => "Heading 4",
            Self::H5 => "Heading 5",
            Self::H6 => "Heading 6",
        }
    }
}

/// List style options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListStyle {
    /// No list.
    #[default]
    None,
    /// Bulleted list.
    Bullet,
    /// Numbered list.
    Numbered,
    /// Lettered list (a, b, c).
    Lettered,
    /// Roman numeral list (i, ii, iii).
    Roman,
}

impl ListStyle {
    /// Check if this is a list style.
    pub fn is_list(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Paragraph formatting properties.
#[derive(Debug, Clone)]
pub struct ParagraphFormat {
    /// Text alignment.
    alignment: TextAlignment,
    /// Left indentation in points.
    left_indent: f32,
    /// Right indentation in points.
    right_indent: f32,
    /// First line indentation in points.
    first_line_indent: f32,
    /// Space before paragraph in points.
    space_before: f32,
    /// Space after paragraph in points.
    space_after: f32,
    /// Line spacing multiplier (1.0 = single, 1.5 = 1.5x, 2.0 = double).
    line_spacing: f32,
    /// Heading level if this is a heading.
    heading: Option<HeadingLevel>,
    /// List style.
    list_style: ListStyle,
}

impl ParagraphFormat {
    /// Create a new paragraph format with defaults.
    pub fn new() -> Self {
        Self {
            alignment: TextAlignment::default(),
            left_indent: 0.0,
            right_indent: 0.0,
            first_line_indent: 0.0,
            space_before: 0.0,
            space_after: 0.0,
            line_spacing: 1.15, // Default line spacing
            heading: None,
            list_style: ListStyle::default(),
        }
    }

    /// Set text alignment.
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set left indentation.
    pub fn with_left_indent(mut self, indent: f32) -> Self {
        self.left_indent = indent.max(0.0);
        self
    }

    /// Set right indentation.
    pub fn with_right_indent(mut self, indent: f32) -> Self {
        self.right_indent = indent.max(0.0);
        self
    }

    /// Set first line indentation.
    pub fn with_first_line_indent(mut self, indent: f32) -> Self {
        self.first_line_indent = indent;
        self
    }

    /// Set space before paragraph.
    pub fn with_space_before(mut self, space: f32) -> Self {
        self.space_before = space.max(0.0);
        self
    }

    /// Set space after paragraph.
    pub fn with_space_after(mut self, space: f32) -> Self {
        self.space_after = space.max(0.0);
        self
    }

    /// Set line spacing.
    pub fn with_line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing.max(0.5);
        self
    }

    /// Set as heading.
    pub fn with_heading(mut self, level: Option<HeadingLevel>) -> Self {
        self.heading = level;
        self
    }

    /// Set list style.
    pub fn with_list_style(mut self, style: ListStyle) -> Self {
        self.list_style = style;
        self
    }

    /// Get text alignment.
    pub fn alignment(&self) -> TextAlignment {
        self.alignment
    }

    /// Get left indentation.
    pub fn left_indent(&self) -> f32 {
        self.left_indent
    }

    /// Get right indentation.
    pub fn right_indent(&self) -> f32 {
        self.right_indent
    }

    /// Get first line indentation.
    pub fn first_line_indent(&self) -> f32 {
        self.first_line_indent
    }

    /// Get space before.
    pub fn space_before(&self) -> f32 {
        self.space_before
    }

    /// Get space after.
    pub fn space_after(&self) -> f32 {
        self.space_after
    }

    /// Get line spacing.
    pub fn line_spacing(&self) -> f32 {
        self.line_spacing
    }

    /// Get heading level.
    pub fn heading(&self) -> Option<HeadingLevel> {
        self.heading
    }

    /// Get list style.
    pub fn list_style(&self) -> ListStyle {
        self.list_style
    }

    /// Check if this is a heading.
    pub fn is_heading(&self) -> bool {
        self.heading.is_some()
    }

    /// Check if this is a list item.
    pub fn is_list_item(&self) -> bool {
        self.list_style.is_list()
    }
}

impl Default for ParagraphFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_default() {
        assert_eq!(TextAlignment::default(), TextAlignment::Left);
    }

    #[test]
    fn test_alignment_css() {
        assert_eq!(TextAlignment::Left.as_css(), "left");
        assert_eq!(TextAlignment::Center.as_css(), "center");
        assert_eq!(TextAlignment::Right.as_css(), "right");
        assert_eq!(TextAlignment::Justify.as_css(), "justify");
    }

    #[test]
    fn test_heading_levels() {
        assert_eq!(HeadingLevel::H1.font_size_multiplier(), 2.0);
        assert_eq!(HeadingLevel::H3.font_size_multiplier(), 1.5);
        assert_eq!(HeadingLevel::H6.font_size_multiplier(), 1.0);
    }

    #[test]
    fn test_heading_names() {
        assert_eq!(HeadingLevel::H1.name(), "Heading 1");
        assert_eq!(HeadingLevel::H6.name(), "Heading 6");
    }

    #[test]
    fn test_list_style_is_list() {
        assert!(!ListStyle::None.is_list());
        assert!(ListStyle::Bullet.is_list());
        assert!(ListStyle::Numbered.is_list());
    }

    #[test]
    fn test_paragraph_format_creation() {
        let format = ParagraphFormat::new();
        assert_eq!(format.alignment(), TextAlignment::Left);
        assert_eq!(format.left_indent(), 0.0);
        assert_eq!(format.line_spacing(), 1.15);
        assert!(!format.is_heading());
        assert!(!format.is_list_item());
    }

    #[test]
    fn test_paragraph_format_builder() {
        let format = ParagraphFormat::new()
            .with_alignment(TextAlignment::Center)
            .with_left_indent(36.0)
            .with_space_before(12.0)
            .with_heading(Some(HeadingLevel::H1));

        assert_eq!(format.alignment(), TextAlignment::Center);
        assert_eq!(format.left_indent(), 36.0);
        assert_eq!(format.space_before(), 12.0);
        assert!(format.is_heading());
    }

    #[test]
    fn test_paragraph_format_list() {
        let format = ParagraphFormat::new().with_list_style(ListStyle::Bullet);
        assert!(format.is_list_item());
        assert_eq!(format.list_style(), ListStyle::Bullet);
    }

    #[test]
    fn test_paragraph_format_indentation() {
        let format = ParagraphFormat::new()
            .with_left_indent(36.0)
            .with_right_indent(36.0)
            .with_first_line_indent(18.0);

        assert_eq!(format.left_indent(), 36.0);
        assert_eq!(format.right_indent(), 36.0);
        assert_eq!(format.first_line_indent(), 18.0);
    }

    #[test]
    fn test_paragraph_format_spacing() {
        let format = ParagraphFormat::new()
            .with_line_spacing(2.0)
            .with_space_before(6.0)
            .with_space_after(12.0);

        assert_eq!(format.line_spacing(), 2.0);
        assert_eq!(format.space_before(), 6.0);
        assert_eq!(format.space_after(), 12.0);
    }
}
