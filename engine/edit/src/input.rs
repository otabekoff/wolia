//! Input handling system for Wolia applications.
//!
//! This module provides keyboard, mouse, and touch input handling,
//! as well as IME (Input Method Editor) support for international text input.

use std::collections::HashMap;

/// Keyboard key representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Alphanumeric
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,

    // Special keys
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Minus,
    Equal,
    BracketLeft,
    BracketRight,
    Backslash,
    Semicolon,
    Quote,
    Backquote,
    Comma,
    Period,
    Slash,

    // Control keys
    Shift,
    Control,
    Alt,
    Meta,
    CapsLock,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Navigation
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // Other
    Unknown,
}

/// Keyboard modifiers (Ctrl, Shift, Alt, etc.)
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyModifiers {
    /// Create new modifiers with all false.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any modifier is pressed.
    pub fn any(&self) -> bool {
        self.shift || self.control || self.alt || self.meta
    }

    /// Check for Ctrl+Key combination.
    pub fn with_ctrl(&self) -> bool {
        self.control
    }

    /// Check for Shift+Key combination.
    pub fn with_shift(&self) -> bool {
        self.shift
    }

    /// Check for Alt+Key combination.
    pub fn with_alt(&self) -> bool {
        self.alt
    }
}

/// Keyboard input event.
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    /// The key that was pressed.
    pub key: Key,
    /// Whether the key was pressed (true) or released (false).
    pub pressed: bool,
    /// Active modifiers.
    pub modifiers: KeyModifiers,
    /// Character code for the key (if applicable).
    pub char_code: Option<char>,
}

impl KeyboardEvent {
    /// Create a new keyboard event.
    pub fn new(key: Key, pressed: bool, modifiers: KeyModifiers) -> Self {
        Self {
            key,
            pressed,
            modifiers,
            char_code: None,
        }
    }

    /// Set the character code.
    pub fn with_char(mut self, c: char) -> Self {
        self.char_code = Some(c);
        self
    }
}

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Mouse input event.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// X position in pixels.
    pub x: f32,
    /// Y position in pixels.
    pub y: f32,
    /// Mouse button involved (if any).
    pub button: Option<MouseButton>,
    /// Number of clicks (for double-click detection).
    pub click_count: u32,
    /// Active modifiers.
    pub modifiers: KeyModifiers,
}

impl MouseEvent {
    /// Create a new mouse event.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            button: None,
            click_count: 1,
            modifiers: KeyModifiers::new(),
        }
    }

    /// Set the mouse button.
    pub fn with_button(mut self, button: MouseButton) -> Self {
        self.button = Some(button);
        self
    }

    /// Set click count.
    pub fn with_click_count(mut self, count: u32) -> Self {
        self.click_count = count;
        self
    }

    /// Set modifiers.
    pub fn with_modifiers(mut self, modifiers: KeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }
}

/// IME (Input Method Editor) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImeState {
    /// No IME input in progress.
    #[default]
    Inactive,
    /// Preedit mode (composing text).
    Preedit,
    /// Committed mode (text accepted).
    Committed,
}

/// IME event.
#[derive(Debug, Clone)]
pub enum ImeEvent {
    /// Start editing.
    Start,
    /// Preedit text changed.
    Preedit(String, Option<usize>), // text, cursor position
    /// Commit the composed text.
    Commit(String),
    /// End editing.
    End,
}

/// Input handler for managing keyboard, mouse, and IME events.
#[derive(Debug, Default)]
pub struct InputHandler {
    /// Pressed keys.
    pressed_keys: HashMap<Key, bool>,
    /// Current modifiers.
    modifiers: KeyModifiers,
    /// IME state.
    ime_state: ImeState,
}

impl InputHandler {
    /// Create a new input handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle a keyboard event.
    pub fn handle_keyboard(&mut self, event: &KeyboardEvent) {
        if event.pressed {
            self.pressed_keys.insert(event.key, true);
        } else {
            self.pressed_keys.remove(&event.key);
        }
        self.modifiers = event.modifiers;
    }

    /// Check if a key is currently pressed.
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.get(&key).copied().unwrap_or(false)
    }

    /// Get currently active modifiers.
    pub fn modifiers(&self) -> KeyModifiers {
        self.modifiers
    }

    /// Handle IME event.
    pub fn handle_ime(&mut self, event: &ImeEvent) {
        match event {
            ImeEvent::Start => {
                self.ime_state = ImeState::Preedit;
            }
            ImeEvent::Preedit(_, _) => {
                self.ime_state = ImeState::Preedit;
            }
            ImeEvent::Commit(_) => {
                self.ime_state = ImeState::Committed;
            }
            ImeEvent::End => {
                self.ime_state = ImeState::Inactive;
            }
        }
    }

    /// Get current IME state.
    pub fn ime_state(&self) -> ImeState {
        self.ime_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_modifiers() {
        let modifiers = KeyModifiers::new();
        assert!(!modifiers.any());

        let modifiers = KeyModifiers {
            shift: true,
            ..Default::default()
        };
        assert!(modifiers.any());
        assert!(modifiers.with_shift());
    }

    #[test]
    fn test_keyboard_event() {
        let event = KeyboardEvent::new(Key::A, true, KeyModifiers::new()).with_char('a');
        assert_eq!(event.key, Key::A);
        assert!(event.pressed);
        assert_eq!(event.char_code, Some('a'));
    }

    #[test]
    fn test_input_handler() {
        let mut handler = InputHandler::new();
        let event = KeyboardEvent::new(Key::A, true, KeyModifiers::new());
        handler.handle_keyboard(&event);

        assert!(handler.is_key_pressed(Key::A));

        let event = KeyboardEvent::new(Key::A, false, KeyModifiers::new());
        handler.handle_keyboard(&event);
        assert!(!handler.is_key_pressed(Key::A));
    }

    #[test]
    fn test_ime_handler() {
        let mut handler = InputHandler::new();
        assert_eq!(handler.ime_state(), ImeState::Inactive);

        handler.handle_ime(&ImeEvent::Start);
        assert_eq!(handler.ime_state(), ImeState::Preedit);

        handler.handle_ime(&ImeEvent::Commit("text".to_string()));
        assert_eq!(handler.ime_state(), ImeState::Committed);

        handler.handle_ime(&ImeEvent::End);
        assert_eq!(handler.ime_state(), ImeState::Inactive);
    }
}
