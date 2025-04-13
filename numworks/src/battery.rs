/// Interface with the raw `eadk` C api.\
/// If you don't know what you are doing, use the safe rust implementations.
pub mod raw_api {
    unsafe extern "C" {
        pub fn eadk_battery_is_charging() -> bool;
        pub fn eadk_battery_level() -> u8;
        pub fn eadk_battery_voltage() -> f32;
    }
}

use raw_api::*;

/// Checks if the battery is in charge.
///
/// The link process will fail if the calculator is not compatible.
/// You may also need to increase the EADK_API_LEVEL for this to work.
pub fn is_charging() -> bool {
    unsafe { eadk_battery_is_charging() }
}

/// Returns the battery level.
///
/// The link process will fail if the calculator is not compatible.
/// You may also need to increase the EADK_API_LEVEL for this to work.
pub fn get_level() -> u8 {
    unsafe { eadk_battery_level() }
}

/// Returns the battery voltage.
///
/// The link process will fail if the calculator is not compatible.
/// You may also need to increase the EADK_API_LEVEL for this to work.
pub fn get_voltage() -> f32 {
    unsafe { eadk_battery_voltage() }
}
