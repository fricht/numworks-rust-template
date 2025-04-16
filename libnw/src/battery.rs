//! Interface with the battery
//!
//! Allows checking if the device is charging, as well as reading battery level and voltage.

pub use eadk::*;

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Checks whether the battery is currently charging.
        ///
        /// # Link
        /// This function may fail to link if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_battery_is_charging"]
        pub safe fn is_charging() -> bool;

        // TODO: specify how the result should be interpreted (percentage, 0-100 or 0-1 ?).
        /// Returns the battery level.
        ///
        /// # Link
        /// This function may fail to link if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_battery_level"]
        pub safe fn get_level() -> u8;

        // TODO: specify how the result should be interpreted.
        /// Returns the battery voltage.
        ///
        /// # Link
        /// This function may fail to link if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_battery_voltage"]
        pub safe fn get_voltage() -> f32;
    }
}
