use crate::eadk;

pub use eadk::backlight_brightness as get_brightness;

pub use eadk::backlight_set_brightness as set_brightness;

/// The maximum brightness.
pub const MAX_BRIGHTNESS: u8 = 240;

/// The brightness increment.
pub const INCREMENT: u8 = 16;