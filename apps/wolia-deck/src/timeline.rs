//! Animation timeline.

/// Animation timeline.
pub struct Timeline {
    /// Current time in seconds.
    pub time: f32,
    /// Total duration.
    pub duration: f32,
    /// Is playing.
    pub playing: bool,
}

impl Timeline {
    /// Create a new timeline.
    pub fn new() -> Self {
        Self {
            time: 0.0,
            duration: 0.0,
            playing: false,
        }
    }

    /// Play the timeline.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Pause the timeline.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Stop and reset.
    pub fn stop(&mut self) {
        self.playing = false;
        self.time = 0.0;
    }

    /// Advance the timeline by delta time.
    pub fn advance(&mut self, dt: f32) {
        if self.playing {
            self.time += dt;
            if self.time >= self.duration {
                self.time = self.duration;
                self.playing = false;
            }
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
