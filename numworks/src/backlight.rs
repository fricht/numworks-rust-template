/// Interface with the raw `eadk` C api.\
/// If you don't know what you are doing, use the safe rust implementations.
pub mod raw_api {
    unsafe extern "C" {
        pub fn eadk_backlight_set_brightness(brightness: u8);
        pub fn eadk_backlight_brightness() -> u8;
    }
}

use raw_api::*;

/// Sets the screen brightness.
pub fn set_brightness(brightness: u8) {
    unsafe { eadk_backlight_set_brightness(brightness) }
}

/// Retrieves the screen brightness.
pub fn get_brightness() -> u8 {
    unsafe { eadk_backlight_brightness() }
}

/// The maximum brightness.
pub const MAX_BRIGHTNESS: u8 = 240;

/// The brightness increment.
///
/// Even though the brightness is stored in a u8, only 16 levels of brightness are available.
pub const BRIGHTNESS_INCREMENT: u8 = 16;
