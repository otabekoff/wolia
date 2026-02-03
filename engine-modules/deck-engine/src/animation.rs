//! Animation system.

use uuid::Uuid;

/// An animation applied to a shape.
#[derive(Debug, Clone)]
pub struct Animation {
    /// Target shape ID.
    pub target: Uuid,
    /// Animation effect.
    pub effect: AnimationEffect,
    /// Trigger type.
    pub trigger: AnimationTrigger,
    /// Duration in seconds.
    pub duration: f32,
    /// Delay in seconds.
    pub delay: f32,
}

impl Animation {
    /// Create a new animation.
    pub fn new(target: Uuid, effect: AnimationEffect) -> Self {
        Self {
            target,
            effect,
            trigger: AnimationTrigger::OnClick,
            duration: 0.5,
            delay: 0.0,
        }
    }
}

/// Animation effects.
#[derive(Debug, Clone)]
pub enum AnimationEffect {
    // Entrance
    Appear,
    FadeIn,
    FlyIn { direction: Direction },
    ZoomIn,

    // Emphasis
    Pulse,
    Spin,
    Grow,

    // Exit
    Disappear,
    FadeOut,
    FlyOut { direction: Direction },
    ZoomOut,

    // Motion paths
    MotionPath { path: String },
}

/// Animation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Animation trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationTrigger {
    /// Start on click.
    OnClick,
    /// Start with previous animation.
    WithPrevious,
    /// Start after previous animation.
    AfterPrevious,
}

/// Animation timeline for a slide.
#[derive(Debug, Clone, Default)]
pub struct AnimationTimeline {
    /// Animations in order.
    pub animations: Vec<Animation>,
    /// Current position in timeline.
    pub position: usize,
}

impl AnimationTimeline {
    /// Create a new timeline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an animation.
    pub fn add(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    /// Advance to the next animation.
    pub fn next(&mut self) -> Option<&Animation> {
        if self.position < self.animations.len() {
            let anim = &self.animations[self.position];
            self.position += 1;
            Some(anim)
        } else {
            None
        }
    }

    /// Reset the timeline.
    pub fn reset(&mut self) {
        self.position = 0;
    }
}
