//! IME (Input Method Editor) support.

/// IME composition state.
#[derive(Debug, Default)]
pub struct ImeState {
    /// Whether IME composition is active.
    pub composing: bool,
    /// The current composition string.
    pub composition: String,
    /// Cursor position within the composition.
    pub cursor: usize,
}

impl ImeState {
    /// Create a new IME state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start composition.
    pub fn start(&mut self) {
        self.composing = true;
        self.composition.clear();
        self.cursor = 0;
    }

    /// Update composition.
    pub fn update(&mut self, text: &str, cursor: usize) {
        self.composition = text.to_string();
        self.cursor = cursor;
    }

    /// End composition.
    pub fn end(&mut self) -> Option<String> {
        if self.composing {
            self.composing = false;
            let result = std::mem::take(&mut self.composition);
            self.cursor = 0;
            Some(result)
        } else {
            None
        }
    }

    /// Cancel composition.
    pub fn cancel(&mut self) {
        self.composing = false;
        self.composition.clear();
        self.cursor = 0;
    }
}
