//! Slide transitions.

/// Transition type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransitionType {
    #[default]
    None,
    Fade,
    Push,
    Wipe,
    Split,
    Reveal,
    Cover,
    Dissolve,
    Zoom,
}

/// Transition direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransitionDirection {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

/// Slide transition settings.
#[derive(Debug, Clone)]
pub struct Transition {
    /// Transition type.
    pub kind: TransitionType,
    /// Transition direction.
    pub direction: TransitionDirection,
    /// Duration in seconds.
    pub duration: f32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            kind: TransitionType::None,
            direction: TransitionDirection::Right,
            duration: 0.5,
        }
    }
}

impl Transition {
    /// Create a fade transition.
    pub fn fade(duration: f32) -> Self {
        Self {
            kind: TransitionType::Fade,
            direction: TransitionDirection::Right,
            duration,
        }
    }

    /// Create a push transition.
    pub fn push(direction: TransitionDirection, duration: f32) -> Self {
        Self {
            kind: TransitionType::Push,
            direction,
            duration,
        }
    }
}
