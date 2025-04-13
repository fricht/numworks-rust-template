#![no_std]

use libnw::display::{self, CHAR_HEIGHT, Color, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

/// How to handle the end of the main app.
pub enum ExitBehaviour {
    Exit,
    Hang,
    Restart,
}

/// The core of the application logic
pub fn main() -> ExitBehaviour {
    const MESSAGE: &str = "Hey !!";
    display::clear(Color::GREEN);
    display::draw_string(
        MESSAGE,
        (SCREEN_WIDTH - (MESSAGE.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
        (SCREEN_HEIGHT - CHAR_HEIGHT) / 2,
        true,
        Color::BLACK,
        Color::WHITE,
    );
    ExitBehaviour::Hang
}
