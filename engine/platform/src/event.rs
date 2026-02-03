//! Event types.

use wolia_math::Point;

/// Application event.
#[derive(Debug, Clone)]
pub enum Event {
    /// Window events.
    Window(WindowEvent),
    /// Keyboard events.
    Key(KeyEvent),
    /// Mouse events.
    Mouse(MouseEvent),
    /// Touch events.
    Touch(TouchEvent),
    /// IME events.
    Ime(ImeEvent),
    /// Application lifecycle events.
    Lifecycle(LifecycleEvent),
}

/// Window event.
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Window resized.
    Resized { width: u32, height: u32 },
    /// Window moved.
    Moved { x: i32, y: i32 },
    /// Window focus changed.
    Focused(bool),
    /// Window close requested.
    CloseRequested,
    /// Window scale factor changed.
    ScaleFactorChanged(f64),
}

/// Keyboard event.
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// Physical key code.
    pub key_code: Option<KeyCode>,
    /// Logical key (with modifiers applied).
    pub logical_key: LogicalKey,
    /// Key state (pressed or released).
    pub state: ElementState,
    /// Modifier keys held.
    pub modifiers: Modifiers,
    /// Whether this is a repeat event.
    pub repeat: bool,
}

/// Mouse event.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// Event kind.
    pub kind: MouseEventKind,
    /// Mouse position.
    pub position: Point,
    /// Modifier keys held.
    pub modifiers: Modifiers,
}

/// Mouse event kind.
#[derive(Debug, Clone)]
pub enum MouseEventKind {
    /// Mouse moved.
    Move,
    /// Mouse button pressed.
    Down(MouseButton),
    /// Mouse button released.
    Up(MouseButton),
    /// Mouse wheel scrolled.
    Scroll { delta_x: f32, delta_y: f32 },
    /// Mouse entered window.
    Enter,
    /// Mouse left window.
    Leave,
}

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// Touch event.
#[derive(Debug, Clone)]
pub struct TouchEvent {
    /// Touch phase.
    pub phase: TouchPhase,
    /// Touch position.
    pub position: Point,
    /// Touch ID.
    pub id: u64,
}

/// Touch phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

/// IME event.
#[derive(Debug, Clone)]
pub enum ImeEvent {
    /// IME enabled.
    Enabled,
    /// IME preedit (composition).
    Preedit(String, Option<(usize, usize)>),
    /// IME commit.
    Commit(String),
    /// IME disabled.
    Disabled,
}

/// Application lifecycle event.
#[derive(Debug, Clone, Copy)]
pub enum LifecycleEvent {
    /// Application started.
    Started,
    /// Application suspended (mobile).
    Suspended,
    /// Application resumed (mobile).
    Resumed,
    /// Application terminating.
    Terminating,
}

/// Element state (pressed/released).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementState {
    Pressed,
    Released,
}

/// Modifier keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool, // Cmd on macOS, Win on Windows
}

impl Modifiers {
    /// Check if any modifier is pressed.
    pub fn any(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }

    /// Check if no modifiers are pressed.
    pub fn none(&self) -> bool {
        !self.any()
    }
}

/// Physical key code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
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

    // Numbers
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

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
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,

    // Editing
    Backspace,
    Delete,
    Insert,
    Enter,
    Tab,
    Escape,
    Space,

    // Modifiers
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,

    // Other
    CapsLock,
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,

    // Numpad
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,
    NumpadEnter,

    // Symbols
    Minus,
    Equal,
    BracketLeft,
    BracketRight,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Grave,
}

/// Logical key (after keyboard layout processing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalKey {
    /// A character.
    Character(String),
    /// A named key.
    Named(NamedKey),
}

/// Named keys (non-character).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedKey {
    Enter,
    Tab,
    Space,
    Backspace,
    Delete,
    Escape,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
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
}
