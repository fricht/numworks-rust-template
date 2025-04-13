#![no_std]

#[used]
#[unsafe(link_section = ".rodata.eadk_api_level")]
static EADK_API_LEVEL: u32 = 0;

mod heap;

/// Interface with the usb state.
pub mod usb;

/// Interface with the battery.
pub mod battery;

/// Interface with the display.
///
/// Contains various utilities for common operations.
pub mod display;

/// Get random values.
pub mod random;

/// Day 13, you are now able to control the time.
pub mod time;

/// Interface with the backlight of the display.
///
/// Even thought the functions use and return u8, there are only 15 different
/// brightness values.
pub mod backlight;

/// Interfaces with the keyboard, retrieve raw state and wait for keypress.
pub mod keyboard;

/// Access the external data of the app.
pub mod external;
