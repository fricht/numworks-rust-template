pub use eadk::*;

/// The maximum brightness.
pub const MAX_BRIGHTNESS: u8 = 240;

/// The brightness increment.
///
/// Even though the brightness is stored in a u8, only 16 levels of brightness are available.
pub const BRIGHTNESS_INCREMENT: u8 = 16;

/// Increments the brightness by one level (increments the value by `BRIGHTNESS_INCREMENT`).
///
/// Returns the new brightness value on success, and returns Err if the
/// brightness was already at the maximum level.
pub fn increment() -> Result<u8, ()> {
    let current_brightness = eadk::get_brightness();
    if current_brightness != MAX_BRIGHTNESS {
        let new_brightness = current_brightness + BRIGHTNESS_INCREMENT;
        eadk::set_brightness(new_brightness);
        Ok(new_brightness)
    } else {
        Err(())
    }
}

/// Decrements the brightness by one level (decrements the value by `BRIGHTNESS_INCREMENT`).
///
/// Returns the new brightness value on success, and returns Err if the
/// brightness was already at the minimum level.
pub fn decrement() -> Result<u8, ()> {
    let current_brightness = eadk::get_brightness();
    if current_brightness != 0 {
        let new_brightness = current_brightness - BRIGHTNESS_INCREMENT;
        eadk::set_brightness(new_brightness);
        Ok(new_brightness)
    } else {
        Err(())
    }
}

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Sets the screen brightness.
        #[link_name = "eadk_backlight_set_brightness"]
        pub safe fn set_brightness(brightness: u8);

        /// Retrieves the screen brightness.
        #[link_name = "eadk_backlight_brightness"]
        pub safe fn get_brightness() -> u8;
    }
}
