#![no_std]

use numworks::display::{
    self, CHAR_HEIGHT, EadkColor, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

/// The core of the application logic
pub fn main() {
    const MESSAGE: &str = "Hey !!";
    display::clear(EadkColor::GREEN);
    display::draw_string(
        MESSAGE,
        (SCREEN_WIDTH - (MESSAGE.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
        (SCREEN_HEIGHT - CHAR_HEIGHT) / 2,
        true,
        EadkColor::BLACK,
        EadkColor::WHITE,
    );
}
