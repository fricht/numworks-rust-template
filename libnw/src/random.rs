//! Provides utility functions for generating random values such as integers, colors, and rectangles.

use crate::display::{Color, Rect, SCREEN_HEIGHT, SCREEN_WIDTH};

pub use eadk::random;

/// Returns a random unsigned integer in [a, b[.
pub fn randuint(a: u32, b: u32) -> u32 {
    let range = b - a;
    let random_number = eadk::random() % range;
    random_number + a
}

/// Returns a random color
pub fn random_color() -> Color {
    Color::from_rgb(
        eadk::random() as u8,
        eadk::random() as u8,
        eadk::random() as u8,
    )
}

/// Returns a random rect.
/// The rect is fully contained within the screen.
pub fn random_rect() -> Rect {
    let x = randuint(0, SCREEN_WIDTH as u32) as u16;
    let y = randuint(0, SCREEN_HEIGHT as u32) as u16;
    let width = randuint(0, (SCREEN_WIDTH - x) as u32) as u16;
    let height = randuint(0, (SCREEN_HEIGHT - y) as u32) as u16;
    Rect::new(x, y, width, height)
}

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Returns a random u32.
        #[link_name = "eadk_random"]
        pub safe fn random() -> u32;
    }
}
