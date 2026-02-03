//! Theming.

use wolia_math::Color;

/// Application theme.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name.
    pub name: String,
    /// Whether this is a dark theme.
    pub dark: bool,
    /// Colors.
    pub colors: ThemeColors,
}

impl Theme {
    /// Default light theme.
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            dark: false,
            colors: ThemeColors::light(),
        }
    }

    /// Default dark theme.
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            dark: true,
            colors: ThemeColors::dark(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

/// Theme colors.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    /// Background color.
    pub background: Color,
    /// Foreground/text color.
    pub foreground: Color,
    /// Primary accent color.
    pub primary: Color,
    /// Secondary color.
    pub secondary: Color,
    /// Border color.
    pub border: Color,
    /// Selection color.
    pub selection: Color,
    /// Error color.
    pub error: Color,
    /// Warning color.
    pub warning: Color,
    /// Success color.
    pub success: Color,
}

impl ThemeColors {
    /// Light theme colors.
    pub fn light() -> Self {
        Self {
            background: Color::from_rgba8(255, 255, 255, 255),
            foreground: Color::from_rgba8(33, 33, 33, 255),
            primary: Color::from_rgba8(25, 118, 210, 255),
            secondary: Color::from_rgba8(156, 39, 176, 255),
            border: Color::from_rgba8(224, 224, 224, 255),
            selection: Color::from_rgba8(25, 118, 210, 64),
            error: Color::from_rgba8(211, 47, 47, 255),
            warning: Color::from_rgba8(245, 124, 0, 255),
            success: Color::from_rgba8(56, 142, 60, 255),
        }
    }

    /// Dark theme colors.
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgba8(30, 30, 30, 255),
            foreground: Color::from_rgba8(212, 212, 212, 255),
            primary: Color::from_rgba8(100, 181, 246, 255),
            secondary: Color::from_rgba8(206, 147, 216, 255),
            border: Color::from_rgba8(66, 66, 66, 255),
            selection: Color::from_rgba8(100, 181, 246, 64),
            error: Color::from_rgba8(239, 83, 80, 255),
            warning: Color::from_rgba8(255, 167, 38, 255),
            success: Color::from_rgba8(102, 187, 106, 255),
        }
    }
}
