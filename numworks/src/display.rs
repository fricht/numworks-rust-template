extern crate alloc;

use crate::eadk;
pub use crate::shared::{EadkColor as Color, EadkRect as ScreenRect};
use alloc::{borrow::Cow, string::String};

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

pub use eadk::push_rect_uniform;

pub use eadk::push_rect;

pub use eadk::wait_for_vblank;

pub use eadk::pull_rect as get_rect;

pub fn get_pixel(x: u16, y: u16) -> Color {
    get_rect(ScreenRect::new_pixel(x, y))[0]
}

pub fn set_pixel(x: u16, y: u16, color: Color) {
    push_rect_uniform(ScreenRect::new_pixel(x, y), color);
}

pub fn clear(color: Color) {
    push_rect_uniform(ScreenRect::SCREEN, color);
}

/// Draws a string with the normal font.
pub fn draw_string(string: &str, x: u16, y: u16, text_color: Color, background_color: Color) {
    let patched_string = terminate_str(string);
    eadk::draw_string(
        patched_string.as_ptr(),
        x,
        y,
        false,
        text_color,
        background_color,
    );
}

/// Draws a string with the large font.
pub fn draw_string_large(string: &str, x: u16, y: u16, text_color: Color, background_color: Color) {
    let patched_string = terminate_str(string);
    eadk::draw_string(
        patched_string.as_ptr(),
        x,
        y,
        true,
        text_color,
        background_color,
    );
}

/// Draws a string, letting you choose the font.
pub fn draw_string_custom(
    string: &str,
    x: u16,
    y: u16,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    let patched_string = terminate_str(string);
    eadk::draw_string(
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
