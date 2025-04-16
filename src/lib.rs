#![no_std]

extern crate alloc;

use alloc::format;
use libnw::display::{self, Color};
use libnw::storage;

/// How to handle the end of the main app.
pub enum ExitBehaviour {
    Exit,
    Hang,
    Restart,
}

/// The core of the application logic
pub fn main() -> ExitBehaviour {
    display::clear(Color::GREEN);
    display::draw_string(
        &format!("Addr : {}", storage::extapp_address()),
        0,
        0,
        false,
        Color::BLACK,
        Color::WHITE,
    );
    display::draw_string(
        &format!("U-Addr : {}", storage::extapp_userland_address() as u32),
        0,
        20,
        false,
        Color::BLACK,
        Color::WHITE,
    );
    display::draw_string(
        &format!("Size : {}", storage::extapp_size()),
        0,
        40,
        false,
        Color::BLACK,
        Color::WHITE,
    );

    ExitBehaviour::Hang
}
