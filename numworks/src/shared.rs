extern crate alloc;

use core::fmt::Display;

use crate::display::{self, SCREEN_HEIGHT, SCREEN_WIDTH};
use alloc::{format, vec::Vec};

/// An RGB 5-6-5 color: 5 bits for red, 6 bits for green and 5 bits for blue.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EadkColor(u16);

impl EadkColor {
    /// Creates a color directly from a u16.
    pub fn new(rgb565: u16) -> Self {
        Self(rgb565)
    }

    /// Creates a color from distincts red, green and blue channels.
    ///
    /// Each channel must be between 0 and 255, and is then converted
    /// so the 3 channels can fit in a u16 (some precision is lost).
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r = (r & 0b11111000) as u16;
        let g = (g & 0b11111100) as u16;
        let b = b as u16;
        Self((r << 8) | (g << 3) | (b >> 3))
    }

    /// Separates the color into 3 channels (0 to 255).
    pub fn separate_channels(&self) -> (u8, u8, u8) {
        let mut r = ((self.0 >> 8) & 0b11111000) as u8;
        r = r | (r >> 5);
        let mut g = ((self.0 >> 3) & 0b11111100) as u8;
        g = g | (g >> 6);
        let mut b = ((self.0 & 0b11111) << 3) as u8;
        b = b | (b >> 5);
        (r, g, b)
    }

    // Basic colors
    /// Pure black (0, 0, 0)
    pub const BLACK: Self = Self(0x0);
    /// Pure white (255, 255, 255)
    pub const WHITE: Self = Self(0xFFFF);
    /// Pure red (255, 0, 0)
    pub const RED: Self = Self(0xF800);
    /// Pure green (0, 255, 0)
    pub const GREEN: Self = Self(0x7E0);
    /// Pure blue (0, 0, 255)
    pub const BLUE: Self = Self(0x1F);
}

/// A rectangle on the screen.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EadkRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl EadkRect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a square
    pub fn new_square(x: u16, y: u16, width: u16) -> Self {
        Self::new(x, y, width, width)
    }

    /// Creates a square with side-length 1, i.e a pixel.
    pub fn new_pixel(x: u16, y: u16) -> Self {
        Self::new(x, y, 1, 1)
    }

    /// The number of pixels covered by the rectangle.
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Fills the rect on the screen with the given color.
    pub fn fill(self, color: EadkColor) {
        display::push_rect_uniform(self, color);
    }

    /// Fills the rect on the screen with the given pixel colors.
    pub fn fill_with_buf(self, pixels: &[EadkColor]) {
        // can we just return an Err and not draw the rect ?
        // i think panicking is too much, isn't it ?
        assert!(self.area() as usize == pixels.len());
        display::push_rect(self, pixels);
    }

    /// Returns the pixels' color in the given rect.
    pub fn get_pixels(self) -> Vec<EadkColor> {
        display::get_rect(self)
    }

    /// Centers the rectangle on the screen
    pub fn center(&mut self) {
        self.x = (SCREEN_WIDTH - self.width) / 2;
        self.y = (SCREEN_HEIGHT - self.height) / 2;
    }

    /// The rectangle the size of the screen.
    pub const SCREEN: Self = Self {
        x: 0,
        y: 0,
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    };
}

/// A hardware key
#[repr(u8)]
pub enum EadkRawKey {
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
pub enum EadkKey {
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

impl EadkKey {
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

impl Display for EadkKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val: u16 = *self as u16;
        f.write_str(&format!("{val}"))
    }
}

/// The state of the keyboard (pressed keys)
#[repr(C)]
pub struct EadkKeyboardState(pub u64);
