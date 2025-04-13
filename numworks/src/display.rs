extern crate alloc;

pub use crate::shared::EadkRect;
use alloc::{borrow::Cow, string::String, vec::Vec};

/// An RGB 5-6-5 color: 5 bits for red, 6 bits for green and 5 bits for blue.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color(u16);

impl Color {
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

/// The width of the screen in pixels.
pub const SCREEN_WIDTH: u16 = 320;
/// The height of the screen in pixels.
pub const SCREEN_HEIGHT: u16 = 240;
/// The number of pixels on the screen.
pub const SCREEN_AREA: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
/// The width of one char of the standard font in pixels.
pub const CHAR_WIDTH: u16 = 7;
/// The height of one char of the standard font in pixels.
pub const CHAR_HEIGHT: u16 = 14;
/// The width of one char of the large font in pixels.
pub const LARGE_CHAR_WIDTH: u16 = 10;
/// The height of one char of the large font in pixels.
pub const LARGE_CHAR_HEIGHT: u16 = 16;

unsafe extern "C" {
    fn eadk_display_push_rect(rect: EadkRect, pixels: *const Color);
    fn eadk_display_push_rect_uniform(rect: EadkRect, color: Color);
    fn eadk_display_pull_rect(rect: EadkRect, pixels: *mut Color);
    fn eadk_display_wait_for_vblank() -> bool;
    fn eadk_display_draw_string(
        text: *const u8,
        point: EadkPoint,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    );
}

/// A point on the screen.
///
/// This is only needed for the eadk_display_draw_string and should not be used elsewhere.
#[repr(C)]
pub struct EadkPoint {
    pub x: u16,
    pub y: u16,
}

impl EadkPoint {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Pushes a slice of colors onto the screen.
///
/// It is your responsibility to ensure that the rect and the slice's length match.
///
/// The screen is filled from left to right then top to bottom.
pub fn push_rect(rect: EadkRect, pixels: &[Color]) {
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }
}

/// Draws a rect with the given color.
pub fn push_rect_uniform(rect: EadkRect, color: Color) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

/// Waits for the screen to finish refreshing.
pub fn wait_for_vblank() {
    unsafe {
        eadk_display_wait_for_vblank();
    }
}

// Is the more than minimal wrapping around the unsafe C functions ?
/// Returns the pixels' color in the given rect.
///
/// The screen is read from left to right then top to bottom.
pub fn get_rect(rect: EadkRect) -> Vec<Color> {
    let buffer_size = rect.area();
    let mut pixels = Vec::with_capacity(buffer_size as usize);
    unsafe {
        eadk_display_pull_rect(rect, pixels.as_mut_ptr());
        // We need to manually adjust the vec length, otherwise
        // vec.len() will return 0 even though it is fully filled.
        pixels.set_len(buffer_size as usize);
    }
    pixels
}

pub fn get_pixel(x: u16, y: u16) -> Color {
    get_rect(EadkRect::new_pixel(x, y))[0]
}

pub fn set_pixel(x: u16, y: u16, color: Color) {
    push_rect_uniform(EadkRect::new_pixel(x, y), color);
}

pub fn clear(color: Color) {
    push_rect_uniform(EadkRect::SCREEN, color);
}

/// Draws a str to the screen.
///
/// It is your responsibility to end the str with a null byte.
pub fn draw_raw_string(
    text: *const u8,
    x: u16,
    y: u16,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    let point = EadkPoint::new(x, y);
    unsafe {
        eadk_display_draw_string(text, point, large_font, text_color, background_color);
    }
}

/// Draws a string, ensuring it ends correctly
pub fn draw_string(
    string: &str,
    x: u16,
    y: u16,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    let patched_string = terminate_str(string);
    draw_raw_string(
        patched_string.as_ptr(),
        x,
        y,
        large_font,
        text_color,
        background_color,
    );
}

/// Adds a null byte at the end of an str if needed.
fn terminate_str(s: &str) -> Cow<'_, str> {
    if s.ends_with('\0') {
        Cow::Borrowed(s)
    } else {
        let mut owned = String::with_capacity(s.len() + 1);
        owned.push_str(s);
        owned.push('\0');
        Cow::Owned(owned)
    }
}
