unsafe extern "C" {
    fn eadk_backlight_set_brightness(brightness: u8);
    fn eadk_backlight_brightness() -> u8;
}

/// Sets the screen brightness.
pub fn set_backlight_brightness(brightness: u8) {
    unsafe { eadk_backlight_set_brightness(brightness) }
}

/// Retrieves the screen brightness.
pub fn get_backlight_brightness() -> u8 {
    unsafe { eadk_backlight_brightness() }
}

/// The maximum brightness.
pub const MAX_BRIGHTNESS: u8 = 240;

/// The brightness increment.
///
/// Even though the brightness is stored in a u8, only 16 levels of brightness are available.
pub const INCREMENT: u8 = 16;
