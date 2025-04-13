extern crate alloc;

use alloc::format;
use core::fmt::Display;

/// A hardware key
#[repr(u8)]
pub enum RawKey {
    Left = 0,
    Up = 1,
    Down = 2,
    Right = 3,
    Ok = 4,
    Back = 5,
    Home = 6,
    OnOff = 8,
    Shift = 12,
    Alpha = 13,
    Xnt = 14,
    Var = 15,
    Toolbox = 16,
    Backspace = 17,
    Exp = 18,
    Ln = 19,
    Log = 20,
    Imaginary = 21,
    Comma = 22,
    Power = 23,
    Sine = 24,
    Cosine = 25,
    Tangent = 26,
    Pi = 27,
    Sqrt = 28,
    Square = 29,
    Seven = 30,
    Eight = 31,
    Nine = 32,
    LeftParenthesis = 33,
    RightParenthesis = 34,
    Four = 36,
    Five = 37,
    Six = 38,
    Multiplication = 39,
    Division = 40,
    One = 42,
    Two = 43,
    Three = 44,
    Plus = 45,
    Minus = 46,
    Zero = 48,
    Dot = 49,
    Ee = 50,
    Ans = 51,
    Exe = 52,
}

/// A keypress event
///
/// The result of pressing a key with or without modifiers (shift and alpha).
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Key {
    Left = 0,
    Up = 1,
    Down = 2,
    Right = 3,
    Ok = 4,
    Back = 5,
    Shift = 12,
    Alpha = 13,
    Xnt = 14,
    Var = 15,
    Toolbox = 16,
    Backspace = 17,
    Exp = 18,
    Ln = 19,
    Log = 20,
    Imaginary = 21,
    Comma = 22,
    Power = 23,
    Sine = 24,
    Cosine = 25,
    Tangent = 26,
    Pi = 27,
    Sqrt = 28,
    Square = 29,
    Seven = 30,
    Eight = 31,
    Nine = 32,
    LeftParenthesis = 33,
    RightParenthesis = 34,
    Four = 36,
    Five = 37,
    Six = 38,
    Multiplication = 39,
    Division = 40,
    One = 42,
    Two = 43,
    Three = 44,
    Plus = 45,
    Minus = 46,
    Zero = 48,
    Dot = 49,
    Ee = 50,
    Ans = 51,
    Exe = 52,
    ShiftLeft = 54,
    ShiftUp = 55,
    ShiftDown = 56,
    ShiftRight = 57,
    AlphaLock = 67,
    Cut = 68,
    Copy = 69,
    Paste = 70,
    Clear = 71,
    LeftBracket = 72,
    RightBracket = 73,
    LeftBrace = 74,
    RightBrace = 75,
    Underscore = 76,
    Sto = 77,
    Arcsine = 78,
    Arccosine = 79,
    Arctangent = 80,
    Equal = 81,
    Lower = 82,
    Greater = 83,
    Colon = 122,
    Semicolon = 123,
    DoubleQuotes = 124,
    Percent = 125,
    LowerA = 126,
    LowerB = 127,
    LowerC = 128,
    LowerD = 129,
    LowerE = 130,
    LowerF = 131,
    LowerG = 132,
    LowerH = 133,
    LowerI = 134,
    LowerJ = 135,
    LowerK = 136,
    LowerL = 137,
    LowerM = 138,
    LowerN = 139,
    LowerO = 140,
    LowerP = 141,
    LowerQ = 142,
    LowerR = 144,
    LowerS = 145,
    LowerT = 146,
    LowerU = 147,
    LowerV = 148,
    LowerW = 150,
    LowerX = 151,
    LowerY = 152,
    LowerZ = 153,
    Space = 154,
    Question = 156,
    Exclamation = 157,
    UpperA = 180,
    UpperB = 181,
    UpperC = 182,
    UpperD = 183,
    UpperE = 184,
    UpperF = 185,
    UpperG = 186,
    UpperH = 187,
    UpperI = 188,
    UpperJ = 189,
    UpperK = 190,
    UpperL = 191,
    UpperM = 192,
    UpperN = 193,
    UpperO = 194,
    UpperP = 195,
    UpperQ = 196,
    UpperR = 198,
    UpperS = 199,
    UpperT = 200,
    UpperU = 201,
    UpperV = 202,
    UpperW = 204,
    UpperX = 205,
    UpperY = 206,
    UpperZ = 207,
    /// Returned by `eadk_event_get` when no key has been pressed
    None = 223,
}

impl Key {
    pub fn is_digit(&self) -> bool {
        matches!(
            self,
            Self::One
                | Self::Two
                | Self::Three
                | Self::Four
                | Self::Five
                | Self::Six
                | Self::Seven
                | Self::Nine
                | Self::Zero
        )
    }

    pub fn to_digit(&self) -> Option<u8> {
        match self {
            Self::Zero => Some(0),
            Self::One => Some(1),
            Self::Two => Some(2),
            Self::Three => Some(3),
            Self::Four => Some(4),
            Self::Five => Some(5),
            Self::Six => Some(6),
            Self::Seven => Some(7),
            Self::Eight => Some(8),
            Self::Nine => Some(9),
            _ => None,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val: u16 = *self as u16;
        f.write_str(&format!("{val}"))
    }
}

/// The state of the keyboard (pressed keys)
#[repr(C)]
pub struct KeyboardState(pub u64);

pub use eadk::scan;

/// Waits for `timeout` or until a key is pressed.
///
/// If the timeout is reached, returns `None`.
pub fn wait_for_input(timeout_ms: i32) -> Option<Key> {
    match eadk::event_get(timeout_ms) {
        Key::None => None,
        key => Some(key),
    }
}

/// Checks if the given key is pressed.
pub fn is_pressed(key: RawKey) -> bool {
    let keyboard_state = eadk::scan();
    eadk::keyboard_key_down(keyboard_state, key)
}

/// Retrieves the currently pressed key.
/// 
/// This uses `eadk::event_get`, so it only detects new events.
pub fn currently_pressed() -> Key {
    eadk::event_get(1)
}

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    use super::{Key, KeyboardState, RawKey};

    /// Retrieves the current state of the keyboard.
    pub fn scan() -> KeyboardState {
        unsafe { KeyboardState(eadk_keyboard_scan()) }
    }

    /// Waits until a key (or combination of keys) is pressed,
    /// or until `timeout` expires.
    ///
    /// Only detects new key presses. Holding a key will not
    /// trigger multiple events if this function is called
    /// repeatedly while the key remains pressed.
    pub fn event_get(timeout: i32) -> Key {
        // copy the value
        let mut timeout = timeout;
        unsafe { eadk_event_get(&mut timeout as *mut _) }
    }

    /// Checks if the key was pressed in the given state.
    pub fn keyboard_key_down(keyboard_state: KeyboardState, key: RawKey) -> bool {
        (keyboard_state.0 >> (key as u8)) & 1 != 0
    }

    unsafe extern "C" {
        fn eadk_keyboard_scan() -> u64;
        fn eadk_event_get(timeout: *mut i32) -> Key;
    }
}
