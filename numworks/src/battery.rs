unsafe extern "C" {
    fn eadk_battery_is_charging() -> bool;
    fn eadk_battery_level() -> u8;
    fn eadk_battery_voltage() -> f32;
}

/// Checks if the battery is in charge.
///
/// The link process will fail if the calculator is not compatible.
/// You also may need to increase the EADK_API_LEVEL for this to work.
pub fn is_battery_charging() -> bool {
    unsafe { eadk_battery_is_charging() }
}

/// Returns the battery level.
///
/// The link process will fail if the calculator is not compatible.
/// You also may need to increase the EADK_API_LEVEL for this to work.
pub fn battery_level() -> u8 {
    unsafe { eadk_battery_level() }
}

/// Returns the battery voltage.
///
/// The link process will fail if the calculator is not compatible.
/// You also may need to increase the EADK_API_LEVEL for this to work.
pub fn battery_voltage() -> f32 {
    unsafe { eadk_battery_voltage() }
}
