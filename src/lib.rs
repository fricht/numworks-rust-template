#![no_std]

extern crate alloc;

use numworks::display::{self, CHAR_HEIGHT, Color, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

/// The core of the application logic
pub fn main() {
    const MESSAGE: &str = "Hey !!";
    display::clear(Color::GREEN);
    display::draw_string_large(
        MESSAGE,
        (SCREEN_WIDTH - (MESSAGE.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
        (SCREEN_HEIGHT - CHAR_HEIGHT) / 2,
        Color::BLACK,
        Color::WHITE,
    );
}
